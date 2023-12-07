use crate::{ast::*, compiler::*, ir::*, tokens::*, wasm::*};
use alloc::{boxed::Box, collections::BTreeMap, format, string::String, vec, vec::Vec};
use LoleTokenType::*;

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

            let type_index = ctx.insert_fn_type(fn_decl.wasm_type);

            let fn_index = ctx.imported_fns_count;
            ctx.imported_fns_count += 1;

            let fn_def = FnDef {
                local: false,
                fn_index,
                type_index,
                kind: fn_decl.lole_type,
            };
            ctx.fn_defs.insert(fn_decl.fn_name.value.clone(), fn_def);
            ctx.wasm_module.borrow_mut().imports.push(WasmImport {
                module_name: module_name.value.clone(),
                item_name: fn_decl.fn_name.value.clone(),
                item_desc: WasmImportDesc::Func { type_index },
            });
        }

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

    if exported {
        ctx.fn_exports.push(FnExport {
            in_name: fn_decl.fn_name.value.clone(),
            out_name: fn_decl.fn_name.value.clone(),
            loc: fn_decl.fn_name.loc.clone(),
        });
    }

    let locals_last_index = fn_decl.wasm_type.inputs.len() as u32;
    let type_index = ctx.insert_fn_type(fn_decl.wasm_type);
    ctx.wasm_module.borrow_mut().functions.push(type_index);

    let fn_index = ctx.wasm_module.borrow_mut().functions.len() as u32 - 1;

    ctx.fn_defs.insert(
        fn_decl.fn_name.value.clone(),
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
    fn_name: LoleToken,
    lole_type: LoleFnType,
    wasm_type: WasmFnType,
    locals: BTreeMap<String, LocalDef>,
}

fn parse_fn_decl(
    ctx: &mut ModuleContext,
    tokens: &mut LoleTokenStream,
) -> Result<FnDecl, LoleError> {
    let fn_name = tokens.expect_any(Symbol)?.clone();
    if ctx.fn_defs.contains_key(&fn_name.value) {
        return Err(LoleError {
            message: format!("Cannot redefine function: {}", fn_name.value),
            loc: fn_name.loc.clone(),
        });
    }

    let mut lole_inputs = vec![];
    let mut wasm_inputs = vec![];
    let mut locals = BTreeMap::new();

    tokens.expect(Delim, "(")?;
    while let None = tokens.eat(Delim, ")")? {
        let p_name = tokens.expect_any(Symbol)?.clone();
        tokens.expect(Operator, ":")?;
        let p_type = parse_lole_type2(ctx, tokens)?;
        if !tokens.check_next(Delim, ")")? {
            tokens.expect(Delim, ",")?;
        }

        if locals.contains_key(&p_name.value) {
            return Err(LoleError {
                message: format!(
                    "Found function param with conflicting name: {}",
                    p_name.value
                ),
                loc: p_name.loc.clone(),
            });
        }

        let local_index = wasm_inputs.len() as u32;
        p_type.emit_components(&ctx, &mut wasm_inputs);

        locals.insert(
            p_name.value.clone(),
            LocalDef {
                index: local_index,
                value_type: p_type.clone(),
            },
        );
        lole_inputs.push(p_type);
    }

    let lole_output = if let Some(_) = tokens.eat(Operator, "->")? {
        parse_lole_type2(ctx, tokens)?
    } else {
        LoleType::Void
    };

    let mut wasm_outputs = vec![];
    lole_output.emit_components(&ctx, &mut wasm_outputs);

    let lole_type = LoleFnType {
        inputs: lole_inputs,
        output: lole_output,
    };
    let wasm_type = WasmFnType {
        inputs: wasm_inputs,
        outputs: wasm_outputs,
    };

    Ok(FnDecl {
        fn_name,
        lole_type,
        wasm_type,
        locals,
    })
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
    let lhs = parse_operand(ctx, tokens)?;

    if tokens.peek().is_none() {
        return Ok(lhs);
    }

    let Some(op) = tokens.eat_any(Operator)?.cloned() else {
        return Ok(lhs);
    };

    let rhs = parse_operand(ctx, tokens)?;

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

fn parse_operand(
    ctx: &mut BlockContext,
    tokens: &mut LoleTokenStream,
) -> Result<LoleInstr, LoleError> {
    if let Some(int) = tokens.eat_any(IntLiteral)?.cloned() {
        return Ok(LoleInstr::U32Const {
            value: int.value.parse().map_err(|_| LoleError {
                message: format!("Parsing u32 (implicit) failed"),
                loc: int.loc.clone(),
            })?,
        });
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

        let value_type = value.get_type(ctx.module);
        let local_indicies = ctx.push_local(local_name.value.clone(), value_type.clone());

        let values = local_indicies
            .map(|i| LoleInstr::UntypedLocalGet { local_index: i })
            .collect();

        let bind_instr = LoleInstr::MultiValueEmit { values };

        return compile_set(ctx, value, bind_instr, &let_token.loc);
    }

    if let Some(value) = tokens.eat_any(Symbol)?.cloned() {
        if let Some(fn_def) = ctx.module.fn_defs.get(&value.value) {
            tokens.expect(Delim, "(")?;
            // TODO: support multiple arguments
            let arg = parse_expr(ctx, tokens)?;
            tokens.expect(Delim, ")")?;

            return Ok(LoleInstr::Call {
                fn_index: fn_def.fn_index,
                return_type: fn_def.kind.output.clone(),
                args: vec![arg],
            });
        }

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
