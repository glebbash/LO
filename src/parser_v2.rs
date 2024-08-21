use crate::{ast::*, core::*, lexer::*};
use alloc::{boxed::Box, format, string::String, vec::Vec};

use LoTokenType::*;

#[derive(Debug)]
pub struct FileInfo {
    pub path: String,
    pub ast: AST,
}

pub fn parse_file_and_deps(
    files: &mut Vec<FileInfo>,
    file_name: &str,
    loc: &LoLocation,
) -> Result<(), LoError> {
    let file_path = resolve_path(file_name, &loc.file_name);

    for file in files.iter() {
        // file already parsed, skip
        if file.path == file_path {
            return Ok(());
        }
    }

    let chars = file_read_utf8(&file_path).map_err(|message| LoError {
        message,
        loc: loc.clone(),
    })?;
    let tokens = Lexer::lex(&file_path, &chars)?;
    let ast = ParserV2::parse(tokens)?;

    let mut includes = Vec::new();
    for expr in &ast.exprs {
        if let TopLevelExpr::Include(include) = expr {
            includes.push(include.clone());
        };
    }

    files.push(FileInfo {
        path: file_path.into(),
        ast,
    });

    for include in includes {
        parse_file_and_deps(files, &include.file_path, &include.loc)?;
    }

    Ok(())
}

pub struct ParserV2 {
    pub tokens: Vec<LoToken>,
    pub tokens_processed: usize,
    pub terminal_token: LoToken,
}

impl ParserV2 {
    pub fn parse(tokens: Tokens) -> Result<AST, LoError> {
        let mut parser = ParserV2 {
            tokens: tokens.tokens,
            tokens_processed: 0,
            terminal_token: LoToken {
                type_: LoTokenType::Symbol,
                value: "<EOF>".into(),
                loc: tokens.end_loc,
            },
        };

        let mut ast = AST {
            exprs: Vec::new(),
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

        if let Some(include_token) = self.eat(Symbol, "include")?.cloned() {
            let mut loc = include_token.loc;
            let file_path = self.expect_any(StringLiteral)?;
            loc.end_pos = file_path.loc.end_pos.clone();

            return Ok(TopLevelExpr::Include(IncludeExpr {
                file_path: file_path.value.clone(),
                loc,
            }));
        }

        let unexpected = self.current();
        return Err(LoError {
            message: format!("Unexpected top level token: {:?}", unexpected.value),
            loc: unexpected.loc.clone(),
        });
    }

    fn parse_fn_def(&mut self, exported: bool, mut loc: LoLocation) -> Result<FnDefExpr, LoError> {
        let fn_name = self.expect_any(Symbol)?.clone().value;
        let fn_params = self.parse_fn_params()?;
        let _ = self.expect(Operator, ":")?;
        let return_type = self.parse_type_expr()?;
        let body = self.parse_code_block_expr()?;

        loc.end_pos = body.loc.end_pos.clone();

        return Ok(FnDefExpr {
            exported,
            fn_name,
            fn_params,
            return_type,
            body,
            loc,
        });
    }

    fn parse_fn_params(&mut self) -> Result<Vec<FnParam>, LoError> {
        let mut params = Vec::<FnParam>::new();

        let _ = self.expect(Delim, "(")?;

        while let None = self.eat(Delim, ")")? {
            let p_name = self.expect_any(Symbol)?.clone();
            self.expect(Operator, ":")?;
            let p_type = self.parse_type_expr()?;

            if !self.next_is(Delim, ")")? {
                self.expect(Delim, ",")?;
            }

            params.push(FnParam {
                name: p_name.value,
                type_: p_type,
            });
        }

        Ok(params)
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
            exprs: Vec::new(),
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

        if let Some(symbol) = self.eat_any(Symbol)? {
            return Ok(CodeExpr::VarLoad(VarLoadExpr {
                name: symbol.value.clone(),
                loc: symbol.loc.clone(),
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

    fn next_is(&mut self, type_: LoTokenType, value: &str) -> Result<bool, LoError> {
        match self.peek() {
            Some(token) if token.is(type_, value) => Ok(true),
            Some(_) => Ok(false),
            _ => self.err_eof(format!("Unexpected EOF")),
        }
    }

    fn peek(&self) -> Option<&LoToken> {
        self.tokens.get(self.tokens_processed)
    }

    fn next(&mut self) -> Option<&LoToken> {
        let token = self.tokens.get(self.tokens_processed);
        self.tokens_processed += 1;
        token
    }

    fn current(&self) -> &LoToken {
        if let Some(token) = self.tokens.get(self.tokens_processed) {
            &token
        } else {
            &self.terminal_token
        }
    }

    fn err_eof<T>(&self, message: String) -> Result<T, LoError> {
        Err(LoError {
            message,
            loc: self.terminal_token.loc.clone(),
        })
    }
}
