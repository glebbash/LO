use crate::{ast::*, core::*, lexer::*};
use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::cell::RefCell;
use LoTokenType::*;

pub struct ParsingContext {
    pub struct_literal_allowed: bool,
}

pub struct Parser {
    // context
    pub lexer: Lexer,

    // state
    pub tokens_processed: RefCell<usize>,
    pub contexts: Vec<ParsingContext>,

    // output
    pub ast: Vec<TopLevelExpr>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut contexts = Vec::new();
        contexts.push(ParsingContext {
            struct_literal_allowed: true,
        });

        Self {
            lexer,
            tokens_processed: RefCell::new(0),
            contexts,
            ast: Vec::new(),
        }
    }

    pub fn parse_file(&self) -> Result<(), LoError> {
        while self.peek().is_some() {
            let expr = self.parse_top_level_expr()?;
            self.ast.be_mut().push(expr);
        }

        if let Some(unexpected) = self.peek() {
            return Err(LoError {
                message: format!(
                    "Unexpected top level token: {}, EOF expected",
                    unexpected.get_value(self.lexer.source)
                ),
                loc: unexpected.loc.clone(),
            });
        }

        Ok(())
    }

    fn parse_top_level_expr(&self) -> Result<TopLevelExpr, LoError> {
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

            let unexpected = self.current();
            return Err(LoError {
                message: format!(
                    "Unexpected exportable: {:?}",
                    unexpected.get_value(self.lexer.source)
                ),
                loc: unexpected.loc.clone(),
            });
        }

        if let Some(_) = self.eat(Symbol, "try")? {
            let mut loc = self.prev().loc.clone();

            self.expect(Symbol, "export")?;
            let in_name = self.parse_ident()?;

            self.expect(Symbol, "as")?;
            let out_name = self.expect_any(StringLiteral)?.clone();

            let mut from_root = false;
            if let Ok(Some(_)) = self.eat(Symbol, "from") {
                self.expect(Symbol, "root")?;
                from_root = true;
            }

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(TopLevelExpr::TryExport(TryExportExpr {
                in_name,
                out_name: EscapedString(out_name.loc),
                from_root,
                loc,
            }));
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
            let mut alias = None;
            if let Ok(Some(_)) = self.eat(Symbol, "as") {
                alias = Some(self.parse_ident()?);
            }

            let mut with_extern = false;
            if let Ok(Some(_)) = self.eat(Symbol, "with") {
                self.expect(Symbol, "extern")?;
                with_extern = true;
            }

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(TopLevelExpr::Include(IncludeExpr {
                file_path: EscapedString(file_path.loc),
                alias,
                with_extern,
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
            }

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(TopLevelExpr::Import(ImportExpr {
                module_name: EscapedString(module_name.loc),
                items,
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "global")? {
            let mut loc = self.prev().loc.clone();

            let global_name = self.parse_ident()?;
            self.expect(Operator, "=")?;

            let global_value = if let Some(_) = self.eat(Operator, "@")? {
                self.expect(Symbol, "data_size")?;
                self.expect(Delim, "(")?;
                self.expect(Delim, ")")?;

                GlobalDefValue::DataSize
            } else {
                let expr = self.parse_code_expr(0)?;
                GlobalDefValue::Expr(expr)
            };

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(TopLevelExpr::GlobalDef(GlobalDefExpr {
                global_name,
                global_value,
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

                let field_name = self.parse_ident()?;
                self.expect(Operator, ":")?;
                let field_type = self.parse_type_expr()?;

                field_loc.end_pos = self.prev().loc.end_pos;

                fields.push(StructDefField {
                    field_name,
                    field_type,
                    loc: field_loc,
                });

                if !self.current().is(Delim, "}", self.lexer.source) {
                    self.expect(Delim, ",")?;
                }
            }

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(TopLevelExpr::StructDef(StructDefExpr {
                struct_name,
                fields,
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "enum")? {
            let mut loc = self.prev().loc.clone();

            let enum_name = self.parse_ident()?;

            let mut variants = Vec::new();

            self.expect(Delim, "{")?;
            while let None = self.eat(Delim, "}")? {
                let mut variant_loc = self.current().loc.clone();

                let variant_name = self.parse_ident()?;

                let mut variant_type = None;
                if let Some(_) = self.eat(Delim, "(")? {
                    variant_type = Some(self.parse_type_expr()?);
                    self.expect(Delim, ")")?;
                }

                variant_loc.end_pos = self.prev().loc.end_pos;

                variants.push(EnumDefVariant {
                    variant_name,
                    variant_type,
                    loc: variant_loc,
                });

                if !self.current().is(Delim, "}", self.lexer.source) {
                    self.expect(Delim, ",")?;
                }
            }

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(TopLevelExpr::EnumDef(EnumDefExpr {
                enum_name,
                variants,
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "type")? {
            let mut loc = self.prev().loc.clone();

            let type_name = self.parse_ident()?;
            self.expect(Operator, "=")?;
            let type_value = self.parse_type_expr()?;

            loc.end_pos = self.prev().loc.end_pos;

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

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(TopLevelExpr::ConstDef(ConstDefExpr {
                const_name,
                const_value,
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
                    macro_type_params.push(type_param.get_value(self.lexer.source).to_string());

                    if !self.current().is(Operator, ">", self.lexer.source) {
                        self.expect(Delim, ",")?;
                    }
                }
            }

            let mut macro_params_trailing_comma = false;
            let macro_params = self.parse_fn_params(true, &mut macro_params_trailing_comma)?;

            let return_type = if let Some(_) = self.eat(Operator, ":")? {
                Some(self.parse_type_expr()?)
            } else {
                None
            };

            let body = self.parse_code_block()?;

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(TopLevelExpr::MacroDef(MacroDefExpr {
                macro_name,
                macro_params,
                macro_params_trailing_comma,
                macro_type_params,
                return_type,
                body,
                loc,
            }));
        }

        let unexpected = self.current();
        return Err(LoError {
            message: format!(
                "Unexpected top level token: {}",
                unexpected.get_value(self.lexer.source)
            ),
            loc: unexpected.loc.clone(),
        });
    }

    fn parse_fn_def(&self, exported: bool, mut loc: LoLocation) -> Result<FnDefExpr, LoError> {
        let decl = self.parse_fn_decl()?;
        let body = self.parse_code_block()?;

        loc.end_pos = self.prev().loc.end_pos;

        Ok(FnDefExpr {
            exported,
            decl,
            body,
            loc,
        })
    }

    fn parse_memory_def(
        &self,
        exported: bool,
        mut loc: LoLocation,
    ) -> Result<MemoryDefExpr, LoError> {
        self.expect(Delim, "{")?;

        let mut min_pages = None;
        if let Some(_) = self.eat(Symbol, "min_pages")? {
            self.expect(Operator, ":")?;
            let int = self.expect_any(IntLiteral)?;
            let int_value =
                Lexer::parse_int_literal_value(&int.get_value(self.lexer.source)) as u32;
            self.expect(Delim, ",")?;

            min_pages = Some(int_value);
        }

        let mut data_start = None;
        if let Some(_) = self.eat(Symbol, "data_start")? {
            self.expect(Operator, ":")?;
            let int = self.expect_any(IntLiteral)?;
            let int_value =
                Lexer::parse_int_literal_value(&int.get_value(self.lexer.source)) as u32;
            self.eat(Delim, ",")?;

            data_start = Some(int_value);
        }
        self.expect(Delim, "}")?;

        loc.end_pos = self.prev().loc.end_pos;

        Ok(MemoryDefExpr {
            exported,
            min_pages,
            data_start,
            loc,
        })
    }

    fn parse_importable(&self) -> Result<ImportItem, LoError> {
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
                unexpected.get_value(self.lexer.source)
            ),
            loc: unexpected.loc.clone(),
        });
    }

    fn parse_fn_decl(&self) -> Result<FnDeclExpr, LoError> {
        let mut loc = self.prev().loc.clone();

        let fn_name = self.parse_ident()?;
        let mut fn_params_trailing_comma = false;
        let fn_params = self.parse_fn_params(false, &mut fn_params_trailing_comma)?;

        let return_type = if let Some(_) = self.eat(Operator, ":")? {
            Some(self.parse_type_expr()?)
        } else {
            None
        };

        loc.end_pos = self.prev().loc.end_pos;

        Ok(FnDeclExpr {
            fn_name,
            fn_params,
            fn_params_trailing_comma,
            return_type,
            loc,
        })
    }

    fn parse_fn_params(
        &self,
        infer_allowed: bool,
        trailing_comma: &mut bool,
    ) -> Result<Vec<FnParam>, LoError> {
        let mut params = Vec::<FnParam>::new();

        let _ = self.expect(Delim, "(")?;

        while let None = self.eat(Delim, ")")? {
            *trailing_comma = false;

            let mut loc = self.current().loc.clone();

            let mut param_type = FnParamType::Self_;
            if let Some(_) = self.eat(Operator, "&")? {
                param_type = FnParamType::SelfRef;
            }

            let param_name = self.parse_ident()?;

            if param_name.repr != "self" {
                if let FnParamType::SelfRef = param_type {
                    return Err(LoError {
                        message: format!(
                            "Only `self` param can be preceded by the reference operator"
                        ),
                        loc: param_name.loc,
                    });
                }

                self.expect(Operator, ":")?;

                if let Some(infer) = self.eat(Symbol, "infer")? {
                    if !infer_allowed {
                        return Err(LoError {
                            message: format!("Cannot use `infer` outside macro parameter list"),
                            loc: infer.loc.clone(),
                        });
                    }

                    let infer_as = self.expect_any(Symbol)?;
                    param_type = FnParamType::Infer {
                        name: infer_as.get_value(self.lexer.source).to_string(),
                    };
                } else {
                    param_type = FnParamType::Type {
                        expr: self.parse_type_expr()?,
                    };
                }
            }

            loc.end_pos = self.prev().loc.end_pos;

            if !self.current().is(Delim, ")", self.lexer.source) {
                self.expect(Delim, ",")?;
                *trailing_comma = true;
            }

            params.push(FnParam {
                param_name,
                param_type,
                loc,
            });
        }

        Ok(params)
    }

    fn parse_type_expr(&self) -> Result<TypeExpr, LoError> {
        let mut loc = self.current().loc.clone();
        let primary = self.parse_type_expr_primary()?;

        if let Ok(Some(_)) = self.eat(Symbol, "of") {
            let container_type = Box::new(primary);
            let item_type = Box::new(self.parse_type_expr()?);
            loc.end_pos = self.prev().loc.end_pos;
            return Ok(TypeExpr::Of(TypeExprOf {
                container_type,
                item_type,
                loc,
            }));
        }

        return Ok(primary);
    }

    fn parse_type_expr_primary(&self) -> Result<TypeExpr, LoError> {
        let mut loc = self.current().loc.clone();

        if let Some(_) = self.eat(Operator, "&")? {
            let pointee = Box::new(self.parse_type_expr()?);
            loc.end_pos = self.prev().loc.end_pos;
            return Ok(TypeExpr::Pointer(TypeExprPointer { pointee, loc }));
        }

        // lexer joins two `&` into `&&`
        if let Some(_) = self.eat(Operator, "&&")? {
            let pointee = Box::new(self.parse_type_expr()?);
            loc.end_pos = self.prev().loc.end_pos;
            return Ok(TypeExpr::Pointer(TypeExprPointer {
                pointee: Box::new(TypeExpr::Pointer(TypeExprPointer {
                    pointee,
                    loc: loc.clone(),
                })),
                loc,
            }));
        }

        if let Some(_) = self.eat(Operator, "*&")? {
            let pointee = Box::new(self.parse_type_expr()?);
            loc.end_pos = self.prev().loc.end_pos;
            return Ok(TypeExpr::SequencePointer(TypeExprSequencePointer {
                pointee,
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "Result")? {
            self.expect(Operator, "<")?;
            let ok_type = Box::new(self.parse_type_expr()?);
            self.expect(Delim, ",")?;
            let err_type = Box::new(self.parse_type_expr()?);
            self.expect(Operator, ">")?;
            loc.end_pos = self.prev().loc.end_pos;

            return Ok(TypeExpr::Result(TypeExprResult {
                ok_type,
                err_type,
                loc,
            }));
        }

        let ident = self.parse_ident()?;
        return Ok(TypeExpr::Named(TypeExprNamed { name: ident }));
    }

    fn parse_code_block(&self) -> Result<CodeBlock, LoError> {
        self.expect(Delim, "{")?;

        let mut code_block = CodeBlock {
            exprs: Vec::new(),
            loc: self.prev().loc.clone(),
        };

        while let None = self.eat(Delim, "}")? {
            let expr = self.parse_code_expr(0)?;
            code_block.exprs.push(expr);
        }

        // close curly pos
        code_block.loc.end_pos = self.prev().loc.end_pos;

        return Ok(code_block);
    }

    fn parse_code_expr(&self, min_bp: u32) -> Result<CodeExpr, LoError> {
        let mut primary = self.parse_code_expr_primary()?;

        while self.peek().is_some() {
            let op_symbol = self.current().clone();
            if op_symbol.loc.pos.line != primary.loc().end_pos.line
                && !self
                    .has_backslashes_between(primary.loc().end_pos.offset, op_symbol.loc.pos.offset)
            {
                break;
            }
            let Some(op) = InfixOp::parse(op_symbol, self.lexer.source) else {
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

    // TODO: this could be optimized by storing backslashes checked index but it's not that simple
    fn has_backslashes_between(&self, offset1: usize, offset2: usize) -> bool {
        for backslash in &self.lexer.backslashes {
            if backslash.pos.offset >= offset1 && backslash.end_pos.offset <= offset2 {
                return true;
            }
        }

        false
    }

    fn parse_code_expr_primary(&self) -> Result<CodeExpr, LoError> {
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
                repr: char.get_value(self.lexer.source).to_string(),
                value: Lexer::parse_char_literal_value(&char.get_value(self.lexer.source)) as u32,
                loc: char.loc.clone(),
            }));
        };

        if let Some(int) = self.eat_any(IntLiteral)?.cloned() {
            let mut tag = None;
            if let Some(_) = self.eat(Symbol, "u64")? {
                tag = Some(String::from("u64"));
            }

            return Ok(CodeExpr::IntLiteral(IntLiteralExpr {
                repr: int.get_value(self.lexer.source).to_string(),
                value: Lexer::parse_int_literal_value(&int.get_value(self.lexer.source)) as u32,
                tag,
                loc: int.loc.clone(),
            }));
        };

        if let Some(string) = self.eat_any(StringLiteral)?.cloned() {
            return Ok(CodeExpr::StringLiteral(StringLiteralExpr {
                value: EscapedString(string.loc.clone()).unescape(self.lexer.source),
                repr: string.get_value(self.lexer.source).to_string(),
                loc: string.loc,
            }));
        };

        if let Some(_) = self.eat(Delim, "(")? {
            let mut loc = self.prev().loc.clone();

            self.push_ctx(ParsingContext {
                struct_literal_allowed: true,
            });

            let expr = Box::new(self.parse_code_expr(0)?);

            self.pop_ctx();

            let has_trailing_comma = self.eat(Delim, ",")?.is_some();
            self.expect(Delim, ")")?;

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(CodeExpr::Paren(ParenExpr {
                expr,
                has_trailing_comma,
                loc,
            }));
        };

        if let Some(_) = self.eat(Symbol, "let")? {
            let mut loc = self.prev().loc.clone();

            let local_name = self.parse_ident()?;
            self.expect(Operator, "=")?;
            let value = self.parse_code_expr(0)?;

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(CodeExpr::Let(LetExpr {
                local_name,
                value: Box::new(value),
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "return")? {
            let mut loc = self.prev().loc.clone();

            let mut expr = None;
            if self.current().loc.end_pos.line == loc.end_pos.line {
                expr = Some(Box::new(self.parse_code_expr(0)?));
            }

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(CodeExpr::Return(ReturnExpr { expr, loc }));
        };

        if let Some(_) = self.eat(Symbol, "if")? {
            let mut loc = self.prev().loc.clone();

            self.push_ctx(ParsingContext {
                struct_literal_allowed: false,
            });

            let cond: IfCond;
            if let Some(_) = self.eat(Symbol, "match")? {
                cond = IfCond::Match(Box::new(self.parse_match_header()?));
            } else {
                cond = IfCond::Expr(Box::new(self.parse_code_expr(0)?));
            };

            self.pop_ctx();

            let then_block = Box::new(self.parse_code_block()?);

            let mut else_block = ElseBlock::None;
            if let Some(_) = self.eat(Symbol, "else")? {
                if self.current().is(Symbol, "if", self.lexer.source) {
                    let if_expr = self.parse_code_expr(0)?;
                    else_block = ElseBlock::ElseIf(Box::new(if_expr));
                } else {
                    let block = self.parse_code_block()?;
                    else_block = ElseBlock::Else(Box::new(block));
                }
            }

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(CodeExpr::If(IfExpr {
                cond,
                then_block,
                else_block,
                loc,
            }));
        };

        if let Some(_) = self.eat(Symbol, "match")? {
            let mut loc = self.prev().loc.clone();

            let match_header = self.parse_match_header()?;
            self.expect(Symbol, "else")?;
            let else_branch = self.parse_code_block()?;

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(CodeExpr::Match(MatchExpr {
                header: Box::new(match_header),
                else_branch,
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "loop")? {
            let mut loc = self.prev().loc.clone();

            let body = self.parse_code_block()?;

            loc.end_pos = self.prev().loc.end_pos;

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

            self.push_ctx(ParsingContext {
                struct_literal_allowed: false,
            });

            let counter = self.parse_ident()?;
            self.expect(Symbol, "in")?;
            let start = self.parse_code_expr(0)?;
            let op = self.expect(Operator, "..")?.clone();
            let end = self.parse_code_expr(0)?;

            self.pop_ctx();

            let body = self.parse_code_block()?;

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(CodeExpr::ForLoop(ForLoopExpr {
                counter,
                start: Box::new(start),
                end: Box::new(end),
                body: Box::new(body),
                op_loc: op.loc,
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "continue")? {
            let loc = self.prev().loc.clone();

            return Ok(CodeExpr::Continue(ContinueExpr { loc }));
        }

        if let Some(_) = self.eat(Symbol, "do")? {
            let mut loc = self.prev().loc.clone();

            let body = Box::new(self.parse_code_expr(0)?);
            let with = self.expect(Symbol, "with")?.clone();
            let args = self.parse_fn_args()?;

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(CodeExpr::DoWith(DoWithExpr {
                body,
                args,
                with_loc: with.loc,
                loc,
            }));
        }

        if let Some(_) = self.eat(Operator, "@")? {
            let mut loc = self.prev().loc.clone();

            let fn_name = self.parse_ident()?;
            let type_args = self.parse_macro_type_args()?;
            let args = self.parse_fn_args()?;

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(CodeExpr::IntrinsicCall(MacroFnCallExpr {
                fn_name,
                args,
                type_args,
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "dbg")? {
            let mut loc = self.prev().loc.clone();

            let message = self.expect_any(StringLiteral)?.clone();

            loc.end_pos = self.prev().loc.end_pos;

            let message = EscapedString(message.loc);
            return Ok(CodeExpr::Dbg(DbgExpr { message, loc }));
        }

        if let Some(_) = self.eat(Symbol, "defer")? {
            let mut loc = self.prev().loc.clone();

            let expr = self.parse_code_expr(0)?;

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(CodeExpr::Defer(DeferExpr {
                expr: Box::new(expr),
                loc,
            }));
        }

        if let Some(token) = self.peek().cloned() {
            if let Some(op) = PrefixOp::parse(token, self.lexer.source) {
                self.next(); // skip operator

                let mut loc = self.prev().loc.clone();

                let min_bp = op.info.get_min_bp_for_next();

                match op.tag {
                    PrefixOpTag::Dereference
                    | PrefixOpTag::Not
                    | PrefixOpTag::Positive
                    | PrefixOpTag::Negative => {
                        let expr = Box::new(self.parse_code_expr(min_bp)?);

                        loc.end_pos = self.prev().loc.end_pos;

                        return Ok(CodeExpr::PrefixOp(PrefixOpExpr {
                            expr,
                            op_tag: op.tag,
                            op_loc: op.token.loc,
                            loc,
                        }));
                    }
                }
            }
        }

        if let Some(_) = self.eat(Symbol, "sizeof")? {
            let mut loc = self.prev().loc.clone();

            let type_expr = self.parse_type_expr()?;

            loc.end_pos = self.prev().loc.end_pos;

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

                if !self.current().is(Delim, "]", self.lexer.source) {
                    self.expect(Delim, ",")?;
                }
            }

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(CodeExpr::ArrayLiteral(ArrayLiteralExpr {
                item_type,
                items,
                loc,
            }));
        }

        if self.eat(Operator, ":")?.is_some() || self.eat(Delim, "{")?.is_some() {
            return Err(LoError {
                message: format!(
                    "Unexpected character '{}'. \
                    If you were trying to create a struct in this context \
                    surround it with parens.",
                    self.prev().get_value(self.lexer.source)
                ),
                loc: self.prev().loc.clone(),
            });
        }

        let ident = self.parse_ident()?;

        if ident.repr == "Ok" || ident.repr == "Err" {
            let mut loc = ident.loc.clone();

            let mut result_type = None;
            if let Some(_) = self.eat(Operator, "::")? {
                self.expect(Operator, "<")?;
                let ok = self.parse_type_expr()?;
                self.expect(Delim, ",")?;
                let err = self.parse_type_expr()?;
                self.expect(Operator, ">")?;

                result_type = Some(ResultTypeExpr { ok, err });
            }

            self.expect(Delim, "(")?;
            let mut value = None;
            if let None = self.eat(Delim, ")")? {
                value = Some(Box::new(self.parse_code_expr(0)?));
                self.expect(Delim, ")")?;
            }

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(CodeExpr::ResultLiteral(ResultLiteralExpr {
                is_ok: ident.repr == "Ok",
                result_type,
                value,
                loc,
            }));
        }

        let ctx = self.contexts.last().unwrap();
        if self.current().is(Delim, "{", self.lexer.source) && ctx.struct_literal_allowed {
            let loc = ident.loc.clone();
            let struct_literal = self.parse_struct_literal(ident, loc)?;
            return Ok(CodeExpr::StructLiteral(struct_literal));
        }

        if self.current().is(Delim, "(", self.lexer.source) {
            let mut loc = ident.loc.clone();

            self.push_ctx(ParsingContext {
                struct_literal_allowed: true,
            });

            let args = self.parse_fn_args()?;

            self.pop_ctx();

            loc.end_pos = self.prev().loc.end_pos;

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

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(CodeExpr::MacroFnCall(MacroFnCallExpr {
                fn_name: ident,
                args,
                type_args,
                loc,
            }));
        }

        Ok(CodeExpr::Ident(ident))
    }

    fn parse_match_header(&self) -> Result<MatchHeader, LoError> {
        let variant_name = self.parse_ident()?;
        self.expect(Delim, "(")?;
        let variant_bind = self.parse_ident()?;
        self.expect(Delim, ")")?;
        self.expect(Operator, "=")?;
        let expr_to_match = self.parse_code_expr(0)?;

        Ok(MatchHeader {
            variant_name,
            variant_bind,
            expr_to_match,
        })
    }

    fn parse_ident(&self) -> Result<IdentExpr, LoError> {
        let mut ident = IdentExpr {
            repr: String::new(),
            parts: Vec::new(),
            loc: self.current().loc.clone(),
        };

        loop {
            let ident_part = self.expect_any(Symbol)?;
            ident
                .parts
                .push(ident_part.get_value(self.lexer.source).to_string());
            ident.repr += ident_part.get_value(self.lexer.source);

            if let Ok(Some(_)) = self.eat(Operator, "::") {
                ident.repr += "::";
                continue;
            }

            break;
        }

        ident.loc.end_pos = self.prev().loc.end_pos;

        Ok(ident)
    }

    fn parse_struct_literal(
        &self,
        ident: IdentExpr,
        mut loc: LoLocation,
    ) -> Result<StructLiteralExpr, LoError> {
        let mut fields = Vec::new();
        let mut has_trailing_comma = false;

        self.expect(Delim, "{")?;
        while let None = self.eat(Delim, "}")? {
            has_trailing_comma = false;

            let mut field_loc = self.current().loc.clone();

            let field_name = self.expect_any(Symbol)?.clone();
            self.expect(Operator, ":")?;
            let value = self.parse_code_expr(0)?;

            field_loc.end_pos = self.prev().loc.end_pos;

            fields.push(StructLiteralField {
                field_name: field_name.get_value(self.lexer.source).to_string(),
                value,
                loc: field_loc,
            });

            if !self.current().is(Delim, "}", self.lexer.source) {
                self.expect(Delim, ",")?;
                has_trailing_comma = true;
            }
        }

        loc.end_pos = self.prev().loc.end_pos;

        return Ok(StructLiteralExpr {
            struct_name: ident,
            fields,
            has_trailing_comma,
            loc,
        });
    }

    fn parse_fn_args(&self) -> Result<CodeExprList, LoError> {
        let mut has_trailing_comma = false;
        let mut items = Vec::new();

        self.expect(Delim, "(")?;
        while let None = self.eat(Delim, ")")? {
            has_trailing_comma = false;

            items.push(self.parse_code_expr(0)?);

            if !self.current().is(Delim, ")", self.lexer.source) {
                self.expect(Delim, ",")?;
                has_trailing_comma = true;
            }
        }

        return Ok(CodeExprList {
            items,
            has_trailing_comma,
        });
    }

    fn parse_macro_type_args(&self) -> Result<Vec<TypeExpr>, LoError> {
        let mut type_args = Vec::new();

        let Some(_) = self.eat(Operator, "<")? else {
            return Ok(type_args);
        };

        while let None = self.eat(Operator, ">")? {
            type_args.push(self.parse_type_expr()?);

            if !self.current().is(Operator, ">", self.lexer.source) {
                self.expect(Delim, ",")?;
            }
        }

        return Ok(type_args);
    }

    fn parse_code_expr_postfix(&self, primary: CodeExpr, op: InfixOp) -> Result<CodeExpr, LoError> {
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
                loc.end_pos = rhs.loc().end_pos;

                Ok(CodeExpr::InfixOp(InfixOpExpr {
                    op_tag: op.tag,
                    op_loc: op.token.loc,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    loc,
                }))
            }
            InfixOpTag::Cast => {
                let mut loc = primary.loc().clone();

                let casted_to = self.parse_type_expr()?;

                loc.end_pos = self.prev().loc.end_pos;

                Ok(CodeExpr::Cast(CastExpr {
                    expr: Box::new(primary),
                    casted_to,
                    loc,
                }))
            }
            InfixOpTag::FieldAccess => {
                let mut loc = primary.loc().clone();

                let field_name = self.parse_ident()?;

                if self.current().is(Delim, "(", self.lexer.source) {
                    let args = self.parse_fn_args()?;

                    loc.end_pos = self.prev().loc.end_pos;

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

                    loc.end_pos = self.prev().loc.end_pos;

                    return Ok(CodeExpr::MacroMethodCall(MacroMethodCallExpr {
                        lhs: Box::new(primary),
                        field_name,
                        args,
                        type_args,
                        loc,
                    }));
                }

                loc.end_pos = self.prev().loc.end_pos;

                Ok(CodeExpr::FieldAccess(FieldAccessExpr {
                    lhs: Box::new(primary),
                    field_name,
                    loc,
                }))
            }
            InfixOpTag::Assign => {
                let mut loc = primary.loc().clone();

                let value = self.parse_code_expr(min_bp)?;

                loc.end_pos = self.prev().loc.end_pos;

                Ok(CodeExpr::Assign(AssignExpr {
                    op_loc: op.token.loc,
                    lhs: Box::new(primary),
                    rhs: Box::new(value),
                    loc,
                }))
            }
            InfixOpTag::Catch => {
                let mut loc = primary.loc().clone();

                let error_bind = self.parse_ident()?;
                let catch_body = self.parse_code_block()?;

                loc.end_pos = self.prev().loc.end_pos;

                Ok(CodeExpr::Catch(CatchExpr {
                    lhs: Box::new(primary),
                    error_bind,
                    catch_body,
                    catch_loc: op.token.loc,
                    loc,
                }))
            }
            InfixOpTag::ErrorPropagation => {
                let mut loc = primary.loc().clone();
                loc.end_pos = self.prev().loc.end_pos;

                Ok(CodeExpr::PropagateError(PropagateErrorExpr {
                    expr: Box::new(primary),
                    loc,
                }))
            }
            InfixOpTag::ExprPipe => {
                let mut loc = primary.loc().clone();

                let rhs = self.parse_code_expr(min_bp)?;

                loc.end_pos = self.prev().loc.end_pos;

                Ok(CodeExpr::ExprPipe(ExprPipeExpr {
                    lhs: Box::new(primary),
                    rhs: Box::new(rhs),
                    op_loc: op.token.loc,
                    loc,
                }))
            }
        }
    }

    // utils

    fn push_ctx(&self, ctx: ParsingContext) {
        self.contexts.be_mut().push(ctx);
    }

    fn pop_ctx(&self) {
        self.contexts.be_mut().pop();
    }

    fn expect_any(&self, type_: LoTokenType) -> Result<&LoToken, LoError> {
        let token = self.current();
        if !token.is_any(type_) {
            return Err(LoError {
                message: format!(
                    "Unexpected token '{}', wanted {type_:?}",
                    token.get_value(self.lexer.source)
                ),
                loc: token.loc.clone(),
            });
        }

        Ok(self.next().unwrap())
    }

    fn expect(&self, type_: LoTokenType, value: &str) -> Result<&LoToken, LoError> {
        let token = self.current();
        if !token.is(type_, value, self.lexer.source) {
            return Err(LoError {
                message: format!(
                    "Unexpected token '{}', wanted '{value}'",
                    token.get_value(self.lexer.source)
                ),
                loc: token.loc.clone(),
            });
        }

        Ok(self.next().unwrap())
    }

    fn eat_any(&self, type_: LoTokenType) -> Result<Option<&LoToken>, LoError> {
        let was_some = self.peek().is_some();
        match self.expect_any(type_) {
            Ok(t) => Ok(Some(t)),
            Err(_) if was_some => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn eat(&self, type_: LoTokenType, value: &str) -> Result<Option<&LoToken>, LoError> {
        let was_some = self.peek().is_some();
        match self.expect(type_, value) {
            Ok(t) => Ok(Some(t)),
            Err(_) if was_some => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn peek(&self) -> Option<&LoToken> {
        self.look(0)
    }

    fn current(&self) -> &LoToken {
        self.look(0)
            .unwrap_or_else(|| self.lexer.tokens.last().unwrap())
    }

    fn prev(&self) -> &LoToken {
        self.look(-1)
            .unwrap_or_else(|| self.lexer.tokens.last().unwrap())
    }

    fn look(&self, relative_offset: isize) -> Option<&LoToken> {
        let index = (*self.tokens_processed.borrow() as isize + relative_offset) as usize;

        // terminal token is never returned
        if index >= self.lexer.tokens.len() - 1 {
            return None;
        }

        Some(&self.lexer.tokens[index])
    }

    fn next(&self) -> Option<&LoToken> {
        let token = self.peek();
        *self.tokens_processed.borrow_mut() += 1;
        token
    }
}
