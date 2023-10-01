use crate::{ast::*, ir::*, lowering::*, type_checker::*, wasm::*};
use alloc::{
    boxed::Box,
    collections::{BTreeMap, BTreeSet},
    format, str,
    string::String,
    vec,
    vec::Vec,
};
use core::{cell::RefCell, slice};

const DEFER_UNTIL_RETURN_LABEL: &str = "return";
const HEAP_ALLOC_ID: i32 = 1;

pub fn compile(exprs: &Vec<SExpr>) -> Result<WasmModule, CompileError> {
    let mut ctx = ModuleContext::default();

    for expr in exprs {
        compile_top_level_expr(&expr, &mut ctx)?;
    }

    // push function exports
    for fn_export in &ctx.fn_exports {
        let Some(fn_def) = ctx.fn_defs.get(&fn_export.in_name) else {
            return Err(CompileError {
                message: format!("Cannot export unknown function {}", fn_export.in_name),
                loc: fn_export.loc.clone(),
            });
        };

        ctx.wasm_module.exports.push(WasmExport {
            export_type: WasmExportType::Func,
            export_name: fn_export.out_name.clone(),
            exported_item_index: ctx.imported_fns_count + fn_def.fn_index,
        });
    }

    // push function codes
    for fn_body in &ctx.fn_bodies {
        let fn_type = ctx
            .wasm_module
            .types
            .get(fn_body.type_index as usize)
            .unwrap();

        let mut fn_ctx = FnContext {
            module: &ctx,
            fn_type,
            locals: &mut fn_body.locals.borrow_mut(),
            locals_last_index: fn_body.locals_last_index,
            non_arg_locals: vec![],
            defers: BTreeMap::default(),
        };

        let mut block_ctx = BlockContext {
            module: &ctx,
            fn_ctx: &mut fn_ctx,
            block: Block {
                block_type: BlockType::Function,
                parent: None,
            },
        };

        let mut lole_exprs = compile_block(&fn_body.body, &mut block_ctx)?;
        if let Some(values) = get_deferred(DEFER_UNTIL_RETURN_LABEL, &mut block_ctx) {
            lole_exprs.append(&mut values?);
        };

        let mut locals = Vec::<WasmLocals>::new();
        for local_type in &block_ctx.fn_ctx.non_arg_locals {
            if let Some(wasm_locals) = locals.last_mut() {
                if (*wasm_locals).value_type == *local_type {
                    wasm_locals.count += 1;
                    continue;
                }
            }
            locals.push(WasmLocals {
                count: 1,
                value_type: *local_type,
            });
        }

        // TODO: move to better place
        let mut instrs = vec![];
        lower_exprs(&mut instrs, lole_exprs);

        ctx.wasm_module.codes.push(WasmFn {
            locals,
            expr: WasmExpr { instrs },
        });
    }

    Ok(ctx.wasm_module)
}

fn compile_block(exprs: &[SExpr], ctx: &mut BlockContext) -> Result<Vec<LoleExpr>, CompileError> {
    let instrs = compile_instrs(exprs, ctx)?;
    for (instr, i) in instrs.iter().zip(0..) {
        if let LoleExpr::NoEmit { .. } = instr {
            continue;
        }

        let types = get_type(ctx, instr)?;
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
            }, output_expr, SExpr::List { value: body, .. }] => {
                if ctx.fn_defs.contains_key(fn_name) {
                    return Err(CompileError {
                        message: format!("Cannot redefine function: {fn_name}"),
                        loc: fn_name_loc.clone(),
                    });
                }

                let mut locals = BTreeMap::new();
                let mut locals_last_index = 0;

                let mut lole_inputs = vec![];
                let mut wasm_inputs = vec![];
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
                    let comp_count = value_type.emit_components(&ctx, &mut wasm_inputs);

                    locals.insert(
                        p_name.clone(),
                        LocalDef {
                            index: locals_last_index,
                            value_type: value_type.clone(),
                        },
                    );
                    lole_inputs.push(value_type);

                    locals_last_index += comp_count;
                }

                let lole_output = parse_lole_type(output_expr, &ctx)?;
                let mut wasm_outputs = vec![];
                lole_output.emit_components(&ctx, &mut wasm_outputs);

                let fn_index = ctx.wasm_module.functions.len() as u32;

                let type_index = ctx.insert_fn_type(WasmFnType {
                    inputs: wasm_inputs,
                    outputs: wasm_outputs,
                });

                ctx.wasm_module.functions.push(type_index);
                let fn_def = FnDef {
                    local: true,
                    fn_index,
                    type_index,
                    kind: LoleFnType {
                        inputs: lole_inputs,
                        output: lole_output,
                    },
                };
                ctx.fn_defs.insert(fn_name.clone(), fn_def);
                ctx.fn_bodies.push(FnBody {
                    fn_index,
                    type_index,
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
                    return Err(CompileError {
                        message: format!("Cannot redefine function: {fn_name}"),
                        loc: fn_name_loc.clone(),
                    });
                }

                let mut param_names = BTreeSet::new();

                let mut lole_inputs = vec![];
                let mut wasm_inputs = vec![];
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
                    value_type.emit_components(&ctx, &mut wasm_inputs);
                    lole_inputs.push(value_type);

                    param_names.insert(p_name.clone());
                }

                let mut wasm_outputs = vec![];
                let lole_output = parse_lole_type(output_expr, &ctx)?;
                lole_output.emit_components(&ctx, &mut wasm_outputs);

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
                    kind: LoleFnType {
                        inputs: lole_inputs,
                        output: lole_output,
                    },
                };
                ctx.fn_defs.insert(fn_name.clone(), fn_def);
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
                return Err(CompileError {
                    message: format!("Invalid arguments for {op}"),
                    loc: op_loc.clone(),
                })
            }
        },
        "global" => {
            let (mutable, global_name, global_type_expr, global_value, name_loc) = match other {
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

            let lole_type = parse_lole_type(global_type_expr, ctx)?;
            let Some(wasm_type) = lole_type.to_wasm_type() else {
                return Err(CompileError {
                    message: format!("Unsupported type: {global_type_expr}"),
                    loc: global_type_expr.loc().clone(),
                });
            };

            // TODO: move to better place
            let mut instrs = vec![];
            lower_expr(&mut instrs, compile_const_instr(global_value, &ctx)?);

            let initial_value = WasmExpr { instrs };

            ctx.globals.insert(
                global_name.clone(),
                GlobalDef {
                    index: ctx.globals.len() as u32,
                    mutable,
                    value_type: lole_type,
                },
            );

            ctx.wasm_module.globals.push(WasmGlobal {
                kind: WasmGlobalKind {
                    value_type: wasm_type,
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

            // TODO: move to better place
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

fn build_struct_fields(
    exprs: &[SExpr],
    struct_name: &str,
    ctx: &ModuleContext,
) -> Result<Vec<StructField>, CompileError> {
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
        }, f_type_expr] = &name_and_type[..] else {
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

        let field_type = parse_lole_type(f_type_expr, ctx)?;

        let mut stats = EmitComponentStats::default();
        field_type
            .emit_sized_component_stats(ctx, &mut stats, &mut vec![])
            .map_err(|err| CompileError {
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

fn compile_instrs(exprs: &[SExpr], ctx: &mut BlockContext) -> Result<Vec<LoleExpr>, CompileError> {
    exprs.iter().map(|expr| compile_instr(expr, ctx)).collect()
}

fn compile_instr(expr: &SExpr, ctx: &mut BlockContext) -> Result<LoleExpr, CompileError> {
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
                                // TODO: move to better place
                                offset: WasmExpr {
                                    instrs: vec![WasmInstr::I32Const { value: string_ptr }],
                                },
                                bytes: value.as_bytes().to_vec(),
                            });

                        string_ptr
                    });

                return Ok(LoleExpr::MultiValueEmit {
                    values: vec![
                        LoleExpr::I32Const { value: string_ptr },
                        LoleExpr::I32Const { value: string_len },
                    ],
                });
            }

            if value == "true" {
                return Ok(LoleExpr::Casted {
                    value_type: LoleType::Primitive(LolePrimitiveType::Bool),
                    expr: Box::new(LoleExpr::I32Const { value: 1 }),
                });
            }

            if value == "false" {
                return Ok(LoleExpr::Casted {
                    value_type: LoleType::Primitive(LolePrimitiveType::Bool),
                    expr: Box::new(LoleExpr::I32Const { value: 0 }),
                });
            }

            if value.chars().all(|c| c.is_ascii_digit()) {
                return Ok(LoleExpr::I32Const {
                    value: (value.parse().map_err(|_| CompileError {
                        message: format!("Parsing i32 (implicit) failed"),
                        loc: loc.clone(),
                    })?),
                });
            }

            if let Some(global) = ctx.module.globals.get(value.as_str()) {
                return Ok(LoleExpr::GlobalGet {
                    global_index: global.index,
                });
            };

            let Some(local) = ctx.fn_ctx.locals.get(value.as_str()) else {
                return Err(CompileError {
                    message: format!("Reading unknown variable: {value}"),
                    loc: loc.clone()
                });
            };

            return compile_local_get(ctx.module, local.index, &local.value_type).map_err(
                |message| CompileError {
                    message,
                    loc: expr.loc().clone(),
                },
            );
        }
    };

    let [SExpr::Atom { value: op, loc: op_loc, .. }, args @ ..] = &items[..] else {
        return Err(CompileError {
            message: format!("Expected operation, got a simple list"),
            loc: expr.loc().clone()
        });
    };

    let instr = match (op.as_str(), &args[..]) {
        ("unreachable", []) => LoleExpr::Unreachable {},
        ("drop", [expr]) => {
            let instr = compile_instr(expr, ctx)?;
            let drop_count = get_type(ctx, &instr)?.len();

            LoleExpr::Drop {
                value: Box::new(instr),
                drop_count,
            }
        }
        ("do", exprs) => LoleExpr::MultiValueEmit {
            values: compile_instrs(exprs, ctx)?,
        },
        (
            "i32",
            [SExpr::Atom {
                value,
                loc,
                kind: AtomKind::Symbol,
            }],
        ) => LoleExpr::I32Const {
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
        ) => LoleExpr::I64Const {
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
        ) => LoleExpr::I32Const {
            value: value.chars().next().unwrap() as i32,
        },
        ("==", [lhs, rhs]) => LoleExpr::BinaryOp {
            kind: WasmBinaryOpKind::I32Equals,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("!=", [lhs, rhs]) => LoleExpr::BinaryOp {
            kind: WasmBinaryOpKind::I32NotEqual,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("not", [lhs]) => LoleExpr::BinaryOp {
            kind: WasmBinaryOpKind::I32Equals,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(LoleExpr::I32Const { value: 0 }),
        },
        ("<", [lhs, rhs]) => LoleExpr::BinaryOp {
            kind: WasmBinaryOpKind::I32LessThenSigned,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        (">", [lhs, rhs]) => LoleExpr::BinaryOp {
            kind: WasmBinaryOpKind::I32GreaterThenSigned,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        (">=", [lhs, rhs]) => LoleExpr::BinaryOp {
            kind: WasmBinaryOpKind::I32GreaterEqualSigned,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("&&", [lhs, rhs]) => LoleExpr::BinaryOp {
            kind: WasmBinaryOpKind::I32And,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("||", [lhs, rhs]) => LoleExpr::BinaryOp {
            kind: WasmBinaryOpKind::I32Or,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("+", [lhs, rhs]) => LoleExpr::BinaryOp {
            kind: WasmBinaryOpKind::I32Add,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("+=", [lhs, rhs]) => {
            let bind = compile_instr(lhs, ctx)?;
            let value = LoleExpr::BinaryOp {
                kind: WasmBinaryOpKind::I32Add,
                lhs: Box::new(bind.clone()),
                rhs: Box::new(compile_instr(rhs, ctx)?),
            };
            compile_set(ctx, value, bind, lhs.loc())?
        }
        ("-", [lhs, rhs]) => LoleExpr::BinaryOp {
            kind: WasmBinaryOpKind::I32Sub,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("-=", [lhs, rhs]) => {
            let bind = compile_instr(lhs, ctx)?;
            let value = LoleExpr::BinaryOp {
                kind: WasmBinaryOpKind::I32Sub,
                lhs: Box::new(bind.clone()),
                rhs: Box::new(compile_instr(rhs, ctx)?),
            };
            compile_set(ctx, value, bind, lhs.loc())?
        }
        ("*", [lhs, rhs]) => LoleExpr::BinaryOp {
            kind: WasmBinaryOpKind::I32Mul,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("/", [lhs, rhs]) => LoleExpr::BinaryOp {
            kind: WasmBinaryOpKind::I32DivUnsigned,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("%", [lhs, rhs]) => LoleExpr::BinaryOp {
            kind: WasmBinaryOpKind::I32RemUnsigned,
            lhs: Box::new(compile_instr(lhs, ctx)?),
            rhs: Box::new(compile_instr(rhs, ctx)?),
        },
        ("data.size", []) => LoleExpr::I32ConstLazy {
            value: ctx.module.data_size.clone(),
        },
        ("memory.size", []) => LoleExpr::MemorySize {},
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

            LoleExpr::MemoryGrow {
                size: Box::new(size),
            }
        }
        ("debug.typeof", [sub_expr]) => {
            let lole_expr = compile_instr(sub_expr, ctx)?;
            let lole_type = get_lole_type(ctx, &lole_expr);
            crate::wasi_io::debug(format!(
                "{}",
                String::from(CompileError {
                    message: format!("{expr} = {:?}", lole_type),
                    loc: expr.loc().clone(),
                })
            ));
            LoleExpr::MultiValueEmit { values: vec![] }
        }
        ("if", [block_type_expr, cond, then_branch, else_branch]) => {
            let then_branch = compile_block(
                slice::from_ref(then_branch),
                &mut BlockContext {
                    module: ctx.module,
                    fn_ctx: ctx.fn_ctx,
                    block: Block {
                        block_type: BlockType::Block,
                        parent: Some(&ctx.block),
                    },
                },
            )?;

            let else_branch = Some(compile_block(
                slice::from_ref(else_branch),
                &mut BlockContext {
                    module: ctx.module,
                    fn_ctx: ctx.fn_ctx,
                    block: Block {
                        block_type: BlockType::Block,
                        parent: Some(&ctx.block),
                    },
                },
            )?);

            let block_type = parse_lole_type(block_type_expr, ctx.module)?;
            if let None = block_type.to_wasm_type() {
                let LoleType::Void = block_type else {
                    return Err(CompileError {
                        message: format!("Unsupported type: {block_type_expr}"),
                        loc: block_type_expr.loc().clone(),
                    });
                };
            }

            LoleExpr::If {
                block_type,
                cond: Box::new(compile_instr(cond, ctx)?),
                then_branch,
                else_branch,
            }
        }
        ("if", [cond, then_branch]) => LoleExpr::If {
            block_type: LoleType::Void,
            cond: Box::new(compile_instr(cond, ctx)?),
            then_branch: compile_block(
                slice::from_ref(then_branch),
                &mut BlockContext {
                    module: ctx.module,
                    fn_ctx: ctx.fn_ctx,
                    block: Block {
                        block_type: BlockType::Block,
                        parent: Some(&ctx.block),
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
                },
            };

            let mut body = compile_block(exprs, &mut ctx)?;

            // add implicit continue
            body.push(LoleExpr::Branch { label_index: 0 });

            LoleExpr::Block {
                block_type: LoleType::Void,
                body: vec![LoleExpr::Loop {
                    block_type: LoleType::Void,
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

            LoleExpr::Branch { label_index }
        }
        ("continue", []) => {
            let mut label_index = 0; // 0 = loop

            let mut current_block = &ctx.block;
            loop {
                if current_block.block_type == BlockType::Loop {
                    break;
                }

                current_block = current_block.parent.unwrap();
                label_index += 1;
            }

            LoleExpr::Branch { label_index }
        }
        ("return", values) => {
            let value = LoleExpr::MultiValueEmit {
                values: compile_instrs(values, ctx)?,
            };

            let return_type = get_type(ctx, &value)?;
            if return_type != ctx.fn_ctx.fn_type.outputs {
                return Err(CompileError {
                    message: format!(
                        "TypeError: Invalid return type, \
                            expected {outputs:?}, got {return_type:?}",
                        outputs = ctx.fn_ctx.fn_type.outputs,
                    ),
                    loc: op_loc.clone(),
                });
            }

            let return_expr = LoleExpr::Return {
                value: Box::new(value),
            };
            if let Some(values) = get_deferred(DEFER_UNTIL_RETURN_LABEL, ctx) {
                let mut values = values?;
                values.push(return_expr);
                LoleExpr::MultiValueEmit { values }
            } else {
                return_expr
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

            let deferred = ctx
                .fn_ctx
                .defers
                .entry(defer_label)
                .or_insert_with(|| vec![]);

            deferred.push(defer_expr.clone());

            LoleExpr::MultiValueEmit { values: vec![] }
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

            LoleExpr::MultiValueEmit { values: values? }
        }
        ("as", [value_expr, type_expr]) => {
            let lole_expr = compile_instr(value_expr, ctx)?;
            let value_type = parse_lole_type(type_expr, ctx.module)?;

            LoleExpr::Casted {
                value_type,
                expr: Box::new(lole_expr),
            }
        }
        ("sizeof", [type_expr]) => {
            let value_type = parse_lole_type(type_expr, ctx.module)?;

            LoleExpr::I32Const {
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
                        "TypeError: Invalid types for {op}, \
                        needed {field_types:?}, \
                        got {value_types:?}",
                    ),
                    loc: op_loc.clone(),
                });
            }

            LoleExpr::Casted {
                value_type: LoleType::StructInstance {
                    name: s_name.clone(),
                },
                expr: Box::new(LoleExpr::MultiValueEmit {
                    values: value_instrs,
                }),
            }
        }
        ("new", [type_expr, init_expr, other @ ..]) => {
            let alloc_id_instr = match other {
                [] => LoleExpr::I32Const {
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

            let return_addr_local_index = ctx.fn_ctx.locals_last_index;
            ctx.fn_ctx.non_arg_locals.push(WasmType::I32);
            ctx.fn_ctx.locals_last_index += 1;

            let init_load = compile_load(
                ctx,
                &value_type,
                Box::new(LoleExpr::LocalGet {
                    local_index: return_addr_local_index,
                }),
                0,
            )
            .map_err(|err| CompileError {
                message: err,
                loc: op_loc.clone(),
            })?;

            let init_store_instr = compile_set(ctx, init_instr, init_load, op_loc)?;

            LoleExpr::Casted {
                value_type: LoleType::Pointer(Box::new(value_type)),
                expr: Box::new(LoleExpr::MultiValueEmit {
                    values: vec![
                        LoleExpr::Call {
                            fn_index: alloc_fn_index,
                            return_type: LoleType::Void, // won't be typechecked
                            args: vec![
                                alloc_id_instr,
                                LoleExpr::I32Const {
                                    value: value_size as i32,
                                },
                            ],
                        },
                        LoleExpr::Set {
                            bind: LoleSetBind::Local {
                                index: return_addr_local_index,
                            },
                        },
                        init_store_instr,
                        LoleExpr::LocalGet {
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
                return Err(CompileError {
                    message: format!("Local name collides with global: {local_name}"),
                    loc: name_loc.clone(),
                });
            };

            if ctx.fn_ctx.locals.contains_key(local_name) {
                return Err(CompileError {
                    message: format!("Duplicate local definition: {local_name}"),
                    loc: name_loc.clone(),
                });
            }

            let value_type = parse_lole_type(value_type, ctx.module)?;

            let start_index = ctx.fn_ctx.locals_last_index;
            let comp_count =
                value_type.emit_components(&ctx.module, &mut ctx.fn_ctx.non_arg_locals);

            ctx.fn_ctx.locals_last_index += comp_count;
            ctx.fn_ctx.locals.insert(
                local_name.clone(),
                LocalDef {
                    index: start_index,
                    value_type: value_type.clone(),
                },
            );

            let values = (start_index..(start_index + comp_count))
                .map(|i| LoleExpr::LocalGet { local_index: i })
                .collect();

            LoleExpr::NoEmit {
                expr: Box::new(LoleExpr::Casted {
                    value_type,
                    expr: Box::new(LoleExpr::MultiValueEmit { values }),
                }),
            }
        }
        ("=", [bind, value]) => {
            let value_instr = compile_instr(value, ctx)?;
            let bind_instr = compile_instr(bind, ctx)?;

            // TODO: enable this once tests pass
            // let value_type = get_lole_type(ctx, &value_instr).map_err(|message| CompileError {
            //     message,
            //     loc: value.loc().clone(),
            // })?;
            // let bind_type = get_lole_type(ctx, &bind_instr).map_err(|message| CompileError {
            //     message,
            //     loc: value.loc().clone(),
            // })?;

            // if value_type != bind_type {
            //     return Err(CompileError {
            //         message: format!(
            //             "TypeError: Invalid types for '{op}', \
            //             needed {bind_type}, \
            //             got {value_type}",
            //         ),
            //         loc: op_loc.clone(),
            //     });
            // }

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
            ":=",
            [SExpr::Atom {
                value: local_name,
                loc: name_loc,
                kind: AtomKind::Symbol,
            }, value],
        ) => {
            if let Some(_) = ctx.module.globals.get(local_name.as_str()) {
                return Err(CompileError {
                    message: format!("Local name collides with global: {local_name}"),
                    loc: name_loc.clone(),
                });
            };

            if ctx.fn_ctx.locals.contains_key(local_name) {
                return Err(CompileError {
                    message: format!("Duplicate local definition: {local_name}"),
                    loc: name_loc.clone(),
                });
            }

            let value_instr = compile_instr(value, ctx)?;
            let lole_type = get_lole_type(ctx, &value_instr).map_err(|message| CompileError {
                message,
                loc: value.loc().clone(),
            })?;

            let start_index = ctx.fn_ctx.locals_last_index;
            let comp_count = lole_type.emit_components(&ctx.module, &mut ctx.fn_ctx.non_arg_locals);

            ctx.fn_ctx.locals_last_index += comp_count;
            ctx.fn_ctx.locals.insert(
                local_name.clone(),
                LocalDef {
                    index: start_index,
                    value_type: lole_type,
                },
            );

            let values = (start_index..(start_index + comp_count))
                .map(|i| LoleExpr::LocalGet { local_index: i })
                .collect();

            let bind_instr = LoleExpr::MultiValueEmit { values };

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

                let Some(local) = ctx.fn_ctx.locals.get(local_name.as_str()) else {
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

                // safe
                let struct_def = ctx.module.struct_defs.get(s_name).unwrap();

                let Some(field) = struct_def.fields.iter().find(|f| f.name == *f_name) else {
                    return Err(CompileError {
                        message: format!("Unknown field {f_name} in struct {s_name}"),
                        loc: f_name_loc.clone(),
                    });
                };

                return compile_local_get(
                    ctx.module,
                    local.index + field.field_index,
                    &field.value_type,
                )
                .map_err(|message| CompileError {
                    message,
                    loc: lhs.loc().clone(),
                });
            }

            let lhs_instr = compile_instr(lhs, ctx)?;

            if let LoleExpr::StructGet {
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

                return compile_local_get(
                    ctx.module,
                    base_index + field.field_index,
                    &field.value_type,
                )
                .map_err(|message| CompileError {
                    message,
                    loc: lhs.loc().clone(),
                });
            };

            if let LoleExpr::StructLoad {
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
        ("*", [pointer_expr]) => {
            let pointer_instr = Box::new(compile_instr(pointer_expr, ctx)?);
            let lole_type = get_lole_type(ctx, &pointer_instr).map_err(|message| CompileError {
                message,
                loc: pointer_expr.loc().clone(),
            })?;

            let LoleType::Pointer(pointee_type) = lole_type else {
                return Err(CompileError {
                    message: format!("Cannot dereference {lole_type:?}"),
                    loc: op_loc.clone(),
                })
            };

            compile_load(ctx, &pointee_type, pointer_instr, 0).map_err(|err| CompileError {
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
            let lole_type =
                get_lole_type(ctx, &struct_ref_instr).map_err(|message| CompileError {
                    message,
                    loc: struct_ref_expr.loc().clone(),
                })?;

            let LoleType::Pointer(pointee_type) = &lole_type else {
                return Err(CompileError {
                    message: format!("Cannot dereference {lole_type:?}"),
                    loc: op_loc.clone(),
                })
            };
            let LoleType::StructInstance { name: s_name } = pointee_type.as_ref() else {
                return Err(CompileError {
                    message: format!("Cannot dereference {lole_type:?}"),
                    loc: op_loc.clone(),
                })
            };

            let struct_def = ctx.module.struct_defs.get(s_name).unwrap();
            let Some(field) = struct_def.fields.iter().find(|f| f.name == *f_name) else {
                return Err(CompileError {
                    message: format!("Unknown field {f_name} in struct {s_name}"),
                    loc: f_name_loc.clone(),
                });
            };

            compile_load(ctx, &field.value_type, struct_ref_instr, field.byte_offset).map_err(
                |e| CompileError {
                    message: e,
                    loc: op_loc.clone(),
                },
            )?
        }
        // TODO(feat): support custom aligns and offsets
        ("@", [load_kind_expr, address_expr]) => {
            let address_instr = Box::new(compile_instr(address_expr, ctx)?);
            let value_type = parse_lole_type(&load_kind_expr, ctx.module)?;

            compile_load(ctx, &value_type, address_instr, 0).map_err(|err| CompileError {
                message: err,
                loc: op_loc.clone(),
            })?
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

            LoleExpr::Call {
                fn_index: fn_def.get_absolute_index(ctx.module),
                args,
                return_type: fn_def.kind.output.clone(),
            }
        }
    };

    Ok(instr)
}

fn get_deferred(
    defer_label: &str,
    ctx: &mut BlockContext,
) -> Option<Result<Vec<LoleExpr>, CompileError>> {
    let Some(deferred) = ctx.fn_ctx.defers.get(defer_label) else {
        return None;
    };

    let mut deferred = deferred.clone();
    deferred.reverse();

    Some(compile_instrs(&deferred, ctx))
}

fn compile_load(
    ctx: &mut BlockContext,
    value_type: &LoleType,
    address_instr: Box<LoleExpr>,
    base_byte_offset: u32,
) -> Result<LoleExpr, String> {
    if let Ok(_) = value_type.to_load_kind() {
        return Ok(LoleExpr::Load {
            kind: value_type.clone(),
            align: 0,
            offset: base_byte_offset,
            address_instr: address_instr.clone(),
        });
    }

    let LoleType::StructInstance { name } = value_type else {
        return Err(format!("Unsupported type for compile_load: {value_type:?}"));
    };

    let mut components = vec![];
    let mut stats = EmitComponentStats {
        count: 0,
        byte_length: base_byte_offset,
    };

    value_type.emit_sized_component_stats(ctx.module, &mut stats, &mut components)?;

    let address_local_index = ctx.fn_ctx.locals_last_index;
    ctx.fn_ctx.non_arg_locals.push(WasmType::I32);
    ctx.fn_ctx.locals_last_index += 1;

    let mut primitive_loads = vec![];
    for comp in components.into_iter() {
        primitive_loads.push(LoleExpr::Load {
            kind: LoleType::Primitive(comp.value_type),
            align: 1,
            offset: comp.byte_offset,
            address_instr: Box::new(LoleExpr::LocalGet {
                local_index: address_local_index,
            }),
        });
    }

    Ok(LoleExpr::StructLoad {
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
    value_type: &LoleType,
) -> Result<LoleExpr, String> {
    let comp_count = value_type.emit_components(ctx, &mut vec![]);
    if comp_count == 1 {
        return Ok(LoleExpr::LocalGet {
            local_index: base_index,
        });
    }

    let LoleType::StructInstance { name } = value_type else {
        return Err(format!("Unsupported type for compile_load: {value_type:?}"));
    };

    let mut primitive_gets = vec![];
    for field_index in 0..comp_count {
        primitive_gets.push(LoleExpr::LocalGet {
            local_index: base_index + field_index as u32,
        });
    }

    Ok(LoleExpr::StructGet {
        struct_name: name.clone(),
        base_index,
        primitive_gets,
    })
}

fn compile_const_instr(expr: &SExpr, ctx: &ModuleContext) -> Result<LoleExpr, CompileError> {
    let items = match expr {
        SExpr::List { value: items, .. } => items,
        SExpr::Atom {
            value,
            loc: op_loc,
            kind,
        } => {
            if *kind != AtomKind::Symbol {
                return Err(CompileError {
                    message: format!("Strings are not allowed in globals"),
                    loc: expr.loc().clone(),
                });
            }

            if value == "true" {
                return Ok(LoleExpr::Casted {
                    value_type: LoleType::Primitive(LolePrimitiveType::Bool),
                    expr: Box::new(LoleExpr::I32Const { value: 1 }),
                });
            }

            if value == "false" {
                return Ok(LoleExpr::Casted {
                    value_type: LoleType::Primitive(LolePrimitiveType::Bool),
                    expr: Box::new(LoleExpr::I32Const { value: 0 }),
                });
            }

            if value.chars().all(|c| c.is_ascii_digit()) {
                return Ok(LoleExpr::I32Const {
                    value: value.parse().map_err(|_| CompileError {
                        message: format!("Parsing i32 (implicit) failed"),
                        loc: op_loc.clone(),
                    })?,
                });
            }

            let Some(global) = ctx.globals.get(value.as_str()) else {
                return Err(CompileError {
                    message: format!("Unknown location for global.get: {value}"),
                    loc: op_loc.clone(),
                });
            };

            return Ok(LoleExpr::GlobalGet {
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
        ("i32", [SExpr::Atom { value, .. }]) => LoleExpr::I32Const {
            value: value.parse().map_err(|_| CompileError {
                message: format!("Parsing i32 failed"),
                loc: op_loc.clone(),
            })?,
        },
        ("data.size", []) => LoleExpr::I32ConstLazy {
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
    ctx: &mut BlockContext,
    value_instr: LoleExpr,
    bind_instr: LoleExpr,
    bind_loc: &Location,
) -> Result<LoleExpr, CompileError> {
    let mut values = vec![];
    compile_set_binds(&mut values, ctx, bind_instr, bind_loc, None)?;
    values.push(value_instr);
    values.reverse();

    Ok(LoleExpr::Casted {
        value_type: LoleType::Void,
        expr: Box::new(LoleExpr::MultiValueEmit { values }),
    })
}

// TODO: figure out better location
fn compile_set_binds(
    output: &mut Vec<LoleExpr>,
    ctx: &mut BlockContext,
    bind_instr: LoleExpr,
    bind_loc: &Location,
    address_index: Option<u32>,
) -> Result<(), CompileError> {
    Ok(match bind_instr {
        LoleExpr::LocalGet { local_index } => {
            output.push(LoleExpr::Set {
                bind: LoleSetBind::Local { index: local_index },
            });
        }
        LoleExpr::GlobalGet { global_index } => {
            output.push(LoleExpr::Set {
                bind: LoleSetBind::Global {
                    index: global_index,
                },
            });
        }
        LoleExpr::Load {
            kind,
            align,
            offset,
            address_instr,
        } => {
            let value_local_index = ctx.fn_ctx.locals_last_index;
            ctx.fn_ctx.non_arg_locals.push(kind.to_wasm_type().unwrap());
            ctx.fn_ctx.locals_last_index += 1;

            let address_instr = match address_index {
                Some(local_index) => Box::new(LoleExpr::LocalGet { local_index }),
                None => address_instr,
            };

            output.push(LoleExpr::Set {
                bind: LoleSetBind::Memory {
                    align,
                    offset,
                    kind: WasmStoreKind::from_load_kind(&kind.to_load_kind().unwrap()),
                    address_instr,
                    value_local_index,
                },
            });
        }
        LoleExpr::StructLoad {
            primitive_loads,
            address_instr,
            address_local_index,
            ..
        } => {
            let mut values = vec![];

            for value in primitive_loads {
                compile_set_binds(&mut values, ctx, value, bind_loc, Some(address_local_index))?;
            }

            values.push(LoleExpr::Set {
                bind: LoleSetBind::Local {
                    index: address_local_index,
                },
            });
            values.push(*address_instr);

            values.reverse();

            output.push(LoleExpr::MultiValueEmit { values });
        }
        // TODO: improve this? (StructGet/MultiValueEmit/NoEmit)
        LoleExpr::StructGet { primitive_gets, .. } => {
            for value in primitive_gets {
                compile_set_binds(output, ctx, value, bind_loc, address_index)?;
            }
        }
        LoleExpr::MultiValueEmit { values } => {
            for value in values {
                compile_set_binds(output, ctx, value, bind_loc, address_index)?;
            }
        }
        LoleExpr::NoEmit { expr: instr } => {
            compile_set_binds(output, ctx, *instr, bind_loc, address_index)?;
        }
        LoleExpr::Casted {
            expr: instr,
            value_type: _,
        } => {
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
    parse_lole_type_checking_ref(expr, ctx, false)
}

fn parse_lole_type_checking_ref(
    expr: &SExpr,
    ctx: &ModuleContext,
    is_referenced: bool,
) -> Result<LoleType, CompileError> {
    match expr {
        SExpr::Atom {
            kind: AtomKind::Symbol,
            value: name,
            ..
        } => match &name[..] {
            "void" => return Ok(LoleType::Void),
            "bool" => return Ok(LoleType::Primitive(LolePrimitiveType::Bool)),
            "u8" => return Ok(LoleType::Primitive(LolePrimitiveType::U8)),
            "i8" => return Ok(LoleType::Primitive(LolePrimitiveType::I8)),
            "u16" => return Ok(LoleType::Primitive(LolePrimitiveType::U16)),
            "i16" => return Ok(LoleType::Primitive(LolePrimitiveType::I16)),
            "u32" => return Ok(LoleType::Primitive(LolePrimitiveType::U32)),
            "i32" => return Ok(LoleType::Primitive(LolePrimitiveType::I32)),
            "f32" => return Ok(LoleType::Primitive(LolePrimitiveType::F32)),
            "u64" => return Ok(LoleType::Primitive(LolePrimitiveType::U64)),
            "i64" => return Ok(LoleType::Primitive(LolePrimitiveType::I64)),
            "f64" => return Ok(LoleType::Primitive(LolePrimitiveType::F64)),
            _ => {
                if let Some(struct_def) = ctx.struct_defs.get(name) {
                    if !struct_def.fully_defined && !is_referenced {
                        return Err(CompileError {
                            message: format!("Cannot use partially defined struct"),
                            loc: expr.loc().clone(),
                        });
                    }

                    return Ok(LoleType::StructInstance {
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
                let pointee = parse_lole_type_checking_ref(ptr_data, ctx, true)?;

                return Ok(LoleType::Pointer(Box::new(pointee)));
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
                    types.push(parse_lole_type_checking_ref(type_expr, ctx, is_referenced)?);
                }
                return Ok(LoleType::Tuple(types));
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
