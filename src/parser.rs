use crate::{ast::*, core::*, lexer::*};
use TokenType::*;
use alloc::{boxed::Box, format, vec::Vec};
use core::cell::RefCell;

pub struct ParsingContext {
    pub struct_literal_allowed: bool,
}

pub struct Parser {
    // context
    pub lexer: Lexer,
    pub source: &'static [u8],
    pub reporter: &'static Reporter,

    // state
    pub tokens_processed: RefCell<usize>,
    pub context_stack: Vec<ParsingContext>,

    // output
    pub ast: Vec<TopLevelExpr>,
}

impl Parser {
    pub fn new(lexer: Lexer, reporter: &'static Reporter) -> Self {
        let mut context_stack = Vec::new();
        context_stack.push(ParsingContext {
            struct_literal_allowed: true,
        });

        Self {
            source: lexer.source,
            lexer,
            reporter,
            tokens_processed: RefCell::new(0),
            context_stack,
            ast: Vec::new(),
        }
    }

    pub fn parse_file(&self) -> Result<(), Error> {
        while !self.current().is_terminal() {
            let expr = self.parse_top_level_expr()?;
            self.ast.be_mut().push(expr);
        }

        Ok(())
    }

    fn parse_top_level_expr(&self) -> Result<TopLevelExpr, Error> {
        if let Some(_) = self.eat(Symbol, "export") {
            let loc = self.prev().loc;

            if let Some(_) = self.eat(Symbol, "fn") {
                let fn_def = self.parse_fn_def(true, false, loc)?;
                return Ok(TopLevelExpr::FnDef(fn_def));
            }

            let unexpected = self.current();
            return Err(Error {
                message: format!(
                    "Unexpected exportable: {}",
                    unexpected.get_value(self.source)
                ),
                loc: unexpected.loc,
            });
        }

        if let Some(_) = self.eat(Operator, "@") {
            let mut loc = self.prev().loc;

            let fn_name = self.parse_ident()?;
            let type_args = self.parse_inline_fn_type_args()?;
            let args = self.parse_fn_args()?;

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(TopLevelExpr::IntrinsicCall(InlineFnCallExpr {
                fn_name,
                args,
                type_args,
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "fn") {
            let loc = self.prev().loc;

            let fn_def = self.parse_fn_def(false, false, loc)?;
            return Ok(TopLevelExpr::FnDef(fn_def));
        }

        if let Some(_) = self.eat(Symbol, "include") {
            let mut loc = self.prev().loc;

            let file_path = self.expect_any(StringLiteral)?.clone();
            let mut alias = None;
            if let Some(_) = self.eat(Symbol, "as") {
                alias = Some(self.parse_ident()?);
            }

            let mut with_extern = false;
            if let Some(_) = self.eat(Symbol, "with") {
                self.expect(Symbol, "extern")?;
                with_extern = true;
            }

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(TopLevelExpr::Include(IncludeExpr {
                file_path: QuotedString::new(file_path.loc),
                alias,
                with_extern,
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "import") {
            let mut loc = self.prev().loc;

            self.expect(Symbol, "from")?;
            let module_name = self.expect_any(StringLiteral)?.clone();

            self.expect(Delim, "{")?;

            let mut items = Vec::new();
            while let None = self.eat(Delim, "}") {
                let item = self.parse_importable()?;
                items.push(item);
            }

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(TopLevelExpr::Import(ImportExpr {
                module_name: QuotedString::new(module_name.loc),
                items,
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "type") {
            let mut loc = self.prev().loc;

            let type_name = self.parse_ident()?;
            self.expect(Operator, "=")?;

            if self.eat(Symbol, "struct").is_some() {
                let mut fields = Vec::new();

                self.expect(Delim, "{")?;
                while let None = self.eat(Delim, "}") {
                    let mut field_loc = self.current().loc;

                    let field_name = self.parse_ident()?;
                    self.expect(Operator, ":")?;
                    let field_type = self.parse_type_expr()?;

                    field_loc.end_pos = self.prev().loc.end_pos;

                    fields.push(StructDefField {
                        field_name,
                        field_type,
                        loc: field_loc,
                    });

                    if !self.current().is(Delim, "}", self.source) {
                        self.expect(Delim, ",")?;
                    }
                }

                loc.end_pos = self.prev().loc.end_pos;

                return Ok(TopLevelExpr::StructDef(StructDefExpr {
                    struct_name: type_name,
                    fields,
                    loc,
                }));
            }

            if self.eat(Symbol, "enum").is_some() {
                let mut variant_type = None;
                if let Some(_) = self.eat(Delim, "(") {
                    variant_type = Some(self.parse_type_expr()?);
                    self.expect(Delim, ")")?;
                }

                let mut variants = Vec::new();

                self.expect(Delim, "{")?;
                while let None = self.eat(Delim, "}") {
                    let mut variant_loc = self.current().loc;

                    let variant_name = self.parse_ident()?;

                    let mut variant_type = None;
                    if let Some(_) = self.eat(Delim, "(") {
                        variant_type = Some(self.parse_type_expr()?);
                        self.expect(Delim, ")")?;
                    }

                    variant_loc.end_pos = self.prev().loc.end_pos;

                    variants.push(EnumDefVariant {
                        variant_name,
                        variant_type,
                        loc: variant_loc,
                    });

                    if !self.current().is(Delim, "}", self.source) {
                        self.expect(Delim, ",")?;
                    }
                }

                loc.end_pos = self.prev().loc.end_pos;

                return Ok(TopLevelExpr::EnumDef(EnumDefExpr {
                    enum_name: type_name,
                    variant_type,
                    variants,
                    loc,
                }));
            }

            let type_value = self.parse_type_expr()?;

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(TopLevelExpr::TypeDef(TypeDefExpr {
                type_name,
                type_value,
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "let") {
            let loc = self.prev().loc;
            let let_expr = self.parse_let(false, loc)?;
            return Ok(TopLevelExpr::Let(let_expr));
        }

        if self.eat(Symbol, "inline").is_some() {
            let loc = self.prev().loc;

            if self.eat(Symbol, "let").is_some() {
                let let_expr = self.parse_let(true, loc)?;
                return Ok(TopLevelExpr::Let(let_expr));
            }

            if let Some(_) = self.eat(Symbol, "fn") {
                let fn_def = self.parse_fn_def(false, true, loc)?;
                return Ok(TopLevelExpr::FnDef(fn_def));
            }

            let unexpected = self.current();
            return Err(Error {
                message: format!(
                    "Unexpected inlineable: {}",
                    unexpected.get_value(self.source)
                ),
                loc: unexpected.loc,
            });
        }

        let unexpected = self.current();
        return Err(Error {
            message: format!(
                "Unexpected top level token: {}",
                unexpected.get_value(self.source)
            ),
            loc: unexpected.loc,
        });
    }

    fn parse_let(&self, is_inline: bool, mut loc: Loc) -> Result<LetExpr, Error> {
        let name = self.parse_ident()?;
        self.expect(Operator, "=")?;
        let value = self.parse_code_expr(0)?;

        loc.end_pos = self.prev().loc.end_pos;

        return Ok(LetExpr {
            is_inline,
            name,
            value: Box::new(value),
            loc,
        });
    }

    fn parse_fn_def(
        &self,
        exported: bool,
        is_inline: bool,
        mut loc: Loc,
    ) -> Result<FnDefExpr, Error> {
        let decl = self.parse_fn_decl(is_inline)?;
        let body = self.parse_code_block()?;

        loc.end_pos = self.prev().loc.end_pos;

        Ok(FnDefExpr {
            exported,
            is_inline,
            decl,
            body,
            loc,
        })
    }

    fn parse_importable(&self) -> Result<ImportItem, Error> {
        if let Some(_) = self.eat(Symbol, "fn") {
            let decl = self.parse_fn_decl(false)?;
            return Ok(ImportItem::FnDecl(decl));
        }

        let unexpected = self.current();
        return Err(Error {
            message: format!(
                "Unexpected token in importable item: {:?}",
                unexpected.get_value(self.source)
            ),
            loc: unexpected.loc,
        });
    }

    fn parse_fn_decl(&self, is_inline: bool) -> Result<FnDeclExpr, Error> {
        let mut loc = self.prev().loc;

        let mut fn_name = self.parse_ident()?;

        let mut type_params = Vec::new();

        if is_inline {
            self.expect(Operator, "!")?;
            self.extend_ident(&mut fn_name, self.prev().loc.end_pos);

            if let Some(_) = self.eat(Operator, "<") {
                while let None = self.eat(Operator, ">") {
                    let type_param = self.expect_any(Symbol)?;
                    type_params.push(type_param.get_value(self.source));

                    if !self.current().is(Operator, ">", self.source) {
                        self.expect(Delim, ",")?;
                    }
                }
            }
        }

        let mut params_trailing_comma = false;
        let params = self.parse_fn_params(&mut params_trailing_comma)?;

        let mut return_type = None;
        if let Some(_) = self.eat(Operator, ":") {
            return_type = Some(self.parse_type_expr()?)
        };

        loc.end_pos = self.prev().loc.end_pos;

        Ok(FnDeclExpr {
            fn_name,
            type_params,
            params,
            params_trailing_comma,
            return_type,
            loc,
        })
    }

    fn parse_fn_params(&self, trailing_comma: &mut bool) -> Result<Vec<FnParam>, Error> {
        let mut params = Vec::<FnParam>::new();

        let _ = self.expect(Delim, "(")?;

        while let None = self.eat(Delim, ")") {
            *trailing_comma = false;

            let mut loc = self.current().loc;

            let mut param_type = FnParamType::Self_;
            if let Some(_) = self.eat(Operator, "&") {
                param_type = FnParamType::SelfRef;
            }

            let param_name = self.parse_ident()?;
            if let Some(_) = self.eat(Operator, ":") {
                param_type = FnParamType::Type {
                    expr: self.parse_type_expr()?,
                };
            }

            loc.end_pos = self.prev().loc.end_pos;

            if !self.current().is(Delim, ")", self.source) {
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

    fn parse_type_expr(&self) -> Result<TypeExpr, Error> {
        let mut loc = self.current().loc;
        let primary = self.parse_type_expr_primary()?;

        if let Some(_) = self.eat(Delim, "(") {
            let container = Box::new(primary);
            let mut items = Vec::new();

            while let None = self.eat(Delim, ")") {
                items.push(self.parse_type_expr()?);

                if !self.current().is(Delim, ")", self.source) {
                    self.expect(Delim, ",")?;
                }
            }

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(TypeExpr::Container(TypeExprContainer {
                container,
                items,
                loc,
            }));
        }

        return Ok(primary);
    }

    fn parse_type_expr_primary(&self) -> Result<TypeExpr, Error> {
        let mut loc = self.current().loc;

        if let Some(_) = self.eat(Operator, "&") {
            let pointee = Box::new(self.parse_type_expr()?);
            loc.end_pos = self.prev().loc.end_pos;
            return Ok(TypeExpr::Pointer(TypeExprPointer { pointee, loc }));
        }

        // lexer joins two `&` into `&&`
        if let Some(_) = self.eat(Operator, "&&") {
            let pointee = Box::new(self.parse_type_expr()?);
            loc.end_pos = self.prev().loc.end_pos;
            return Ok(TypeExpr::Pointer(TypeExprPointer {
                pointee: Box::new(TypeExpr::Pointer(TypeExprPointer { pointee, loc })),
                loc,
            }));
        }

        if let Some(_) = self.eat(Operator, "*&") {
            let pointee = Box::new(self.parse_type_expr()?);
            loc.end_pos = self.prev().loc.end_pos;
            return Ok(TypeExpr::SequencePointer(TypeExprSequencePointer {
                pointee,
                loc,
            }));
        }

        let ident = self.parse_ident()?;
        return Ok(TypeExpr::Named(TypeExprNamed { name: ident }));
    }

    fn parse_code_block(&self) -> Result<CodeBlock, Error> {
        self.expect(Delim, "{")?;

        let mut code_block = CodeBlock {
            exprs: Vec::new(),
            loc: self.prev().loc,
        };

        while let None = self.eat(Delim, "}") {
            let expr = self.parse_code_expr(0)?;
            code_block.exprs.push(expr);
        }

        // closing curly pos
        code_block.loc.end_pos = self.prev().loc.end_pos;

        return Ok(code_block);
    }

    fn parse_code_expr(&self, min_bp: u32) -> Result<CodeExpr, Error> {
        let mut primary = self.parse_code_expr_primary()?;
        let mut backslash_start_hint = 0;

        while !self.current().is_terminal() {
            let op_symbol = self.current().clone();

            let backslash_between = self.has_backslashes_between(
                &mut backslash_start_hint,
                primary.loc().end_pos.offset,
                op_symbol.loc.pos.offset,
            );

            if op_symbol.loc.pos.line != primary.loc().end_pos.line && !backslash_between {
                break;
            }

            let Some(op) = InfixOp::parse(op_symbol.get_value(self.source)) else {
                break;
            };

            if op.bp < min_bp {
                break;
            }

            self.next(); // skip operator
            primary = self.parse_code_expr_postfix(primary, op, &op_symbol.loc)?;
        }

        Ok(primary)
    }

    fn parse_code_expr_primary(&self) -> Result<CodeExpr, Error> {
        if let Some(_) = self.eat(Symbol, "true") {
            let loc = self.prev().loc;

            return Ok(CodeExpr::BoolLiteral(BoolLiteralExpr { value: true, loc }));
        }

        if let Some(_) = self.eat(Symbol, "false") {
            let loc = self.prev().loc;

            return Ok(CodeExpr::BoolLiteral(BoolLiteralExpr { value: false, loc }));
        }

        if let Some(_) = self.eat(Symbol, "null") {
            let loc = self.prev().loc;

            return Ok(CodeExpr::NullLiteral(NullLiteralExpr { loc }));
        }

        if let Some(char) = self.eat_any(CharLiteral).cloned() {
            return Ok(CodeExpr::CharLiteral(CharLiteralExpr {
                repr: char.get_value(self.source),
                value: Lexer::parse_char_literal_value(&char.get_value(self.source)) as u32,
                loc: char.loc,
            }));
        };

        if let Some(int) = self.eat_any(IntLiteral).cloned() {
            let mut tag = None;
            let tag_token = self.current();
            if tag_token.is_any(Symbol) && tag_token.loc.pos.offset == int.loc.end_pos.offset {
                tag = Some(tag_token.get_value(self.source));
                self.next();
            }

            let repr = int.get_value(self.source);
            let value = Lexer::parse_int_literal_value(repr);

            return Ok(CodeExpr::IntLiteral(IntLiteralExpr {
                repr,
                value,
                tag,
                loc: int.loc,
            }));
        };

        if let Some(_) = self.eat_any(StringLiteral) {
            let string = QuotedString::new(self.prev().loc);

            return Ok(CodeExpr::StringLiteral(StringLiteralExpr {
                repr: string.get_repr(self.source),
                value: string.get_value(self.source),
                loc: string.loc,
            }));
        };

        if let Some(_) = self.eat(Delim, "(") {
            let mut loc = self.prev().loc;

            self.push_ctx(ParsingContext {
                struct_literal_allowed: true,
            });

            let expr = Box::new(self.parse_code_expr(0)?);

            self.pop_ctx();

            let has_trailing_comma = self.eat(Delim, ",").is_some();
            self.expect(Delim, ")")?;

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(CodeExpr::Paren(ParenExpr {
                expr,
                has_trailing_comma,
                loc,
            }));
        };

        if let Some(_) = self.eat(Symbol, "let") {
            let loc = self.prev().loc;

            let let_expr = self.parse_let(false, loc)?;
            return Ok(CodeExpr::Let(let_expr));
        }

        if let Some(_) = self.eat(Symbol, "inline") {
            let loc = self.prev().loc;

            if let Some(_) = self.eat(Symbol, "let") {
                let let_expr = self.parse_let(true, loc)?;
                return Ok(CodeExpr::Let(let_expr));
            }

            let unexpected = self.current();
            return Err(Error {
                message: format!(
                    "Unexpected inlineable: {}",
                    unexpected.get_value(self.source)
                ),
                loc: unexpected.loc,
            });
        }

        if let Some(_) = self.eat(Symbol, "return") {
            let mut loc = self.prev().loc;

            let mut expr = None;
            if self.current().loc.end_pos.line == loc.end_pos.line {
                expr = Some(Box::new(self.parse_code_expr(0)?));
            }

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(CodeExpr::Return(ReturnExpr { expr, loc }));
        };

        if let Some(_) = self.eat(Symbol, "if") {
            let mut loc = self.prev().loc;

            self.push_ctx(ParsingContext {
                struct_literal_allowed: false,
            });

            let cond: IfCond;
            if let Some(_) = self.eat(Symbol, "match") {
                cond = IfCond::Match(Box::new(self.parse_match_header()?));
            } else {
                cond = IfCond::Expr(Box::new(self.parse_code_expr(0)?));
            };

            self.pop_ctx();

            let then_block = Box::new(self.parse_code_block()?);

            let mut else_block = ElseBlock::None;
            if let Some(_) = self.eat(Symbol, "else") {
                if self.current().is(Symbol, "if", self.source) {
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

        if let Some(_) = self.eat(Symbol, "match") {
            let mut loc = self.prev().loc;

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

        if let Some(_) = self.eat(Symbol, "loop") {
            let mut loc = self.prev().loc;

            let body = self.parse_code_block()?;

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(CodeExpr::While(WhileExpr {
                cond: None,
                body: Box::new(body),
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "while") {
            let mut loc = self.prev().loc;

            self.push_ctx(ParsingContext {
                struct_literal_allowed: false,
            });
            let cond = self.parse_code_expr(0)?;
            self.pop_ctx();

            let body = self.parse_code_block()?;

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(CodeExpr::While(WhileExpr {
                cond: Some(Box::new(cond)),
                body: Box::new(body),
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "for") {
            let mut loc = self.prev().loc;

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

            return Ok(CodeExpr::For(ForExpr {
                counter,
                start: Box::new(start),
                end: Box::new(end),
                body: Box::new(body),
                op_loc: op.loc,
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "break") {
            let loc = self.prev().loc;

            return Ok(CodeExpr::Break(BreakExpr { loc }));
        }

        if let Some(_) = self.eat(Symbol, "continue") {
            let loc = self.prev().loc;

            return Ok(CodeExpr::Continue(ContinueExpr { loc }));
        }

        if let Some(_) = self.eat(Symbol, "do") {
            let mut loc = self.prev().loc;

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

        if let Some(_) = self.eat(Operator, "@") {
            let mut loc = self.prev().loc;

            let fn_name = self.parse_ident()?;
            let type_args = self.parse_inline_fn_type_args()?;
            let args = self.parse_fn_args()?;

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(CodeExpr::IntrinsicCall(InlineFnCallExpr {
                fn_name,
                args,
                type_args,
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "defer") {
            let mut loc = self.prev().loc;

            let expr = self.parse_code_expr(0)?;

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(CodeExpr::Defer(DeferExpr {
                expr: Box::new(expr),
                loc,
            }));
        }

        if let Some(_) = self.eat(Symbol, "sizeof") {
            let mut loc = self.prev().loc;

            let type_expr = self.parse_type_expr()?;

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(CodeExpr::Sizeof(SizeofExpr { type_expr, loc }));
        };

        if let Some(_) = self.eat(Delim, "[") {
            let mut loc = self.prev().loc;

            let item_type = self.parse_type_expr()?;
            self.expect(Delim, "]")?;

            let mut has_trailing_comma = false;

            self.expect(Delim, "[")?;
            let mut items = Vec::new();
            while let None = self.eat(Delim, "]") {
                let item = self.parse_code_expr(0)?;
                items.push(item);

                if !self.current().is(Delim, "]", self.source) {
                    self.expect(Delim, ",")?;
                    has_trailing_comma = true;
                } else {
                    has_trailing_comma = false;
                }
            }

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(CodeExpr::ArrayLiteral(ArrayLiteralExpr {
                item_type,
                items,
                has_trailing_comma,
                loc,
            }));
        }

        if self.eat(Operator, ":").is_some() || self.eat(Delim, "{").is_some() {
            return Err(Error {
                message: format!(
                    "Unexpected character '{}'. \
                    If you were trying to create a struct in this context \
                    surround it with parens.",
                    self.prev().get_value(self.source)
                ),
                loc: self.prev().loc,
            });
        }

        let op_token = self.current();
        if !op_token.is_terminal() {
            if let Some(op) = PrefixOp::parse(op_token.get_value(self.source)) {
                self.next(); // skip operator

                let mut loc = self.prev().loc;

                match op.tag {
                    PrefixOpTag::Reference
                    | PrefixOpTag::Dereference
                    | PrefixOpTag::Not
                    | PrefixOpTag::Positive
                    | PrefixOpTag::Negative => {
                        let expr = Box::new(self.parse_code_expr(op.bp_next)?);

                        loc.end_pos = self.prev().loc.end_pos;

                        return Ok(CodeExpr::PrefixOp(PrefixOpExpr {
                            expr,
                            op_tag: op.tag,
                            op_loc: op_token.loc,
                            loc,
                        }));
                    }
                }
            }
        }

        let mut ident = self.parse_ident()?;

        if ident.repr == "Ok" || ident.repr == "Err" {
            let mut loc = ident.loc;

            let mut result_type = None;
            if let Some(_) = self.eat(Operator, ":") {
                self.expect(Operator, "<")?;
                let ok = self.parse_type_expr()?;
                self.expect(Delim, ",")?;
                let err = self.parse_type_expr()?;
                self.expect(Operator, ">")?;

                result_type = Some(ResultTypeExpr { ok, err });
            }

            self.expect(Delim, "(")?;
            let mut value = None;
            if let None = self.eat(Delim, ")") {
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

        let ctx = self.context_stack.last().unwrap();
        if self.current().is(Delim, "{", self.source) && ctx.struct_literal_allowed {
            let mut loc = ident.loc;
            let body = self.parse_code_expr_map()?;
            loc.end_pos = body.loc.end_pos;

            return Ok(CodeExpr::StructLiteral(StructLiteralExpr {
                struct_name: ident,
                body,
                loc,
            }));
        }

        if self.current().is(Delim, "(", self.source) {
            let mut loc = ident.loc;

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

        if let Some(_) = self.eat(Operator, "!") {
            self.extend_ident(&mut ident, self.prev().loc.end_pos);

            let mut loc = ident.loc;

            let type_args = self.parse_inline_fn_type_args()?;
            let args = self.parse_fn_args()?;

            loc.end_pos = self.prev().loc.end_pos;

            return Ok(CodeExpr::InlineFnCall(InlineFnCallExpr {
                fn_name: ident,
                args,
                type_args,
                loc,
            }));
        }

        Ok(CodeExpr::Ident(ident))
    }

    fn parse_code_expr_postfix(
        &self,
        primary: CodeExpr,
        op: InfixOp,
        op_loc: &Loc,
    ) -> Result<CodeExpr, Error> {
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
                let rhs = self.parse_code_expr(op.bp_next)?;

                let mut loc = lhs.loc();
                loc.end_pos = rhs.loc().end_pos;

                Ok(CodeExpr::InfixOp(InfixOpExpr {
                    op_tag: op.tag,
                    op_loc: *op_loc,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    loc,
                }))
            }
            InfixOpTag::Cast => {
                let mut loc = primary.loc();

                let casted_to = self.parse_type_expr()?;

                loc.end_pos = self.prev().loc.end_pos;

                Ok(CodeExpr::Cast(CastExpr {
                    expr: Box::new(primary),
                    casted_to,
                    loc,
                }))
            }
            InfixOpTag::FieldAccess => {
                let mut loc = primary.loc();

                let mut field_name = self.parse_ident()?;

                if self.current().is(Delim, "(", self.source) {
                    let args = self.parse_fn_args()?;

                    loc.end_pos = self.prev().loc.end_pos;

                    return Ok(CodeExpr::MethodCall(MethodCallExpr {
                        lhs: Box::new(primary),
                        field_name,
                        args,
                        loc,
                    }));
                }

                if let Some(_) = self.eat(Operator, "!") {
                    self.extend_ident(&mut field_name, self.prev().loc.end_pos);

                    let type_args = self.parse_inline_fn_type_args()?;
                    let args = self.parse_fn_args()?;

                    loc.end_pos = self.prev().loc.end_pos;

                    return Ok(CodeExpr::InlineMethodCall(InlineMethodCallExpr {
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
                let mut loc = primary.loc();

                let value = self.parse_code_expr(op.bp_next)?;

                loc.end_pos = self.prev().loc.end_pos;

                Ok(CodeExpr::Assign(AssignExpr {
                    op_loc: *op_loc,
                    lhs: Box::new(primary),
                    rhs: Box::new(value),
                    loc,
                }))
            }
            InfixOpTag::Catch => {
                let mut loc = primary.loc();

                let error_bind = self.parse_ident()?;
                let catch_body = self.parse_code_block()?;

                loc.end_pos = self.prev().loc.end_pos;

                Ok(CodeExpr::Catch(CatchExpr {
                    lhs: Box::new(primary),
                    error_bind,
                    catch_body,
                    catch_loc: *op_loc,
                    loc,
                }))
            }
            InfixOpTag::ErrorPropagation => {
                let mut loc = primary.loc();
                loc.end_pos = self.prev().loc.end_pos;

                Ok(CodeExpr::PropagateError(PropagateErrorExpr {
                    expr: Box::new(primary),
                    loc,
                }))
            }
            InfixOpTag::ExprPipe => {
                let mut loc = primary.loc();

                let rhs = self.parse_code_expr(op.bp_next)?;

                loc.end_pos = self.prev().loc.end_pos;

                Ok(CodeExpr::ExprPipe(ExprPipeExpr {
                    lhs: Box::new(primary),
                    rhs: Box::new(rhs),
                    op_loc: *op_loc,
                    loc,
                }))
            }
        }
    }

    fn has_backslashes_between(
        &self,
        start_hint: &mut usize,
        offset_start: usize,
        offset_end: usize,
    ) -> bool {
        let mut i = *start_hint;

        while i < self.lexer.backslashes.len() {
            let backslash = &self.lexer.backslashes[i];
            if backslash.pos.offset < offset_start {
                i += 1;
                continue;
            }

            *start_hint = i;
            return backslash.end_pos.offset <= offset_end;
        }

        *start_hint = i;
        false
    }

    fn parse_match_header(&self) -> Result<MatchHeader, Error> {
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

    fn parse_ident(&self) -> Result<IdentExpr, Error> {
        let mut ident = IdentExpr {
            repr: "", // stub
            parts: Vec::new(),
            loc: self.current().loc,
        };

        loop {
            let ident_part = self.expect_any(Symbol)?;
            ident.parts.push(ident_part.loc);

            if let None = self.eat(Operator, "::") {
                break;
            }
        }

        ident.loc.end_pos = self.prev().loc.end_pos;

        ident.repr = ident.loc.read_span(self.source);
        if ident.repr.contains(" ") {
            self.reporter.error(&Error {
                message: format!("Unexpected space in identifier"),
                loc: ident.loc,
            });
        }

        Ok(ident)
    }

    fn parse_code_expr_map(&self) -> Result<CodeExprMap, Error> {
        let mut fields = Vec::new();
        let mut has_trailing_comma = false;

        let mut loc = self.expect(Delim, "{")?.loc;

        while let None = self.eat(Delim, "}") {
            let mut field_loc = self.current().loc;

            let field_name = self.expect_any(Symbol)?.clone();
            self.expect(Operator, ":")?;
            let value = self.parse_code_expr(0)?;

            field_loc.end_pos = self.prev().loc.end_pos;

            fields.push(CodeExprMapField {
                key: field_name.get_value(self.source),
                value,
                loc: field_loc,
            });

            if !self.current().is(Delim, "}", self.source) {
                self.expect(Delim, ",")?;
                has_trailing_comma = true;
            } else {
                has_trailing_comma = false;
            }
        }

        loc.end_pos = self.prev().loc.end_pos;

        Ok(CodeExprMap {
            fields,
            has_trailing_comma,
            loc,
        })
    }

    fn parse_fn_args(&self) -> Result<CodeExprList, Error> {
        let mut has_trailing_comma = false;
        let mut items = Vec::new();

        self.expect(Delim, "(")?;
        while let None = self.eat(Delim, ")") {
            items.push(self.parse_code_expr(0)?);

            if !self.current().is(Delim, ")", self.source) {
                self.expect(Delim, ",")?;
                has_trailing_comma = true;
            } else {
                has_trailing_comma = false;
            }
        }

        return Ok(CodeExprList {
            items,
            has_trailing_comma,
        });
    }

    fn parse_inline_fn_type_args(&self) -> Result<Vec<TypeExpr>, Error> {
        let mut type_args = Vec::new();

        let Some(_) = self.eat(Operator, "<") else {
            return Ok(type_args);
        };

        while let None = self.eat(Operator, ">") {
            type_args.push(self.parse_type_expr()?);

            if !self.current().is(Operator, ">", self.source) {
                self.expect(Delim, ",")?;
            }
        }

        return Ok(type_args);
    }

    fn extend_ident(&self, ident: &mut IdentExpr, new_end_pos: Pos) {
        ident.loc.end_pos = new_end_pos;
        ident.repr = ident.loc.read_span(self.source);
    }

    // utils

    fn push_ctx(&self, ctx: ParsingContext) {
        self.context_stack.be_mut().push(ctx);
    }

    fn pop_ctx(&self) {
        self.context_stack.be_mut().pop();
    }

    fn expect_any(&self, type_: TokenType) -> Result<&Token, Error> {
        let token = self.current();
        if !token.is_any(type_) {
            return Err(Error {
                message: format!(
                    "Unexpected token '{}', wanted {type_:?}",
                    token.get_value(self.source)
                ),
                loc: token.loc,
            });
        }

        Ok(self.next())
    }

    fn expect(&self, type_: TokenType, value: &str) -> Result<&Token, Error> {
        let token = self.current();
        if !token.is(type_, value, self.source) {
            return Err(Error {
                message: format!(
                    "Unexpected token '{}', wanted '{value}'",
                    token.get_value(self.source)
                ),
                loc: token.loc,
            });
        }

        Ok(self.next())
    }

    fn eat_any(&self, type_: TokenType) -> Option<&Token> {
        if !self.current().is_any(type_) {
            return None;
        }

        Some(self.next())
    }

    fn eat(&self, type_: TokenType, value: &str) -> Option<&Token> {
        if !self.current().is(type_, value, self.source) {
            return None;
        }

        Some(self.next())
    }

    // returns current or terminal token
    fn current(&self) -> &Token {
        let mut index = *self.tokens_processed.borrow();
        if index >= self.lexer.tokens.len() - 1 {
            index = self.lexer.tokens.len() - 1;
        }

        &self.lexer.tokens[index]
    }

    fn prev(&self) -> &Token {
        &self.lexer.tokens[*self.tokens_processed.borrow() - 1]
    }

    fn next(&self) -> &Token {
        let token = self.current();
        *self.tokens_processed.borrow_mut() += 1;
        token
    }
}
