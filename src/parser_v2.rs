use crate::{ast::*, core::*, lexer::*};
use alloc::{boxed::Box, format, vec, vec::Vec};

use LoTokenType::*;

pub struct ParserV2 {
    pub tokens: Vec<LoToken>,
    pub index: usize,
    pub terminal_token: LoToken,
}

impl ParserV2 {
    pub fn parse(tokens: Tokens) -> Result<AST, LoError> {
        let mut parser = ParserV2::new(tokens.tokens, tokens.end_loc);
        let ast = parser.parse_file()?;
        Ok(ast)
    }

    fn new(tokens: Vec<LoToken>, end_location: LoLocation) -> Self {
        Self {
            tokens,
            index: 0,
            terminal_token: LoToken {
                type_: LoTokenType::Symbol,
                value: "<EOF>".into(),
                loc: end_location,
            },
        }
    }

    fn parse_file(&mut self) -> Result<AST, LoError> {
        let mut ast = AST { exprs: vec![] };

        while self.peek().is_some() {
            let expr = self.parse_top_level_expr()?;
            ast.exprs.push(expr);

            self.expect(Delim, ";")?;
        }

        if let Some(unexpected) = self.peek() {
            return Err(LoError {
                message: format!("Unexpected top level token: {}", unexpected.value),
                loc: unexpected.loc.clone(),
            });
        }

        Ok(ast)
    }

    fn parse_top_level_expr(&mut self) -> Result<TopLevelExpr, LoError> {
        if let Some(_) = self.eat(Symbol, "export")? {
            if let Some(_) = self.eat(Symbol, "fn")? {
                let fn_def = self.parse_fn_def(true)?;
                return Ok(TopLevelExpr::FnDef(fn_def));
            }

            let unexpected = self.current();
            return Err(LoError {
                message: format!("Unexpected exportable: {:?}", unexpected.value),
                loc: unexpected.loc.clone(),
            });
        }

        if let Some(_) = self.eat(Symbol, "fn")? {
            let fn_def = self.parse_fn_def(false)?;
            return Ok(TopLevelExpr::FnDef(fn_def));
        }

        let unexpected = self.current();
        return Err(LoError {
            message: format!("Unexpected top level token: {:?}", unexpected.value),
            loc: unexpected.loc.clone(),
        });
    }

    fn parse_fn_def(&mut self, exported: bool) -> Result<FnDefExpr, LoError> {
        let fn_name = self.expect_any(Symbol)?.clone().value;
        let _ = self.expect(Delim, "(")?;
        let _ = self.expect(Delim, ")")?;
        let _ = self.expect(Operator, ":")?;
        let return_type = self.parse_type_expr()?;
        let body = self.parse_code_block_expr()?;

        return Ok(FnDefExpr {
            exported,
            fn_name,
            return_type,
            body,
        });
    }

    fn parse_type_expr(&mut self) -> Result<TypeExpr, LoError> {
        if let Some(_) = self.eat(Symbol, "u32")? {
            return Ok(TypeExpr::U32);
        }

        let unexpected = self.current();
        return Err(LoError {
            message: format!("Unexpected type expr token: {:?}", unexpected.value),
            loc: unexpected.loc.clone(),
        });
    }

    fn parse_code_block_expr(&mut self) -> Result<CodeBlockExpr, LoError> {
        let mut code_block = CodeBlockExpr { exprs: vec![] };

        self.expect(Delim, "{")?;

        while let None = self.eat(Delim, "}")? {
            let expr = self.parse_code_expr()?;
            code_block.exprs.push(expr);

            self.expect(Delim, ";")?;
        }

        return Ok(code_block);
    }

    fn parse_code_expr(&mut self) -> Result<CodeExpr, LoError> {
        if let Some(_) = self.eat(Symbol, "return")? {
            let expr = self.parse_code_expr()?;
            return Ok(CodeExpr::Return(ReturnExpr {
                expr: Box::new(expr),
            }));
        };

        if let Some(t) = self.eat_any(IntLiteral)? {
            return Ok(CodeExpr::IntLiteral(IntLiteralExpr {
                value: t.value.clone(),
            }));
        };

        let unexpected = self.current();
        return Err(LoError {
            message: format!("Unexpected code expr token: {:?}", unexpected.value),
            loc: unexpected.loc.clone(),
        });
    }

    // utils

    fn expect_any(&mut self, type_: LoTokenType) -> Result<&LoToken, LoError> {
        match self.peek() {
            Some(token) if token.is_any(type_) => Ok(self.next().unwrap()),
            other => {
                let unexpected = other.unwrap_or(&self.terminal_token);
                Err(LoError {
                    message: format!("Unexpected token '{}', wanted {type_:?}", unexpected.value),
                    loc: unexpected.loc.clone(),
                })
            }
        }
    }

    fn expect(&mut self, type_: LoTokenType, value: &str) -> Result<&LoToken, LoError> {
        match self.peek() {
            Some(token) if token.is(type_, value) => Ok(self.next().unwrap()),
            other => {
                let unexpected = other.unwrap_or(&self.terminal_token);
                Err(LoError {
                    message: format!("Unexpected token '{}', wanted '{value}'", unexpected.value),
                    loc: unexpected.loc.clone(),
                })
            }
        }
    }

    fn eat_any(&mut self, type_: LoTokenType) -> Result<Option<&LoToken>, LoError> {
        let was_some = self.peek().is_some();
        match self.expect_any(type_) {
            Ok(t) => Ok(Some(t)),
            Err(_) if was_some => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn eat(&mut self, type_: LoTokenType, value: &str) -> Result<Option<&LoToken>, LoError> {
        let was_some = self.peek().is_some();
        match self.expect(type_, value) {
            Ok(t) => Ok(Some(t)),
            Err(_) if was_some => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn peek(&self) -> Option<&LoToken> {
        self.tokens.get(self.index)
    }

    fn next(&mut self) -> Option<&LoToken> {
        let token = self.tokens.get(self.index);
        self.index += 1;
        token
    }

    fn current(&self) -> &LoToken {
        if let Some(token) = self.tokens.get(self.index) {
            &token
        } else {
            &self.terminal_token
        }
    }
}
