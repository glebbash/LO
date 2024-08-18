use crate::{ast::*, core::*, lexer::*};
use alloc::{boxed::Box, format, vec};

use LoTokenType::*;

pub fn parse(mut tokens: LoTokenStream) -> Result<AST, LoError> {
    let mut ast = AST { exprs: vec![] };

    while tokens.peek().is_some() {
        let expr = parse_top_level_expr(&mut tokens)?;
        ast.exprs.push(expr);

        tokens.expect(Delim, ";")?;
    }

    if let Some(unexpected) = tokens.peek() {
        return Err(LoError {
            message: format!("Unexpected top level token: {}", unexpected.value),
            loc: unexpected.loc.clone(),
        });
    }

    Ok(ast)
}

fn parse_top_level_expr(tokens: &mut LoTokenStream) -> Result<TopLevelExpr, LoError> {
    if let Some(_) = tokens.eat(Symbol, "export")? {
        if let Some(_) = tokens.eat(Symbol, "fn")? {
            let fn_def = parse_fn_def(tokens, true)?;
            return Ok(TopLevelExpr::FnDef(fn_def));
        }

        let unexpected = tokens.current();
        return Err(LoError {
            message: format!("Unexpected exportable: {:?}", unexpected.value),
            loc: unexpected.loc.clone(),
        });
    }

    if let Some(_) = tokens.eat(Symbol, "fn")? {
        let fn_def = parse_fn_def(tokens, false)?;
        return Ok(TopLevelExpr::FnDef(fn_def));
    }

    let unexpected = tokens.current();
    return Err(LoError {
        message: format!("Unexpected top level token: {:?}", unexpected.value),
        loc: unexpected.loc.clone(),
    });
}

fn parse_fn_def(tokens: &mut LoTokenStream, exported: bool) -> Result<FnDefExpr, LoError> {
    let fn_name = tokens.expect_any(Symbol)?.clone().value;
    let _ = tokens.expect(Delim, "(")?;
    let _ = tokens.expect(Delim, ")")?;
    let _ = tokens.expect(Operator, ":")?;
    let return_type = parse_type_expr(tokens)?;
    let body = parse_code_block_expr(tokens)?;

    return Ok(FnDefExpr {
        exported,
        fn_name,
        return_type,
        body,
    });
}

fn parse_type_expr(tokens: &mut LoTokenStream) -> Result<TypeExpr, LoError> {
    if let Some(_) = tokens.eat(Symbol, "u32")? {
        return Ok(TypeExpr::U32);
    }

    let unexpected = tokens.current();
    return Err(LoError {
        message: format!("Unexpected type expr token: {:?}", unexpected.value),
        loc: unexpected.loc.clone(),
    });
}

fn parse_code_block_expr(tokens: &mut LoTokenStream) -> Result<CodeBlockExpr, LoError> {
    let mut code_block = CodeBlockExpr { exprs: vec![] };

    tokens.expect(Delim, "{")?;

    while let None = tokens.eat(Delim, "}")? {
        let expr = parse_code_expr(tokens)?;
        code_block.exprs.push(expr);

        tokens.expect(Delim, ";")?;
    }

    return Ok(code_block);
}

fn parse_code_expr(tokens: &mut LoTokenStream) -> Result<CodeExpr, LoError> {
    if let Some(_) = tokens.eat(Symbol, "return")? {
        let expr = parse_code_expr(tokens)?;
        return Ok(CodeExpr::Return(ReturnExpr {
            expr: Box::new(expr),
        }));
    };

    if let Some(t) = tokens.eat_any(IntLiteral)? {
        return Ok(CodeExpr::IntLiteral(IntLiteralExpr {
            value: t.value.clone(),
        }));
    };

    let unexpected = tokens.current();
    return Err(LoError {
        message: format!("Unexpected code expr token: {:?}", unexpected.value),
        loc: unexpected.loc.clone(),
    });
}
