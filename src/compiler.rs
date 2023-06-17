use crate::{
    parser::{Location, SExpr},
    type_checker::get_type,
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
    string::String,
    vec,
    vec::Vec,
};
use core::cell::RefCell;

struct FnBody {
    fn_index: u32,
    locals: RefCell<BTreeMap<String, LocalDef>>,
    locals_last_index: u32,
    body: Vec<SExpr>,
}

struct StructDef {
    fields: Vec<StructField>,
}

struct StructField {
    name: String,
    value_type: WasmValueType,
}

enum LoleValueType {
    Primitive(WasmValueType),
    StructInstance { name: String },
}

#[derive(Default)]
struct ModuleContext {
    fn_defs: BTreeMap<String, FnDef>,
    fn_bodies: BTreeMap<String, FnBody>,
    fn_exports: BTreeMap<String, String>,
    memory_names: Vec<String>,
    struct_defs: BTreeMap<String, StructDef>,
    enum_kinds: BTreeMap<String, u32>,
    globals: BTreeMap<String, GlobalDef>,
    imported_fns_count: u32,
}

struct FnDef {
    local: bool,
    fn_index: u32,
}

struct FnContext<'a> {
    module: &'a ModuleContext,
    locals: &'a mut BTreeMap<String, LocalDef>,
    locals_last_index: u32,
    non_arg_locals: Vec<WasmValueType>,
}

struct LocalDef {
    index: u32,
    value_type: LoleValueType,
}

struct GlobalDef {
    index: u32,
    mutable: bool,
}

pub struct CompileError {
    pub message: String,
    pub loc: Location,
}

pub fn compile_module(exprs: &Vec<SExpr>) -> Result<WasmModule, CompileError> {
    let mut module = WasmModule::default();

    let mut ctx = ModuleContext::default();

    for expr in exprs {
        compile_top_level_expr(expr, &mut ctx, &mut module)?;
    }

    // push function exports
    for (in_name, out_name) in ctx.fn_exports.iter() {
        module.exports.push(WasmExport {
            export_type: WasmExportType::Func,
            export_name: out_name.clone(),
            exported_item_index: ctx
                .fn_defs
                .get(in_name)
                .map(|fd| ctx.imported_fns_count + fd.fn_index)
                .ok_or_else(|| CompileError {
                    message: format!("Cannot export unknown function {in_name}"),
                    loc: Location {
                        // TODO: add correct error location
                        offset: 0,
                        length: 0,
                    },
                })?,
        });
    }

    let mut fn_defs_sorted = ctx.fn_bodies.values().collect::<Vec<_>>();
    fn_defs_sorted.sort_by_key(|fd| fd.fn_index);

    // push function codes
    for fn_def in fn_defs_sorted {
        let mut fn_ctx = FnContext {
            module: &ctx,
            locals: &mut fn_def.locals.borrow_mut(),
            locals_last_index: fn_def.locals_last_index,
            non_arg_locals: vec![],
        };

        let instrs = parse_instrs(&fn_def.body, &mut fn_ctx)?;
        for instr in &instrs {
            let types = get_type(instr);

            if types.len() > 0 {
                return Err(CompileError {
                    message: format!("TypeError: excess values"),
                    loc: instr.loc().clone(),
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

        module.codes.push(WasmFn {
            locals,
            expr: WasmExpr { instrs },
        });
    }

    Ok(module)
}

fn compile_top_level_expr(
    expr: &SExpr,
    ctx: &mut ModuleContext,
    module: &mut WasmModule,
) -> Result<(), CompileError> {
    let SExpr::List { value: items, .. } = expr else {
        return Err(CompileError {
            message: format!("Unexpected atom"),
            loc: expr.loc().clone()
        });
    };

    let [SExpr::Atom{ value: op, loc: op_loc }, other @ ..] = &items[..] else {
        return Err(CompileError {
            message: format!("Expected operation, got a simple list"),
            loc: expr.loc().clone()
        });
    };

    match op.as_str() {
        "mem" => match other {
            [SExpr::Atom {
                value: mem_name,
                loc: mem_name_loc,
            }, SExpr::Atom {
                value: min_literal, ..
            }, SExpr::Atom {
                value: min_memory,
                loc: min_memory_loc,
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
                        value: max_literal, ..
                    }, SExpr::Atom {
                        value: max_memory,
                        loc: max_memory_loc,
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
                module.memories.push(WasmLimits {
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

                    let [SExpr::Atom { value: p_name, loc: p_name_loc }, SExpr::Atom { value: p_type, loc: p_type_loc }] = &name_and_type[..] else {
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

                module.types.push(WasmFnType { inputs, outputs });
                let type_index = module.types.len() as u32 - 1;
                let fn_index = module.functions.len() as u32;

                module.functions.push(type_index);
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
            [SExpr::Atom { value: fn_literal, .. }, SExpr::Atom { value: fn_name, loc: fn_name_loc }, /* * */
             SExpr::List{ value: input_exprs, .. }, SExpr::List{ value: output_exprs, .. }, /* * */
             SExpr::Atom { value: from_literal, .. }, SExpr::Atom { value: module_name, .. }, SExpr::Atom { value: extern_fn_name, .. }]
                if fn_literal == "fn" && from_literal == ":from" =>
            {
                if ctx.fn_defs.contains_key(fn_name) {
                    return Err(CompileError {
                        message: format!("Cannot redefine function: {fn_name}"),
                        loc: fn_name_loc.clone()
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

                    let [SExpr::Atom { value: p_name, loc: name_loc }, SExpr::Atom { value: p_type, loc: type_loc }] = &name_and_type[..] else {
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
                            loc: name_loc.clone()
                        });
                    }

 let value_type = LoleValueType::parse(p_type, &ctx).map_err(|e| CompileError {
                        message: e,
                        loc: type_loc.clone()
                    })?;
                    emit_value_components(&value_type, &ctx, &mut inputs);

                    param_names.insert(p_name.clone());
                }

                let mut outputs = vec![];
                for output_expr in output_exprs {
                    let l_type = match output_expr {
                        SExpr::Atom { value: atom_text, .. } => atom_text,
                        _ => return Err(CompileError {
                            message: format!("Atom expected, list found"),
                            loc: output_expr.loc().clone()
                        }),
                    };

let value_type = LoleValueType::parse(l_type, &ctx).map_err(|e| CompileError {
                        message: e,
                        loc: output_expr.loc().clone()
                    })?;
                    emit_value_components(&value_type, &ctx, &mut outputs);
                }

                let type_index = module.types.len() as u32;
                let fn_index =  ctx.imported_fns_count;

                ctx.imported_fns_count += 1;
                ctx.fn_defs.insert(fn_name.clone(), FnDef {
                    local: false,
                    fn_index,
                });
                module.types.push(WasmFnType { inputs, outputs });
                module.imports.push(WasmImport {
                    module_name: module_name.clone(),
                    item_name: extern_fn_name.clone(),
                    item_desc: WasmImportDesc::Func { type_index },
                });
            }
            _ => return Err(CompileError {
                message: format!("Invalid arguments for {op}"),
                loc: op_loc.clone(),
            })
        },
        "export" => match other {
            [SExpr::Atom {
                value: mem_literal, ..
            }, SExpr::Atom {
                value: in_name,
                loc: name_loc,
            }, SExpr::Atom {
                value: as_literal, ..
            }, SExpr::Atom {
                value: out_name, ..
            }] if mem_literal == "mem" && as_literal == ":as" => {
                if !ctx.memory_names.contains(in_name) {
                    return Err(CompileError {
                        message: format!("Cannot export mem {in_name}, not found"),
                        loc: name_loc.clone(),
                    });
                }

                module.exports.push(WasmExport {
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
                    ..
                }, SExpr::Atom {
                    value: global_name,
                    loc: name_loc,
                }, SExpr::Atom {
                    value: global_type,
                    loc: type_loc,
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
                }, SExpr::Atom {
                    value: global_type,
                    loc: type_loc,
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

            module.globals.push(WasmGlobal {
                kind: WasmGlobalKind {
                    value_type,
                    mutable,
                },
                initial_value,
            });
        }
        "data" => {
            let [
                SExpr::Atom { value: offset, loc },
                SExpr::Atom { value: data_base64, .. }
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

            module.datas.push(WasmData::Active {
                offset: WasmExpr {
                    instrs: vec![WasmInstr::I32Const {
                        value: offset,
                        loc: loc.clone(),
                    }],
                },
                bytes: base64_decode(data_base64.as_bytes()),
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

        let [SExpr::Atom { value: f_name, loc: name_loc }, SExpr::Atom { value: f_type, loc: type_loc }] = &name_and_type[..] else {
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

fn emit_value_components(
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
        SExpr::Atom {
            value: var_name,
            loc,
        } => {
            if var_name.chars().all(|c| c.is_ascii_digit()) {
                return Ok(WasmInstr::I32Const {
                    value: (var_name.parse().map_err(|_| CompileError {
                        message: format!("Parsing i32 (implicit) failed"),
                        loc: loc.clone(),
                    })?),
                    loc: loc.clone(),
                });
            }

            if let Some(global) = ctx.module.globals.get(var_name.as_str()) {
                return Ok(WasmInstr::GlobalGet {
                    local_index: global.index,
                    loc: loc.clone(),
                });
            };

            let Some(local) = ctx.locals.get(var_name.as_str()) else {
                return Err(CompileError {
                    message: format!("Reading unknown variable: {var_name}"),
                    loc: loc.clone()
                });
            };

            if let LoleValueType::StructInstance { name } = &local.value_type {
                let struct_def = ctx.module.struct_defs.get(name).unwrap();

                let mut values = vec![];

                for field_offset in 0..struct_def.fields.len() {
                    values.push(WasmInstr::LocalGet {
                        local_index: local.index + field_offset as u32,
                        loc: loc.clone(),
                    });
                }

                return Ok(WasmInstr::MultiValueEmit {
                    values,
                    loc: loc.clone(),
                });
            }

            return Ok(WasmInstr::LocalGet {
                local_index: local.index,
                loc: loc.clone(),
            });
        }
    };

    let [SExpr::Atom { value: op, loc: op_loc }, args @ ..] = &items[..] else {
        return Err(CompileError {
            message: format!("Expected operation, got a simple list"),
            loc: expr.loc().clone()
        });
    };

    let instr = match (op.as_str(), &args[..]) {
        ("i32", [SExpr::Atom { value, loc }]) => WasmInstr::I32Const {
            value: value.parse().map_err(|_| CompileError {
                message: format!("Parsing i32 failed"),
                loc: loc.clone(),
            })?,
            loc: op_loc.clone(),
        },
        ("i32.eq" | "==", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Equals,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
            loc: op_loc.clone(),
        },
        ("i32.ne" | "!=", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32NotEqual,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
            loc: op_loc.clone(),
        },
        ("i32.not" | "not" | "!", [lhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Equals,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(WasmInstr::I32Const {
                value: 0,
                loc: op_loc.clone(), // TODO: add better location
            }),
            loc: op_loc.clone(),
        },
        ("i32.lt_s" | "<", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32LessThenSigned,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
            loc: op_loc.clone(),
        },
        ("i32.ge_s" | ">=", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32GreaterEqualSigned,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
            loc: op_loc.clone(),
        },
        ("i32.and" | "&&", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32And,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
            loc: op_loc.clone(),
        },
        ("i32.or" | "||", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Or,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
            loc: op_loc.clone(),
        },
        ("i32.add" | "+", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Add,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
            loc: op_loc.clone(),
        },
        ("i32.sub" | "-", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Sub,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
            loc: op_loc.clone(),
        },
        ("i32.mul" | "*", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Mul,
            lhs: Box::new(parse_instr(lhs, ctx)?),
            rhs: Box::new(parse_instr(rhs, ctx)?),
            loc: op_loc.clone(),
        },
        (
            "if",
            [SExpr::Atom {
                value: block_type,
                loc: type_loc,
            }, cond, then_branch, else_branch],
        ) => WasmInstr::If {
            block_type: WasmValueType::parse(block_type).map_err(|e| CompileError {
                message: e,
                loc: type_loc.clone(),
            })?,
            cond: Box::new(parse_instr(cond, ctx)?),
            then_branch: Box::new(parse_instr(then_branch, ctx)?),
            else_branch: Box::new(parse_instr(else_branch, ctx)?),
            loc: op_loc.clone(),
        },
        ("if", [cond, then_branch]) => WasmInstr::IfSingleBranch {
            cond: Box::new(parse_instr(cond, ctx)?),
            then_branch: Box::new(parse_instr(then_branch, ctx)?),
            loc: op_loc.clone(),
        },
        ("loop", [SExpr::List { value: exprs, .. }]) => WasmInstr::Loop {
            instrs: parse_instrs(exprs, ctx)?,
            loc: op_loc.clone(),
        },
        ("break", []) => WasmInstr::LoopBreak {
            loc: op_loc.clone(),
        },
        ("continue", []) => WasmInstr::LoopContinue {
            loc: op_loc.clone(),
        },
        ("return", values) => WasmInstr::Return {
            value: Box::new(WasmInstr::MultiValueEmit {
                values: parse_instrs(values, ctx)?,
                loc: op_loc.clone(), // TODO: add better location
            }),
            loc: op_loc.clone(),
        },
        // TODO: support custom aligns and offsets
        (
            "store",
            [SExpr::Atom {
                value: store_kind,
                loc: kind_loc,
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
                    loc: op_loc.clone(),
                })
            };

            let mut value_instrs = match parse_instr(value_expr, ctx)? {
                WasmInstr::MultiValueEmit { values, .. } => values,
                instr => vec![instr],
            };

            // TODO: add better check if type inference is implemented
            if value_instrs.len() != struct_def.fields.len() {
                return Err(CompileError {
                    message: format!(
                        "Invalid number of receiving variables for {op}, needed {}, got {}",
                        struct_def.fields.len(),
                        value_instrs.len()
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
                    loc: op_loc.clone(), // TODO: add better location
                });

                offset += field.value_type.byte_size();
            }

            WasmInstr::MultiValueEmit {
                values: instrs,
                loc: op_loc.clone(),
            }
        }
        // TODO: support custom aligns and offsets
        (
            "load",
            [SExpr::Atom {
                value: load_kind,
                loc: kind_loc,
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
                    loc: op_loc.clone(),
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
                    loc: op_loc.clone(), // TODO: add better location
                });

                offset += field.value_type.byte_size();
            }

            WasmInstr::MultiValueEmit {
                values: primitive_loads,
                loc: op_loc.clone(),
            }
        }
        /*
            TODO: validate that number values matches the one of fields
            WARNING: it's not that simple because it involves nested `MultiValueEmit`s
        */
        (
            "struct.new",
            [SExpr::Atom {
                value: s_name,
                loc: name_loc,
            }, values @ ..],
        ) => {
            if !ctx.module.struct_defs.contains_key(s_name) {
                return Err(CompileError {
                    message: format!("Unknown struct encountered in {op}: {s_name}"),
                    loc: name_loc.clone(),
                });
            }

            WasmInstr::MultiValueEmit {
                values: parse_instrs(values, ctx)?,
                loc: op_loc.clone(),
            }
        }
        (
            "enum.kind",
            [SExpr::Atom {
                value: enum_variant,
                loc: name_loc,
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
                loc: op_loc.clone(),
            }
        }
        (
            "sizeof",
            [SExpr::Atom {
                value: type_name,
                loc: type_loc,
            }],
        ) => {
            let value_type =
                LoleValueType::parse(type_name, ctx.module).map_err(|e| CompileError {
                    message: e,
                    loc: type_loc.clone(),
                })?;

            WasmInstr::I32Const {
                value: value_type.byte_size(ctx.module) as i32,
                loc: op_loc.clone(),
            }
        }
        (
            "let" | ":",
            [SExpr::Atom {
                value: local_name,
                loc: name_loc,
            }, SExpr::Atom {
                value: local_type,
                loc: type_loc,
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

            WasmInstr::NoInstr {
                loc: op_loc.clone(),
            }
        }
        (
            "set" | "=",
            [SExpr::Atom {
                value: var_name,
                loc: name_loc,
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
                    loc: op_loc.clone(),
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
                        loc: op_loc.clone(),
                    });
                }
                LoleValueType::Primitive(_) => WasmInstr::LocalSet {
                    local_index: local.index,
                    value: Box::new(parse_instr(value, ctx)?),
                    loc: op_loc.clone(),
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
                loc: op_loc.clone(),
            }
        }
        (
            "set" | "=",
            [SExpr::Atom {
                value: local_name,
                loc: name_loc,
            }, SExpr::Atom {
                value: f_name,
                loc: f_name_loc,
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
                loc: op_loc.clone(),
            }
        }
        (
            "get" | ".",
            [SExpr::Atom {
                value: local_name,
                loc: name_loc,
            }, SExpr::Atom {
                value: f_name,
                loc: f_name_loc,
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
                loc: op_loc.clone(),
            }
        }
        ("pack" | "do", exprs) => WasmInstr::MultiValueEmit {
            values: parse_instrs(exprs, ctx)?,
            loc: op_loc.clone(),
        },
        (fn_name, args) => {
            let Some(fn_def) = ctx.module.fn_defs.get(fn_name) else {
                return Err(CompileError {
                    message: format!("Unknown instruction or function: {fn_name}"),
                    loc: op_loc.clone()
                });
            };
            let fn_index = if fn_def.local {
                fn_def.fn_index + ctx.module.imported_fns_count
            } else {
                fn_def.fn_index
            };

            WasmInstr::Call {
                fn_index,
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
        } => {
            if global_name.chars().all(|c| c.is_ascii_digit()) {
                return Ok(WasmInstr::I32Const {
                    value: global_name.parse().map_err(|_| CompileError {
                        message: format!("Parsing i32 (implicit) failed"),
                        loc: op_loc.clone(),
                    })?,
                    loc: op_loc.clone(),
                });
            }

            let Some(global) = ctx.globals.get(global_name.as_str()) else {
                return Err(CompileError {
                    message: format!("Unknown location for global.get: {global_name}"),
                    loc: op_loc.clone(),
                });
            };

            return Ok(WasmInstr::GlobalGet {
                local_index: global.index,
                loc: op_loc.clone(),
            });
        }
    };

    let [SExpr::Atom { value: op, loc: op_loc }, args @ ..] = &items[..] else {
        return Err(CompileError {
            message: format!("Expected operation, got a simple list"),
            loc: expr.loc().clone(),
        });
    };

    let instr = match (op.as_str(), &args[..]) {
        ("i32", [SExpr::Atom { value, .. }]) => WasmInstr::I32Const {
            value: value.parse().map_err(|_| CompileError {
                message: format!("Parsing i32 failed"),
                loc: op_loc.clone(),
            })?,
            loc: op_loc.clone(),
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

    fn byte_size(&self, ctx: &ModuleContext) -> u32 {
        match self {
            Self::Primitive(primitive) => primitive.byte_size(),
            Self::StructInstance { name } => {
                let struct_def = ctx.struct_defs.get(name).unwrap();

                let mut size = 0;
                for field in &struct_def.fields {
                    size += Self::Primitive(field.value_type).byte_size(ctx);
                }
                size
            }
        }
    }
}

impl WasmValueType {
    fn parse(name: &str) -> Result<Self, String> {
        match name {
            "bool" | "u8" | "u32" | "i32" | "ptr" => Ok(Self::I32),
            "i64" => Ok(Self::I64),
            "f32" => Ok(Self::F32),
            "f64" => Ok(Self::F64),
            "v128" => Ok(Self::V128),
            "funcref" => Ok(Self::FuncRef),
            "externref" => Ok(Self::ExternRef),
            _ => return Err(format!("Unknown value type: {name}")),
        }
    }

    fn byte_size(&self) -> u32 {
        match self {
            Self::FuncRef | Self::ExternRef => 4, // TODO: check this
            Self::I32 | Self::F32 => 4,
            Self::I64 | Self::F64 => 8,
            Self::V128 => 16,
        }
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
