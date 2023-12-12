use crate::{ast::*, ir_lisp::*, lexer_lisp::*, lowering_lisp::*, wasm::*};
use alloc::{
    boxed::Box,
    collections::{BTreeMap, BTreeSet},
    format, str,
    string::String,
    vec,
    vec::Vec,
};

const DEFER_UNTIL_RETURN_LABEL: &str = "return";
const HEAP_ALLOC_ID: u32 = 1;

pub fn parse(exprs: &Vec<SExpr>) -> Result<WasmModule, LoError> {
    let mut ctx = ModuleContext::default();

    for expr in exprs {
        compile_top_level_expr(&expr, &mut ctx)?;
    }

    process_delayed_actions(&mut ctx)?;

    Ok(ctx.wasm_module.take())
}

fn process_delayed_actions(ctx: &mut ModuleContext) -> Result<(), LoError> {
    // push function exports
    for fn_export in &ctx.fn_exports {
        let Some(fn_def) = ctx.fn_defs.get(&fn_export.in_name) else {
            return Err(LoError {
                message: format!("Cannot export unknown function {}", fn_export.in_name),
                loc: fn_export.loc.clone(),
            });
        };

        ctx.wasm_module.borrow_mut().exports.push(WasmExport {
            export_type: WasmExportType::Func,
            export_name: fn_export.out_name.clone(),
            exported_item_index: ctx.imported_fns_count + fn_def.fn_index,
        });
    }

    // push function codes
    for fn_body in ctx.fn_bodies.take() {
        let fn_def = ctx
            .fn_defs
            .values()
            .find(|fd| fd.local && fd.fn_index == fn_body.fn_index)
            .unwrap();

        let mut fn_ctx = FnContext {
            module: &ctx,
            lo_fn_type: &fn_def.kind,
            locals_last_index: fn_body.locals_last_index,
            non_arg_wasm_locals: vec![],
            defers: BTreeMap::default(),
        };

        let locals_block = Block {
            block_type: BlockType::Block,
            parent: None,
            locals: fn_body.locals,
        };

        let mut block_ctx = BlockContext {
            module: &ctx,
            fn_ctx: &mut fn_ctx,
            block: Block {
                block_type: BlockType::Function,
                parent: Some(&locals_block),
                locals: BTreeMap::new(),
            },
        };

        let mut lo_exprs = compile_block(&fn_body.body, &mut block_ctx)?;
        if let Some(values) = get_deferred(&mut block_ctx, DEFER_UNTIL_RETURN_LABEL) {
            lo_exprs.append(&mut values?);
        };

        let mut locals = Vec::<WasmLocals>::new();
        for local_type in &block_ctx.fn_ctx.non_arg_wasm_locals {
            if let Some(wasm_locals) = locals.last_mut() {
                if (*wasm_locals).value_type == *local_type {
                    wasm_locals.count += 1;
                    continue;
                }
            }
            locals.push(WasmLocals {
                count: 1,
                value_type: local_type.clone(),
            });
        }

        // TODO: move to better place
        let mut instrs = vec![];
        lower_exprs(&mut instrs, lo_exprs);

        ctx.wasm_module.borrow_mut().codes.push(WasmFn {
            locals,
            expr: WasmExpr { instrs },
        });
    }

    Ok(())
}

fn compile_block(exprs: &[SExpr], ctx: &mut BlockContext) -> Result<Vec<LoInstr>, LoError> {
    let instrs = compile_instrs(exprs, ctx)?;
    for (instr, expr) in instrs.iter().zip(exprs) {
        if let LoInstr::NoEmit { .. } = instr {
            continue;
        }

        let instr_type = instr.get_type(ctx.module);
        if instr_type != LoType::Void {
            return Err(LoError {
                message: format!("TypeError: Excess values"),
                loc: expr.loc().clone(),
            });
        }
    }
    Ok(instrs)
}

fn compile_top_level_expr(expr: &SExpr, ctx: &mut ModuleContext) -> Result<(), LoError> {
    let SExpr::List { value: items, .. } = expr else {
        return Err(LoError {
            message: format!("Unexpected atom"),
            loc: expr.loc().clone(),
        });
    };

    let [SExpr::Atom {
        value: op,
        loc: op_loc,
        kind,
    }, args @ ..] = &items[..]
    else {
        return Err(LoError {
            message: format!("Expected operation, got a simple list"),
            loc: expr.loc().clone(),
        });
    };

    if *kind != AtomKind::Symbol {
        return Err(LoError {
            message: format!("Expected operation, got a string"),
            loc: expr.loc().clone(),
        });
    }

    match op.as_str() {
        "mem" => match args {
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
                if ctx.memories.contains_key(mem_name) {
                    return Err(LoError {
                        message: format!("Duplicate memory definition: {mem_name}"),
                        loc: mem_name_loc.clone(),
                    });
                }

                let min_memory = min_memory.parse().map_err(|_| LoError {
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
                        Some(max_memory.parse::<u32>().map_err(|_| LoError {
                            message: format!("Parsing {op} :max (u32) failed"),
                            loc: max_memory_loc.clone(),
                        })?)
                    }
                    [] => None,
                    _ => {
                        return Err(LoError {
                            message: format!("Invalid arguments for {op}"),
                            loc: op_loc.clone(),
                        })
                    }
                };

                let memory_index = ctx.wasm_module.borrow().memories.len() as u32;
                ctx.memories.insert(mem_name.clone(), memory_index);
                ctx.wasm_module.borrow_mut().memories.push(WasmLimits {
                    min: min_memory,
                    max: max_memory,
                });
            }
            _ => {
                return Err(LoError {
                    message: format!("Invalid arguments for {op}"),
                    loc: op_loc.clone(),
                })
            }
        },
        "fn" => match args {
            [SExpr::Atom {
                value: fn_name,
                loc: fn_name_loc,
                kind: AtomKind::Symbol,
            }, SExpr::List {
                value: input_exprs, ..
            }, output_expr, SExpr::List { value: body, .. }] => {
                if ctx.fn_defs.contains_key(fn_name) {
                    return Err(LoError {
                        message: format!("Cannot redefine function: {fn_name}"),
                        loc: fn_name_loc.clone(),
                    });
                }

                let mut locals = BTreeMap::new();

                let mut lo_inputs = vec![];
                let mut wasm_inputs = vec![];
                for input_expr in input_exprs.iter() {
                    let SExpr::List {
                        value: name_and_type,
                        ..
                    } = input_expr
                    else {
                        return Err(LoError {
                            message: format!("Unexpected atom in function params list"),
                            loc: input_expr.loc().clone(),
                        });
                    };

                    let [SExpr::Atom {
                        value: p_name,
                        loc: p_name_loc,
                        kind: AtomKind::Symbol,
                    }, p_type] = &name_and_type[..]
                    else {
                        return Err(LoError {
                            message: format!(
                                "Expected name and parameter pairs in function params list"
                            ),
                            loc: input_expr.loc().clone(),
                        });
                    };

                    if locals.contains_key(p_name) {
                        return Err(LoError {
                            message: format!(
                                "Found function param with conflicting name: {p_name}"
                            ),
                            loc: p_name_loc.clone(),
                        });
                    }

                    let local_index = wasm_inputs.len() as u32;
                    let value_type = parse_lo_type(p_type, &ctx)?;
                    value_type.emit_components(&ctx, &mut wasm_inputs);

                    locals.insert(
                        p_name.clone(),
                        LocalDef {
                            index: local_index,
                            value_type: value_type.clone(),
                        },
                    );
                    lo_inputs.push(value_type);
                }

                let lo_output = parse_lo_type(output_expr, &ctx)?;
                let mut wasm_outputs = vec![];
                lo_output.emit_components(&ctx, &mut wasm_outputs);

                let fn_index = ctx.wasm_module.borrow_mut().functions.len() as u32;
                let locals_last_index = wasm_inputs.len() as u32;

                let type_index = ctx.insert_fn_type(WasmFnType {
                    inputs: wasm_inputs,
                    outputs: wasm_outputs,
                });

                ctx.wasm_module.borrow_mut().functions.push(type_index);
                let fn_def = FnDef {
                    local: true,
                    fn_index,
                    type_index,
                    kind: LoFnType {
                        inputs: lo_inputs,
                        output: lo_output,
                    },
                };
                ctx.fn_defs.insert(fn_name.clone(), fn_def);
                ctx.fn_bodies.borrow_mut().push(FnBody {
                    fn_index,
                    type_index,
                    locals,
                    locals_last_index,
                    body: body.clone(),
                });
            }
            _ => {
                return Err(LoError {
                    message: format!("Invalid arguments for {op}"),
                    loc: op_loc.clone(),
                })
            }
        },
        "import" => match args {
            [SExpr::Atom {
                value: fn_literal, ..
            }, SExpr::Atom {
                value: fn_name,
                loc: fn_name_loc,
                kind: AtomKind::Symbol,
            }, SExpr::List {
                value: input_exprs, ..
            }, output_expr, SExpr::Atom {
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
                    return Err(LoError {
                        message: format!("Cannot redefine function: {fn_name}"),
                        loc: fn_name_loc.clone(),
                    });
                }

                let mut param_names = BTreeSet::new();

                let mut lo_inputs = vec![];
                let mut wasm_inputs = vec![];
                for input_expr in input_exprs.iter() {
                    let SExpr::List {
                        value: name_and_type,
                        ..
                    } = input_expr
                    else {
                        return Err(LoError {
                            message: format!("Unexpected atom in function params list"),
                            loc: input_expr.loc().clone(),
                        });
                    };

                    let [SExpr::Atom {
                        value: p_name,
                        loc: name_loc,
                        kind: AtomKind::Symbol,
                    }, p_type] = &name_and_type[..]
                    else {
                        return Err(LoError {
                            message: format!(
                                "Expected name and parameter pairs in function params list"
                            ),
                            loc: input_expr.loc().clone(),
                        });
                    };

                    if param_names.contains(p_name) {
                        return Err(LoError {
                            message: format!(
                                "Found function param with conflicting name: {p_name}"
                            ),
                            loc: name_loc.clone(),
                        });
                    }

                    let value_type = parse_lo_type(p_type, &ctx)?;
                    value_type.emit_components(&ctx, &mut wasm_inputs);
                    lo_inputs.push(value_type);

                    param_names.insert(p_name.clone());
                }

                let mut wasm_outputs = vec![];
                let lo_output = parse_lo_type(output_expr, &ctx)?;
                lo_output.emit_components(&ctx, &mut wasm_outputs);

                let type_index = ctx.insert_fn_type(WasmFnType {
                    inputs: wasm_inputs,
                    outputs: wasm_outputs,
                });

                let fn_index = ctx.imported_fns_count;
                ctx.imported_fns_count += 1;

                let fn_def = FnDef {
                    local: false,
                    fn_index,
                    type_index,
                    kind: LoFnType {
                        inputs: lo_inputs,
                        output: lo_output,
                    },
                };
                ctx.fn_defs.insert(fn_name.clone(), fn_def);
                ctx.wasm_module.borrow_mut().imports.push(WasmImport {
                    module_name: module_name.clone(),
                    item_name: extern_fn_name.clone(),
                    item_desc: WasmImportDesc::Func { type_index },
                });
            }
            _ => {
                return Err(LoError {
                    message: format!("Invalid arguments for {op}"),
                    loc: op_loc.clone(),
                })
            }
        },
        "export" => match args {
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
                let Some(memory_index) = ctx.memories.get(in_name).cloned() else {
                    return Err(LoError {
                        message: format!("Cannot export mem {in_name}, not found"),
                        loc: name_loc.clone(),
                    });
                };

                ctx.wasm_module.borrow_mut().exports.push(WasmExport {
                    export_type: WasmExportType::Mem,
                    export_name: out_name.clone(),
                    exported_item_index: memory_index,
                });
            }
            [SExpr::Atom {
                value: in_name,
                loc: in_name_loc,
                ..
            }, SExpr::Atom {
                value: as_literal, ..
            }, SExpr::Atom {
                value: out_name, ..
            }] if as_literal == ":as" => {
                ctx.fn_exports.push(FnExport {
                    in_name: in_name.clone(),
                    out_name: out_name.clone(),
                    loc: in_name_loc.clone(),
                });
            }
            _ => {
                return Err(LoError {
                    message: format!("Invalid arguments for {op}"),
                    loc: op_loc.clone(),
                })
            }
        },
        "struct" => match args {
            [SExpr::Atom {
                value: struct_name,
                loc: name_loc,
                kind: AtomKind::Symbol,
            }, field_defs @ ..] => {
                if ctx.struct_defs.contains_key(struct_name) {
                    return Err(LoError {
                        message: format!("Cannot redefine struct {struct_name}"),
                        loc: name_loc.clone(),
                    });
                }

                ctx.struct_defs.insert(
                    struct_name.clone(),
                    StructDef {
                        fields: vec![],
                        fully_defined: false,
                    },
                );

                let mut struct_fields = build_struct_fields(field_defs, struct_name, ctx)?;

                let struct_def = ctx.struct_defs.get_mut(struct_name).unwrap();
                struct_def.fields.append(&mut struct_fields);
                struct_def.fully_defined = true;
            }
            _ => {
                return Err(LoError {
                    message: format!("Invalid arguments for {op}"),
                    loc: op_loc.clone(),
                })
            }
        },
        "global" => {
            let (mutable, global_name, global_type_expr, global_value, name_loc) = match args {
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
                    return Err(LoError {
                        message: format!("Invalid arguments for {op}"),
                        loc: op_loc.clone(),
                    })
                }
            };

            if ctx.globals.contains_key(global_name) {
                return Err(LoError {
                    message: format!("Cannot redefine global: {global_name}"),
                    loc: name_loc.clone(),
                });
            }

            let lo_type = parse_lo_type(global_type_expr, ctx)?;
            let Some(wasm_type) = lo_type.to_wasm_type() else {
                return Err(LoError {
                    message: format!("Unsupported type: {global_type_expr}"),
                    loc: global_type_expr.loc().clone(),
                });
            };

            ctx.globals.insert(
                global_name.clone(),
                GlobalDef {
                    index: ctx.globals.len() as u32,
                    mutable,
                    value_type: lo_type,
                },
            );

            // TODO: move to better place
            let global_instr = compile_const_instr(global_value, &ctx)?;
            let mut initial_value = WasmExpr { instrs: vec![] };
            lower_expr(&mut initial_value.instrs, global_instr);

            ctx.wasm_module.borrow_mut().globals.push(WasmGlobal {
                kind: WasmGlobalKind {
                    value_type: wasm_type,
                    mutable,
                },
                initial_value,
            });
        }
        "data" => {
            let [SExpr::Atom {
                value: offset,
                loc,
                kind: AtomKind::Symbol,
            }, SExpr::Atom {
                value: data_base64,
                loc: _,
                kind,
            }] = args
            else {
                return Err(LoError {
                    message: format!("Invalid arguments for {op}"),
                    loc: op_loc.clone(),
                });
            };

            let offset = offset.parse().map_err(|_| LoError {
                message: format!("Parsing i32 (implicit) failed"),
                loc: loc.clone(),
            })?;

            let bytes = match *kind {
                AtomKind::String => data_base64.as_bytes().iter().map(|b| *b).collect(),
                AtomKind::Symbol => base64_decode(data_base64.as_bytes()),
            };

            // TODO: move to better place
            ctx.wasm_module.borrow_mut().datas.push(WasmData::Active {
                offset: WasmExpr {
                    instrs: vec![WasmInstr::I32Const { value: offset }],
                },
                bytes,
            });
        }
        _ => {
            return Err(LoError {
                message: format!("Unknown operation: {op}"),
                loc: op_loc.clone(),
            })
        }
    }

    Ok(())
}

fn build_struct_fields(
    exprs: &[SExpr],
    struct_name: &str,
    ctx: &ModuleContext,
) -> Result<Vec<StructField>, LoError> {
    let mut field_index = 0;
    let mut byte_offset = 0;

    let mut fields = Vec::<StructField>::new();
    for field_def in exprs {
        let SExpr::List {
            value: name_and_type,
            ..
        } = field_def
        else {
            return Err(LoError {
                message: format!("Unexpected atom in fields list of struct {struct_name}"),
                loc: field_def.loc().clone(),
            });
        };

        let [SExpr::Atom {
            value: f_name,
            loc: name_loc,
            kind: AtomKind::Symbol,
        }, f_type_expr] = &name_and_type[..]
        else {
            return Err(LoError {
                message: format!(
                    "Expected name and parameter pairs in fields list of struct {struct_name}"
                ),
                loc: field_def.loc().clone(),
            });
        };

        if fields.iter().find(|f| &f.name == f_name).is_some() {
            return Err(LoError {
                message: format!(
                    "Found duplicate struct field name: '{f_name}' of struct {struct_name}"
                ),
                loc: name_loc.clone(),
            });
        }

        let field_type = parse_lo_type(f_type_expr, ctx)?;

        let mut stats = EmitComponentStats::default();
        field_type
            .emit_sized_component_stats(ctx, &mut stats, &mut vec![])
            .map_err(|err| LoError {
                message: err,
                loc: f_type_expr.loc().clone(),
            })?;

        fields.push(StructField {
            name: f_name.clone(),
            value_type: field_type,
            field_index,
            byte_offset,
        });

        field_index += stats.count;
        byte_offset += stats.byte_length;
    }
    Ok(fields)
}

fn compile_instrs(exprs: &[SExpr], ctx: &mut BlockContext) -> Result<Vec<LoInstr>, LoError> {
    exprs.iter().map(|expr| compile_instr(expr, ctx)).collect()
}

fn compile_instr(expr: &SExpr, ctx: &mut BlockContext) -> Result<LoInstr, LoError> {
    let items = match expr {
        SExpr::List { value: items, .. } => items,
        SExpr::Atom { value, loc, kind } => {
            if *kind == AtomKind::String {
                let string_len = value.as_bytes().len() as u32;

                let string_ptr_ = ctx.module.string_pool.borrow().get(value).cloned();
                let string_ptr = match string_ptr_ {
                    Some(string_ptr) => string_ptr,
                    None => {
                        let new_string_ptr = *ctx.module.data_size.borrow();
                        ctx.module
                            .string_pool
                            .borrow_mut()
                            .insert(value.clone(), new_string_ptr);

                        *ctx.module.data_size.borrow_mut() += string_len;
                        ctx.module
                            .wasm_module
                            .borrow_mut()
                            .datas
                            .push(WasmData::Active {
                                // TODO: move to better place
                                offset: WasmExpr {
                                    instrs: vec![WasmInstr::I32Const {
                                        value: new_string_ptr as i32,
                                    }],
                                },
                                bytes: value.as_bytes().to_vec(),
                            });
                        new_string_ptr
                    }
                };

                return Ok(LoInstr::MultiValueEmit {
                    values: vec![
                        LoInstr::U32Const { value: string_ptr },
                        LoInstr::U32Const { value: string_len },
                    ],
                });
            }

            if value == "true" {
                return Ok(LoInstr::Casted {
                    value_type: LoType::Bool,
                    expr: Box::new(LoInstr::U32Const { value: 1 }),
                });
            }

            if value == "false" {
                return Ok(LoInstr::Casted {
                    value_type: LoType::Bool,
                    expr: Box::new(LoInstr::U32Const { value: 0 }),
                });
            }

            if value.chars().all(|c| c.is_ascii_digit()) {
                return Ok(LoInstr::U32Const {
                    value: (value.parse().map_err(|_| LoError {
                        message: format!("Parsing u32 (implicit) failed"),
                        loc: loc.clone(),
                    })?),
                });
            }

            if let Some(global) = ctx.module.globals.get(value.as_str()) {
                return Ok(LoInstr::GlobalGet {
                    global_index: global.index,
                });
            };

            let Some(local) = ctx.block.get_local(value.as_str()) else {
                return Err(LoError {
                    message: format!("Reading unknown variable: {value}"),
                    loc: loc.clone(),
                });
            };

            return compile_local_get(&ctx.module, local.index, &local.value_type).map_err(
                |message| LoError {
                    message,
                    loc: expr.loc().clone(),
                },
            );
        }
    };

    let [SExpr::Atom {
        value: op,
        loc: op_loc,
        ..
    }, args @ ..] = &items[..]
    else {
        return Err(LoError {
            message: format!("Expected operation, got a simple list"),
            loc: expr.loc().clone(),
        });
    };

    let instr = match (op.as_str(), &args[..]) {
        ("unreachable", []) => LoInstr::Unreachable {},
        ("drop", [expr]) => {
            let instr = compile_instr(expr, ctx)?;
            let instr_type = instr.get_type(ctx.module);
            let drop_count = instr_type.emit_components(&ctx.module, &mut vec![]);

            LoInstr::Drop {
                value: Box::new(instr),
                drop_count,
            }
        }
        ("do", exprs) => LoInstr::MultiValueEmit {
            values: compile_instrs(exprs, ctx)?,
        },
        (
            "i32",
            [SExpr::Atom {
                value,
                loc,
                kind: AtomKind::Symbol,
            }],
        ) => LoInstr::U32Const {
            value: value.parse().map_err(|_| LoError {
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
        ) => LoInstr::I64Const {
            value: value.parse::<i64>().map_err(|_| LoError {
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
        ) => LoInstr::U32Const {
            value: value.chars().next().unwrap() as u32,
        },
        ("==", [lhs, rhs]) => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Equal,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("!=", [lhs, rhs]) => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32NotEqual,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("not", [lhs]) => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Equal,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(LoInstr::U32Const { value: 0 }),
        },
        ("<", [lhs, rhs]) => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32LessThenUnsigned,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        (">", [lhs, rhs]) => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32GreaterThanUnsigned,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        (">=", [lhs, rhs]) => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32GreaterEqualUnsigned,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("&&", [lhs, rhs]) => LoInstr::Casted {
            value_type: LoType::Bool,
            expr: Box::new(LoInstr::BinaryOp {
                kind: WasmBinaryOpKind::I32And,
                lhs: Box::new(compile_instr(lhs, ctx)?),
                rhs: Box::new(compile_instr(rhs, ctx)?),
            }),
        },
        ("||", [lhs, rhs]) => LoInstr::Casted {
            value_type: LoType::Bool,
            expr: Box::new(LoInstr::BinaryOp {
                kind: WasmBinaryOpKind::I32Or,
                lhs: Box::new(compile_instr(lhs, ctx)?),
                rhs: Box::new(compile_instr(rhs, ctx)?),
            }),
        },
        ("+", [lhs, rhs]) => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Add,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("+=", [lhs, rhs]) => {
            let bind = compile_instr(lhs, ctx)?;
            let value = LoInstr::BinaryOp {
                kind: WasmBinaryOpKind::I32Add,
                lhs: Box::new(bind.clone()),
                rhs: Box::new(compile_instr(rhs, ctx)?),
            };
            compile_set(ctx, value, bind, lhs.loc())?
        }
        ("-", [lhs, rhs]) => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Sub,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("-=", [lhs, rhs]) => {
            let bind = compile_instr(lhs, ctx)?;
            let value = LoInstr::BinaryOp {
                kind: WasmBinaryOpKind::I32Sub,
                lhs: Box::new(bind.clone()),
                rhs: Box::new(compile_instr(rhs, ctx)?),
            };
            compile_set(ctx, value, bind, lhs.loc())?
        }
        ("*", [lhs, rhs]) => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Mul,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("/", [lhs, rhs]) => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32DivUnsigned,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("%", [lhs, rhs]) => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32RemUnsigned,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("data.size", []) => LoInstr::U32ConstLazy {
            value: ctx.module.data_size.clone(),
        },
        ("memory.size", []) => LoInstr::MemorySize {},
        ("memory.grow", [size_expr]) => {
            let size = compile_instr(size_expr, ctx)?;
            let size_type = size.get_type(ctx.module);

            if size_type != LoType::U32 {
                return Err(LoError {
                    message: format!("Invalid arguments for {op}"),
                    loc: size_expr.loc().clone(),
                });
            };

            LoInstr::MemoryGrow {
                size: Box::new(size),
            }
        }
        ("debug.typeof", [sub_expr]) => {
            let lo_instr = compile_instr(sub_expr, ctx)?;
            let lo_type = lo_instr.get_type(ctx.module);
            crate::wasi_io::debug(format!(
                "{}",
                String::from(LoError {
                    message: format!("{expr} = {}", lo_type),
                    loc: expr.loc().clone(),
                })
            ));
            LoInstr::Casted {
                value_type: LoType::Void,
                expr: Box::new(LoInstr::MultiValueEmit { values: vec![] }),
            }
        }
        (
            "if",
            [cond, SExpr::List {
                value: then_branch,
                loc: _,
            }, SExpr::List {
                value: else_branch,
                loc: _,
            }],
        ) => {
            let then_branch = compile_block(
                then_branch,
                &mut BlockContext {
                    module: ctx.module,
                    fn_ctx: ctx.fn_ctx,
                    block: Block {
                        block_type: BlockType::Block,
                        parent: Some(&ctx.block),
                        locals: BTreeMap::new(),
                    },
                },
            )?;

            let else_branch = Some(compile_block(
                else_branch,
                &mut BlockContext {
                    module: ctx.module,
                    fn_ctx: ctx.fn_ctx,
                    block: Block {
                        block_type: BlockType::Block,
                        parent: Some(&ctx.block),
                        locals: BTreeMap::new(),
                    },
                },
            )?);

            LoInstr::If {
                block_type: LoType::Void,
                cond: Box::new(compile_instr(cond, ctx)?),
                then_branch,
                else_branch,
            }
        }
        (
            "if",
            [cond, SExpr::List {
                value: then_branch,
                loc: _,
            }],
        ) => LoInstr::If {
            block_type: LoType::Void,
            cond: Box::new(compile_instr(cond, ctx)?),
            then_branch: compile_block(
                then_branch,
                &mut BlockContext {
                    module: ctx.module,
                    fn_ctx: ctx.fn_ctx,
                    block: Block {
                        block_type: BlockType::Block,
                        parent: Some(&ctx.block),
                        locals: BTreeMap::new(),
                    },
                },
            )?,
            else_branch: None,
        },
        ("loop", [SExpr::List { value: exprs, .. }]) => {
            let mut ctx = BlockContext {
                module: ctx.module,
                fn_ctx: ctx.fn_ctx,
                block: Block {
                    block_type: BlockType::Loop,
                    parent: Some(&ctx.block),
                    locals: BTreeMap::new(),
                },
            };

            let mut body = compile_block(exprs, &mut ctx)?;

            // add implicit continue
            body.push(LoInstr::Branch { label_index: 0 });

            LoInstr::Block {
                block_type: LoType::Void,
                body: vec![LoInstr::Loop {
                    block_type: LoType::Void,
                    body,
                }],
            }
        }
        ("break", []) => {
            let mut label_index = 1; // 0 = loop, 1 = loop wrapper block

            let mut current_block = &ctx.block;
            loop {
                if current_block.block_type == BlockType::Loop {
                    break;
                }

                current_block = current_block.parent.unwrap();
                label_index += 1;
            }

            LoInstr::Branch { label_index }
        }
        ("continue", []) => {
            let mut label_index = 0; // 0 = loop

            let mut current_block = &ctx.block;
            loop {
                if current_block.block_type == BlockType::Loop {
                    break;
                }

                current_block = &current_block.parent.unwrap();
                label_index += 1;
            }

            LoInstr::Branch { label_index }
        }
        ("return", values) if values.len() < 2 => {
            let value = if let Some(value_expr) = values.get(0) {
                compile_instr(value_expr, ctx)?
            } else {
                LoInstr::Casted {
                    value_type: LoType::Void,
                    expr: Box::new(LoInstr::MultiValueEmit { values: vec![] }),
                }
            };

            let return_type = value.get_type(ctx.module);
            if return_type != ctx.fn_ctx.lo_fn_type.output {
                return Err(LoError {
                    message: format!(
                        "TypeError: Invalid return type, \
                            expected {output:?}, got {return_type:?}",
                        output = ctx.fn_ctx.lo_fn_type.output,
                    ),
                    loc: op_loc.clone(),
                });
            }

            let return_expr = LoInstr::Return {
                value: Box::new(value),
            };
            if let Some(values) = get_deferred(ctx, DEFER_UNTIL_RETURN_LABEL) {
                let mut values = values?;
                values.push(return_expr);
                LoInstr::Casted {
                    value_type: LoType::Void,
                    expr: Box::new(LoInstr::MultiValueEmit { values }),
                }
            } else {
                return_expr
            }
        }
        ("defer", [defer_label_exprs @ .., defer_expr]) => {
            let defer_instr = compile_instr(defer_expr, ctx)?;
            let defer_label = match &defer_label_exprs[..] {
                [SExpr::Atom {
                    kind: AtomKind::Symbol,
                    value: defer_label,
                    loc: _,
                }] => defer_label.clone(),
                [] => String::from(DEFER_UNTIL_RETURN_LABEL),
                _ => {
                    return Err(LoError {
                        message: format!("Invalid arguments for {op}"),
                        loc: op_loc.clone(),
                    })
                }
            };

            let deferred = ctx
                .fn_ctx
                .defers
                .entry(defer_label)
                .or_insert_with(|| vec![]);

            deferred.push(defer_instr);

            LoInstr::Casted {
                value_type: LoType::Void,
                expr: Box::new(LoInstr::MultiValueEmit { values: vec![] }),
            }
        }
        (
            "defer.eval",
            [SExpr::Atom {
                kind: AtomKind::Symbol,
                value: defer_label,
                loc: defer_label_loc,
            }],
        ) => {
            let Some(values) = get_deferred(ctx, defer_label) else {
                return Err(LoError {
                    message: format!("Unknown defer scope: {defer_label}"),
                    loc: defer_label_loc.clone(),
                });
            };

            LoInstr::Casted {
                value_type: LoType::Void,
                expr: Box::new(LoInstr::MultiValueEmit { values: values? }),
            }
        }
        ("as", [value_expr, type_expr]) => {
            let lo_expr = compile_instr(value_expr, ctx)?;
            let value_type = parse_lo_type(type_expr, ctx.module)?;

            LoInstr::Casted {
                value_type,
                expr: Box::new(lo_expr),
            }
        }
        ("sizeof", [type_expr]) => {
            let value_type = parse_lo_type(type_expr, &ctx.module)?;

            LoInstr::U32Const {
                value: value_type
                    .sized_comp_stats(&ctx.module)
                    .map_err(|err| LoError {
                        message: err,
                        loc: op_loc.clone(),
                    })?
                    .byte_length as u32,
            }
        }
        ("new", [type_expr, init_expr, other @ ..]) => {
            let alloc_id_instr = match other {
                [] => LoInstr::U32Const {
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
                    return Err(LoError {
                        message: format!("Invalid arguments for {op}"),
                        loc: op_loc.clone(),
                    });
                }
            };

            let value_type = parse_lo_type(type_expr, &ctx.module)?;

            let init_instr = compile_instr(init_expr, ctx)?;
            let init_type = init_instr.get_type(ctx.module);

            if init_type != value_type {
                return Err(LoError {
                    message: format!(
                        "TypeError: Invalid types for {op}, needed {:?}, got {:?}",
                        value_type, init_type
                    ),
                    loc: op_loc.clone(),
                });
            }

            let alloc_fn_def = ctx.module.fn_defs.get("alloc").ok_or_else(|| LoError {
                message: format!("`alloc` not defined, required for using {}", op),
                loc: op_loc.clone(),
            })?;
            let alloc_fn_index = alloc_fn_def.get_absolute_index(&ctx.module);

            let value_size = value_type
                .sized_comp_stats(&ctx.module)
                .map_err(|err| LoError {
                    message: err,
                    loc: op_loc.clone(),
                })?
                .byte_length;

            let return_addr_local_index = ctx.fn_ctx.locals_last_index;
            ctx.fn_ctx.non_arg_wasm_locals.push(WasmType::I32);
            ctx.fn_ctx.locals_last_index += 1;

            let init_load = compile_load(
                ctx,
                &value_type,
                Box::new(LoInstr::UntypedLocalGet {
                    local_index: return_addr_local_index,
                }),
                0,
            )
            .map_err(|err| LoError {
                message: err,
                loc: op_loc.clone(),
            })?;

            let init_store_instr = compile_set(ctx, init_instr, init_load, op_loc)?;

            LoInstr::Casted {
                value_type: LoType::Pointer(Box::new(value_type)),
                expr: Box::new(LoInstr::MultiValueEmit {
                    values: vec![
                        LoInstr::Call {
                            fn_index: alloc_fn_index,
                            return_type: LoType::Void, // won't be typechecked
                            args: vec![
                                alloc_id_instr,
                                LoInstr::U32Const {
                                    value: value_size as u32,
                                },
                            ],
                        },
                        LoInstr::Set {
                            bind: LoSetBind::Local {
                                index: return_addr_local_index,
                            },
                        },
                        init_store_instr,
                        LoInstr::UntypedLocalGet {
                            local_index: return_addr_local_index,
                        },
                    ],
                }),
            }
        }
        (
            ":",
            [SExpr::Atom {
                value: local_name,
                loc: name_loc,
                kind: AtomKind::Symbol,
            }, value_type],
        ) => {
            if let Some(_) = ctx.module.globals.get(local_name.as_str()) {
                return Err(LoError {
                    message: format!("Local name collides with global: {local_name}"),
                    loc: name_loc.clone(),
                });
            };

            if ctx.block.get_own_local(local_name).is_some() {
                return Err(LoError {
                    message: format!("Duplicate local definition: {local_name}"),
                    loc: name_loc.clone(),
                });
            }

            let value_type = parse_lo_type(value_type, &ctx.module)?;
            let local_indicies = ctx.push_local(local_name.clone(), value_type.clone());

            let values = local_indicies
                .map(|i| LoInstr::UntypedLocalGet { local_index: i })
                .collect();

            LoInstr::NoEmit {
                expr: Box::new(LoInstr::Casted {
                    value_type,
                    expr: Box::new(LoInstr::MultiValueEmit { values }),
                }),
            }
        }
        ("=", [bind, value]) => {
            let value_instr = compile_instr(value, ctx)?;
            let bind_instr = compile_instr(bind, ctx)?;

            let value_type = value_instr.get_type(ctx.module);
            let bind_type = bind_instr.get_type(ctx.module);

            if value_type != bind_type {
                return Err(LoError {
                    message: format!(
                        "TypeError: Invalid types for '{op}', \
                        needed {bind_type}, got {value_type}",
                    ),
                    loc: op_loc.clone(),
                });
            }

            compile_set(ctx, value_instr, bind_instr, op_loc)?
        }
        (
            ":=",
            [SExpr::Atom {
                value: local_name,
                loc: name_loc,
                kind: AtomKind::Symbol,
            }, value],
        ) => {
            if let Some(_) = ctx.module.globals.get(local_name.as_str()) {
                return Err(LoError {
                    message: format!("Local name collides with global: {local_name}"),
                    loc: name_loc.clone(),
                });
            };

            if ctx.block.get_own_local(local_name).is_some() {
                return Err(LoError {
                    message: format!("Duplicate local definition: {local_name}"),
                    loc: name_loc.clone(),
                });
            }

            let value_instr = compile_instr(value, ctx)?;
            let value_type = value_instr.get_type(ctx.module);
            let local_indicies = ctx.push_local(local_name.clone(), value_type.clone());

            let values = local_indicies
                .map(|i| LoInstr::UntypedLocalGet { local_index: i })
                .collect();

            let bind_instr = LoInstr::MultiValueEmit { values };

            compile_set(ctx, value_instr, bind_instr, op_loc)?
        }
        (
            ".",
            [lhs, SExpr::Atom {
                value: f_name,
                loc: f_name_loc,
                kind: AtomKind::Symbol,
            }],
        ) => {
            let lhs_instr = compile_instr(lhs, ctx)?;

            if let LoInstr::StructGet {
                struct_name,
                base_index,
                ..
            } = lhs_instr
            {
                // safe to unwrap as it was already checked in `StructGet`
                let struct_def = ctx.module.struct_defs.get(&struct_name).unwrap();

                let Some(field) = struct_def.fields.iter().find(|f| f.name == *f_name) else {
                    return Err(LoError {
                        message: format!("Unknown field {f_name} in struct {struct_name}"),
                        loc: f_name_loc.clone(),
                    });
                };

                return compile_local_get(
                    &ctx.module,
                    base_index + field.field_index,
                    &field.value_type,
                )
                .map_err(|message| LoError {
                    message,
                    loc: lhs.loc().clone(),
                });
            };

            if let LoInstr::StructLoad {
                struct_name,
                address_instr,
                base_byte_offset,
                ..
            } = lhs_instr
            {
                // safe to unwrap as it was already checked in `StructLoad`
                let struct_def = ctx.module.struct_defs.get(&struct_name).unwrap();

                let Some(field) = struct_def.fields.iter().find(|f| f.name == *f_name) else {
                    return Err(LoError {
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
                .map_err(|e| LoError {
                    message: e,
                    loc: op_loc.clone(),
                });
            }

            return Err(LoError {
                message: format!("Invalid arguments for {op}"),
                loc: op_loc.clone(),
            });
        }
        ("*", [pointer_expr]) => {
            let pointer_instr = compile_instr(pointer_expr, ctx)?;
            let lo_type = pointer_instr.get_type(ctx.module);

            let LoType::Pointer(pointee_type) = lo_type else {
                return Err(LoError {
                    message: format!("Cannot dereference {lo_type:?}"),
                    loc: op_loc.clone(),
                });
            };

            compile_load(ctx, &pointee_type, Box::new(pointer_instr), 0).map_err(|err| LoError {
                message: err,
                loc: op_loc.clone(),
            })?
        }
        (
            "->",
            [struct_ref_expr, SExpr::Atom {
                value: f_name,
                loc: f_name_loc,
                kind: AtomKind::Symbol,
            }],
        ) => {
            let struct_ref_instr = Box::new(compile_instr(struct_ref_expr, ctx)?);
            let lo_type = struct_ref_instr.get_type(ctx.module);

            let LoType::Pointer(pointee_type) = &lo_type else {
                return Err(LoError {
                    message: format!("Cannot dereference {lo_type:?}"),
                    loc: op_loc.clone(),
                });
            };
            let LoType::StructInstance { name: s_name } = pointee_type.as_ref() else {
                return Err(LoError {
                    message: format!("Cannot dereference {lo_type:?}"),
                    loc: op_loc.clone(),
                });
            };

            let struct_def = ctx.module.struct_defs.get(s_name).unwrap();
            let Some(field) = struct_def.fields.iter().find(|f| f.name == *f_name) else {
                return Err(LoError {
                    message: format!("Unknown field {f_name} in struct {s_name}"),
                    loc: f_name_loc.clone(),
                });
            };

            compile_load(ctx, &field.value_type, struct_ref_instr, field.byte_offset).map_err(
                |e| LoError {
                    message: e,
                    loc: op_loc.clone(),
                },
            )?
        }
        // TODO(feat): support custom aligns and offsets
        ("@", [load_kind_expr, address_expr]) => {
            let address_instr = Box::new(compile_instr(address_expr, ctx)?);
            let value_type = parse_lo_type(&load_kind_expr, &ctx.module)?;

            compile_load(ctx, &value_type, address_instr, 0).map_err(|err| LoError {
                message: err,
                loc: op_loc.clone(),
            })?
        }
        (fn_name, args) => {
            if let Some(struct_def) = ctx.module.struct_defs.get(fn_name) {
                let struct_name = fn_name;
                if args.len() / 2 != struct_def.fields.len() {
                    return Err(LoError {
                        message: format!(
                            "Invalid number of struct fields, expected: {}",
                            struct_def.fields.len()
                        ),
                        loc: op_loc.clone(),
                    });
                }

                let mut values = vec![];
                for i in 0..args.len() / 2 {
                    let struct_field = &struct_def.fields[i];
                    let field_name_expr = &args[i * 2];
                    let field_value_expr = &args[i * 2 + 1];

                    let SExpr::Atom {
                        value: field_name,
                        kind: AtomKind::Symbol,
                        loc: _,
                    } = field_name_expr
                    else {
                        return Err(LoError {
                            message: format!("Field name expected, got {field_name_expr}"),
                            loc: field_name_expr.loc().clone(),
                        });
                    };

                    let expected_field_name = format!(":{}", struct_field.name);
                    if field_name != &expected_field_name[..] {
                        return Err(LoError {
                            message: format!(
                                "Unexpected field name, expecting: `{expected_field_name}`"
                            ),
                            loc: field_name_expr.loc().clone(),
                        });
                    }

                    let field_value = compile_instr(field_value_expr, ctx)?;
                    let field_value_type = field_value.get_type(ctx.module);

                    let field_type = &struct_field.value_type;
                    if field_value_type != *field_type {
                        return Err(LoError {
                            message: format!(
                                "Invalid type for field {struct_name}{field_name}, \
                                expected: {field_type}, \
                                got: {field_value_type}"
                            ),
                            loc: field_value_expr.loc().clone(),
                        });
                    }
                    values.push(field_value);
                }

                return Ok(LoInstr::Casted {
                    value_type: LoType::StructInstance {
                        name: struct_name.into(),
                    },
                    expr: Box::new(LoInstr::MultiValueEmit { values }),
                });
            };

            let Some(fn_def) = ctx.module.fn_defs.get(fn_name) else {
                return Err(LoError {
                    message: format!("Unknown instruction or function: {fn_name}"),
                    loc: op_loc.clone(),
                });
            };

            let args = compile_instrs(args, ctx)?;

            let fn_type_index = fn_def.type_index;
            let wasm_module = ctx.module.wasm_module.borrow_mut();
            let fn_type = wasm_module
                .types
                .get(fn_type_index as usize)
                .ok_or_else(|| LoError::unreachable(file!(), line!()))?;

            let mut arg_types = vec![];
            for arg in &args {
                let lo_type = arg.get_type(ctx.module);
                lo_type.emit_components(&ctx.module, &mut arg_types);
            }

            if fn_type.inputs != arg_types {
                return Err(LoError {
                    message: format!(
                        "TypeError: Mismatched arguments for function \
                            '{fn_name}', expected {inputs:?}, got {args:?}",
                        inputs = fn_type.inputs,
                        args = arg_types,
                    ),
                    loc: op_loc.clone(),
                });
            }

            // TODO: use this eventually
            // let mut arg_types = vec![];
            // for arg in &args {
            //     ctx,.get_typeactx((ctx, &arg));
            // }
            // if arg_types != fn_def.kind.inputs {
            //     return Err(CompileError {
            //         message: format!(
            //             "TypeError: Mismatched arguments for function \
            //                         '{fn_name}', expected {inputs:?}, got {args:?}",
            //             inputs = fn_def.kind.inputs,
            //             args = arg_types,
            //         ),
            //         loc: op_loc.clone(),
            //     });
            // }

            LoInstr::Call {
                fn_index: fn_def.get_absolute_index(&ctx.module),
                args,
                return_type: fn_def.kind.output.clone(),
            }
        }
    };

    Ok(instr)
}

fn get_deferred(
    ctx: &mut BlockContext,
    defer_label: &str,
) -> Option<Result<Vec<LoInstr>, LoError>> {
    let Some(deferred) = ctx.fn_ctx.defers.get(defer_label) else {
        return None;
    };

    let mut deferred = deferred.clone();
    deferred.reverse();

    Some(Ok(deferred))
}

fn compile_load(
    ctx: &mut BlockContext,
    value_type: &LoType,
    address_instr: Box<LoInstr>,
    base_byte_offset: u32,
) -> Result<LoInstr, String> {
    if let Ok(_) = value_type.to_load_kind() {
        return Ok(LoInstr::Load {
            kind: value_type.clone(),
            align: 0,
            offset: base_byte_offset,
            address_instr: address_instr.clone(),
        });
    }

    if let LoType::Tuple(item_types) = value_type {
        let mut item_gets = vec![];
        let mut item_byte_offset = 0;
        for item_type in item_types {
            item_gets.push(compile_load(
                ctx,
                item_type,
                address_instr.clone(),
                base_byte_offset + item_byte_offset,
            )?);
            item_byte_offset += item_type.sized_comp_stats(&ctx.module)?.byte_length;
        }

        return Ok(LoInstr::Casted {
            value_type: value_type.clone(),
            expr: Box::new(LoInstr::MultiValueEmit { values: item_gets }),
        });
    }

    let LoType::StructInstance { name } = value_type else {
        return Err(format!("Unsupported type for compile_load: {value_type:?}"));
    };

    let mut components = vec![];
    let mut stats = EmitComponentStats {
        count: 0,
        byte_length: base_byte_offset,
    };

    value_type.emit_sized_component_stats(&ctx.module, &mut stats, &mut components)?;

    let address_local_index = ctx.fn_ctx.locals_last_index;
    ctx.fn_ctx.non_arg_wasm_locals.push(WasmType::I32);
    ctx.fn_ctx.locals_last_index += 1;

    let mut primitive_loads = vec![];
    for comp in components.into_iter() {
        primitive_loads.push(LoInstr::Load {
            kind: comp.value_type,
            align: 1,
            offset: comp.byte_offset,
            address_instr: Box::new(LoInstr::UntypedLocalGet {
                local_index: address_local_index,
            }),
        });
    }

    Ok(LoInstr::StructLoad {
        struct_name: name.clone(),
        address_instr,
        address_local_index,
        base_byte_offset,
        primitive_loads,
    })
}

fn compile_local_get(
    ctx: &ModuleContext,
    base_index: u32,
    value_type: &LoType,
) -> Result<LoInstr, String> {
    let comp_count = value_type.emit_components(ctx, &mut vec![]);
    if comp_count == 1 {
        return Ok(LoInstr::LocalGet {
            local_index: base_index,
            value_type: value_type.clone(),
        });
    }

    if let LoType::Tuple(item_types) = value_type {
        let mut item_gets = vec![];
        for (item_index, item_type) in (0..).zip(item_types) {
            item_gets.push(compile_local_get(ctx, base_index + item_index, item_type)?);
        }

        return Ok(LoInstr::Casted {
            value_type: value_type.clone(),
            expr: Box::new(LoInstr::MultiValueEmit { values: item_gets }),
        });
    }

    let LoType::StructInstance { name } = value_type else {
        return Err(format!("Unsupported type for compile_load: {value_type:?}"));
    };

    let mut primitive_gets = vec![];
    for field_index in 0..comp_count {
        primitive_gets.push(LoInstr::UntypedLocalGet {
            local_index: base_index + field_index as u32,
        });
    }

    Ok(LoInstr::StructGet {
        struct_name: name.clone(),
        base_index,
        primitive_gets,
    })
}

fn compile_const_instr(expr: &SExpr, ctx: &ModuleContext) -> Result<LoInstr, LoError> {
    let items = match expr {
        SExpr::List { value: items, .. } => items,
        SExpr::Atom {
            value,
            loc: op_loc,
            kind,
        } => {
            if *kind != AtomKind::Symbol {
                return Err(LoError {
                    message: format!("Strings are not allowed in globals"),
                    loc: expr.loc().clone(),
                });
            }

            if value == "true" {
                return Ok(LoInstr::Casted {
                    value_type: LoType::Bool,
                    expr: Box::new(LoInstr::U32Const { value: 1 }),
                });
            }

            if value == "false" {
                return Ok(LoInstr::Casted {
                    value_type: LoType::Bool,
                    expr: Box::new(LoInstr::U32Const { value: 0 }),
                });
            }

            if value.chars().all(|c| c.is_ascii_digit()) {
                return Ok(LoInstr::U32Const {
                    value: value.parse().map_err(|_| LoError {
                        message: format!("Parsing u32 (implicit) failed"),
                        loc: op_loc.clone(),
                    })?,
                });
            }

            let Some(global) = ctx.globals.get(value.as_str()) else {
                return Err(LoError {
                    message: format!("Reading unknown global: {value}"),
                    loc: op_loc.clone(),
                });
            };

            return Ok(LoInstr::GlobalGet {
                global_index: global.index,
            });
        }
    };

    let [SExpr::Atom {
        value: op,
        loc: op_loc,
        kind,
    }, args @ ..] = &items[..]
    else {
        return Err(LoError {
            message: format!("Expected operation, got a simple list"),
            loc: expr.loc().clone(),
        });
    };

    if *kind != AtomKind::Symbol {
        return Err(LoError {
            message: format!("Expected operation, got a string"),
            loc: expr.loc().clone(),
        });
    }

    let instr = match (op.as_str(), &args[..]) {
        ("i32", [SExpr::Atom { value, .. }]) => LoInstr::U32Const {
            value: value.parse().map_err(|_| LoError {
                message: format!("Parsing i32 failed"),
                loc: op_loc.clone(),
            })?,
        },
        ("data.size", []) => LoInstr::U32ConstLazy {
            value: ctx.data_size.clone(),
        },
        (instr_name, _args) => {
            return Err(LoError {
                message: format!("Unknown instruction: {instr_name}"),
                loc: op_loc.clone(),
            });
        }
    };

    Ok(instr)
}

fn compile_set(
    ctx: &mut BlockContext,
    value_instr: LoInstr,
    bind_instr: LoInstr,
    bind_loc: &LoLocation,
) -> Result<LoInstr, LoError> {
    let mut values = vec![];
    compile_set_binds(&mut values, ctx, bind_instr, bind_loc, None)?;
    values.push(value_instr);
    values.reverse();

    Ok(LoInstr::Casted {
        value_type: LoType::Void,
        expr: Box::new(LoInstr::MultiValueEmit { values }),
    })
}

// TODO: figure out better location
fn compile_set_binds(
    output: &mut Vec<LoInstr>,
    ctx: &mut BlockContext,
    bind_instr: LoInstr,
    bind_loc: &LoLocation,
    address_index: Option<u32>,
) -> Result<(), LoError> {
    Ok(match bind_instr {
        LoInstr::LocalGet {
            local_index,
            value_type: _,
        }
        | LoInstr::UntypedLocalGet { local_index } => {
            output.push(LoInstr::Set {
                bind: LoSetBind::Local { index: local_index },
            });
        }
        LoInstr::GlobalGet { global_index } => {
            output.push(LoInstr::Set {
                bind: LoSetBind::Global {
                    index: global_index,
                },
            });
        }
        LoInstr::Load {
            kind,
            align,
            offset,
            address_instr,
        } => {
            let value_local_index = ctx.fn_ctx.locals_last_index;
            ctx.fn_ctx
                .non_arg_wasm_locals
                .push(kind.to_wasm_type().unwrap());
            ctx.fn_ctx.locals_last_index += 1;

            let address_instr = match address_index {
                Some(local_index) => Box::new(LoInstr::UntypedLocalGet { local_index }),
                None => address_instr,
            };

            output.push(LoInstr::Set {
                bind: LoSetBind::Memory {
                    align,
                    offset,
                    kind: WasmStoreKind::from_load_kind(&kind.to_load_kind().unwrap()),
                    address_instr,
                    value_local_index,
                },
            });
        }
        LoInstr::StructLoad {
            primitive_loads,
            address_instr,
            address_local_index,
            ..
        } => {
            let mut values = vec![];

            for value in primitive_loads {
                compile_set_binds(&mut values, ctx, value, bind_loc, Some(address_local_index))?;
            }

            values.push(LoInstr::Set {
                bind: LoSetBind::Local {
                    index: address_local_index,
                },
            });
            values.push(*address_instr);

            values.reverse();

            output.push(LoInstr::MultiValueEmit { values });
        }
        // TODO: improve this? (StructGet/MultiValueEmit/NoEmit)
        LoInstr::StructGet { primitive_gets, .. } => {
            for value in primitive_gets {
                compile_set_binds(output, ctx, value, bind_loc, address_index)?;
            }
        }
        LoInstr::MultiValueEmit { values } => {
            for value in values {
                compile_set_binds(output, ctx, value, bind_loc, address_index)?;
            }
        }
        LoInstr::NoEmit { expr: instr } => {
            compile_set_binds(output, ctx, *instr, bind_loc, address_index)?;
        }
        LoInstr::Casted {
            expr: instr,
            value_type: _,
        } => {
            compile_set_binds(output, ctx, *instr, bind_loc, address_index)?;
        }
        _ => {
            return Err(LoError {
                message: format!("Invalid bind"),
                loc: bind_loc.clone(),
            });
        }
    })
}

// types

fn parse_lo_type(expr: &SExpr, ctx: &ModuleContext) -> Result<LoType, LoError> {
    parse_lo_type_checking_ref(expr, ctx, false)
}

fn parse_lo_type_checking_ref(
    expr: &SExpr,
    ctx: &ModuleContext,
    is_referenced: bool,
) -> Result<LoType, LoError> {
    match expr {
        SExpr::Atom {
            kind: AtomKind::Symbol,
            value: name,
            ..
        } => match &name[..] {
            "void" => return Ok(LoType::Void),
            "bool" => return Ok(LoType::Bool),
            "u8" => return Ok(LoType::U8),
            "i8" => return Ok(LoType::I8),
            "u16" => return Ok(LoType::U16),
            "i16" => return Ok(LoType::I16),
            "u32" => return Ok(LoType::U32),
            "i32" => return Ok(LoType::I32),
            "f32" => return Ok(LoType::F32),
            "u64" => return Ok(LoType::U64),
            "i64" => return Ok(LoType::I64),
            "f64" => return Ok(LoType::F64),
            _ => {
                if let Some(struct_def) = ctx.struct_defs.get(name) {
                    if !struct_def.fully_defined && !is_referenced {
                        return Err(LoError {
                            message: format!("Cannot use partially defined struct"),
                            loc: expr.loc().clone(),
                        });
                    }

                    return Ok(LoType::StructInstance {
                        name: String::from(name),
                    });
                }
            }
        },
        SExpr::List { value, .. } => match &value[..] {
            [SExpr::Atom {
                kind: AtomKind::Symbol,
                value,
                ..
            }, ptr_data]
                if value == "&" || value == "&*" =>
            {
                let pointee = parse_lo_type_checking_ref(ptr_data, ctx, true)?;

                return Ok(LoType::Pointer(Box::new(pointee)));
            }
            [SExpr::Atom {
                kind: AtomKind::Symbol,
                value,
                ..
            }, type_exprs @ ..]
                if value == "tuple" =>
            {
                let mut types = vec![];
                for type_expr in type_exprs {
                    types.push(parse_lo_type_checking_ref(type_expr, ctx, is_referenced)?);
                }
                return Ok(LoType::Tuple(types));
            }
            _ => {}
        },
        _ => {}
    };

    Err(LoError {
        message: format!("Unknown value type: {expr}"),
        loc: expr.loc().clone(),
    })
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
