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
        let mut parser = ParserV2 {
            tokens: tokens.tokens,
            index: 0,
            terminal_token: LoToken {
                type_: LoTokenType::Symbol,
                value: "<EOF>".into(),
                loc: tokens.end_loc,
            },
        };

        let mut ast = AST {
            exprs: vec![],
            comments: tokens.comments,
        };

        parser.parse_file(&mut ast)?;

        Ok(ast)
    }

    fn parse_file(&mut self, ast: &mut AST) -> Result<(), LoError> {
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

        Ok(())
    }

    fn parse_top_level_expr(&mut self) -> Result<TopLevelExpr, LoError> {
        if let Some(export_token) = self.eat(Symbol, "export")?.cloned() {
            if let Some(_) = self.eat(Symbol, "fn")? {
                let fn_def = self.parse_fn_def(true, export_token.loc)?;
                return Ok(TopLevelExpr::FnDef(fn_def));
            }

            let unexpected = self.current();
            return Err(LoError {
                message: format!("Unexpected exportable: {:?}", unexpected.value),
                loc: unexpected.loc.clone(),
            });
        }

        if let Some(fn_token) = self.eat(Symbol, "fn")?.cloned() {
            let fn_def = self.parse_fn_def(false, fn_token.loc)?;
            return Ok(TopLevelExpr::FnDef(fn_def));
        }

        let unexpected = self.current();
        return Err(LoError {
            message: format!("Unexpected top level token: {:?}", unexpected.value),
            loc: unexpected.loc.clone(),
        });
    }

    fn parse_fn_def(&mut self, exported: bool, mut loc: LoLocation) -> Result<FnDefExpr, LoError> {
        let fn_name = self.expect_any(Symbol)?.clone().value;
        let _ = self.expect(Delim, "(")?;
        let _ = self.expect(Delim, ")")?;
        let _ = self.expect(Operator, ":")?;
        let return_type = self.parse_type_expr()?;
        let body = self.parse_code_block_expr()?;

        // TODO: is it pos or end_pos?
        loc.end_pos = self.current().loc.pos.clone();

        return Ok(FnDefExpr {
            exported,
            fn_name,
            return_type,
            body,
            loc,
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
        let open_brace = self.expect(Delim, "{")?;

        let mut code_block = CodeBlockExpr {
            exprs: vec![],
            loc: open_brace.loc.clone(),
        };

        while let None = self.eat(Delim, "}")? {
            let expr = self.parse_code_expr()?;
            code_block.exprs.push(expr);

            self.expect(Delim, ";")?;
        }

        // close curly pos
        code_block.loc.end_pos = self.current().loc.end_pos.clone();

        return Ok(code_block);
    }

    fn parse_code_expr(&mut self) -> Result<CodeExpr, LoError> {
        if let Some(return_token) = self.eat(Symbol, "return")?.cloned() {
            let mut loc = return_token.loc;
            let expr = self.parse_code_expr()?;
            loc.end_pos = expr.loc().end_pos.clone();

            return Ok(CodeExpr::Return(ReturnExpr {
                expr: Box::new(expr),
                loc,
            }));
        };

        if let Some(int) = self.eat_any(IntLiteral)? {
            return Ok(CodeExpr::IntLiteral(IntLiteralExpr {
                value: int.value.clone(),
                loc: int.loc.clone(),
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
