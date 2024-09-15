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

            if let Some(_) = self.eat(Symbol, "memory")? {
                let memory_def = self.parse_memory_def(true, loc)?;
                return Ok(TopLevelExpr::MemoryDef(memory_def));
            }

            if let Some(_) = self.eat(Symbol, "existing")? {
                let mut loc = self.prev().loc.clone();

                self.expect(Symbol, "fn")?;
                let in_fn_name = self.parse_ident()?;
                self.expect(Symbol, "as")?;
                let out_fn_name = self.expect_any(StringLiteral)?.clone();

                loc.end_pos = self.prev().loc.end_pos.clone();

                return Ok(TopLevelExpr::ExportExistingFn(ExportExistingFnExpr {
                    in_fn_name,
                    out_fn_name: out_fn_name.value,
                    loc,
                }));
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

        if let Some(_) = self.eat(Symbol, "memory")? {
            let loc = self.prev().loc.clone();

            let memory_def = self.parse_memory_def(false, loc)?;
            return Ok(TopLevelExpr::MemoryDef(memory_def));
        }

        if let Some(_) = self.eat(Symbol, "include")? {
            let mut loc = self.prev().loc.clone();

            let file_path = self.expect_any(StringLiteral)?.clone();

            loc.end_pos = self.prev().loc.end_pos.clone();

            return Ok(TopLevelExpr::Include(IncludeExpr {
                file_path: file_path.value,
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "import")? {
            let mut loc = self.prev().loc.clone();

            self.expect(Symbol, "from")?;
            let module_name = self.expect_any(StringLiteral)?.clone();

            self.expect(Delim, "{")?;

            let mut items = Vec::new();
            while let None = self.eat(Delim, "}")? {
                let item = self.parse_importable()?;
                items.push(item);
                self.expect(Delim, ";")?;
            }

            loc.end_pos = self.prev().loc.end_pos.clone();

            return Ok(TopLevelExpr::Import(ImportExpr {
                module_name: module_name.value,
                items,
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "global")? {
            let mut loc = self.prev().loc.clone();

            let global_name = self.parse_ident()?;
            self.expect(Operator, "=")?;
            let expr = self.parse_code_expr(0)?;

            loc.end_pos = self.prev().loc.end_pos.clone();

            return Ok(TopLevelExpr::GlobalDef(GlobalDefExpr {
                global_name,
                expr,
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "struct")? {
            let mut loc = self.prev().loc.clone();

            let struct_name = self.parse_ident()?;

            let mut fields = Vec::new();

            self.expect(Delim, "{")?;
            while let None = self.eat(Delim, "}")? {
                let mut field_loc = self.current().loc.clone();

                let field_name = self.expect_any(Symbol)?.clone();
                self.expect(Operator, ":")?;
                let field_type = self.parse_type_expr()?;

                field_loc.end_pos = self.prev().loc.end_pos.clone();

                fields.push(StructDefField {
                    field_name: field_name.value,
                    field_type,
                    loc: field_loc,
                });

                if !self.current().is(Delim, "}") {
                    self.expect(Delim, ",")?;
                }
            }

            loc.end_pos = self.prev().loc.end_pos.clone();

            return Ok(TopLevelExpr::StructDef(StructDefExpr {
                struct_name,
                fields,
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "type")? {
            let mut loc = self.prev().loc.clone();

            let type_name = self.parse_ident()?;
            self.expect(Operator, "=")?;
            let type_value = self.parse_type_expr()?;

            loc.end_pos = self.prev().loc.end_pos.clone();

            return Ok(TopLevelExpr::TypeDef(TypeDefExpr {
                type_name,
                type_value,
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "const")? {
            let mut loc = self.prev().loc.clone();

            let const_name = self.parse_ident()?;
            self.expect(Operator, "=")?;
            let const_value = self.parse_code_expr(0)?;

            loc.end_pos = self.prev().loc.end_pos.clone();

            return Ok(TopLevelExpr::ConstDef(ConstDefExpr {
                const_name,
                const_value,
                loc,
            }));
        }

        if let Some(_) = self.eat(Operator, "*")? {
            let mut loc = self.prev().loc.clone();

            // can't use `parse_code_expr` as that will capture `=` token
            let addr = self.parse_code_expr_primary()?;
            self.expect(Operator, "=")?;
            let chars = self.expect_any(StringLiteral)?.clone();

            loc.end_pos = self.prev().loc.end_pos.clone();

            return Ok(TopLevelExpr::StaticDataStore(StaticDataStoreExpr {
                addr,
                data: StaticDataStorePayload::String { value: chars.value },
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "macro")? {
            let mut loc = self.prev().loc.clone();

            let macro_name = self.parse_ident()?;
            self.expect(Operator, "!")?;

            let mut macro_type_params = Vec::new();
            if let Some(_) = self.eat(Operator, "<")? {
                while let None = self.eat(Operator, ">")? {
                    let type_param = self.expect_any(Symbol)?;
                    macro_type_params.push(type_param.value.clone());

                    if !self.current().is(Operator, ">") {
                        self.expect(Delim, ",")?;
                    }
                }
            }

            let macro_params = self.parse_fn_params()?;

            let return_type = if let Some(_) = self.eat(Operator, ":")? {
                Some(self.parse_type_expr()?)
            } else {
                None
            };

            let body = self.parse_code_block_expr()?;

            loc.end_pos = self.prev().loc.end_pos.clone();

            return Ok(TopLevelExpr::MacroDef(MacroDefExpr {
                macro_name,
                macro_params,
                macro_type_params,
                return_type,
                body,
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

        loc.end_pos = self.prev().loc.end_pos.clone();

        Ok(FnDefExpr {
            exported,
            decl,
            body,
            loc,
        })
    }

    fn parse_memory_def(
        &mut self,
        exported: bool,
        mut loc: LoLocation,
    ) -> Result<MemoryDefExpr, LoError> {
        self.expect(Delim, "{")?;

        let mut min_pages = None;
        if let Some(_) = self.eat(Symbol, "min_pages")? {
            self.expect(Operator, ":")?;
            let int = self.expect_any(IntLiteral)?;
            let int_value = Lexer::parse_int_literal_value(&int.value) as u32;
            self.expect(Delim, ",")?;

            min_pages = Some(int_value);
        }

        let mut data_start = None;
        if let Some(_) = self.eat(Symbol, "data_start")? {
            self.expect(Operator, ":")?;
            let int = self.expect_any(IntLiteral)?;
            let int_value = Lexer::parse_int_literal_value(&int.value) as u32;
            self.eat(Delim, ",")?;

            data_start = Some(int_value);
        }
        self.expect(Delim, "}")?;

        loc.end_pos = self.prev().loc.end_pos.clone();

        Ok(MemoryDefExpr {
            exported,
            min_pages,
            data_start,
            loc,
        })
    }

    fn parse_importable(&mut self) -> Result<ImportItem, LoError> {
        if let Some(_) = self.eat(Symbol, "fn")? {
            let decl = self.parse_fn_decl()?;
            return Ok(ImportItem::FnDecl(decl));
        }

        if let Some(_) = self.eat(Symbol, "memory")? {
            let loc = self.prev().loc.clone();
            let memory_def = self.parse_memory_def(false, loc)?;
            return Ok(ImportItem::Memory(memory_def));
        }

        let unexpected = self.current();
        return Err(LoError {
            message: format!(
                "Unexpected token in importable item: {:?}",
                unexpected.value
            ),
            loc: unexpected.loc.clone(),
        });
    }

    fn parse_fn_decl(&mut self) -> Result<FnDeclExpr, LoError> {
        let mut loc = self.prev().loc.clone();

        let fn_name = self.parse_ident()?;
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
            let mut loc = self.current().loc.clone();

            let mut p_type = FnParamType::Self_;
            if let Some(_) = self.eat(Operator, "&")? {
                p_type = FnParamType::SelfRef;
            }

            let p_name = self.expect_any(Symbol)?.clone();

            if p_name.value != "self" {
                if let FnParamType::SelfRef = p_type {
                    return Err(LoError {
                        message: format!(
                            "Only `self` param can be preceded by the reference operator"
                        ),
                        loc: p_name.loc,
                    });
                }

                self.expect(Operator, ":")?;
                p_type = FnParamType::Type {
                    expr: self.parse_type_expr()?,
                };
            }

            loc.end_pos = self.prev().loc.end_pos.clone();

            if !self.current().is(Delim, ")") {
                self.expect(Delim, ",")?;
            }

            params.push(FnParam {
                param_name: p_name.value,
                param_type: p_type,
                loc,
            });
        }

        Ok(params)
    }

    fn parse_type_expr(&mut self) -> Result<TypeExpr, LoError> {
        let primary = self.parse_type_expr_primary()?;

        if let Some(_) = self.eat(Symbol, "of")? {
            let item_type = self.parse_type_expr()?;

            return Ok(TypeExpr::Of {
                container_type: Box::new(primary),
                item_type: Box::new(item_type),
            });
        }

        return Ok(primary);
    }

    fn parse_type_expr_primary(&mut self) -> Result<TypeExpr, LoError> {
        if let Some(_) = self.eat(Operator, "&")? {
            return Ok(TypeExpr::Pointer {
                pointee: Box::new(self.parse_type_expr()?),
            });
        }

        if let Some(_) = self.eat(Operator, "*&")? {
            return Ok(TypeExpr::SequencePointer {
                pointee: Box::new(self.parse_type_expr()?),
            });
        }

        if let Some(_) = self.eat(Symbol, "Result")? {
            self.expect(Operator, "<")?;
            let ok_type = Box::new(self.parse_type_expr()?);
            self.expect(Delim, ",")?;
            let err_type = Box::new(self.parse_type_expr()?);
            self.expect(Operator, ">")?;

            return Ok(TypeExpr::Result { ok_type, err_type });
        }

        let ident = self.parse_ident()?;
        return Ok(TypeExpr::Named { name: ident });
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

            let mut expr = None;
            if !self.current().is(Delim, ";") {
                expr = Some(Box::new(self.parse_code_expr(0)?));
            }

            loc.end_pos = self.prev().loc.end_pos.clone();

            return Ok(CodeExpr::Return(ReturnExpr { expr, loc }));
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

        if let Some(_) = self.eat(Symbol, "true")? {
            let loc = self.prev().loc.clone();

            return Ok(CodeExpr::BoolLiteral(BoolLiteralExpr { value: true, loc }));
        }

        if let Some(_) = self.eat(Symbol, "false")? {
            let loc = self.prev().loc.clone();

            return Ok(CodeExpr::BoolLiteral(BoolLiteralExpr { value: false, loc }));
        }

        if let Some(char) = self.eat_any(CharLiteral)?.cloned() {
            return Ok(CodeExpr::CharLiteral(CharLiteralExpr {
                repr: char.value.clone(),
                value: Lexer::parse_char_literal_value(&char.value) as u32,
                loc: char.loc.clone(),
            }));
        };

        if let Some(int) = self.eat_any(IntLiteral)?.cloned() {
            let mut tag = None;
            if let Some(_) = self.eat(Symbol, "u64")? {
                tag = Some(String::from("u64"));
            }

            return Ok(CodeExpr::IntLiteral(IntLiteralExpr {
                repr: int.value.clone(),
                value: Lexer::parse_int_literal_value(&int.value) as u32,
                tag,
                loc: int.loc.clone(),
            }));
        };

        if let Some(string) = self.eat_any(StringLiteral)?.cloned() {
            let mut zero_terminated = false;
            if let Some(_) = self.eat(IntLiteral, "0")? {
                zero_terminated = true;
            }

            return Ok(CodeExpr::StringLiteral(StringLiteralExpr {
                value: Lexer::unescape_string(&string.value),
                repr: string.value,
                zero_terminated,
                loc: string.loc,
            }));
        };

        if let Some(_) = self.eat(Delim, "(")? {
            let mut loc = self.prev().loc.clone();

            let expr = Box::new(self.parse_code_expr(0)?);
            self.expect(Delim, ")")?;

            loc.end_pos = self.prev().loc.end_pos.clone();

            return Ok(CodeExpr::Paren(ParenExpr { expr, loc }));
        };

        if let Some(_) = self.eat(Symbol, "let")? {
            let mut loc = self.prev().loc.clone();

            let local_name = self.expect_any(Symbol)?.clone();
            self.expect(Operator, "=")?;
            let value = self.parse_code_expr(0)?;

            loc.end_pos = self.prev().loc.end_pos.clone();

            return Ok(CodeExpr::Let(LetExpr {
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
                message: message.value,
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

        if let Some(token) = self.peek().cloned() {
            if let Some(op) = PrefixOp::parse(token) {
                self.next(); // skip operator

                let mut loc = self.prev().loc.clone();

                let min_bp = op.info.get_min_bp_for_next();

                match op.tag {
                    PrefixOpTag::Dereference
                    | PrefixOpTag::Not
                    | PrefixOpTag::Positive
                    | PrefixOpTag::Negative => {
                        let expr = Box::new(self.parse_code_expr(min_bp)?);

                        loc.end_pos = self.prev().loc.end_pos.clone();

                        return Ok(CodeExpr::PrefixOp(PrefixOpExpr {
                            expr,
                            op_tag: op.tag,
                            loc,
                        }));
                    }
                }
            }
        }

        if let Some(_) = self.eat(Symbol, "sizeof")? {
            let mut loc = self.prev().loc.clone();

            let type_expr = self.parse_type_expr()?;

            loc.end_pos = self.prev().loc.end_pos.clone();

            return Ok(CodeExpr::Sizeof(SizeofExpr { type_expr, loc }));
        };

        if let Some(_) = self.eat(Delim, "[")? {
            let mut loc = self.prev().loc.clone();

            let item_type = self.parse_type_expr()?;
            self.expect(Delim, "]")?;

            self.expect(Delim, "[")?;
            let mut items = Vec::new();
            while let None = self.eat(Delim, "]")? {
                let item = self.parse_code_expr(0)?;
                items.push(item);

                if !self.current().is(Delim, "]") {
                    self.expect(Delim, ",")?;
                }
            }

            loc.end_pos = self.prev().loc.end_pos.clone();

            return Ok(CodeExpr::ArrayLiteral(ArrayLiteralExpr {
                item_type,
                items,
                loc,
            }));
        }

        if let Some(_) = self.eat(Operator, "@")? {
            let mut loc = self.prev().loc.clone();

            self.expect(Symbol, "data_size")?;

            loc.end_pos = self.prev().loc.end_pos.clone();

            return Ok(CodeExpr::GetDataSize(GetDataSizeExpr { loc }));
        }

        let ident = self.parse_ident()?;

        if self.current().is(Delim, "(") {
            let mut loc = ident.loc.clone();

            let args = self.parse_fn_args()?;

            loc.end_pos = self.prev().loc.end_pos.clone();

            return Ok(CodeExpr::FnCall(FnCallExpr {
                fn_name: ident,
                args,
                loc,
            }));
        }

        if let Some(_) = self.eat(Operator, "!")? {
            let mut loc = ident.loc.clone();

            let type_args = self.parse_macro_type_args()?;
            let args = self.parse_fn_args()?;

            loc.end_pos = self.prev().loc.end_pos.clone();

            return Ok(CodeExpr::MacroFnCall(MacroFnCallExpr {
                fn_name: ident,
                args,
                type_args,
                loc,
            }));
        }

        // TODO: remove after struct syntax is changed
        if ident.repr.chars().next().unwrap().is_ascii_uppercase()
            || ident.repr == "str"
            || ident.repr == "wasi::IOVec"
        {
            if let Some(_) = self.eat(Delim, "{")? {
                let mut loc = ident.loc.clone();

                let mut fields = Vec::new();

                while let None = self.eat(Delim, "}")? {
                    let mut field_loc = self.current().loc.clone();

                    let field_name = self.expect_any(Symbol)?.clone();
                    self.expect(Operator, ":")?;
                    let value = self.parse_code_expr(0)?;

                    field_loc.end_pos = self.prev().loc.end_pos.clone();

                    fields.push(StructInitField {
                        field_name: field_name.value,
                        value,
                        loc: field_loc,
                    });

                    if !self.current().is(Delim, "}") {
                        self.expect(Delim, ",")?;
                    }
                }

                loc.end_pos = self.prev().loc.end_pos.clone();

                return Ok(CodeExpr::StructLiteral(StructLiteralExpr {
                    struct_name: ident,
                    fields,
                    loc,
                }));
            }
        }

        Ok(CodeExpr::Ident(ident))
    }

    fn parse_ident(&mut self) -> Result<IdentExpr, LoError> {
        let mut ident = IdentExpr {
            repr: String::new(),
            parts: Vec::new(),
            loc: self.current().loc.clone(),
        };

        loop {
            let ident_part = self.expect_any(Symbol)?;
            ident.parts.push(ident_part.value.clone());
            ident.repr += ident_part.value.as_str();

            if let Some(_) = self.eat(Operator, "::")? {
                ident.repr += "::";
                continue;
            }

            break;
        }

        ident.loc.end_pos = self.prev().loc.end_pos.clone();

        Ok(ident)
    }

    fn parse_fn_args(&mut self) -> Result<Vec<CodeExpr>, LoError> {
        let mut args = Vec::new();
        self.expect(Delim, "(")?;
        while let None = self.eat(Delim, ")")? {
            args.push(self.parse_code_expr(0)?);

            if !self.current().is(Delim, ")") {
                self.expect(Delim, ",")?;
            }
        }

        return Ok(args);
    }

    fn parse_macro_type_args(&mut self) -> Result<Vec<TypeExpr>, LoError> {
        let mut type_args = Vec::new();

        let Some(_) = self.eat(Operator, "<")? else {
            return Ok(type_args);
        };

        while let None = self.eat(Operator, ">")? {
            type_args.push(self.parse_type_expr()?);

            if !self.current().is(Operator, ">") {
                self.expect(Delim, ",")?;
            }
        }

        return Ok(type_args);
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

                Ok(CodeExpr::InfixOp(InfixOpExpr {
                    op_tag: op.tag,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    loc,
                }))
            }
            InfixOpTag::Cast => {
                let mut loc = primary.loc().clone();

                let casted_to = self.parse_type_expr()?;

                loc.end_pos = self.prev().loc.end_pos.clone();

                Ok(CodeExpr::Cast(CastExpr {
                    expr: Box::new(primary),
                    casted_to,
                    loc,
                }))
            }
            InfixOpTag::FieldAccess => {
                let mut loc = primary.loc().clone();

                let field_name = self.parse_ident()?;

                if self.current().is(Delim, "(") {
                    let args = self.parse_fn_args()?;

                    loc.end_pos = self.prev().loc.end_pos.clone();

                    return Ok(CodeExpr::MethodCall(MethodCallExpr {
                        lhs: Box::new(primary),
                        field_name,
                        args,
                        loc,
                    }));
                }

                if let Some(_) = self.eat(Operator, "!")? {
                    let type_args = self.parse_macro_type_args()?;
                    let args = self.parse_fn_args()?;

                    loc.end_pos = self.prev().loc.end_pos.clone();

                    return Ok(CodeExpr::MacroMethodCall(MacroMethodCallExpr {
                        lhs: Box::new(primary),
                        field_name,
                        args,
                        type_args,
                        loc,
                    }));
                }

                loc.end_pos = self.prev().loc.end_pos.clone();

                Ok(CodeExpr::FieldAccess(FieldAccessExpr {
                    lhs: Box::new(primary),
                    field_name,
                    loc,
                }))
            }
            InfixOpTag::Assign => {
                let mut loc = primary.loc().clone();

                // TODO: validate that this is a proper lhs for assignment
                let value = self.parse_code_expr(min_bp)?;

                loc.end_pos = self.prev().loc.end_pos.clone();

                Ok(CodeExpr::Assign(AssignExpr {
                    lhs: Box::new(primary),
                    rhs: Box::new(value),
                    loc,
                }))
            }
            InfixOpTag::Catch => {
                let mut loc = primary.loc().clone();

                let error_bind = self.expect_any(Symbol)?.clone();
                let catch_body = self.parse_code_block_expr()?;

                loc.end_pos = self.prev().loc.end_pos.clone();

                Ok(CodeExpr::Catch(CatchExpr {
                    lhs: Box::new(primary),
                    error_bind: error_bind.value,
                    catch_body,
                    loc,
                }))
            }
            InfixOpTag::ErrorPropagation => {
                let mut loc = primary.loc().clone();
                loc.end_pos = self.prev().loc.end_pos.clone();

                Ok(CodeExpr::PropagateError(PropagateErrorExpr {
                    expr: Box::new(primary),
                    loc,
                }))
            }
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
