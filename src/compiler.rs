use crate::{
    parse_file,
    parser::{AtomKind, Location, SExpr},
    type_checker::{get_type, get_types},
    wasi_io::{fd_open, fd_read_all},
    wasm_module::{
        WasmBinaryOpKind, WasmData, WasmExport, WasmExportType, WasmExpr, WasmFn, WasmFnType,
        WasmGlobal, WasmGlobalKind, WasmImport, WasmImportDesc, WasmInstr, WasmLimits,
        WasmLoadKind, WasmLocals, WasmModule, WasmStoreKind, WasmValueType,
    },
};
use alloc::{
    boxed::Box,
    collections::{BTreeMap, BTreeSet},
    format,
    rc::Rc,
    str,
    string::String,
    vec,
    vec::Vec,
};
use core::cell::RefCell;

pub struct FnBody {
    pub fn_index: u32,
    pub locals: RefCell<BTreeMap<String, LocalDef>>,
    pub locals_last_index: u32,
    pub body: Vec<SExpr>,
}

pub struct StructDef {
    pub fields: Vec<StructField>,
}

pub struct StructField {
    pub name: String,
    pub value_type: WasmValueType,
}

pub enum LoleValueType {
    Primitive(WasmValueType),
    StructInstance { name: String },
}

#[derive(Default)]
pub struct ModuleContext {
    pub included_modules: BTreeSet<String>,
    pub wasm_module: WasmModule,
    pub fn_defs: BTreeMap<String, FnDef>,
    pub fn_bodies: BTreeMap<String, FnBody>,
    pub fn_exports: BTreeMap<String, String>,
    pub memory_names: Vec<String>,
    pub struct_defs: BTreeMap<String, StructDef>,
    pub enum_kinds: BTreeMap<String, u32>,
    pub globals: BTreeMap<String, GlobalDef>,
    pub imported_fns_count: u32,
    pub data_size: Rc<RefCell<i32>>,
}

pub struct FnDef {
    pub local: bool,
    pub fn_index: u32,
}

impl FnDef {
    pub fn get_absolute_index(&self, ctx: &ModuleContext) -> u32 {
        if self.local {
            self.fn_index + ctx.imported_fns_count
        } else {
            self.fn_index
        }
    }
}

pub struct FnContext<'a> {
    pub module: &'a ModuleContext,
    pub fn_type: &'a WasmFnType,
    pub locals: &'a mut BTreeMap<String, LocalDef>,
    pub locals_last_index: u32,
    pub non_arg_locals: Vec<WasmValueType>,
}

pub struct LocalDef {
    pub index: u32,
    pub value_type: LoleValueType,
}

pub struct GlobalDef {
    pub index: u32,
    pub mutable: bool,
}

pub struct CompileError {
    pub message: String,
    pub loc: Location,
}

pub fn compile_module(exprs: Vec<SExpr>) -> Result<WasmModule, CompileError> {
    let mut ctx = ModuleContext::default();

    for expr in exprs {
        compile_top_level_expr(&expr, &mut ctx)?;
    }

    // push function exports
    for (in_name, out_name) in ctx.fn_exports.iter() {
        ctx.wasm_module.exports.push(WasmExport {
            export_type: WasmExportType::Func,
            export_name: out_name.clone(),
            exported_item_index: ctx
                .fn_defs
                .get(in_name)
                .map(|fd| ctx.imported_fns_count + fd.fn_index)
                .ok_or_else(|| CompileError {
                    message: format!("Cannot export unknown function {in_name}"),
                    loc: Location::internal(),
                })?,
        });
    }

    let mut fn_bodies_sorted = ctx.fn_bodies.values().collect::<Vec<_>>();
    fn_bodies_sorted.sort_by_key(|fd| fd.fn_index);

    // push function codes
    for fn_body in fn_bodies_sorted {
        let type_index = ctx
            .wasm_module
            .functions
            .get(fn_body.fn_index as usize)
            .unwrap();

        let fn_type = ctx.wasm_module.types.get(*type_index as usize).unwrap();

        let mut fn_ctx = FnContext {
            module: &ctx,
            fn_type,
            locals: &mut fn_body.locals.borrow_mut(),
            locals_last_index: fn_body.locals_last_index,
            non_arg_locals: vec![],
        };

        let instrs = parse_instrs(&fn_body.body, &mut fn_ctx)?;
        for (instr, i) in instrs.iter().zip(0..) {
            let types = get_type(&fn_ctx, instr)?;

            if types.len() > 0 {
                return Err(CompileError {
                    message: format!("TypeError: Excess values"),
                    loc: fn_body.body[i].loc().clone(),
                });
            }
        }

        let mut locals = vec![];
        for local_type in fn_ctx.non_arg_locals {
            locals.push(WasmLocals {
                count: 1,
                value_type: local_type.clone(),
            });
        }

        ctx.wasm_module.codes.push(WasmFn {
            locals,
            expr: WasmExpr { instrs },
        });
    }

    Ok(ctx.wasm_module)
}

fn compile_top_level_expr(expr: &SExpr, ctx: &mut ModuleContext) -> Result<(), CompileError> {
    let SExpr::List { value: items, .. } = expr else {
        return Err(CompileError {
            message: format!("Unexpected atom"),
            loc: expr.loc().clone()
        });
    };

    let [SExpr::Atom { value: op, loc: op_loc, kind }, other @ ..] = &items[..] else {
        return Err(CompileError {
            message: format!("Expected operation, got a simple list"),
            loc: expr.loc().clone()
        });
    };

    if *kind != AtomKind::Symbol {
        return Err(CompileError {
            message: format!("Expected operation, got a string"),
            loc: expr.loc().clone(),
        });
    }

    match op.as_str() {
        "mod" => match other {
            [SExpr::Atom {
                value: mod_name,
                loc: _,
                kind: _,
            }] => {
                if !ctx.included_modules.insert(mod_name.clone()) {
                    // do not include module twice
                    return Ok(());
                };

                let file_name = format!("{}.lole", mod_name);
                let mod_fd = fd_open(&file_name).map_err(|err| CompileError {
                    message: format!("Cannot load file {file_name}: {err}"),
                    loc: op_loc.clone(),
                })?;

                let source_buf = fd_read_all(mod_fd);
                let source = str::from_utf8(source_buf.as_slice()).unwrap();

                let exprs = parse_file(&file_name, source)?;

                for expr in exprs {
                    compile_top_level_expr(&expr, ctx)?;
                }
            }
            _ => {
                return Err(CompileError {
                    message: format!("Invalid arguments for {op}"),
                    loc: op_loc.clone(),
                });
            }
        },
        "mem" => match other {
            [SExpr::Atom {
                value: mem_name,
                loc: mem_name_loc,
                kind: AtomKind::Symbol,
            }, SExpr::Atom {
                value: min_literal,
                loc: _,
                kind: AtomKind::Symbol,
            }, SExpr::Atom {
                value: min_memory,
                loc: min_memory_loc,
                kind: AtomKind::Symbol,
            }, optional @ ..]
                if min_literal == ":min" =>
            {
                if ctx.memory_names.contains(mem_name) {
                    return Err(CompileError {
                        message: format!("Duplicate memory definition: {mem_name}"),
                        loc: mem_name_loc.clone(),
                    });
                }

                let min_memory = min_memory.parse().map_err(|_| CompileError {
                    message: format!("Parsing {op} :min (u32) failed"),
                    loc: min_memory_loc.clone(),
                })?;

                let max_memory = match optional {
                    [SExpr::Atom {
                        value: max_literal,
                        loc: _,
                        kind: AtomKind::Symbol,
                    }, SExpr::Atom {
                        value: max_memory,
                        loc: max_memory_loc,
                        kind: AtomKind::Symbol,
                    }] if max_literal == ":max" => {
                        Some(max_memory.parse::<u32>().map_err(|_| CompileError {
                            message: format!("Parsing {op} :max (u32) failed"),
                            loc: max_memory_loc.clone(),
                        })?)
                    }
                    [] => None,
                    _ => {
                        return Err(CompileError {
                            message: format!("Invalid arguments for {op}"),
                            loc: op_loc.clone(),
                        })
                    }
                };

                ctx.memory_names.push(mem_name.clone());
                ctx.wasm_module.memories.push(WasmLimits {
                    min: min_memory,
                    max: max_memory,
                });
            }
            _ => {
                return Err(CompileError {
                    message: format!("Invalid arguments for {op}"),
                    loc: op_loc.clone(),
                })
            }
        },
        "fn" => match other {
            [SExpr::Atom {
                value: fn_name,
                loc: fn_name_loc,
                kind: AtomKind::Symbol,
            }, SExpr::List {
                value: input_exprs, ..
            }, SExpr::List {
                value: output_exprs,
                ..
            }, SExpr::List { value: body, .. }] => {
                if ctx.fn_defs.contains_key(fn_name) {
                    return Err(CompileError {
                        message: format!("Cannot redefine function: {fn_name}"),
                        loc: fn_name_loc.clone(),
                    });
                }

                let mut locals = BTreeMap::new();
                let mut locals_last_index = 0;

                let mut inputs = vec![];
                for input_expr in input_exprs.iter() {
                    let SExpr::List{ value: name_and_type, .. } = input_expr else {
                        return Err(CompileError {
                            message: format!("Unexpected atom in function params list"),
                            loc: input_expr.loc().clone()
                        });
                    };

                    let [SExpr::Atom {
                        value: p_name, loc: p_name_loc, kind: AtomKind::Symbol,
                    }, SExpr::Atom {
                        value: p_type, loc: p_type_loc, kind: AtomKind::Symbol,
                    }] = &name_and_type[..] else {
                        return Err(CompileError {
                            message: format!("Expected name and parameter pairs in function params list"),
                            loc: input_expr.loc().clone()
                        });
                    };

                    if locals.contains_key(p_name) {
                        return Err(CompileError {
                            message: format!(
                                "Found function param with conflicting name: {p_name}"
                            ),
                            loc: p_name_loc.clone(),
                        });
                    }

                    let value_type =
                        LoleValueType::parse(p_type, &ctx).map_err(|e| CompileError {
                            message: e,
                            loc: p_type_loc.clone(),
                        })?;
                    let comp_count = emit_value_components(&value_type, &ctx, &mut inputs);

                    locals.insert(
                        p_name.clone(),
                        LocalDef {
                            index: locals_last_index,
                            value_type,
                        },
                    );

                    locals_last_index += comp_count;
                }

                let mut outputs = vec![];
                for output_expr in output_exprs {
                    let l_type = match output_expr {
                        SExpr::Atom {
                            value: atom_text, ..
                        } => atom_text,
                        _ => {
                            return Err(CompileError {
                                message: format!("Atom expected, list found"),
                                loc: output_expr.loc().clone(),
                            })
                        }
                    };

                    let value_type =
                        LoleValueType::parse(l_type, &ctx).map_err(|e| CompileError {
                            message: e,
                            loc: output_expr.loc().clone(),
                        })?;
                    emit_value_components(&value_type, &ctx, &mut outputs);
                }

                ctx.wasm_module.types.push(WasmFnType { inputs, outputs });
                let type_index = ctx.wasm_module.types.len() as u32 - 1;
                let fn_index = ctx.wasm_module.functions.len() as u32;

                ctx.wasm_module.functions.push(type_index);
                ctx.fn_defs.insert(
                    fn_name.clone(),
                    FnDef {
                        local: true,
                        fn_index,
                    },
                );
                ctx.fn_bodies.insert(
                    fn_name.clone(),
                    FnBody {
                        fn_index,
                        locals: RefCell::new(locals),
                        locals_last_index,
                        body: body.clone(),
                    },
                );
            }
            _ => {
                return Err(CompileError {
                    message: format!("Invalid arguments for {op}"),
                    loc: op_loc.clone(),
                })
            }
        },
        "import" => match other {
            [SExpr::Atom {
                value: fn_literal, ..
            }, SExpr::Atom {
                value: fn_name,
                loc: fn_name_loc,
                kind: AtomKind::Symbol,
            }, SExpr::List {
                value: input_exprs, ..
            }, SExpr::List {
                value: output_exprs,
                loc: _,
            }, SExpr::Atom {
                value: from_literal,
                loc: _,
                kind: AtomKind::Symbol,
            }, SExpr::Atom {
                value: module_name,
                loc: _,
                kind: _,
            }, SExpr::Atom {
                value: extern_fn_name,
                loc: _,
                kind: _,
            }] if fn_literal == "fn" && from_literal == ":from" => {
                if ctx.fn_defs.contains_key(fn_name) {
                    return Err(CompileError {
                        message: format!("Cannot redefine function: {fn_name}"),
                        loc: fn_name_loc.clone(),
                    });
                }

                let mut param_names = BTreeSet::new();

                let mut inputs = vec![];
                for input_expr in input_exprs.iter() {
                    let SExpr::List { value: name_and_type, .. } = input_expr else {
                        return Err(CompileError {
                            message: format!("Unexpected atom in function params list"),
                            loc: input_expr.loc().clone()
                        });
                    };

                    let [SExpr::Atom {
                        value: p_name, loc: name_loc,kind: AtomKind::Symbol,
                    }, SExpr::Atom {
                        value: p_type, loc: type_loc, kind: AtomKind::Symbol,
                    }] = &name_and_type[..] else {
                        return Err(CompileError {
                            message: format!("Expected name and parameter pairs in function params list"),
                            loc: input_expr.loc().clone()
                        });
                    };

                    if param_names.contains(p_name) {
                        return Err(CompileError {
                            message: format!(
                                "Found function param with conflicting name: {p_name}"
                            ),
                            loc: name_loc.clone(),
                        });
                    }

                    let value_type =
                        LoleValueType::parse(p_type, &ctx).map_err(|e| CompileError {
                            message: e,
                            loc: type_loc.clone(),
                        })?;
                    emit_value_components(&value_type, &ctx, &mut inputs);

                    param_names.insert(p_name.clone());
                }

                let mut outputs = vec![];
                for output_expr in output_exprs {
                    let l_type = match output_expr {
                        SExpr::Atom {
                            value: atom_text, ..
                        } => atom_text,
                        _ => {
                            return Err(CompileError {
                                message: format!("Atom expected, list found"),
                                loc: output_expr.loc().clone(),
                            })
                        }
                    };

                    let value_type =
                        LoleValueType::parse(l_type, &ctx).map_err(|e| CompileError {
                            message: e,
                            loc: output_expr.loc().clone(),
                        })?;
                    emit_value_components(&value_type, &ctx, &mut outputs);
                }

                let type_index = ctx.wasm_module.types.len() as u32;
                let fn_index = ctx.imported_fns_count;

                ctx.imported_fns_count += 1;
                ctx.fn_defs.insert(
                    fn_name.clone(),
                    FnDef {
                        local: false,
                        fn_index,
                    },
                );
                ctx.wasm_module.types.push(WasmFnType { inputs, outputs });
                ctx.wasm_module.imports.push(WasmImport {
                    module_name: module_name.clone(),
                    item_name: extern_fn_name.clone(),
                    item_desc: WasmImportDesc::Func { type_index },
                });
            }
            _ => {
                return Err(CompileError {
                    message: format!("Invalid arguments for {op}"),
                    loc: op_loc.clone(),
                })
            }
        },
        "export" => match other {
            [SExpr::Atom {
                value: mem_literal,
                loc: _,
                kind: AtomKind::Symbol,
            }, SExpr::Atom {
                value: in_name,
                loc: name_loc,
                kind: AtomKind::Symbol,
            }, SExpr::Atom {
                value: as_literal, ..
            }, SExpr::Atom {
                value: out_name,
                loc: _,
                kind: _,
            }] if mem_literal == "mem" && as_literal == ":as" => {
                if !ctx.memory_names.contains(in_name) {
                    return Err(CompileError {
                        message: format!("Cannot export mem {in_name}, not found"),
                        loc: name_loc.clone(),
                    });
                }

                ctx.wasm_module.exports.push(WasmExport {
                    export_type: WasmExportType::Mem,
                    export_name: out_name.clone(),
                    exported_item_index: ctx.memory_names.iter().position(|n| n == in_name).unwrap()
                        as u32,
                });
            }
            [SExpr::Atom { value: in_name, .. }, SExpr::Atom {
                value: as_literal, ..
            }, SExpr::Atom {
                value: out_name, ..
            }] if as_literal == ":as" => {
                ctx.fn_exports.insert(in_name.clone(), out_name.clone());
            }
            _ => {
                return Err(CompileError {
                    message: format!("Invalid arguments for {op}"),
                    loc: op_loc.clone(),
                })
            }
        },
        "struct" => match other {
            [SExpr::Atom {
                value: struct_name,
                loc: name_loc,
                kind: AtomKind::Symbol,
            }, field_defs @ ..] => {
                if ctx.struct_defs.contains_key(struct_name) {
                    return Err(CompileError {
                        message: format!("Cannot redefine struct {struct_name}"),
                        loc: name_loc.clone(),
                    });
                }

                let fields = parse_struct_field_defs(field_defs, struct_name)?;

                ctx.struct_defs
                    .insert(struct_name.clone(), StructDef { fields });
            }
            _ => {
                return Err(CompileError {
                    message: format!("Invalid arguments for {op}"),
                    loc: op_loc.clone(),
                })
            }
        },
        "enum" => match other {
            [SExpr::Atom {
                value: enum_name,
                loc: name_loc,
                kind: AtomKind::Symbol,
            }, variants @ ..] => {
                for (kind, variant) in variants.iter().enumerate() {
                    let SExpr::List{ value: variant_body, .. } = variant else {
                        return Err(CompileError { message: format!("Invalid arguments for {op}"), loc: op_loc.clone() });
                    };

                    let [SExpr::Atom { value: struct_name, .. }, field_defs @ ..] = &variant_body[..] else {
                        return Err(CompileError { message: format!("Invalid arguments for {op}"), loc: op_loc.clone() });
                    };

                    let full_name = format!("{enum_name}/{struct_name}");
                    if ctx.struct_defs.contains_key(struct_name) {
                        return Err(CompileError {
                            message: format!("Cannot redefine struct {full_name}"),
                            loc: name_loc.clone(),
                        });
                    }

                    let fields = parse_struct_field_defs(field_defs, &full_name)?;

                    ctx.struct_defs
                        .insert(full_name.clone(), StructDef { fields });

                    ctx.enum_kinds.insert(full_name, kind as u32);
                }

                ctx.struct_defs.insert(
                    enum_name.clone(),
                    StructDef {
                        fields: vec![
                            StructField {
                                name: String::from("kind"),
                                value_type: WasmValueType::I32,
                            },
                            StructField {
                                name: String::from("ref"),
                                value_type: WasmValueType::I32,
                            },
                        ],
                    },
                );
            }
            _ => {
                return Err(CompileError {
                    message: format!("Invalid arguments for {op}"),
                    loc: op_loc.clone(),
                })
            }
        },
        "global" => {
            let (mutable, global_name, global_type, global_value, name_loc, type_loc) = match other
            {
                [SExpr::Atom {
                    value: mutable_literal,
                    loc: _,
                    kind: AtomKind::Symbol,
                }, SExpr::Atom {
                    value: global_name,
                    loc: name_loc,
                    kind: AtomKind::Symbol,
                }, SExpr::Atom {
                    value: global_type,
                    loc: type_loc,
                    kind: AtomKind::Symbol,
                }, global_value]
                    if mutable_literal == "mut" =>
                {
                    (
                        true,
                        global_name,
                        global_type,
                        global_value,
                        name_loc,
                        type_loc,
                    )
                }
                [SExpr::Atom {
                    value: global_name,
                    loc: name_loc,
                    kind: AtomKind::Symbol,
                }, SExpr::Atom {
                    value: global_type,
                    loc: type_loc,
                    kind: AtomKind::Symbol,
                }, global_value] => (
                    false,
                    global_name,
                    global_type,
                    global_value,
                    name_loc,
                    type_loc,
                ),
                _ => {
                    return Err(CompileError {
                        message: format!("Invalid arguments for {op}"),
                        loc: op_loc.clone(),
                    })
                }
            };

            if ctx.globals.contains_key(global_name) {
                return Err(CompileError {
                    message: format!("Cannot redefine global: {global_name}"),
                    loc: name_loc.clone(),
                });
            }

            let value_type = WasmValueType::parse(global_type).map_err(|e| CompileError {
                message: e,
                loc: type_loc.clone(),
            })?;
            let initial_value = WasmExpr {
                instrs: vec![parse_const_instr(global_value, &ctx)?],
            };

            ctx.globals.insert(
                global_name.clone(),
                GlobalDef {
                    index: ctx.globals.len() as u32,
                    mutable,
                },
            );

            ctx.wasm_module.globals.push(WasmGlobal {
                kind: WasmGlobalKind {
                    value_type,
                    mutable,
                },
                initial_value,
            });
        }
        "data" => {
            let [
                SExpr::Atom { value: offset, loc, kind: AtomKind::Symbol, },
                SExpr::Atom { value: data_base64, loc: _, kind }
            ] = other else {
                return Err(CompileError {
                    message: format!("Invalid arguments for {op}"),
                    loc: op_loc.clone(),
                })
            };

            let offset = offset.parse().map_err(|_| CompileError {
                message: format!("Parsing i32 (implicit) failed"),
                loc: loc.clone(),
            })?;

            let bytes = match *kind {
                AtomKind::String => data_base64.as_bytes().iter().map(|b| *b).collect(),
                AtomKind::Symbol => base64_decode(data_base64.as_bytes()),
            };

            ctx.wasm_module.datas.borrow_mut().push(WasmData::Active {
                offset: WasmExpr {
                    instrs: vec![WasmInstr::I32Const { value: offset }],
                },
                bytes,
            });
        }
        _ => {
            return Err(CompileError {
                message: format!("Unknown operation: {op}"),
                loc: op_loc.clone(),
            })
        }
    }

    Ok(())
}

fn parse_struct_field_defs(
    exprs: &[SExpr],
    struct_name: &str,
) -> Result<Vec<StructField>, CompileError> {
    let mut fields = Vec::<StructField>::new();
    for field_def in exprs {
        let SExpr::List { value: name_and_type, .. } = field_def else {
            return Err(CompileError {
                message: format!("Unexpected atom in fields list of struct {struct_name}"),
                loc: field_def.loc().clone()
            });
        };

        let [SExpr::Atom {
            value: f_name, loc: name_loc, kind: AtomKind::Symbol,
        }, SExpr::Atom {
            value: f_type, loc: type_loc, kind: AtomKind::Symbol,
        }] = &name_and_type[..] else {
            return Err(CompileError{
                message: format!("Expected name and parameter pairs in fields list of struct {struct_name}"),
                loc: field_def.loc().clone(),
            });
        };

        if fields.iter().find(|f| &f.name == f_name).is_some() {
            return Err(CompileError {
                message: format!(
                    "Found duplicate struct field name: '{f_name}' of struct {struct_name}"
                ),
                loc: name_loc.clone(),
            });
        }

        fields.push(StructField {
            name: f_name.clone(),
            value_type: WasmValueType::parse(f_type).map_err(|e| CompileError {
                message: e,
                loc: type_loc.clone(),
            })?,
        });
    }
    Ok(fields)
}

pub fn emit_value_components(
    value_type: &LoleValueType,
    ctx: &ModuleContext,
    components: &mut Vec<WasmValueType>,
) -> u32 {
    match value_type {
        LoleValueType::Primitive(value_type) => {
            components.push(*value_type);
            1
        }
        LoleValueType::StructInstance { name } => {
            let fields = &ctx.struct_defs.get(name).unwrap().fields;

            for field in fields {
                components.push(field.value_type);
            }

            fields.len() as u32
        }
    }
}

fn parse_instr(expr: &SExpr, ctx: &mut FnContext) -> Result<WasmInstr, CompileError> {
    let items = match expr {
        SExpr::List { value: items, .. } => items,
        SExpr::Atom { value, loc, kind } => {
            if *kind == AtomKind::String {
                let string_ptr = *ctx.module.data_size.borrow();
                let string_len = value.as_bytes().len() as i32;

                // TODO(optimize): use string pooling
                *ctx.module.data_size.borrow_mut() += string_len;
                ctx.module
                    .wasm_module
                    .datas
                    .borrow_mut()
                    .push(WasmData::Active {
                        offset: WasmExpr {
                            instrs: vec![WasmInstr::I32Const { value: string_ptr }],
                        },
                        bytes: value.as_bytes().to_vec(),
                    });

                return Ok(WasmInstr::MultiValueEmit {
                    values: vec![
                        WasmInstr::I32Const { value: string_ptr },
                        WasmInstr::I32Const { value: string_len },
                    ],
                });
            }

            if value.chars().all(|c| c.is_ascii_digit()) {
                return Ok(WasmInstr::I32Const {
                    value: (value.parse().map_err(|_| CompileError {
                        message: format!("Parsing i32 (implicit) failed"),
                        loc: loc.clone(),
                    })?),
                });
            }

            if let Some(global) = ctx.module.globals.get(value.as_str()) {
                return Ok(WasmInstr::GlobalGet {
                    global_index: global.index,
                });
            };

            let Some(local) = ctx.locals.get(value.as_str()) else {
                return Err(CompileError {
                    message: format!("Reading unknown variable: {value}"),
                    loc: loc.clone()
                });
            };

            if let LoleValueType::StructInstance { name } = &local.value_type {
                let struct_def = ctx.module.struct_defs.get(name).unwrap();

                let mut values = vec![];

                for field_offset in 0..struct_def.fields.len() {
                    values.push(WasmInstr::LocalGet {
                        local_index: local.index + field_offset as u32,
                    });
                }

                return Ok(WasmInstr::MultiValueEmit { values });
            }

            return Ok(WasmInstr::LocalGet {
                local_index: local.index,
            });
        }
    };

    let [SExpr::Atom { value: op, loc: op_loc, .. }, args @ ..] = &items[..] else {
        return Err(CompileError {
            message: format!("Expected operation, got a simple list"),
            loc: expr.loc().clone()
        });
    };

    let instr = match (op.as_str(), &args[..]) {
        ("unreachable", []) => WasmInstr::Unreachable {},
        (
            "i32",
            [SExpr::Atom {
                value,
                loc,
                kind: AtomKind::Symbol,
            }],
        ) => WasmInstr::I32Const {
            value: value.parse().map_err(|_| CompileError {
                message: format!("Parsing i32 failed"),
                loc: loc.clone(),
            })?,
        },
        ("data.size", []) => WasmInstr::I32ConstLazy {
            value: ctx.module.data_size.clone(),
        },
        (
            "i64",
            [SExpr::Atom {
                value,
                loc,
                kind: AtomKind::Symbol,
            }],
        ) => WasmInstr::I64Const {
            // TODO(3rd-party-bug): figure out why I can't use parse::<i64>
            value: value.parse::<i32>().map_err(|_| CompileError {
                message: format!("Parsing i64 failed"),
                loc: loc.clone(),
            })? as i64,
        },
        ("i32.eq" | "==", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Equals,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        ("i32.ne" | "!=", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32NotEqual,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        ("i32.not" | "not" | "!", [lhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Equals,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(WasmInstr::I32Const { value: 0 }),
        },
        ("i32.lt_s" | "<", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32LessThenSigned,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        ("i32.gt_s" | ">", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32GreaterThenSigned,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        ("i32.ge_s" | ">=", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32GreaterEqualSigned,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        ("i32.and" | "&&", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32And,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        ("i32.or" | "||", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Or,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        ("i32.add" | "+", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Add,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        ("i32.sub" | "-", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Sub,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        ("i32.mul" | "*", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Mul,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        // TODO: should default `div` and `rem` be unsigned?
        ("i32.div" | "/", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32DivUnsigned,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        ("i32.rem" | "%", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32RemUnsigned,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
        },
        (
            "if",
            [SExpr::Atom {
                value: block_type,
                loc: type_loc,
                kind: AtomKind::Symbol,
            }, cond, then_branch, else_branch],
        ) => WasmInstr::If {
            block_type: WasmValueType::parse(block_type).map_err(|e| CompileError {
                message: e,
                loc: type_loc.clone(),
            })?,
            cond: Box::new(parse_instr(cond, ctx)?),
            then_branch: Box::new(parse_instr(then_branch, ctx)?),
            else_branch: Box::new(parse_instr(else_branch, ctx)?),
        },
        ("if", [cond, then_branch]) => WasmInstr::IfSingleBranch {
            cond: Box::new(parse_instr(cond, ctx)?),
            then_branch: Box::new(parse_instr(then_branch, ctx)?),
        },
        ("loop", [SExpr::List { value: exprs, .. }]) => WasmInstr::Loop {
            instrs: parse_instrs(exprs, ctx)?,
            loc: op_loc.clone(),
        },
        ("break", []) => WasmInstr::LoopBreak {},
        ("continue", []) => WasmInstr::LoopContinue {},
        ("return", values) => WasmInstr::Return {
            value: Box::new(WasmInstr::MultiValueEmit {
                values: parse_instrs(values, ctx)?,
            }),
            loc: op_loc.clone(),
        },
        // TODO(feat): support custom aligns and offsets
        (
            "store",
            [SExpr::Atom {
                value: store_kind,
                loc: kind_loc,
                kind: AtomKind::Symbol,
            }, address_expr, value_expr],
        ) => {
            let Some(struct_def) = ctx.module.struct_defs.get(store_kind) else {
                return Ok(WasmInstr::Store {
                    kind: WasmStoreKind::parse(store_kind).map_err(|e| CompileError {
                        message: e,
                        loc: kind_loc.clone()
                    })?,
                    align: 0,
                    offset: 0,
                    value_instr: Box::new(parse_instr(value_expr, ctx)?),
                    address_instr: Rc::new(parse_instr(address_expr, ctx)?),
                })
            };

            let mut value_instrs = match parse_instr(value_expr, ctx)? {
                WasmInstr::MultiValueEmit { values, .. } => values,
                instr => vec![instr],
            };

            // TODO(perf): check if this is doing duplicate work
            let value_types = get_types(ctx, &value_instrs)?;
            let field_types = struct_def
                .fields
                .iter()
                .map(|f| f.value_type)
                .collect::<Vec<_>>();

            if value_types != field_types {
                return Err(CompileError {
                    message: format!(
                        "TypeError: Invalid types for {op}, needed {:?}, got {:?}",
                        field_types, value_types
                    ),
                    loc: op_loc.clone(),
                });
            }

            let address_instr = Rc::new(parse_instr(address_expr, ctx)?);

            let mut offset = 0;
            let mut instrs = Vec::<WasmInstr>::with_capacity(struct_def.fields.len());

            for field in struct_def.fields.iter() {
                instrs.push(WasmInstr::Store {
                    kind: WasmStoreKind::from_value_type(&field.value_type).map_err(|e| {
                        CompileError {
                            message: e,
                            loc: op_loc.clone(),
                        }
                    })?,
                    align: 1,
                    offset,
                    value_instr: Box::new(value_instrs.remove(0)),
                    address_instr: address_instr.clone(),
                });

                // unwrap uncreachable because of `WasmStoreKind::from_value_type` check
                offset += field.value_type.byte_size().unwrap();
            }

            WasmInstr::MultiValueEmit { values: instrs }
        }
        // TODO(feat): support custom aligns and offsets
        (
            "load",
            [SExpr::Atom {
                value: load_kind,
                loc: kind_loc,
                kind: AtomKind::Symbol,
            }, address_expr],
        ) => {
            let Some(struct_def) = ctx.module.struct_defs.get(load_kind) else {
                return Ok(WasmInstr::Load {
                        kind: WasmLoadKind::parse(load_kind).map_err(|e| CompileError {
                        message: e,
                        loc: kind_loc.clone(),
                    })?,
                    align: 0,
                    offset: 0,
                    address_instr: Rc::new(parse_instr(address_expr, ctx)?),
                })
            };

            let address_instr = Rc::new(parse_instr(address_expr, ctx)?);

            let mut offset = 0;
            let mut primitive_loads = Vec::<WasmInstr>::with_capacity(struct_def.fields.len());

            for field in &struct_def.fields {
                primitive_loads.push(WasmInstr::Load {
                    kind: WasmLoadKind::from_value_type(&field.value_type).map_err(|e| {
                        CompileError {
                            message: e,
                            loc: op_loc.clone(),
                        }
                    })?,
                    align: 1,
                    offset,
                    address_instr: address_instr.clone(),
                });

                // unwrap uncreachable because of `WasmLoadKind::from_value_type` check
                offset += field.value_type.byte_size().unwrap();
            }

            WasmInstr::MultiValueEmit {
                values: primitive_loads,
            }
        }
        (
            "struct.new",
            [SExpr::Atom {
                value: s_name,
                loc: name_loc,
                kind: AtomKind::Symbol,
            }, values @ ..],
        ) => {
            let Some(struct_def) = ctx.module.struct_defs.get(s_name) else {
                return Err(CompileError {
                    message: format!("Unknown struct encountered in {op}: {s_name}"),
                    loc: name_loc.clone(),
                });
            };

            let value_instrs = parse_instrs(values, ctx)?;

            // TODO(perf): check if this is doing duplicate work
            let value_types = get_types(ctx, &value_instrs)?;
            let field_types = struct_def
                .fields
                .iter()
                .map(|f| f.value_type)
                .collect::<Vec<_>>();

            if value_types != field_types {
                return Err(CompileError {
                    message: format!(
                        "TypeError: Invalid types for {op}, needed {:?}, got {:?}",
                        field_types, value_types
                    ),
                    loc: op_loc.clone(),
                });
            }

            WasmInstr::MultiValueEmit {
                values: value_instrs,
            }
        }
        (
            "enum.kind",
            [SExpr::Atom {
                value: enum_variant,
                loc: name_loc,
                kind: AtomKind::Symbol,
            }],
        ) => {
            let Some(kind) = ctx.module.enum_kinds.get(enum_variant) else {
                return Err(CompileError {
                    message: format!("Unknown enum variant in {op}: {enum_variant}"),
                    loc: name_loc.clone()
                });
            };

            WasmInstr::I32Const {
                value: *kind as i32,
            }
        }
        (
            "sizeof",
            [SExpr::Atom {
                value: type_name,
                loc: type_loc,
                kind: AtomKind::Symbol,
            }],
        ) => {
            let value_type =
                LoleValueType::parse(type_name, ctx.module).map_err(|e| CompileError {
                    message: e,
                    loc: type_loc.clone(),
                })?;

            WasmInstr::I32Const {
                value: value_type.byte_size(ctx.module).map_err(|e| CompileError {
                    message: e,
                    loc: type_loc.clone(),
                })? as i32,
            }
        }
        (
            "let" | ":",
            [SExpr::Atom {
                value: local_name,
                loc: name_loc,
                kind: AtomKind::Symbol,
            }, SExpr::Atom {
                value: local_type,
                loc: type_loc,
                kind: AtomKind::Symbol,
            }],
        ) => {
            if let Some(_) = ctx.module.globals.get(local_name.as_str()) {
                return Err(CompileError {
                    message: format!("Local name collides with global: {local_name}"),
                    loc: name_loc.clone(),
                });
            };

            if ctx.locals.contains_key(local_name) {
                return Err(CompileError {
                    message: format!("Duplicate local definition: {local_name}"),
                    loc: name_loc.clone(),
                });
            }

            let value_type =
                LoleValueType::parse(local_type, ctx.module).map_err(|e| CompileError {
                    message: e,
                    loc: type_loc.clone(),
                })?;
            let comp_count =
                emit_value_components(&value_type, &ctx.module, &mut ctx.non_arg_locals);

            ctx.locals.insert(
                local_name.clone(),
                LocalDef {
                    index: ctx.locals_last_index,
                    value_type,
                },
            );

            ctx.locals_last_index += comp_count;

            WasmInstr::NoInstr
        }
        (
            "set" | "=",
            [SExpr::Atom {
                value: var_name,
                loc: name_loc,
                kind: AtomKind::Symbol,
            }, value],
        ) => {
            if let Some(global) = ctx.module.globals.get(var_name.as_str()) {
                if !global.mutable {
                    return Err(CompileError {
                        message: format!("Setting immutable global: {var_name}"),
                        loc: name_loc.clone(),
                    });
                }

                return Ok(WasmInstr::GlobalSet {
                    global_index: global.index,
                    value: Box::new(parse_instr(value, ctx)?),
                });
            };

            let Some(local) = ctx.locals.get(var_name.as_str()) else {
                return Err(CompileError {
                    message: format!("Unknown variable: {var_name}"),
                    loc: name_loc.clone(),
                });
            };

            match &local.value_type {
                LoleValueType::StructInstance { name } => {
                    let struct_def = ctx.module.struct_defs.get(name).unwrap();

                    let mut local_indices = vec![];
                    for field_offset in 0..struct_def.fields.len() {
                        local_indices.push(local.index + field_offset as u32);
                    }

                    return Ok(WasmInstr::MultiValueLocalSet {
                        local_indices,
                        value: Box::new(parse_instr(value, ctx)?),
                    });
                }
                LoleValueType::Primitive(_) => WasmInstr::LocalSet {
                    local_index: local.index,
                    value: Box::new(parse_instr(value, ctx)?),
                },
            }
        }
        (
            "set" | "=",
            [SExpr::List {
                value: local_names,
                loc: name_loc,
            }, value],
        ) => {
            let mut local_indices = vec![];

            for local_name_expr in local_names {
                let SExpr::Atom { value: local_name, .. } = local_name_expr else {
                    return Err(CompileError {
                        message: format!("Unexpected list in lhs of set"),
                        loc: local_name_expr.loc().clone(),
                    });
                };

                if let Some(_) = ctx.module.globals.get(local_name.as_str()) {
                    return Err(CompileError {
                        message: format!("Cannot set globals in multivalue set: {local_name}"),
                        loc: name_loc.clone(),
                    });
                };

                let Some(local) = ctx.locals.get(local_name.as_str()) else {
                    return Err(CompileError {
                        message: format!("Unknown location for set: {local_name}"),
                        loc: name_loc.clone(),
                    });
                };

                match &local.value_type {
                    LoleValueType::StructInstance { name } => {
                        let struct_def = ctx.module.struct_defs.get(name).unwrap();

                        for field_offset in 0..struct_def.fields.len() {
                            local_indices.push(local.index + field_offset as u32);
                        }
                    }
                    LoleValueType::Primitive(_) => local_indices.push(local.index),
                }
            }

            WasmInstr::MultiValueLocalSet {
                local_indices,
                value: Box::new(parse_instr(value, ctx)?),
            }
        }
        (
            "set" | "=",
            [SExpr::Atom {
                value: local_name,
                loc: name_loc,
                kind: AtomKind::Symbol,
            }, SExpr::Atom {
                value: f_name,
                loc: f_name_loc,
                kind: AtomKind::Symbol,
            }, value],
        ) => {
            if let Some(_) = ctx.module.globals.get(local_name.as_str()) {
                return Err(CompileError {
                    message: format!("Setting struct field from global variable: {local_name}"),
                    loc: name_loc.clone(),
                });
            };

            let Some(local) = ctx.locals.get(local_name.as_str()) else {
                return Err(CompileError {
                    message: format!("Unknown location for {op}: {local_name}"),
                    loc: name_loc.clone(),
                });
            };

            let LoleValueType::StructInstance { name: s_name } = &local.value_type else {
                return Err(CompileError {
                    message: format!("Trying to set field '{f_name}' on non struct: {local_name}"),
                    loc: f_name_loc.clone(),
                });
            };

            let struct_def = match ctx.module.struct_defs.get(s_name) {
                Some(struct_def) => struct_def,
                None => {
                    return Err(CompileError {
                        message: format!("Unknown struct in {op}: {s_name}"),
                        loc: name_loc.clone(),
                    })
                }
            };

            let Some(field_offset) = struct_def.fields.iter().position(|f| f.name == *f_name) else {
                return Err(CompileError {
                    message: format!("Unknown field {f_name} in struct {s_name}"),
                    loc: f_name_loc.clone(),
                });
            };

            WasmInstr::LocalSet {
                local_index: local.index + field_offset as u32,
                value: Box::new(parse_instr(value, ctx)?),
            }
        }
        (
            "get" | ".",
            [SExpr::Atom {
                value: local_name,
                loc: name_loc,
                kind: AtomKind::Symbol,
            }, SExpr::Atom {
                value: f_name,
                loc: f_name_loc,
                kind: AtomKind::Symbol,
            }],
        ) => {
            if let Some(_) = ctx.module.globals.get(local_name.as_str()) {
                return Err(CompileError {
                    message: format!("Getting struct field from global variable: {local_name}"),
                    loc: name_loc.clone(),
                });
            };

            let Some(local) = ctx.locals.get(local_name.as_str()) else {
                return Err(CompileError {
                    message: format!("Reading unknown variable: {local_name}"),
                    loc: name_loc.clone(),
                });
            };

            let LoleValueType::StructInstance { name: s_name } = &local.value_type else {
                return Err(CompileError {
                    message: format!("Trying to get field '{f_name}' on non struct: {local_name}"),
                    loc: f_name_loc.clone(),
                });
            };

            let struct_def = match ctx.module.struct_defs.get(s_name) {
                Some(struct_def) => struct_def,
                None => {
                    return Err(CompileError {
                        message: format!("Unknown struct in get: {s_name}"),
                        loc: name_loc.clone(),
                    })
                }
            };

            let Some(field_offset) = struct_def.fields.iter().position(|f| f.name == *f_name) else {
                return Err(CompileError {
                    message: format!("Unknown field {f_name} in struct {s_name}"),
                    loc: f_name_loc.clone(),
                });
            };

            WasmInstr::LocalGet {
                local_index: local.index + field_offset as u32,
            }
        }
        ("pack" | "do", exprs) => WasmInstr::MultiValueEmit {
            values: parse_instrs(exprs, ctx)?,
        },
        (fn_name, args) => {
            let Some(fn_def) = ctx.module.fn_defs.get(fn_name) else {
                return Err(CompileError {
                    message: format!("Unknown instruction or function: {fn_name}"),
                    loc: op_loc.clone()
                });
            };

            WasmInstr::Call {
                fn_index: fn_def.get_absolute_index(ctx.module),
                args: parse_instrs(args, ctx)?,
                loc: op_loc.clone(),
            }
        }
    };

    Ok(instr)
}

fn parse_const_instr(expr: &SExpr, ctx: &ModuleContext) -> Result<WasmInstr, CompileError> {
    let items = match expr {
        SExpr::List { value: items, .. } => items,
        SExpr::Atom {
            value: global_name,
            loc: op_loc,
            kind,
        } => {
            if *kind != AtomKind::Symbol {
                return Err(CompileError {
                    message: format!("Strings are not allowed in globals"),
                    loc: expr.loc().clone(),
                });
            }

            if global_name.chars().all(|c| c.is_ascii_digit()) {
                return Ok(WasmInstr::I32Const {
                    value: global_name.parse().map_err(|_| CompileError {
                        message: format!("Parsing i32 (implicit) failed"),
                        loc: op_loc.clone(),
                    })?,
                });
            }

            let Some(global) = ctx.globals.get(global_name.as_str()) else {
                return Err(CompileError {
                    message: format!("Unknown location for global.get: {global_name}"),
                    loc: op_loc.clone(),
                });
            };

            return Ok(WasmInstr::GlobalGet {
                global_index: global.index,
            });
        }
    };

    let [SExpr::Atom { value: op, loc: op_loc, kind }, args @ ..] = &items[..] else {
        return Err(CompileError {
            message: format!("Expected operation, got a simple list"),
            loc: expr.loc().clone(),
        });
    };

    if *kind != AtomKind::Symbol {
        return Err(CompileError {
            message: format!("Expected operation, got a string"),
            loc: expr.loc().clone(),
        });
    }

    let instr = match (op.as_str(), &args[..]) {
        ("i32", [SExpr::Atom { value, .. }]) => WasmInstr::I32Const {
            value: value.parse().map_err(|_| CompileError {
                message: format!("Parsing i32 failed"),
                loc: op_loc.clone(),
            })?,
        },
        ("data.size", []) => WasmInstr::I32ConstLazy {
            value: ctx.data_size.clone(),
        },
        (instr_name, _args) => {
            return Err(CompileError {
                message: format!("Unknown instruction: {instr_name}"),
                loc: op_loc.clone(),
            });
        }
    };

    Ok(instr)
}

fn parse_instrs(exprs: &[SExpr], ctx: &mut FnContext) -> Result<Vec<WasmInstr>, CompileError> {
    exprs.iter().map(|expr| parse_instr(expr, ctx)).collect()
}

// types

impl LoleValueType {
    fn parse(l_type: &String, ctx: &ModuleContext) -> Result<Self, String> {
        if let Ok(value_type) = WasmValueType::parse(l_type) {
            return Ok(Self::Primitive(value_type));
        }

        if ctx.struct_defs.contains_key(l_type) {
            return Ok(Self::StructInstance {
                name: l_type.clone(),
            });
        }

        Err(format!("Unknown value type: {l_type}"))
    }

    fn byte_size(&self, ctx: &ModuleContext) -> Result<u32, String> {
        match self {
            Self::Primitive(primitive) => primitive.byte_size(),
            Self::StructInstance { name } => {
                let struct_def = ctx.struct_defs.get(name).unwrap();

                let mut size = 0;
                for field in &struct_def.fields {
                    size += Self::Primitive(field.value_type).byte_size(ctx)?;
                }
                Ok(size)
            }
        }
    }
}

impl WasmValueType {
    fn parse(name: &str) -> Result<Self, String> {
        match name {
            "bool" | "u8" | "u32" | "i32" | "ptr" => Ok(Self::I32),
            "i64" | "u64" => Ok(Self::I64),
            "f32" => Ok(Self::F32),
            "f64" => Ok(Self::F64),
            "v128" => Ok(Self::V128),
            "funcref" => Ok(Self::FuncRef),
            "externref" => Ok(Self::ExternRef),
            _ => return Err(format!("Unknown value type: {name}")),
        }
    }

    fn byte_size(&self) -> Result<u32, String> {
        Ok(match self {
            Self::FuncRef | Self::ExternRef => {
                return Err(format!("Cannot get byte size of FuncRef/ExternRef"))
            }
            Self::I32 | Self::F32 => 4,
            Self::I64 | Self::F64 => 8,
            Self::V128 => 16,
        })
    }
}

impl WasmLoadKind {
    fn parse(kind: &str) -> Result<Self, String> {
        match kind {
            "i32" => Ok(Self::I32),
            "i32/u8" => Ok(Self::I32U8),
            _ => Err(format!("Unknown load kind: {kind}")),
        }
    }

    pub fn get_value_type(&self) -> WasmValueType {
        match &self {
            Self::I32 => WasmValueType::I32,
            Self::I32U8 => WasmValueType::I32,
        }
    }

    fn from_value_type(value_type: &WasmValueType) -> Result<Self, String> {
        match value_type {
            WasmValueType::I32 => Ok(Self::I32),
            _ => return Err(format!("Unsupported type for load: {value_type:?}")),
        }
    }
}

impl WasmStoreKind {
    fn parse(kind: &str) -> Result<Self, String> {
        match kind {
            "i32" => Ok(Self::I32),
            "i32/u8" => Ok(Self::I32U8),
            _ => return Err(format!("Unknown store kind: {kind}")),
        }
    }

    fn from_value_type(value_type: &WasmValueType) -> Result<Self, String> {
        match value_type {
            WasmValueType::I32 => Ok(Self::I32),
            _ => Err(format!("Unsupported type for store: {value_type:?}")),
        }
    }
}

// Stolen from https://keyboardsmash.dev/posts/base64-implementation-in-rust-decoding/
fn base64_decode(input: &[u8]) -> Vec<u8> {
    const BASE_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    fn decode_char(input: u8) -> u8 {
        BASE_CHARS.iter().position(|&c| c == input).unwrap_or(0) as u8
    }

    let mut output: Vec<u8> = Vec::new();

    for chunk in input.chunks(4) {
        let a = decode_char(chunk[0]);
        let b = decode_char(chunk[1]);
        let c = decode_char(chunk[2]);
        let d = decode_char(chunk[3]);

        let dec1 = ((a << 2) | (b & 0x30) >> 4) as u8;
        let dec2 = (((b & 0x0F) << 4) | (c & 0x3C) >> 2) as u8;
        let dec3 = (((c & 0x03) << 6) | (d)) as u8;

        output.push(dec1);
        output.push(dec2);
        output.push(dec3);
    }

    output
}
