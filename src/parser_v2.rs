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
        parse_file_and_deps(
            files,
            &Lexer::unescape_string(&include.file_path),
            &include.loc,
        )?;
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
                type_: LoTokenType::Terminal,
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
                message: format!(
                    "Unexpected top level token: {}, EOF expected",
                    unexpected.value
                ),
                loc: unexpected.loc.clone(),
            });
        }

        Ok(())
    }

    fn parse_top_level_expr(&mut self) -> Result<TopLevelExpr, LoError> {
        if let Some(_) = self.eat(Symbol, "export")? {
            let loc = self.prev().loc.clone();

            if let Some(_) = self.eat(Symbol, "fn")? {
                let fn_def = self.parse_fn_def(true, loc)?;
                return Ok(TopLevelExpr::FnDef(fn_def));
            }

            let unexpected = self.current();
            return Err(LoError {
                message: format!("Unexpected exportable: {:?}", unexpected.value),
                loc: unexpected.loc.clone(),
            });
        }

        if let Some(_) = self.eat(Symbol, "fn")? {
            let loc = self.prev().loc.clone();

            let fn_def = self.parse_fn_def(false, loc)?;
            return Ok(TopLevelExpr::FnDef(fn_def));
        }

        if let Some(_) = self.eat(Symbol, "include")? {
            let mut loc = self.prev().loc.clone();

            let file_path = self.expect_any(StringLiteral)?;
            loc.end_pos = file_path.loc.end_pos.clone();

            return Ok(TopLevelExpr::Include(IncludeExpr {
                file_path: file_path.value.clone(),
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "import")? {
            let mut loc = self.prev().loc.clone();

            self.expect(Symbol, "from")?;
            let module_name = self.expect_any(StringLiteral)?.clone();

            self.expect(Delim, "{")?;

            let mut items = Vec::new();
            loop {
                if let Some(_) = self.eat(Delim, "}")? {
                    loc.end_pos = self.prev().loc.end_pos.clone();
                    break;
                }

                if let Some(_) = self.eat(Symbol, "fn")? {
                    let decl = self.parse_fn_decl()?;
                    items.push(ImportItem::FnDecl(decl))
                } else {
                    let unexpected = self.current();

                    return Err(LoError {
                        message: format!(
                            "Unexpected token in importable item: {:?}",
                            unexpected.value
                        ),
                        loc: unexpected.loc.clone(),
                    });
                }

                self.expect(Delim, ";")?;
            }

            return Ok(TopLevelExpr::Import(ImportExpr {
                module_name: module_name.value,
                items,
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "global")? {
            let mut loc = self.prev().loc.clone();

            let global_name = self.expect_any(Symbol)?.clone();
            self.expect(Operator, "=")?;
            let expr = self.parse_code_expr(0)?;

            loc.end_pos = self.prev().loc.end_pos.clone();

            return Ok(TopLevelExpr::GlobalDef(GlobalDefExpr {
                global_name: global_name.value.clone(),
                expr,
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
        let decl = self.parse_fn_decl()?;
        let body = self.parse_code_block_expr()?;

        loc.end_pos = body.loc.end_pos.clone();

        Ok(FnDefExpr {
            exported,
            decl,
            body,
            loc,
        })
    }

    fn parse_fn_decl(&mut self) -> Result<FnDeclExpr, LoError> {
        let mut loc = self.prev().loc.clone();

        let fn_name = self.expect_any(Symbol)?.clone().value;
        let fn_params = self.parse_fn_params()?;

        let return_type = if let Some(_) = self.eat(Operator, ":")? {
            Some(self.parse_type_expr()?)
        } else {
            None
        };

        loc.end_pos = self.prev().loc.end_pos.clone();

        Ok(FnDeclExpr {
            fn_name,
            fn_params,
            return_type,
            loc,
        })
    }

    fn parse_fn_params(&mut self) -> Result<Vec<FnParam>, LoError> {
        let mut params = Vec::<FnParam>::new();

        let _ = self.expect(Delim, "(")?;

        while let None = self.eat(Delim, ")")? {
            let p_name = self.expect_any(Symbol)?.clone();
            self.expect(Operator, ":")?;
            let p_type = self.parse_type_expr()?;

            if !self.current().is(Delim, ")") {
                self.expect(Delim, ",")?;
            }

            params.push(FnParam {
                name: p_name.value,
                type_: p_type,
                loc: p_name.loc,
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
        self.expect(Delim, "{")?;

        let mut code_block = CodeBlockExpr {
            exprs: Vec::new(),
            loc: self.prev().loc.clone(),
        };

        while let None = self.eat(Delim, "}")? {
            let expr = self.parse_code_expr(0)?;
            code_block.exprs.push(expr);

            self.expect(Delim, ";")?;
        }

        // close curly pos
        code_block.loc.end_pos = self.prev().loc.end_pos.clone();

        return Ok(code_block);
    }

    fn parse_code_expr(&mut self, min_bp: u32) -> Result<CodeExpr, LoError> {
        let mut primary = self.parse_code_expr_primary()?;

        while self.peek().is_some() {
            let op_symbol = self.current().clone();
            let Some(op) = InfixOp::parse(op_symbol) else {
                break;
            };

            if op.info.bp < min_bp {
                break;
            }

            self.next(); // skip operator
            primary = self.parse_code_expr_postfix(primary, op)?;
        }

        Ok(primary)
    }

    fn parse_code_expr_primary(&mut self) -> Result<CodeExpr, LoError> {
        if let Some(_) = self.eat(Symbol, "return")? {
            let mut loc = self.prev().loc.clone();

            let expr = self.parse_code_expr(0)?;

            loc.end_pos = self.prev().loc.end_pos.clone();

            return Ok(CodeExpr::Return(ReturnExpr {
                expr: Box::new(expr),
                loc,
            }));
        };

        if let Some(_) = self.eat(Symbol, "if")? {
            let mut loc = self.prev().loc.clone();

            let expr = Box::new(self.parse_code_expr(0)?);
            let then_block = Box::new(self.parse_code_block_expr()?);
            let mut else_block = ElseBlock::None;
            if let Some(_) = self.eat(Symbol, "else")? {
                if self.current().is(Symbol, "if") {
                    let if_expr = self.parse_code_expr(0)?;
                    else_block = ElseBlock::ElseIf(Box::new(if_expr));
                } else {
                    let block = self.parse_code_block_expr()?;
                    else_block = ElseBlock::Else(Box::new(block));
                }
            }

            loc.end_pos = self.prev().loc.end_pos.clone();

            return Ok(CodeExpr::If(IfExpr {
                cond: expr,
                then_block,
                else_block,
                loc,
            }));
        };

        if let Some(int) = self.eat_any(IntLiteral)? {
            let result = if int.value.starts_with("0x") {
                u32::from_str_radix(&int.value[2..], 16)
            } else {
                int.value.parse()
            };

            return Ok(CodeExpr::IntLiteral(IntLiteralExpr {
                repr: int.value.clone(),
                value: result.map_err(|_| LoError {
                    message: format!("Parsing u32 failed: {}", int.value),
                    loc: int.loc.clone(),
                })?,
                loc: int.loc.clone(),
            }));
        };

        if let Some(string) = self.eat_any(StringLiteral)? {
            return Ok(CodeExpr::StringLiteral(StringLiteralExpr {
                repr: string.value.clone(),
                value: Lexer::unescape_string(&string.value),
                loc: string.loc.clone(),
            }));
        };

        if let Some(_) = self.eat(Symbol, "let")? {
            let mut loc = self.prev().loc.clone();

            let local_name = self.expect_any(Symbol)?.clone();
            self.expect(Operator, "=")?;
            let value = self.parse_code_expr(0)?;

            loc.end_pos = self.prev().loc.end_pos.clone();

            return Ok(CodeExpr::Local(LocalExpr {
                local_name: local_name.value,
                value: Box::new(value),
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "loop")? {
            let mut loc = self.prev().loc.clone();

            let body = self.parse_code_block_expr()?;

            loc.end_pos = self.prev().loc.end_pos.clone();

            return Ok(CodeExpr::Loop(LoopExpr {
                body: Box::new(body),
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "break")? {
            let loc = self.prev().loc.clone();

            return Ok(CodeExpr::Break(BreakExpr { loc }));
        }

        if let Some(_) = self.eat(Symbol, "for")? {
            let mut loc = self.prev().loc.clone();

            let counter = self.expect_any(Symbol)?.clone();
            self.expect(Symbol, "in")?;
            let start = self.parse_code_expr(0)?;
            self.expect(Operator, "..")?;
            let end = self.parse_code_expr(0)?;
            let body = self.parse_code_block_expr()?;

            loc.end_pos = self.prev().loc.end_pos.clone();

            return Ok(CodeExpr::ForLoop(ForLoopExpr {
                counter: counter.value,
                start: Box::new(start),
                end: Box::new(end),
                body: Box::new(body),
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "continue")? {
            let loc = self.prev().loc.clone();

            return Ok(CodeExpr::Continue(ContinueExpr { loc }));
        }

        if let Some(_) = self.eat(Symbol, "dbg")? {
            let mut loc = self.prev().loc.clone();

            let message = self.expect_any(StringLiteral)?.clone();

            loc.end_pos = self.prev().loc.end_pos.clone();

            return Ok(CodeExpr::Dbg(DbgExpr {
                message: message.value.clone(),
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "defer")? {
            let mut loc = self.prev().loc.clone();

            let expr = self.parse_code_expr(0)?;

            loc.end_pos = self.prev().loc.end_pos.clone();

            return Ok(CodeExpr::Defer(DeferExpr {
                expr: Box::new(expr),
                loc,
            }));
        }

        if self.current().is_any(Symbol) && self.look_ahead(1).is(Delim, "(") {
            let fn_name = self.expect_any(Symbol)?.clone();
            let mut args = Vec::new();
            let mut loc = self.prev().loc.clone();

            self.expect(Delim, "(")?;
            while let None = self.eat(Delim, ")")? {
                args.push(self.parse_code_expr(0)?);

                if !self.current().is(Delim, ")") {
                    self.expect(Delim, ",")?;
                }
            }

            loc.end_pos = self.prev().loc.end_pos.clone();

            return Ok(CodeExpr::Call(CallExpr {
                fn_name: fn_name.value,
                args,
                loc,
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

    fn parse_code_expr_postfix(
        &mut self,
        primary: CodeExpr,
        op: InfixOp,
    ) -> Result<CodeExpr, LoError> {
        let min_bp = op.info.get_min_bp_for_next();

        match op.tag {
            InfixOpTag::Equal
            | InfixOpTag::NotEqual
            | InfixOpTag::Less
            | InfixOpTag::Greater
            | InfixOpTag::LessEqual
            | InfixOpTag::GreaterEqual
            | InfixOpTag::Add
            | InfixOpTag::Sub
            | InfixOpTag::Mul
            | InfixOpTag::Div
            | InfixOpTag::Mod
            | InfixOpTag::And
            | InfixOpTag::BitAnd
            | InfixOpTag::Or
            | InfixOpTag::BitOr
            | InfixOpTag::ShiftLeft
            | InfixOpTag::ShiftRight
            | InfixOpTag::AddAssign
            | InfixOpTag::SubAssign
            | InfixOpTag::MulAssign
            | InfixOpTag::DivAssign
            | InfixOpTag::ModAssign
            | InfixOpTag::BitAndAssign
            | InfixOpTag::BitOrAssign
            | InfixOpTag::ShiftLeftAssign
            | InfixOpTag::ShiftRightAssign => {
                let lhs = primary;
                let rhs = self.parse_code_expr(min_bp)?;

                let mut loc = lhs.loc().clone();
                loc.end_pos = rhs.loc().end_pos.clone();

                Ok(CodeExpr::BinaryOp(BinaryOpExpr {
                    op_tag: op.tag,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    loc,
                }))
            }
            InfixOpTag::Assign
            | InfixOpTag::Cast
            | InfixOpTag::FieldAccess
            | InfixOpTag::Catch
            | InfixOpTag::ErrorPropagation => Err(LoError::todo(file!(), line!())),
        }
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
        self.tokens.get(self.tokens_processed)
    }

    fn next(&mut self) -> Option<&LoToken> {
        let token = self.tokens.get(self.tokens_processed);
        self.tokens_processed += 1;
        token
    }

    fn prev(&mut self) -> &LoToken {
        self.look_ahead(-1)
    }

    fn current(&mut self) -> &LoToken {
        self.look_ahead(0)
    }

    fn look_ahead(&mut self, token_count: isize) -> &LoToken {
        if let Some(token) = self
            .tokens
            .get((self.tokens_processed as isize + token_count) as usize)
        {
            &token
        } else {
            &self.terminal_token
        }
    }
}
