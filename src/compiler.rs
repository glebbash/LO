use crate::{ast::*, ir::*, parser::*, type_checker::*, wasi_io::*, wasm::*};
use alloc::{
    boxed::Box,
    collections::{BTreeMap, BTreeSet},
    format, str,
    string::String,
    vec,
    vec::Vec,
};
use core::cell::RefCell;

const DEFER_UNTIL_RETURN_LABEL: &str = "return";
const HEAP_ALLOC_ID: i32 = 1;

pub fn compile(exprs: &Vec<SExpr>) -> Result<WasmModule, CompileError> {
    let mut ctx = ModuleContext::default();

    for expr in exprs {
        compile_top_level_expr(&expr, &mut ctx)?;
    }

    // push function exports
    for (in_name, out_name) in &ctx.fn_exports {
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

    // push function codes
    for fn_body in &ctx.fn_bodies {
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
            defers: BTreeMap::default(),
        };

        let mut instrs = compile_block(&fn_body.body, &mut fn_ctx)?;
        if let Some(values) = get_deferred("return", &mut fn_ctx) {
            instrs.append(&mut values?);
        };

        let mut locals = Vec::<WasmLocals>::new();
        for local_type in fn_ctx.non_arg_locals {
            if let Some(wasm_locals) = locals.last_mut() {
                if (*wasm_locals).value_type == local_type {
                    wasm_locals.count += 1;
                    continue;
                }
            }
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

fn compile_block(
    exprs: &Vec<SExpr>,
    fn_ctx: &mut FnContext,
) -> Result<Vec<WasmInstr>, CompileError> {
    let instrs = compile_instrs(exprs, fn_ctx)?;
    for (instr, i) in instrs.iter().zip(0..) {
        if let WasmInstr::NoEmit { .. } = instr {
            continue;
        }

        let types = get_type(&fn_ctx, instr)?;

        if types.len() > 0 {
            return Err(CompileError {
                message: format!("TypeError: Excess values"),
                loc: exprs[i].loc().clone(),
            });
        }
    }
    Ok(instrs)
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
                loc: mod_name_loc,
                kind: _,
            }] => {
                if !ctx.included_modules.insert(mod_name.clone()) {
                    // do not include module twice
                    return Ok(());
                };

                let file_name = format!("{}.lole", mod_name);
                let mod_fd = open(&file_name).map_err(|err| CompileError {
                    message: format!("Cannot load file {file_name}: {err}"),
                    loc: mod_name_loc.clone(),
                })?;

                let source_buf = fd_read_all(mod_fd);
                let source = str::from_utf8(source_buf.as_slice()).unwrap();

                let exprs = parse(&file_name, source)?;

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
                    }, p_type] = &name_and_type[..] else {
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

                    let value_type = parse_lole_type(p_type, &ctx)?;
                    let comp_count = value_type.emit_components(&ctx, &mut inputs);

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
                for output_type in output_exprs {
                    let value_type = parse_lole_type(output_type, &ctx)?;
                    value_type.emit_components(&ctx, &mut outputs);
                }

                let fn_index = ctx.wasm_module.functions.len() as u32;

                let type_index = ctx.insert_fn_type(WasmFnType { inputs, outputs });

                ctx.wasm_module.functions.push(type_index);
                ctx.fn_defs.insert(
                    fn_name.clone(),
                    FnDef {
                        local: true,
                        fn_index,
                        type_index,
                    },
                );
                ctx.fn_bodies.push(FnBody {
                    fn_index,
                    locals: RefCell::new(locals),
                    locals_last_index,
                    body: body.clone(),
                });
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
                    }, p_type] = &name_and_type[..] else {
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

                    let value_type = parse_lole_type(p_type, &ctx)?;
                    value_type.emit_components(&ctx, &mut inputs);

                    param_names.insert(p_name.clone());
                }

                let mut outputs = vec![];
                for output_type in output_exprs {
                    let value_type = parse_lole_type(output_type, &ctx)?;
                    value_type.emit_components(&ctx, &mut outputs);
                }

                let type_index = ctx.insert_fn_type(WasmFnType { inputs, outputs });

                let fn_index = ctx.imported_fns_count;
                ctx.imported_fns_count += 1;

                ctx.fn_defs.insert(
                    fn_name.clone(),
                    FnDef {
                        local: false,
                        fn_index,
                        type_index,
                    },
                );
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

                let struct_def = compile_struct(field_defs, struct_name, ctx)?;
                ctx.struct_defs.insert(struct_name.clone(), struct_def);
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

                    let struct_def = compile_struct(field_defs, &full_name, ctx)?;
                    ctx.struct_defs.insert(full_name.clone(), struct_def);
                    ctx.enum_kinds.insert(full_name, kind as u32);
                }

                ctx.struct_defs.insert(
                    enum_name.clone(),
                    StructDef {
                        fields: vec![
                            StructField {
                                name: String::from("kind"),
                                value_type: LoleType::Primitive(WasmType::I32),
                                field_index: 0,
                                byte_offset: 0,
                            },
                            StructField {
                                name: String::from("ref"),
                                value_type: LoleType::Primitive(WasmType::I32),
                                field_index: 1,
                                byte_offset: WasmType::I32.byte_length().unwrap(),
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
            let (mutable, global_name, global_type, global_value, name_loc) = match other {
                [SExpr::Atom {
                    value: mutable_literal,
                    loc: _,
                    kind: AtomKind::Symbol,
                }, SExpr::Atom {
                    value: global_name,
                    loc: name_loc,
                    kind: AtomKind::Symbol,
                }, global_type, global_value]
                    if mutable_literal == "mut" =>
                {
                    (true, global_name, global_type, global_value, name_loc)
                }
                [SExpr::Atom {
                    value: global_name,
                    loc: name_loc,
                    kind: AtomKind::Symbol,
                }, global_type, global_value] => {
                    (false, global_name, global_type, global_value, name_loc)
                }
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

            let value_type = parse_wasm_type(global_type, ctx)?;
            let initial_value = WasmExpr {
                instrs: vec![compile_const_instr(global_value, &ctx)?],
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

fn compile_struct(
    exprs: &[SExpr],
    struct_name: &str,
    ctx: &ModuleContext,
) -> Result<StructDef, CompileError> {
    let mut field_index = 0;
    let mut byte_offset = 0;

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
        }, f_type] = &name_and_type[..] else {
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

        let value_type = parse_lole_type(f_type, ctx)?;

        let mut stats = EmitComponentStats::default();
        value_type
            .emit_sized_component_stats(ctx, &mut stats, &mut vec![])
            .map_err(|err| CompileError {
                message: err,
                loc: f_type.loc().clone(),
            })?;

        fields.push(StructField {
            name: f_name.clone(),
            value_type,
            field_index,
            byte_offset,
        });

        field_index += stats.count;
        byte_offset += stats.byte_length;
    }
    Ok(StructDef { fields })
}

fn compile_instrs(exprs: &[SExpr], ctx: &mut FnContext) -> Result<Vec<WasmInstr>, CompileError> {
    exprs.iter().map(|expr| compile_instr(expr, ctx)).collect()
}

fn compile_instr(expr: &SExpr, ctx: &mut FnContext) -> Result<WasmInstr, CompileError> {
    let items = match expr {
        SExpr::List { value: items, .. } => items,
        SExpr::Atom { value, loc, kind } => {
            if *kind == AtomKind::String {
                let string_len = value.as_bytes().len() as i32;

                let string_ptr = *ctx
                    .module
                    .string_pool
                    .borrow_mut()
                    .entry(value.clone())
                    .or_insert_with(|| {
                        let string_ptr = *ctx.module.data_size.borrow();

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

                        string_ptr
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

            return Ok(compile_local_get(
                ctx.module,
                local.index,
                &local.value_type,
            ));
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
        ("drop", [expr]) => {
            let instr = compile_instr(expr, ctx)?;
            let drop_count = get_type(ctx, &instr)?.len();

            WasmInstr::Drop {
                value: Box::new(instr),
                drop_count,
            }
        }
        ("do", exprs) => WasmInstr::MultiValueEmit {
            values: compile_instrs(exprs, ctx)?,
        },
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
        (
            "char_code",
            [SExpr::Atom {
                value,
                kind: AtomKind::String,
                ..
            }],
        ) => WasmInstr::I32Const {
            value: value.chars().next().unwrap() as i32,
        },
        ("i32.eq" | "==", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Equals,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("i32.ne" | "!=", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32NotEqual,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("i32.not" | "not" | "!", [lhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Equals,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(WasmInstr::I32Const { value: 0 }),
        },
        ("i32.lt_s" | "<", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32LessThenSigned,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("i32.gt_s" | ">", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32GreaterThenSigned,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("i32.ge_s" | ">=", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32GreaterEqualSigned,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("i32.and" | "&&", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32And,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("i32.or" | "||", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Or,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("i32.add" | "+", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Add,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("+=", [lhs, rhs]) => {
            let bind = compile_instr(lhs, ctx)?;
            let value = WasmInstr::BinaryOp {
                kind: WasmBinaryOpKind::I32Add,
                lhs: Box::new(bind.clone()),
                rhs: Box::new(compile_instr(rhs, ctx)?),
            };
            compile_set(ctx, value, bind, lhs.loc())?
        }
        ("i32.sub" | "-", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Sub,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("-=", [lhs, rhs]) => {
            let bind = compile_instr(lhs, ctx)?;
            let value = WasmInstr::BinaryOp {
                kind: WasmBinaryOpKind::I32Sub,
                lhs: Box::new(bind.clone()),
                rhs: Box::new(compile_instr(rhs, ctx)?),
            };
            compile_set(ctx, value, bind, lhs.loc())?
        }
        ("i32.mul" | "*", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Mul,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        // TODO: should default `div` and `rem` be unsigned?
        ("i32.div" | "/", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32DivUnsigned,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("i32.rem" | "%", [lhs, rhs]) => WasmInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32RemUnsigned,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("data.size", []) => WasmInstr::I32ConstLazy {
            value: ctx.module.data_size.clone(),
        },
        ("memory.size", []) => WasmInstr::MemorySize {},
        ("memory.grow", [size_expr]) => {
            let size = compile_instr(size_expr, ctx)?;
            let size_type = get_type(ctx, &size)?;

            if let [WasmType::I32] = size_type[..] {
            } else {
                return Err(CompileError {
                    message: format!("Invalid arguments for {op}"),
                    loc: size_expr.loc().clone(),
                });
            }

            WasmInstr::MemoryGrow {
                size: Box::new(size),
            }
        }
        ("if", [block_type, cond, then_branch, else_branch]) => WasmInstr::If {
            block_type: parse_wasm_type(block_type, ctx.module)?,
            cond: Box::new(compile_instr(cond, ctx)?),
            then_branch: Box::new(compile_instr(then_branch, ctx)?),
            else_branch: Box::new(compile_instr(else_branch, ctx)?),
        },
        ("if", [cond, then_branch]) => WasmInstr::IfSingleBranch {
            cond: Box::new(compile_instr(cond, ctx)?),
            then_branch: Box::new(compile_instr(then_branch, ctx)?),
        },
        ("loop", [SExpr::List { value: exprs, .. }]) => WasmInstr::Loop {
            instrs: compile_block(exprs, ctx)?,
        },
        ("break", []) => WasmInstr::LoopBreak {},
        ("continue", []) => WasmInstr::LoopContinue {},
        ("return", values) => {
            let value = WasmInstr::MultiValueEmit {
                values: compile_instrs(values, ctx)?,
            };

            let return_type = get_type(ctx, &value)?;
            if return_type != ctx.fn_type.outputs {
                return Err(CompileError {
                    message: format!(
                        "TypeError: Invalid return type, \
                            expected {outputs:?}, got {return_type:?}",
                        outputs = ctx.fn_type.outputs,
                    ),
                    loc: op_loc.clone(),
                });
            }

            if let Some(values) = get_deferred(DEFER_UNTIL_RETURN_LABEL, ctx) {
                let mut values = values?;
                values.push(WasmInstr::Return {
                    value: Box::new(value),
                });
                return Ok(WasmInstr::MultiValueEmit { values });
            };

            WasmInstr::Return {
                value: Box::new(value),
            }
        }
        ("defer", [defer_label_exprs @ .., defer_expr]) => {
            let defer_label = match &defer_label_exprs[..] {
                [SExpr::Atom {
                    kind: AtomKind::Symbol,
                    value: defer_label,
                    loc: _,
                }] => defer_label.clone(),
                [] => String::from(DEFER_UNTIL_RETURN_LABEL),
                _ => {
                    return Err(CompileError {
                        message: format!("Invalid arguments for {op}"),
                        loc: op_loc.clone(),
                    })
                }
            };

            let deferred = ctx.defers.entry(defer_label).or_insert_with(|| vec![]);

            deferred.push(defer_expr.clone());

            WasmInstr::MultiValueEmit { values: vec![] }
        }
        (
            "defer.eval",
            [SExpr::Atom {
                kind: AtomKind::Symbol,
                value: defer_label,
                loc: defer_label_loc,
            }],
        ) => {
            let Some(values) = get_deferred(defer_label, ctx) else {
                return Err(CompileError {
                    message: format!("Unknown defer scope: {defer_label}"),
                    loc: defer_label_loc.clone(),
                });
            };

            WasmInstr::MultiValueEmit { values: values? }
        }
        ("sizeof", [type_expr]) => {
            let value_type = parse_lole_type(type_expr, ctx.module)?;

            WasmInstr::I32Const {
                value: value_type
                    .sized_comp_stats(ctx.module)
                    .map_err(|err| CompileError {
                        message: err,
                        loc: op_loc.clone(),
                    })?
                    .byte_length as i32,
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
            "struct.new",
            [SExpr::Atom {
                value: s_name,
                loc: name_loc,
                kind: AtomKind::Symbol,
            }, values @ ..],
        ) => {
            let Some(_) = ctx.module.struct_defs.get(s_name) else {
                return Err(CompileError {
                    message: format!("Unknown struct encountered in {op}: {s_name}"),
                    loc: name_loc.clone(),
                });
            };

            let value_instrs = compile_instrs(values, ctx)?;
            let value_types = get_types(ctx, &value_instrs)?;

            let mut field_types = vec![];
            LoleType::StructInstance {
                name: s_name.clone(),
            }
            .emit_components(ctx.module, &mut field_types);

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
        ("new", [type_expr, init_expr, other @ ..]) => {
            let alloc_id_instr = match other {
                [] => WasmInstr::I32Const {
                    value: HEAP_ALLOC_ID,
                },
                [SExpr::Atom {
                    value: using_literal,
                    kind: AtomKind::Symbol,
                    ..
                }, alloc_id_expr]
                    if using_literal == ":using" =>
                {
                    compile_instr(alloc_id_expr, ctx)?
                }
                _ => {
                    return Err(CompileError {
                        message: format!("Invalid arguments for {op}"),
                        loc: op_loc.clone(),
                    });
                }
            };

            let init_instr = compile_instr(init_expr, ctx)?;
            let init_types = get_type(ctx, &init_instr)?;

            let value_type = parse_lole_type(type_expr, ctx.module)?;

            let mut comp_types = vec![];
            value_type.emit_components(ctx.module, &mut comp_types);

            if init_types != comp_types {
                return Err(CompileError {
                    message: format!(
                        "TypeError: Invalid types for {op}, needed {:?}, got {:?}",
                        comp_types, init_types
                    ),
                    loc: op_loc.clone(),
                });
            }

            let alloc_fn_def = ctx
                .module
                .fn_defs
                .get("alloc")
                .ok_or_else(|| CompileError {
                    message: format!("`alloc` not defined, required for using {}", op),
                    loc: op_loc.clone(),
                })?;
            let alloc_fn_index = alloc_fn_def.get_absolute_index(ctx.module);

            let value_size = value_type
                .sized_comp_stats(ctx.module)
                .map_err(|err| CompileError {
                    message: err,
                    loc: op_loc.clone(),
                })?
                .byte_length;

            let return_addr_local_index = ctx.locals_last_index;
            ctx.non_arg_locals.push(WasmType::I32);
            ctx.locals_last_index += 1;

            let init_load = compile_load(
                ctx,
                &value_type,
                Box::new(WasmInstr::LocalGet {
                    local_index: return_addr_local_index,
                }),
                0,
            )
            .map_err(|err| CompileError {
                message: err,
                loc: op_loc.clone(),
            })?;

            let init_store_instr = compile_set(ctx, init_instr, init_load, op_loc)?;

            WasmInstr::MultiValueEmit {
                values: vec![
                    WasmInstr::NoTypeCheck {
                        instr: Box::new(WasmInstr::Call {
                            fn_index: alloc_fn_index,
                            fn_type_index: 0, // doesn't matter as it's inside NoTypeCheck
                            args: vec![
                                alloc_id_instr,
                                WasmInstr::I32Const {
                                    value: value_size as i32,
                                },
                            ],
                        }),
                    },
                    WasmInstr::Set {
                        bind: WasmSetBind::Local {
                            index: return_addr_local_index,
                        },
                    },
                    init_store_instr,
                    WasmInstr::LocalGet {
                        local_index: return_addr_local_index,
                    },
                ],
            }
        }
        (
            "let" | ":",
            [SExpr::Atom {
                value: local_name,
                loc: name_loc,
                kind: AtomKind::Symbol,
            }, value_type],
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

            let value_type = parse_lole_type(value_type, ctx.module)?;

            let start_index = ctx.locals_last_index;
            let comp_count = value_type.emit_components(&ctx.module, &mut ctx.non_arg_locals);

            ctx.locals_last_index += comp_count;
            ctx.locals.insert(
                local_name.clone(),
                LocalDef {
                    index: start_index,
                    value_type,
                },
            );

            let values = (start_index..(start_index + comp_count))
                .map(|i| WasmInstr::LocalGet { local_index: i })
                .collect();

            WasmInstr::NoEmit {
                instr: Box::new(WasmInstr::MultiValueEmit { values }),
            }
        }
        ("set" | "=", [bind, value]) => {
            let value_instr = compile_instr(value, ctx)?;
            let bind_instr = compile_instr(bind, ctx)?;

            let value_types = get_type(ctx, &value_instr)?;
            let bind_types = get_type(ctx, &bind_instr)?;

            if value_types != bind_types {
                return Err(CompileError {
                    message: format!(
                        "TypeError: Invalid types for '{op}', needed {:?}, got {:?}",
                        bind_types, value_types
                    ),
                    loc: op_loc.clone(),
                });
            }

            compile_set(ctx, value_instr, bind_instr, op_loc)?
        }
        (
            "get" | ".",
            [lhs, SExpr::Atom {
                value: f_name,
                loc: f_name_loc,
                kind: AtomKind::Symbol,
            }],
        ) => {
            if let SExpr::Atom {
                value: local_name,
                loc: name_loc,
                kind: AtomKind::Symbol,
            } = lhs
            {
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

                let LoleType::StructInstance { name: s_name } = &local.value_type else {
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

                let Some(field) = struct_def.fields.iter().find(|f| f.name == *f_name) else {
                    return Err(CompileError {
                        message: format!("Unknown field {f_name} in struct {s_name}"),
                        loc: f_name_loc.clone(),
                    });
                };

                return Ok(compile_local_get(
                    ctx.module,
                    local.index + field.field_index,
                    &field.value_type,
                ));
            }

            let lhs_instr = compile_instr(lhs, ctx)?;

            if let WasmInstr::StructGet {
                struct_name,
                base_index,
                ..
            } = lhs_instr
            {
                // safe to unwrap as it was already checked in `StructGet`
                let struct_def = ctx.module.struct_defs.get(&struct_name).unwrap();

                let Some(field) = struct_def.fields.iter().find(|f| f.name == *f_name) else {
                    return Err(CompileError {
                        message: format!("Unknown field {f_name} in struct {struct_name}"),
                        loc: f_name_loc.clone(),
                    });
                };

                return Ok(compile_local_get(
                    ctx.module,
                    base_index + field.field_index,
                    &field.value_type,
                ));
            };

            if let WasmInstr::StructLoad {
                struct_name,
                address_instr,
                base_byte_offset,
                ..
            } = lhs_instr
            {
                // safe to unwrap as it was already checked in `StructLoad`
                let struct_def = ctx.module.struct_defs.get(&struct_name).unwrap();

                let Some(field) = struct_def.fields.iter().find(|f| f.name == *f_name) else {
                    return Err(CompileError {
                        message: format!("Unknown field {f_name} in struct {struct_name}"),
                        loc: f_name_loc.clone(),
                    });
                };

                return compile_load(
                    ctx,
                    &field.value_type,
                    address_instr,
                    base_byte_offset + field.byte_offset,
                )
                .map_err(|e| CompileError {
                    message: e,
                    loc: op_loc.clone(),
                });
            }

            return Err(CompileError {
                message: format!("Invalid arguments for {op}"),
                loc: op_loc.clone(),
            });
        }
        // TODO(feat): support custom aligns and offsets
        (
            "@" | "load",
            [SExpr::Atom {
                value: load_kind,
                loc: kind_loc,
                kind: AtomKind::Symbol,
            }, address_expr],
        ) => {
            let address_instr = Box::new(compile_instr(address_expr, ctx)?);

            let Some(_) = ctx.module.struct_defs.get(load_kind) else {
                return Ok(WasmInstr::Load {
                    kind: parse_load_kind(load_kind).map_err(|e| CompileError {
                        message: e,
                        loc: kind_loc.clone(),
                    })?,
                    align: 0,
                    offset: 0,
                    address_instr,
                })
            };

            compile_load(
                ctx,
                &LoleType::StructInstance {
                    name: load_kind.clone(),
                },
                address_instr,
                0,
            )
            .map_err(|err| CompileError {
                message: err,
                loc: op_loc.clone(),
            })?
        }
        ("@" | "load", [load, offset]) => {
            let load_instr = compile_instr(load, ctx)?;
            let offset_instr = compile_instr(offset, ctx)?;

            match load_instr {
                WasmInstr::Load {
                    kind,
                    align,
                    offset,
                    address_instr,
                } => WasmInstr::Load {
                    kind,
                    align,
                    offset,
                    address_instr: Box::new(WasmInstr::BinaryOp {
                        kind: WasmBinaryOpKind::I32Add,
                        lhs: address_instr,
                        rhs: Box::new(offset_instr),
                    }),
                },
                // TODO: this is like real mess
                WasmInstr::StructLoad {
                    struct_name,
                    address_instr,
                    address_local_index,
                    primitive_loads,
                    base_byte_offset,
                } => {
                    let new_address = Box::new(WasmInstr::BinaryOp {
                        kind: WasmBinaryOpKind::I32Add,
                        lhs: address_instr,
                        rhs: Box::new(offset_instr),
                    });

                    WasmInstr::StructLoad {
                        struct_name,
                        address_instr: new_address.clone(),
                        address_local_index,
                        base_byte_offset,
                        primitive_loads: primitive_loads
                            .into_iter()
                            .map(|load_instr| {
                                let WasmInstr::Load {
                                    kind,
                                    align,
                                    offset,
                                    ..
                                } = load_instr else { unreachable!() };

                                WasmInstr::Load {
                                    kind,
                                    align,
                                    offset,
                                    address_instr: new_address.clone(),
                                }
                            })
                            .collect(),
                    }
                }
                _ => {
                    return Err(CompileError {
                        message: format!("Invalid arguments for {op}"),
                        loc: op_loc.clone(),
                    })
                }
            }
        }
        (fn_name, args) => {
            let Some(fn_def) = ctx.module.fn_defs.get(fn_name) else {
                return Err(CompileError {
                    message: format!("Unknown instruction or function: {fn_name}"),
                    loc: op_loc.clone()
                });
            };

            let fn_type_index = fn_def.type_index;

            let fn_type = ctx
                .module
                .wasm_module
                .types
                .get(fn_type_index as usize)
                .ok_or_else(|| CompileError::unreachable(file!(), line!()))?;

            let args = compile_instrs(args, ctx)?;
            let arg_types = get_types(ctx, &args)?;
            if fn_type.inputs != arg_types {
                return Err(CompileError {
                    message: format!(
                        "TypeError: Mismatched arguments for function \
                            '{fn_name}', expected {inputs:?}, got {args:?}",
                        inputs = fn_type.inputs,
                        args = arg_types,
                    ),
                    loc: op_loc.clone(),
                });
            }

            WasmInstr::Call {
                fn_index: fn_def.get_absolute_index(ctx.module),
                fn_type_index,
                args,
            }
        }
    };

    Ok(instr)
}

fn get_deferred(
    defer_label: &str,
    ctx: &mut FnContext,
) -> Option<Result<Vec<WasmInstr>, CompileError>> {
    let Some(deferred) = ctx.defers.get(defer_label) else {
        return None;
    };

    let mut deferred = deferred.clone();
    deferred.reverse();

    Some(compile_instrs(&deferred, ctx))
}

fn compile_load(
    ctx: &mut FnContext,
    value_type: &LoleType,
    address_instr: Box<WasmInstr>,
    base_byte_offset: u32,
) -> Result<WasmInstr, String> {
    if let LoleType::Primitive(value_type) = value_type {
        return Ok(WasmInstr::Load {
            kind: WasmLoadKind::from_value_type(value_type)?,
            align: 1,
            offset: base_byte_offset,
            address_instr: address_instr.clone(),
        });
    }

    let LoleType::StructInstance { name } = value_type else {
        unreachable!()
    };

    let mut components = vec![];
    let mut stats = EmitComponentStats {
        count: 0,
        byte_length: base_byte_offset,
    };

    value_type.emit_sized_component_stats(ctx.module, &mut stats, &mut components)?;

    let address_local_index = ctx.locals_last_index;
    ctx.non_arg_locals.push(WasmType::I32);
    ctx.locals_last_index += 1;

    let mut primitive_loads = vec![];
    for comp in components.into_iter() {
        primitive_loads.push(WasmInstr::Load {
            kind: WasmLoadKind::from_value_type(&comp.value_type)?,
            align: 1,
            offset: comp.byte_offset,
            address_instr: Box::new(WasmInstr::LocalGet {
                local_index: address_local_index,
            }),
        });
    }

    Ok(WasmInstr::StructLoad {
        struct_name: name.clone(),
        address_instr,
        address_local_index,
        base_byte_offset,
        primitive_loads,
    })
}

fn compile_local_get(ctx: &ModuleContext, base_index: u32, value_type: &LoleType) -> WasmInstr {
    let comp_count = value_type.emit_components(ctx, &mut vec![]);
    if comp_count == 1 {
        return WasmInstr::LocalGet {
            local_index: base_index,
        };
    }

    let LoleType::StructInstance { name } = value_type else {
        unreachable!()
    };

    let mut primitive_gets = vec![];
    for field_index in 0..comp_count {
        primitive_gets.push(WasmInstr::LocalGet {
            local_index: base_index + field_index as u32,
        });
    }

    WasmInstr::StructGet {
        struct_name: name.clone(),
        base_index,
        primitive_gets,
    }
}

fn compile_const_instr(expr: &SExpr, ctx: &ModuleContext) -> Result<WasmInstr, CompileError> {
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

fn compile_set(
    ctx: &mut FnContext,
    value_instr: WasmInstr,
    bind_instr: WasmInstr,
    bind_loc: &Location,
) -> Result<WasmInstr, CompileError> {
    let mut values = vec![];
    compile_set_binds(&mut values, ctx, bind_instr, bind_loc, None)?;
    values.push(value_instr);
    values.reverse();

    Ok(WasmInstr::NoTypeCheck {
        instr: Box::new(WasmInstr::MultiValueEmit { values }),
    })
}

// TODO: figure out better location
fn compile_set_binds(
    output: &mut Vec<WasmInstr>,
    ctx: &mut FnContext,
    bind_instr: WasmInstr,
    bind_loc: &Location,
    address_index: Option<u32>,
) -> Result<(), CompileError> {
    Ok(match bind_instr {
        WasmInstr::LocalGet { local_index } => {
            output.push(WasmInstr::Set {
                bind: WasmSetBind::Local { index: local_index },
            });
        }
        WasmInstr::GlobalGet { global_index } => {
            output.push(WasmInstr::Set {
                bind: WasmSetBind::Global {
                    index: global_index,
                },
            });
        }
        WasmInstr::Load {
            kind,
            align,
            offset,
            address_instr,
        } => {
            let value_local_index = ctx.locals_last_index;
            ctx.non_arg_locals.push(kind.get_value_type());
            ctx.locals_last_index += 1;

            let address_instr = match address_index {
                Some(local_index) => Box::new(WasmInstr::LocalGet { local_index }),
                None => address_instr,
            };

            output.push(WasmInstr::Set {
                bind: WasmSetBind::Memory {
                    align,
                    offset,
                    kind: WasmStoreKind::from_load_kind(&kind),
                    address_instr,
                    value_local_index,
                },
            });
        }
        WasmInstr::StructLoad {
            primitive_loads,
            address_instr,
            address_local_index,
            ..
        } => {
            let mut values = vec![];

            for value in primitive_loads {
                compile_set_binds(&mut values, ctx, value, bind_loc, Some(address_local_index))?;
            }

            values.push(WasmInstr::Set {
                bind: WasmSetBind::Local {
                    index: address_local_index,
                },
            });
            values.push(*address_instr);

            values.reverse();

            output.push(WasmInstr::MultiValueEmit { values });
        }
        // TODO: improve this? (StructGet/MultiValueEmit/NoEmit)
        WasmInstr::StructGet { primitive_gets, .. } => {
            for value in primitive_gets {
                compile_set_binds(output, ctx, value, bind_loc, address_index)?;
            }
        }
        WasmInstr::MultiValueEmit { values } => {
            for value in values {
                compile_set_binds(output, ctx, value, bind_loc, address_index)?;
            }
        }
        WasmInstr::NoEmit { instr } => {
            compile_set_binds(output, ctx, *instr, bind_loc, address_index)?;
        }
        _ => {
            return Err(CompileError {
                message: format!("Invalid bind"),
                loc: bind_loc.clone(),
            });
        }
    })
}

// types

fn parse_lole_type(expr: &SExpr, ctx: &ModuleContext) -> Result<LoleType, CompileError> {
    match parse_wasm_type(expr, ctx) {
        Ok(value_type) => Ok(LoleType::Primitive(value_type)),
        Err(err) => {
            if let SExpr::Atom { value: s_name, .. } = expr {
                if ctx.struct_defs.contains_key(s_name) {
                    return Ok(LoleType::StructInstance {
                        name: s_name.clone(),
                    });
                }
            }

            Err(err)
        }
    }
}

fn parse_wasm_type(expr: &SExpr, ctx: &ModuleContext) -> Result<WasmType, CompileError> {
    match expr {
        SExpr::Atom {
            kind: AtomKind::Symbol,
            value: name,
            ..
        } => match &name[..] {
            "bool" | "u8" | "u32" | "i32" | "ptr" => return Ok(WasmType::I32),
            "i64" | "u64" => return Ok(WasmType::I64),
            "f32" => return Ok(WasmType::F32),
            "f64" => return Ok(WasmType::F64),
            "v128" => return Ok(WasmType::V128),
            "funcref" => return Ok(WasmType::FuncRef),
            "externref" => return Ok(WasmType::ExternRef),
            _ => {}
        },
        SExpr::List { value, .. } => match &value[..] {
            [SExpr::Atom {
                kind: AtomKind::Symbol,
                value,
                ..
            }, ptr_data]
                if value == "&" || value == "&*" =>
            {
                // ignoring ptr data type
                parse_lole_type(ptr_data, ctx)?;

                // pointer type
                return Ok(WasmType::I32);
            }
            _ => {}
        },
        _ => {}
    };

    Err(CompileError {
        message: format!("Unknown value type: {expr}"),
        loc: expr.loc().clone(),
    })
}

fn parse_load_kind(kind: &str) -> Result<WasmLoadKind, String> {
    match kind {
        "i32" => Ok(WasmLoadKind::I32),
        "i32/u8" => Ok(WasmLoadKind::I32U8),
        _ => Err(format!("Unknown load kind: {kind}")),
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
