use crate::{ast::*, ir::*, lowering::*, operators::*, parser::*, tokens::*, wasm::*};
use alloc::{boxed::Box, collections::BTreeMap, format, string::String, vec, vec::Vec};
use LoTokenType::*;

const RECEIVER_PARAM_NAME: &str = "self";

pub fn parse(mut tokens: LoTokenStream) -> Result<WasmModule, LoError> {
    let mut ctx = ModuleContext::default();

    while tokens.peek().is_some() {
        parse_top_level_expr(&mut ctx, &mut tokens)?;
        tokens.expect(LoTokenType::Delim, ";")?;
    }

    if let Some(unexpected) = tokens.peek() {
        return Err(LoError {
            message: format!(
                "Unexpected token after top level expression: {}",
                unexpected.value
            ),
            loc: unexpected.loc.clone(),
        });
    }

    process_delayed_actions(&mut ctx)?;

    Ok(ctx.wasm_module.take())
}

fn parse_top_level_expr(
    ctx: &mut ModuleContext,
    tokens: &mut LoTokenStream,
) -> Result<(), LoError> {
    if tokens.peek().is_none() {
        return Ok(());
    }

    if let Some(_) = tokens.eat(Symbol, "fn")? {
        return parse_fn_def(ctx, tokens, false);
    }

    if let Some(_) = tokens.eat(Symbol, "export")? {
        if let Some(_) = tokens.eat(Symbol, "fn")? {
            return parse_fn_def(ctx, tokens, true);
        }
    }

    if let Some(_) = tokens.eat(Symbol, "import")? {
        tokens.expect(Symbol, "from")?;
        let module_name = tokens.expect_any(StringLiteral)?.clone();

        tokens.expect(Delim, "{")?;
        while let None = tokens.eat(Delim, "}")? {
            tokens.expect(Symbol, "fn")?;
            let fn_decl = parse_fn_decl(ctx, tokens)?;
            tokens.expect(LoTokenType::Delim, ";")?;

            if ctx.fn_defs.contains_key(&fn_decl.fn_name) {
                return Err(LoError {
                    message: format!("Cannot redefine function: {}", fn_decl.fn_name),
                    loc: fn_decl.loc,
                });
            }

            let type_index = ctx.insert_fn_type(fn_decl.wasm_type);

            let fn_index = ctx.imported_fns_count;
            ctx.imported_fns_count += 1;

            let fn_def = FnDef {
                local: false,
                fn_index,
                type_index,
                kind: fn_decl.lo_type,
            };
            ctx.fn_defs.insert(fn_decl.fn_name.clone(), fn_def);
            ctx.wasm_module.borrow_mut().imports.push(WasmImport {
                module_name: module_name.value.clone(),
                item_name: fn_decl.fn_name,
                item_desc: WasmImportDesc::Func { type_index },
            });
        }

        return Ok(());
    }

    if let Some(let_token) = tokens.eat(Symbol, "let")?.cloned() {
        let mutable = true;
        let global_name = tokens.expect_any(Symbol)?.clone();
        tokens.expect(Operator, "=")?;

        let global_value = parse_const_expr(ctx, tokens, 0)?;

        let lo_type = global_value.get_type(ctx);
        let Some(wasm_type) = lo_type.to_wasm_type() else {
            return Err(LoError {
                message: format!("Unsupported type: {lo_type}"),
                // TODO: value.loc() is not available
                loc: let_token.loc,
            });
        };

        if ctx.globals.contains_key(&global_name.value) {
            return Err(LoError {
                message: format!("Cannot redefine global: {}", global_name.value),
                loc: global_name.loc,
            });
        }

        ctx.globals.insert(
            global_name.value.clone(),
            GlobalDef {
                index: ctx.globals.len() as u32,
                mutable,
                value_type: lo_type,
            },
        );

        let mut initial_value = WasmExpr { instrs: vec![] };
        lower_expr(&mut initial_value.instrs, global_value);

        ctx.wasm_module.borrow_mut().globals.push(WasmGlobal {
            kind: WasmGlobalKind {
                value_type: wasm_type,
                mutable,
            },
            initial_value,
        });

        return Ok(());
    }

    if let Some(_) = tokens.eat(Symbol, "struct")? {
        let struct_name = tokens.expect_any(Symbol)?.clone();

        if ctx.struct_defs.contains_key(&struct_name.value) {
            return Err(LoError {
                message: format!("Cannot redefine struct {}", struct_name.value),
                loc: struct_name.loc,
            });
        }

        // declare not fully defined struct to use in self-references
        ctx.struct_defs.insert(
            struct_name.value.clone(),
            StructDef {
                fields: vec![],
                fully_defined: false,
            },
        );

        let mut field_index = 0;
        let mut byte_offset = 0;
        let mut struct_fields = Vec::<StructField>::new();

        tokens.expect(Delim, "{")?;
        while let None = tokens.eat(Delim, "}")? {
            let field_name = tokens.expect_any(Symbol)?.clone();
            tokens.expect(Operator, ":")?;
            let field_type = parse_lo_type2(ctx, tokens)?;
            if !tokens.next_is(Delim, "}")? {
                tokens.expect(Delim, ",")?;
            }

            if struct_fields
                .iter()
                .find(|f| f.name == field_name.value)
                .is_some()
            {
                return Err(LoError {
                    message: format!(
                        "Found duplicate struct field name: '{}' of struct {}",
                        field_name.value, struct_name.value,
                    ),
                    loc: field_name.loc,
                });
            }

            let mut stats = EmitComponentStats::default();
            field_type
                .emit_sized_component_stats(ctx, &mut stats, &mut vec![])
                .map_err(|err| LoError {
                    message: err,
                    // TODO: field_type.loc() is not available
                    loc: field_name.loc,
                })?;

            struct_fields.push(StructField {
                name: field_name.value,
                value_type: field_type,
                field_index,
                byte_offset,
            });

            field_index += stats.count;
            byte_offset += stats.byte_length;
        }

        let struct_def = ctx.struct_defs.get_mut(&struct_name.value).unwrap();
        struct_def.fields.append(&mut struct_fields);
        struct_def.fully_defined = true;

        return Ok(());
    }

    let unexpected = tokens.peek().unwrap();
    return Err(LoError {
        message: format!("Unexpected top level token: {}", unexpected.value),
        loc: unexpected.loc.clone(),
    });
}

fn parse_fn_def(
    ctx: &mut ModuleContext,
    tokens: &mut LoTokenStream,
    exported: bool,
) -> Result<(), LoError> {
    let fn_decl = parse_fn_decl(ctx, tokens)?;
    let body = parse_block(ctx, tokens)?;

    if ctx.fn_defs.contains_key(&fn_decl.fn_name) {
        return Err(LoError {
            message: format!("Cannot redefine function: {}", fn_decl.fn_name),
            loc: fn_decl.loc,
        });
    }

    if exported {
        ctx.fn_exports.push(FnExport {
            in_name: fn_decl.fn_name.clone(),
            out_name: fn_decl.fn_name.clone(),
            loc: fn_decl.loc.clone(),
        });
    }

    let locals_last_index = fn_decl.wasm_type.inputs.len() as u32;
    let type_index = ctx.insert_fn_type(fn_decl.wasm_type);
    ctx.wasm_module.borrow_mut().functions.push(type_index);

    let fn_index = ctx.wasm_module.borrow_mut().functions.len() as u32 - 1;

    ctx.fn_defs.insert(
        fn_decl.fn_name,
        FnDef {
            local: true,
            fn_index,
            type_index,
            kind: fn_decl.lo_type,
        },
    );

    ctx.fn_bodies.borrow_mut().push(FnBody {
        fn_index,
        type_index,
        locals: fn_decl.locals,
        locals_last_index,
        body: FnBodyExprs::V2(body),
    });

    return Ok(());
}

struct FnDecl {
    fn_name: String,
    loc: LoLocation,
    lo_type: LoFnType,
    wasm_type: WasmFnType,
    locals: BTreeMap<String, LocalDef>,
}

fn parse_fn_decl(ctx: &mut ModuleContext, tokens: &mut LoTokenStream) -> Result<FnDecl, LoError> {
    let (receiver_name, method_name) = {
        let method_name = tokens.expect_any(Symbol)?.clone();
        if let None = tokens.eat(Operator, ".")? {
            (None, method_name)
        } else {
            (Some(method_name), tokens.expect_any(Symbol)?.clone())
        }
    };

    let mut fn_decl = FnDecl {
        fn_name: method_name.value,
        loc: method_name.loc,
        lo_type: LoFnType {
            inputs: vec![],
            output: LoType::Void,
        },
        wasm_type: WasmFnType {
            inputs: vec![],
            outputs: vec![],
        },
        locals: BTreeMap::new(),
    };

    if let Some(receiver_name) = receiver_name {
        let receiver_type = parse_lo_type(&receiver_name.to_sexpr(), ctx)?;
        fn_decl.fn_name = fn_name_for_method(&receiver_type, &fn_decl.fn_name);
        add_fn_param(ctx, &mut fn_decl, RECEIVER_PARAM_NAME.into(), receiver_type);
    }

    tokens.expect(Delim, "(")?;
    while let None = tokens.eat(Delim, ")")? {
        let p_name = tokens.expect_any(Symbol)?.clone();
        tokens.expect(Operator, ":")?;
        let p_type = parse_lo_type2(ctx, tokens)?;
        if !tokens.next_is(Delim, ")")? {
            tokens.expect(Delim, ",")?;
        }

        if fn_decl.locals.contains_key(&p_name.value) {
            return Err(LoError {
                message: format!(
                    "Found function param with conflicting name: {}",
                    p_name.value
                ),
                loc: p_name.loc.clone(),
            });
        }

        add_fn_param(ctx, &mut fn_decl, p_name.value, p_type);
    }

    let lo_output = if let Some(_) = tokens.eat(Operator, "->")? {
        parse_lo_type2(ctx, tokens)?
    } else {
        LoType::Void
    };

    lo_output.emit_components(&ctx, &mut fn_decl.wasm_type.outputs);
    fn_decl.lo_type.output = lo_output;

    Ok(fn_decl)
}

fn add_fn_param(ctx: &ModuleContext, fn_decl: &mut FnDecl, p_name: String, p_type: LoType) {
    let local_def = LocalDef {
        index: fn_decl.wasm_type.inputs.len() as u32,
        value_type: p_type.clone(),
    };
    fn_decl.locals.insert(p_name.clone(), local_def);

    p_type.emit_components(ctx, &mut fn_decl.wasm_type.inputs);
    fn_decl.lo_type.inputs.push(p_type);
}

fn parse_block(
    _ctx: &ModuleContext,
    tokens: &mut LoTokenStream,
) -> Result<Vec<LoTokenStream>, LoError> {
    let mut raw_exprs = vec![];
    let mut last_expr_tokens = LoTokenStream::new(vec![], LoLocation::internal());

    let mut depth = 0;
    tokens.expect(Delim, "{")?;
    loop {
        if let Some(t) = tokens.eat(Delim, "{")? {
            last_expr_tokens.tokens.push(t.clone());
            depth += 1;
            continue;
        }
        if let Some(t) = tokens.eat(Delim, "}")? {
            if depth == 0 {
                break;
            }
            last_expr_tokens.tokens.push(t.clone());
            depth -= 1;
            continue;
        }
        if depth == 0 {
            if let Some(semi) = tokens.eat(Delim, ";")? {
                last_expr_tokens.terminal_token = semi.clone();
                raw_exprs.push(last_expr_tokens);
                last_expr_tokens = LoTokenStream::new(vec![], LoLocation::internal());
                continue;
            }
        }
        last_expr_tokens.tokens.push(tokens.next().unwrap().clone());
    }
    if last_expr_tokens.tokens.len() != 0 {
        // reports error about missing semicolon
        tokens.expect(Delim, ";")?;
    }

    Ok(raw_exprs)
}

// TODO: support complex types
fn parse_lo_type2(ctx: &mut ModuleContext, tokens: &mut LoTokenStream) -> Result<LoType, LoError> {
    let return_type = tokens.expect_any(Symbol)?.clone();
    parse_lo_type(&return_type.to_sexpr(), ctx)
}

// pub for use in v1
pub fn parse_exprs(
    ctx: &mut BlockContext,
    body: Vec<LoTokenStream>,
) -> Result<Vec<LoInstr>, LoError> {
    let mut exprs = vec![];
    for mut tokens in body {
        exprs.push(parse_expr_to_end(ctx, &mut tokens)?);
    }
    Ok(exprs)
}

fn parse_expr_to_end(
    ctx: &mut BlockContext,
    tokens: &mut LoTokenStream,
) -> Result<LoInstr, LoError> {
    let expr = parse_expr(ctx, tokens, 0)?;

    if let Some(unexpected) = tokens.peek() {
        return Err(LoError {
            message: format!("Unexpected token after expression: {}", unexpected.value),
            loc: unexpected.loc.clone(),
        });
    }

    Ok(expr)
}

fn parse_expr(
    ctx: &mut BlockContext,
    tokens: &mut LoTokenStream,
    min_bp: u32,
) -> Result<LoInstr, LoError> {
    let mut primary = parse_primary(ctx, tokens)?;

    while tokens.peek().is_some() {
        let op_symbol = tokens.peek().unwrap().clone();
        let Some(op) = Op::parse(op_symbol) else {
            break;
        };

        if op.info.bp < min_bp {
            break;
        }

        tokens.next(); // skip operator
        primary = parse_postfix(ctx, tokens, primary, op)?;
    }

    Ok(primary)
}

fn parse_primary(ctx: &mut BlockContext, tokens: &mut LoTokenStream) -> Result<LoInstr, LoError> {
    if let Some(int) = tokens.eat_any(IntLiteral)?.cloned() {
        return parse_int_literal(int);
    }

    if let Some(_) = tokens.eat(Delim, "(")? {
        let expr = parse_expr(ctx, tokens, 0)?;
        tokens.expect(Delim, ")")?;
        return Ok(expr);
    }

    if let Some(_) = tokens.eat(Symbol, "return")? {
        return Ok(LoInstr::Return {
            value: Box::new(parse_expr(ctx, tokens, 0)?),
        });
    }

    // TODO: support `else`
    if let Some(_) = tokens.eat(Symbol, "if")? {
        let cond = parse_expr(ctx, tokens, 0)?;
        let then_branch_tokens = parse_block(ctx.module, tokens)?;
        let then_branch = parse_exprs(ctx, then_branch_tokens)?;

        return Ok(LoInstr::If {
            block_type: LoType::Void,
            cond: Box::new(cond),
            then_branch,
            else_branch: None,
        });
    }

    if let Some(let_token) = tokens.eat(Symbol, "let")?.cloned() {
        let local_name = tokens.expect_any(Symbol)?.clone();
        tokens.expect(Operator, "=")?;
        let value = parse_expr(ctx, tokens, 0)?;
        let value_type = value.get_type(ctx.module);

        if local_name.value == "_" {
            let drop_count = value_type.emit_components(&ctx.module, &mut vec![]);

            return Ok(LoInstr::Drop {
                value: Box::new(value),
                drop_count,
            });
        }

        if let Some(_) = ctx.module.globals.get(&local_name.value) {
            return Err(LoError {
                message: format!("Local name collides with global: {}", local_name.value),
                loc: local_name.loc.clone(),
            });
        };

        if ctx.block.get_own_local(&local_name.value).is_some() {
            return Err(LoError {
                message: format!("Duplicate local definition: {}", local_name.value),
                loc: local_name.loc.clone(),
            });
        }

        let local_indicies = ctx.push_local(local_name.value.clone(), value_type.clone());
        let values = local_indicies
            .map(|i| LoInstr::UntypedLocalGet { local_index: i })
            .collect();
        let bind_instr = LoInstr::MultiValueEmit { values };
        return compile_set(ctx, value, bind_instr, &let_token.loc);
    }

    if let Some(value) = tokens.eat_any(Symbol)?.cloned() {
        if let Some(fn_def) = ctx.module.fn_defs.get(&value.value) {
            let mut args = vec![];
            parse_fn_call_args(ctx, tokens, &mut args)?;

            return Ok(LoInstr::Call {
                fn_index: fn_def.fn_index,
                return_type: fn_def.kind.output.clone(),
                args,
            });
        }

        if let Some(struct_def) = ctx.module.struct_defs.get(&value.value) {
            let struct_name = value;

            let mut values = vec![];
            tokens.expect(Delim, "{")?;
            while let None = tokens.eat(Delim, "}")? {
                let field_name = tokens.expect_any(Symbol)?.clone();
                tokens.expect(Operator, ":")?;
                let field_value = parse_expr(ctx, tokens, 0)?;

                if !tokens.next_is(Delim, "}")? {
                    tokens.expect(Delim, ",")?;
                }

                let field_index = values.len();
                let Some(struct_field) = struct_def.fields.get(field_index) else {
                    return Err(LoError {
                        message: format!("Excess field values"),
                        loc: field_name.loc,
                    });
                };

                if &field_name.value != &struct_field.name {
                    return Err(LoError {
                        message: format!(
                            "Unexpected field name, expecting: `{}`",
                            struct_field.name
                        ),
                        loc: field_name.loc,
                    });
                }

                let field_value_type = field_value.get_type(ctx.module);
                if field_value_type != struct_field.value_type {
                    return Err(LoError {
                        message: format!(
                            "Invalid type for field {}.{}, expected: {}, got: {}",
                            struct_name.value,
                            field_name.value,
                            struct_field.value_type,
                            field_value_type
                        ),
                        // TODO: field_value.loc() is not available
                        loc: field_name.loc.clone(),
                    });
                }
                values.push(field_value);
            }

            return Ok(LoInstr::Casted {
                value_type: LoType::StructInstance {
                    name: struct_name.value,
                },
                expr: Box::new(LoInstr::MultiValueEmit { values }),
            });
        };

        if let Some(global) = ctx.module.globals.get(&value.value) {
            return Ok(LoInstr::GlobalGet {
                global_index: global.index,
            });
        };

        let Some(local) = ctx.block.get_local(&value.value) else {
            return Err(LoError {
                message: format!("Reading unknown variable: {}", value.value),
                loc: value.loc
            });
        };

        return compile_local_get(&ctx.module, local.index, &local.value_type).map_err(|message| {
            LoError {
                message,
                loc: value.loc,
            }
        });
    }

    let unexpected = tokens.peek().unwrap();
    return Err(LoError {
        message: format!("Unexpected token: {}", unexpected.value),
        loc: unexpected.loc.clone(),
    });
}

fn parse_postfix(
    ctx: &mut BlockContext,
    tokens: &mut LoTokenStream,
    primary: LoInstr,
    op: Op,
) -> Result<LoInstr, LoError> {
    let min_bp = op.info.get_min_bp_for_next();

    Ok(match op.tag {
        OpTag::Less => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32LessThenUnsigned,
            lhs: Box::new(primary),
            rhs: Box::new(parse_expr(ctx, tokens, min_bp)?),
        },
        OpTag::Add => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Add,
            lhs: Box::new(primary),
            rhs: Box::new(parse_expr(ctx, tokens, min_bp)?),
        },
        OpTag::Sub => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Sub,
            lhs: Box::new(primary),
            rhs: Box::new(parse_expr(ctx, tokens, min_bp)?),
        },
        OpTag::Mul => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Mul,
            lhs: Box::new(primary),
            rhs: Box::new(parse_expr(ctx, tokens, min_bp)?),
        },
        OpTag::Assign => {
            let value = parse_expr(ctx, tokens, min_bp)?;
            let value_type = value.get_type(ctx.module);
            let bind_type = primary.get_type(ctx.module);

            if value_type != bind_type {
                return Err(LoError {
                    message: format!(
                        "TypeError: Invalid types for '{}', \
                        needed {bind_type}, got {value_type}",
                        op.token.value
                    ),
                    loc: op.token.loc.clone(),
                });
            }

            compile_set(ctx, value, primary, &op.token.loc)?
        }
        OpTag::AddAssign => {
            let value = LoInstr::BinaryOp {
                kind: WasmBinaryOpKind::I32Add,
                lhs: Box::new(primary.clone()),
                rhs: Box::new(parse_expr(ctx, tokens, min_bp)?),
            };
            // TODO: lhs.loc() is not available
            compile_set(ctx, value, primary, &op.token.loc)?
        }
        OpTag::Dot => {
            let field_or_method_name = tokens.expect_any(Symbol)?.clone();
            if !tokens.next_is(Delim, "(").unwrap_or(false) {
                let field_name = field_or_method_name;

                let LoInstr::StructGet { struct_name, base_index, .. } = &primary else {
                    let lhs_type = primary.get_type(ctx.module);
                    return Err(LoError {
                        message: format!("Trying to get field '{}' on non struct: {lhs_type}", field_name.value),
                        loc: field_name.loc,
                    });
                };

                let struct_def = ctx.module.struct_defs.get(struct_name).unwrap(); // safe
                let Some(field) = struct_def.fields.iter().find(|f| &f.name == &field_name.value) else {
                    return Err(LoError {
                        message: format!("Unknown field {} in struct {struct_name}", field_name.value),
                        loc: field_name.loc,
                    });
                };

                let res = compile_local_get(
                    &ctx.module,
                    base_index + field.field_index,
                    &field.value_type,
                )
                .map_err(|message| LoError {
                    message,
                    // TODO: lhs.loc() is not available
                    loc: op.token.loc,
                })?;

                return Ok(res);
            }

            let method_name = field_or_method_name;
            let recevier_type = primary.get_type(ctx.module);

            let mut args = vec![primary];
            parse_fn_call_args(ctx, tokens, &mut args)?;

            let fn_name = fn_name_for_method(&recevier_type, &method_name.value);
            let Some(fn_def) = ctx.module.fn_defs.get(&fn_name) else {
                return Err(LoError {
                    message: format!("Unknown function: {fn_name}"),
                    loc: method_name.loc,
                });
            };

            LoInstr::Call {
                fn_index: fn_def.fn_index,
                return_type: fn_def.kind.output.clone(),
                args,
            }
        }
    })
}

fn parse_fn_call_args(
    ctx: &mut BlockContext,
    tokens: &mut LoTokenStream,
    args: &mut Vec<LoInstr>,
) -> Result<(), LoError> {
    tokens.expect(Delim, "(")?;
    while let None = tokens.eat(Delim, ")")? {
        args.push(parse_expr(ctx, tokens, 0)?);

        if !tokens.next_is(Delim, ")")? {
            tokens.expect(Delim, ",")?;
        }
    }

    Ok(())
}

fn parse_const_expr(
    ctx: &ModuleContext,
    tokens: &mut LoTokenStream,
    min_bp: u32,
) -> Result<LoInstr, LoError> {
    let mut primary = parse_const_primary(ctx, tokens)?;

    while tokens.peek().is_some() {
        let op_symbol = tokens.peek().unwrap().clone();
        let Some(op) = Op::parse(op_symbol) else {
            break;
        };

        if op.info.bp < min_bp {
            break;
        }

        tokens.next(); // skip operator
        primary = parse_const_postfix(ctx, tokens, primary, op)?;
    }

    Ok(primary)
}

fn parse_const_primary(
    ctx: &ModuleContext,
    tokens: &mut LoTokenStream,
) -> Result<LoInstr, LoError> {
    if let Some(int) = tokens.eat_any(IntLiteral)?.cloned() {
        return parse_int_literal(int);
    }

    if let Some(global_name) = tokens.eat_any(Symbol)?.cloned() {
        let Some(global) = ctx.globals.get(&global_name.value) else {
            return Err(LoError {
                message: format!("Reading unknown global: {}", global_name.value),
                loc: global_name.loc,
            });
        };

        return Ok(LoInstr::GlobalGet {
            global_index: global.index,
        });
    }

    let unexpected = tokens.peek().unwrap();
    return Err(LoError {
        message: format!("Unexpected token: {}", unexpected.value),
        loc: unexpected.loc.clone(),
    });
}

fn parse_const_postfix(
    ctx: &ModuleContext,
    tokens: &mut LoTokenStream,
    primary: LoInstr,
    op: Op,
) -> Result<LoInstr, LoError> {
    let min_bp = op.info.get_min_bp_for_next();

    Ok(match op.tag {
        OpTag::Less => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32LessThenUnsigned,
            lhs: Box::new(primary),
            rhs: Box::new(parse_const_expr(ctx, tokens, min_bp)?),
        },
        OpTag::Add => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Add,
            lhs: Box::new(primary),
            rhs: Box::new(parse_const_expr(ctx, tokens, min_bp)?),
        },
        OpTag::Sub => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Sub,
            lhs: Box::new(primary),
            rhs: Box::new(parse_const_expr(ctx, tokens, min_bp)?),
        },
        OpTag::Mul => LoInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Mul,
            lhs: Box::new(primary),
            rhs: Box::new(parse_const_expr(ctx, tokens, min_bp)?),
        },
        _ => {
            return Err(LoError {
                message: format!("Unsupported operator in const context: {}", op.token.value),
                loc: op.token.loc,
            });
        }
    })
}

fn parse_int_literal(int: LoToken) -> Result<LoInstr, LoError> {
    let value = int.value.parse().map_err(|_| LoError {
        message: format!("Parsing u32 (implicit) failed"),
        loc: int.loc,
    })?;

    Ok(LoInstr::U32Const { value })
}

fn fn_name_for_method(receiver_type: &LoType, method_name: &str) -> String {
    format!("{receiver_type}_{method_name}")
}
