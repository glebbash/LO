use crate::{ast::*, compiler::*, ir::*, lowering::*, tokens::*, wasm::*};
use alloc::{boxed::Box, collections::BTreeMap, format, string::String, vec, vec::Vec};
use LoleTokenType::*;

const RECEIVER_PARAM_NAME: &str = "self";

pub fn parse(mut tokens: LoleTokenStream) -> Result<WasmModule, LoleError> {
    let mut ctx = ModuleContext::default();

    while tokens.peek().is_some() {
        parse_top_level_expr(&mut ctx, &mut tokens)?;
        tokens.expect(LoleTokenType::Delim, ";")?;
    }

    process_delayed_actions(&mut ctx)?;

    Ok(ctx.wasm_module.take())
}

fn parse_top_level_expr(
    ctx: &mut ModuleContext,
    tokens: &mut LoleTokenStream,
) -> Result<(), LoleError> {
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
            tokens.expect(LoleTokenType::Delim, ";")?;

            if ctx.fn_defs.contains_key(&fn_decl.fn_name) {
                return Err(LoleError {
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
                kind: fn_decl.lole_type,
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

        let global_value = parse_const_expr(ctx, tokens)?;

        let lole_type = global_value.get_type(ctx);
        let Some(wasm_type) = lole_type.to_wasm_type() else {
            return Err(LoleError {
                message: format!("Unsupported type: {lole_type}"),
                // TODO: value.loc() is not available
                loc: let_token.loc,
            });
        };

        if ctx.globals.contains_key(&global_name.value) {
            return Err(LoleError {
                message: format!("Cannot redefine global: {}", global_name.value),
                loc: global_name.loc,
            });
        }

        ctx.globals.insert(
            global_name.value.clone(),
            GlobalDef {
                index: ctx.globals.len() as u32,
                mutable,
                value_type: lole_type,
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

    let unexpected = tokens.peek().unwrap();
    return Err(LoleError {
        message: format!("Unexpected top level token: {}", unexpected.value),
        loc: unexpected.loc.clone(),
    });
}

fn parse_fn_def(
    ctx: &mut ModuleContext,
    tokens: &mut LoleTokenStream,
    exported: bool,
) -> Result<(), LoleError> {
    let fn_decl = parse_fn_decl(ctx, tokens)?;
    let body = parse_block(ctx, tokens)?;

    if ctx.fn_defs.contains_key(&fn_decl.fn_name) {
        return Err(LoleError {
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
            kind: fn_decl.lole_type,
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
    loc: LoleLocation,
    lole_type: LoleFnType,
    wasm_type: WasmFnType,
    locals: BTreeMap<String, LocalDef>,
}

fn parse_fn_decl(
    ctx: &mut ModuleContext,
    tokens: &mut LoleTokenStream,
) -> Result<FnDecl, LoleError> {
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
        lole_type: LoleFnType {
            inputs: vec![],
            output: LoleType::Void,
        },
        wasm_type: WasmFnType {
            inputs: vec![],
            outputs: vec![],
        },
        locals: BTreeMap::new(),
    };

    if let Some(receiver_name) = receiver_name {
        let receiver_type = parse_lole_type(&receiver_name.to_sexpr(), ctx)?;
        fn_decl.fn_name = fn_name_for_method(&receiver_type, &fn_decl.fn_name);
        add_fn_param(ctx, &mut fn_decl, RECEIVER_PARAM_NAME.into(), receiver_type);
    }

    tokens.expect(Delim, "(")?;
    while let None = tokens.eat(Delim, ")")? {
        let p_name = tokens.expect_any(Symbol)?.clone();
        tokens.expect(Operator, ":")?;
        let p_type = parse_lole_type2(ctx, tokens)?;
        if !tokens.next_is(Delim, ")")? {
            tokens.expect(Delim, ",")?;
        }

        if fn_decl.locals.contains_key(&p_name.value) {
            return Err(LoleError {
                message: format!(
                    "Found function param with conflicting name: {}",
                    p_name.value
                ),
                loc: p_name.loc.clone(),
            });
        }

        add_fn_param(ctx, &mut fn_decl, p_name.value, p_type);
    }

    let lole_output = if let Some(_) = tokens.eat(Operator, "->")? {
        parse_lole_type2(ctx, tokens)?
    } else {
        LoleType::Void
    };

    lole_output.emit_components(&ctx, &mut fn_decl.wasm_type.outputs);

    Ok(fn_decl)
}

fn add_fn_param(ctx: &ModuleContext, fn_decl: &mut FnDecl, p_name: String, p_type: LoleType) {
    let local_def = LocalDef {
        index: fn_decl.wasm_type.inputs.len() as u32,
        value_type: p_type.clone(),
    };
    fn_decl.locals.insert(p_name.clone(), local_def);

    p_type.emit_components(ctx, &mut fn_decl.wasm_type.inputs);
    fn_decl.lole_type.inputs.push(p_type);
}

fn parse_block(
    _ctx: &ModuleContext,
    tokens: &mut LoleTokenStream,
) -> Result<Vec<LoleTokenStream>, LoleError> {
    let mut raw_exprs = vec![];
    let mut last_expr_tokens = LoleTokenStream::new(vec![], LoleLocation::internal());

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
                last_expr_tokens = LoleTokenStream::new(vec![], LoleLocation::internal());
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
fn parse_lole_type2(
    ctx: &mut ModuleContext,
    tokens: &mut LoleTokenStream,
) -> Result<LoleType, LoleError> {
    let return_type = tokens.expect_any(Symbol)?.clone();
    parse_lole_type(&return_type.to_sexpr(), ctx)
}

// pub for use in v1
pub fn parse_exprs(
    ctx: &mut BlockContext,
    body: Vec<LoleTokenStream>,
) -> Result<Vec<LoleInstr>, LoleError> {
    let mut exprs = vec![];
    for mut tokens in body {
        exprs.push(parse_expr(ctx, &mut tokens)?);
    }
    Ok(exprs)
}

fn parse_expr(
    ctx: &mut BlockContext,
    tokens: &mut LoleTokenStream,
) -> Result<LoleInstr, LoleError> {
    let lhs = parse_primary(ctx, tokens)?;

    let Ok(op) = tokens.expect_any(Operator).cloned() else {
        return Ok(lhs);
    };

    match op.value.as_str() {
        "<" => Ok(LoleInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32LessThenUnsigned,
            lhs: Box::new(lhs),
            rhs: Box::new(parse_primary(ctx, tokens)?),
        }),
        "*" => Ok(LoleInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Mul,
            lhs: Box::new(lhs),
            rhs: Box::new(parse_primary(ctx, tokens)?),
        }),
        "-" => Ok(LoleInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Sub,
            lhs: Box::new(lhs),
            rhs: Box::new(parse_primary(ctx, tokens)?),
        }),
        "+=" => {
            let value = LoleInstr::BinaryOp {
                kind: WasmBinaryOpKind::I32Add,
                lhs: Box::new(lhs.clone()),
                rhs: Box::new(parse_primary(ctx, tokens)?),
            };
            // TODO: lhs.loc() is not available
            compile_set(ctx, value, lhs, &op.loc)
        }
        "." => {
            let recevier_type = lhs.get_type(ctx.module);
            let method_name = tokens.expect_any(Symbol)?.clone();

            let mut args = vec![lhs];
            parse_fn_call_args(ctx, tokens, &mut args)?;

            let fn_name = fn_name_for_method(&recevier_type, &method_name.value);
            let Some(fn_def) = ctx.module.fn_defs.get(&fn_name) else {
                return Err(LoleError {
                    message: format!("Unknown function: {fn_name}"),
                    loc: method_name.loc,
                });
            };

            Ok(LoleInstr::Call {
                fn_index: fn_def.fn_index,
                return_type: fn_def.kind.output.clone(),
                args,
            })
        }
        _ => {
            return Err(LoleError {
                message: format!("Unknown operator: {}", op.value),
                loc: op.loc.clone(),
            });
        }
    }
}

fn parse_primary(
    ctx: &mut BlockContext,
    tokens: &mut LoleTokenStream,
) -> Result<LoleInstr, LoleError> {
    if let Some(int) = tokens.eat_any(IntLiteral)?.cloned() {
        return parse_int_literal(int);
    }

    if let Some(_) = tokens.eat(Symbol, "return")? {
        return Ok(LoleInstr::Return {
            value: Box::new(parse_expr(ctx, tokens)?),
        });
    }

    // TODO: support `else`
    if let Some(_) = tokens.eat(Symbol, "if")? {
        let cond = parse_expr(ctx, tokens)?;
        let then_branch_tokens = parse_block(ctx.module, tokens)?;
        let then_branch = parse_exprs(ctx, then_branch_tokens)?;

        return Ok(LoleInstr::If {
            block_type: LoleType::Void,
            cond: Box::new(cond),
            then_branch,
            else_branch: None,
        });
    }

    if let Some(let_token) = tokens.eat(Symbol, "let")?.cloned() {
        let local_name = tokens.expect_any(Symbol)?.clone();
        tokens.expect(Operator, "=")?;
        let value = parse_expr(ctx, tokens)?;
        let value_type = value.get_type(ctx.module);

        if local_name.value == "_" {
            let drop_count = value_type.emit_components(&ctx.module, &mut vec![]);

            return Ok(LoleInstr::Drop {
                value: Box::new(value),
                drop_count,
            });
        }

        if let Some(_) = ctx.module.globals.get(&local_name.value) {
            return Err(LoleError {
                message: format!("Local name collides with global: {}", local_name.value),
                loc: local_name.loc.clone(),
            });
        };

        if ctx.block.get_own_local(&local_name.value).is_some() {
            return Err(LoleError {
                message: format!("Duplicate local definition: {}", local_name.value),
                loc: local_name.loc.clone(),
            });
        }

        let local_indicies = ctx.push_local(local_name.value.clone(), value_type.clone());
        let values = local_indicies
            .map(|i| LoleInstr::UntypedLocalGet { local_index: i })
            .collect();
        let bind_instr = LoleInstr::MultiValueEmit { values };
        return compile_set(ctx, value, bind_instr, &let_token.loc);
    }

    if let Some(value) = tokens.eat_any(Symbol)?.cloned() {
        if let Some(fn_def) = ctx.module.fn_defs.get(&value.value) {
            let mut args = vec![];
            parse_fn_call_args(ctx, tokens, &mut args)?;

            return Ok(LoleInstr::Call {
                fn_index: fn_def.fn_index,
                return_type: fn_def.kind.output.clone(),
                args,
            });
        }

        if let Some(global) = ctx.module.globals.get(&value.value) {
            return Ok(LoleInstr::GlobalGet {
                global_index: global.index,
            });
        };

        let Some(local) = ctx.block.get_local(&value.value) else {
            return Err(LoleError {
                message: format!("Reading unknown variable: {}", value.value),
                loc: value.loc
            });
        };

        return compile_local_get(&ctx.module, local.index, &local.value_type).map_err(|message| {
            LoleError {
                message,
                loc: value.loc,
            }
        });
    }

    let unexpected = tokens.peek().unwrap();
    return Err(LoleError {
        message: format!("Unexpected token: {}", unexpected.value),
        loc: unexpected.loc.clone(),
    });
}

fn parse_fn_call_args(
    ctx: &mut BlockContext,
    tokens: &mut LoleTokenStream,
    args: &mut Vec<LoleInstr>,
) -> Result<(), LoleError> {
    tokens.expect(Delim, "(")?;
    while let None = tokens.eat(Delim, ")")? {
        args.push(parse_expr(ctx, tokens)?);

        if !tokens.next_is(Delim, ")")? {
            tokens.expect(Delim, ",")?;
        }
    }

    Ok(())
}

fn parse_const_expr(
    ctx: &ModuleContext,
    tokens: &mut LoleTokenStream,
) -> Result<LoleInstr, LoleError> {
    let lhs = parse_const_primary(ctx, tokens)?;

    let Ok(op) = tokens.expect_any(Operator).cloned() else {
        return Ok(lhs);
    };

    let rhs = parse_const_primary(ctx, tokens)?;

    match op.value.as_str() {
        "<" => Ok(LoleInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32LessThenUnsigned,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }),
        "*" => Ok(LoleInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Mul,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }),
        "-" => Ok(LoleInstr::BinaryOp {
            kind: WasmBinaryOpKind::I32Sub,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }),
        _ => {
            return Err(LoleError {
                message: format!("Unknown operator: {}", op.value),
                loc: op.loc.clone(),
            });
        }
    }
}

fn parse_const_primary(
    ctx: &ModuleContext,
    tokens: &mut LoleTokenStream,
) -> Result<LoleInstr, LoleError> {
    if let Some(int) = tokens.eat_any(IntLiteral)?.cloned() {
        return parse_int_literal(int);
    }

    if let Some(global_name) = tokens.eat_any(Symbol)?.cloned() {
        let Some(global) = ctx.globals.get(&global_name.value) else {
            return Err(LoleError {
                message: format!("Reading unknown global: {}", global_name.value),
                loc: global_name.loc,
            });
        };

        return Ok(LoleInstr::GlobalGet {
            global_index: global.index,
        });
    }

    let unexpected = tokens.peek().unwrap();
    return Err(LoleError {
        message: format!("Unexpected token: {}", unexpected.value),
        loc: unexpected.loc.clone(),
    });
}

fn parse_int_literal(int: LoleToken) -> Result<LoleInstr, LoleError> {
    let value = int.value.parse().map_err(|_| LoleError {
        message: format!("Parsing u32 (implicit) failed"),
        loc: int.loc,
    })?;

    Ok(LoleInstr::U32Const { value })
}

fn fn_name_for_method(receiver_type: &LoleType, method_name: &str) -> String {
    format!("{receiver_type}_{method_name}")
}
