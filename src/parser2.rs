use core::cell::RefCell;

use crate::{ast::*, compiler::*, ir::*, tokens::*, wasm::*};
use alloc::{boxed::Box, collections::BTreeMap, format, vec, vec::Vec};
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

        // TODO: implement other exports
        crate::wasi_io::debug(format!("\nunhandled: {:?}", tokens.peek()));
        return Err(LoleError::unreachable(file!(), line!()));
    }

    crate::wasi_io::debug(format!("{:?}", tokens.peek()));

    // TODO: implement everything else
    crate::wasi_io::debug(format!("\nunhandled: {:?}", tokens.peek()));
    return Err(LoleError::unreachable(file!(), line!()));
}

fn parse_fn_def(
    ctx: &mut ModuleContext,
    tokens: &mut LoleTokenStream,
    exported: bool,
) -> Result<(), LoleError> {
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
        if let Some(_) = tokens.eat(Delim, ")")? {
            break;
        };
        tokens.expect(Operator, ",")?;

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

    let body = parse_block(ctx, tokens)?;

    // v1 infra reuse
    {
        if exported {
            ctx.fn_exports.push(FnExport {
                in_name: fn_name.value.clone(),
                out_name: fn_name.value.clone(),
                loc: fn_name.loc.clone(),
            });
        }

        let mut wasm_outputs = vec![];
        lole_output.emit_components(&ctx, &mut wasm_outputs);

        let lole_fn_type = LoleFnType {
            inputs: lole_inputs,
            output: lole_output,
        };
        let locals_last_index = wasm_inputs.len() as u32;
        let wasm_fn_type = WasmFnType {
            inputs: wasm_inputs,
            outputs: wasm_outputs,
        };

        let type_index = ctx.insert_fn_type(wasm_fn_type);
        ctx.wasm_module.borrow_mut().functions.push(type_index);

        let fn_index = ctx.wasm_module.borrow_mut().functions.len() as u32 - 1;

        ctx.fn_defs.insert(
            fn_name.value.clone(),
            FnDef {
                local: true,
                fn_index,
                type_index,
                kind: lole_fn_type,
            },
        );

        ctx.fn_bodies.push(FnBody {
            fn_index,
            type_index,
            locals: RefCell::new(locals),
            locals_last_index,
            body: FnBodyExprs::V2(RefCell::new(body)),
        });
    }

    return Ok(());
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
        if let Some(_) = tokens.eat(Delim, "{")? {
            depth += 1;
        }
        if let Some(_) = tokens.eat(Delim, "}")? {
            if depth == 0 {
                break;
            }
            depth -= 1;
        }
        if let Some(semi) = tokens.eat(Delim, ";")? {
            last_expr_tokens.terminal_token = semi.clone();
            raw_exprs.push(last_expr_tokens);

            last_expr_tokens = LoleTokenStream::new(vec![], LoleLocation::internal());
        } else {
            last_expr_tokens.tokens.push(tokens.next().unwrap().clone());
        }
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
pub fn parse_expr(
    ctx: &ModuleContext,
    tokens: &mut LoleTokenStream,
) -> Result<LoleInstr, LoleError> {
    if let Some(int) = tokens.eat_any(IntLiteral)? {
        return Ok(LoleInstr::U32Const {
            // TODO: check for overflow
            value: int.value.parse().unwrap(),
        });
    }

    if let Some(_) = tokens.eat(Symbol, "return")? {
        return Ok(LoleInstr::Return {
            value: Box::new(parse_expr(ctx, tokens)?),
        });
    }

    if let Some(_) = tokens.eat(Symbol, "if")? {
        let cond = parse_expr(ctx, tokens)?;
        let body = parse_block(ctx, tokens)?;

        crate::wasi_io::debug(format!("\ncond: {cond:?}\nbody.len(): {}", body.len()));
        return Err(LoleError::unreachable(file!(), line!()));
    }

    // TODO: implement
    crate::wasi_io::debug(format!("\nunhandled: {:?}", tokens.peek()));
    return Err(LoleError::unreachable(file!(), line!()));
}
