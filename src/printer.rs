use crate::{ast::*, core::*, parser::*};
use alloc::vec::Vec;

pub struct Printer {
    parser: &'static Parser,

    indent: usize,
    comments_printed: usize,
    last_printed_item_line: usize,
    backslashes_printed: usize,
    backslash_stack: Vec<bool>,
    last_stmt_had_backslash: bool,
    double_backslashes_printed: usize,
}

impl Printer {
    pub fn new(parser: &'static Parser) -> Self {
        Printer {
            parser,

            indent: 0,
            comments_printed: 0,
            last_printed_item_line: 1,
            backslashes_printed: 0,
            backslash_stack: Vec::new(),
            last_stmt_had_backslash: false,
            double_backslashes_printed: 0,
        }
    }

    pub fn print_file(&mut self) {
        stdout_enable_buffering();

        for expr in &self.parser.ast {
            self.print_top_level_expr(expr);
            stdout_write("\n");
            self.last_printed_item_line = expr.loc().end_pos.line;
        }

        // print the rest of the comments
        self.print_comments_before(Pos {
            offset: usize::MAX,
            line: self.last_printed_item_line,
            col: usize::MAX,
        });

        stdout_disable_buffering();
    }

    fn print_top_level_expr(&mut self, expr: &TopLevelExpr) {
        self.print_comments_before(expr.loc().pos);

        match &expr {
            TopLevelExpr::FnDef(FnDefExpr {
                exported,
                is_inline,
                decl,
                body,
                loc: _,
            }) => {
                if *exported {
                    stdout_write("export ");
                }
                if *is_inline {
                    stdout_write("inline ");
                }
                self.print_fn_decl(decl);

                self.print_code_block(body);
            }
            TopLevelExpr::FnImport(FnImportExpr {
                decl,
                imported_from,
                loc: _,
            }) => {
                self.print_fn_decl(decl);
                stdout_write(" = @import_from(");
                stdout_write(imported_from.get_repr(self.parser.source));
                stdout_write(")");
            }
            TopLevelExpr::Let(LetExpr {
                is_inline: inline,
                name,
                value,
                loc: _,
            }) => {
                if *inline {
                    stdout_write("inline ");
                }
                stdout_write("let ");
                stdout_write(&name.repr);
                stdout_write(" = ");
                self.print_code_expr(value);
            }
            TopLevelExpr::TypeDef(TypeDefExpr {
                name: type_name,
                value: type_value,
                loc,
            }) => {
                stdout_write("type ");
                stdout_write(&type_name.repr);
                stdout_write(" = ");

                match type_value {
                    TypeDefValue::Struct { fields } => {
                        stdout_write("struct");

                        if fields.len() == 0 {
                            stdout_write(" {}");
                            return;
                        }

                        stdout_writeln(" {");
                        self.indent += 1;
                        for field in fields {
                            self.print_comments_before(field.loc.pos);
                            self.print_indent();
                            stdout_write(&field.field_name.repr);
                            stdout_write(": ");
                            self.print_type_expr(&field.field_type);
                            stdout_writeln(",");
                        }

                        // print the rest of the comments
                        self.print_comments_before(loc.end_pos);

                        self.indent -= 1;
                        self.print_indent();

                        stdout_write("}");
                    }
                    TypeDefValue::Enum {
                        variant_type,
                        variants,
                    } => {
                        stdout_write("enum");

                        if let Some(data_type) = variant_type {
                            stdout_write(" (");
                            self.print_type_expr(data_type);
                            stdout_write(")");
                        }

                        if variants.len() == 0 {
                            stdout_write(" {}");
                            return;
                        }

                        stdout_writeln(" {");
                        self.indent += 1;
                        for variant in variants {
                            self.print_comments_before(variant.loc.pos);
                            self.print_indent();
                            stdout_write(&variant.variant_name.repr);
                            if let Some(variant_type) = &variant.variant_type {
                                stdout_write("(");
                                self.print_type_expr(variant_type);
                                stdout_write(")");
                            }
                            stdout_writeln(",");
                        }

                        // print the rest of the comments
                        self.print_comments_before(loc.end_pos);

                        self.indent -= 1;
                        self.print_indent();

                        stdout_write("}");
                    }
                    TypeDefValue::Alias(type_expr) => {
                        self.print_type_expr(type_expr);
                    }
                }
            }
            TopLevelExpr::IntrinsicCall(InlineFnCallExpr {
                fn_name,
                type_args,
                args,
                loc,
            }) => {
                stdout_write("@");
                stdout_write(&fn_name.repr);
                self.print_type_args(type_args);
                self.print_args(args, loc);
            }
            TopLevelExpr::Include(IncludeExpr {
                file_path,
                alias,
                with_extern,
                loc: _,
            }) => {
                stdout_write("include ");
                stdout_write(file_path.get_repr(self.parser.lexer.source));
                if let Some(alias) = alias {
                    stdout_write(" as ");
                    stdout_write(&alias.repr);
                }
                if *with_extern {
                    stdout_write(" with extern");
                }
            }
        }
    }

    fn print_fn_decl(&mut self, fn_decl: &FnDeclExpr) {
        stdout_write("fn ");
        stdout_write(&fn_decl.fn_name.repr);

        if fn_decl.type_params.len() != 0 {
            stdout_write("<");
            for (type_param, i) in fn_decl.type_params.iter().zip(0..) {
                stdout_write(type_param);
                if i != fn_decl.type_params.len() - 1 {
                    stdout_write(",");
                }
            }
            stdout_write(">");
        }

        self.print_fn_params(&fn_decl.params, fn_decl.params_trailing_comma);

        if let Some(return_type) = &fn_decl.return_type {
            stdout_write(": ");
            self.print_type_expr(&return_type);
        }

        self.last_printed_item_line = fn_decl.loc.end_pos.line;
    }

    fn print_fn_params(&mut self, params: &Vec<FnParam>, is_multiline: bool) {
        stdout_write("(");

        if is_multiline {
            self.indent += 1;
            stdout_write("\n");
        }

        for (param, index) in params.iter().zip(0..) {
            if index != 0 {
                stdout_write(",");
            }

            let continues = self.print_double_backslashes_before(param.loc.pos.offset);

            if is_multiline && !continues {
                if index != 0 {
                    stdout_write("\n");
                }

                self.print_comments_before(param.loc.pos);
                self.print_indent();
            } else if index != 0 {
                stdout_write(" ");
            }

            match &param.param_type {
                FnParamType::Self_ => {
                    stdout_write(&param.param_name.repr);
                }
                FnParamType::SelfRef => {
                    stdout_write("&");
                    stdout_write(&param.param_name.repr);
                }
                FnParamType::Type { expr } => {
                    stdout_write(&param.param_name.repr);
                    stdout_write(": ");
                    self.print_type_expr(&expr);
                }
            }

            self.last_printed_item_line = param.loc.pos.line;
        }

        if is_multiline {
            stdout_writeln(",");
            self.indent -= 1;
            self.print_indent();
        }

        stdout_write(")");
    }

    fn print_type_expr(&mut self, type_expr: &TypeExpr) {
        match type_expr {
            TypeExpr::Named(TypeExprNamed { name }) => {
                stdout_write(&name.repr);
            }
            TypeExpr::Pointer(TypeExprPointer { pointee, loc: _ }) => {
                stdout_write("&");
                self.print_type_expr(pointee);
            }
            TypeExpr::SequencePointer(TypeExprSequencePointer { pointee, loc: _ }) => {
                stdout_write("*&");
                self.print_type_expr(pointee);
            }
            TypeExpr::Container(TypeExprContainer {
                container,
                items,
                loc: _,
            }) => {
                self.print_type_expr(container);
                stdout_write("(");
                for (i, item) in items.iter().enumerate() {
                    if i != 0 {
                        stdout_write(", ");
                    }
                    self.print_type_expr(item);
                }
                stdout_write(")");
            }
        }
    }

    fn print_code_block(&mut self, code_block: &CodeBlock) {
        if self.last_stmt_had_backslash {
            self.last_stmt_had_backslash = false;
            stdout_write("\n");
            self.print_indent();
        } else {
            stdout_write(" ");
        }

        if code_block.loc.pos.line == code_block.loc.end_pos.line {
            return stdout_write("{}");
        }

        stdout_write("{");
        self.last_printed_item_line = code_block.loc.pos.line;

        self.indent += 1;

        for expr in &code_block.exprs {
            let continues = self.print_double_backslashes_before(expr.loc().pos.offset);
            if continues {
                stdout_write(" ");
            } else {
                stdout_write("\n");
                self.print_comments_before(expr.loc().pos);
                self.print_indent();
            }

            self.print_code_expr(expr);
            self.last_stmt_had_backslash = false;
        }
        stdout_write("\n");

        // print the rest of the comments
        self.print_comments_before(code_block.loc.end_pos);

        self.indent -= 1;

        self.print_indent();
        stdout_write("}");
    }

    fn print_code_expr(&mut self, expr: &CodeExpr) {
        self.backslash_stack.push(false);

        self.print_code_expr_(expr);
        self.last_printed_item_line = expr.loc().end_pos.line;

        self.backslash_stack.pop();
    }

    fn print_code_expr_(&mut self, expr: &CodeExpr) {
        match expr {
            CodeExpr::BoolLiteral(BoolLiteralExpr { value, loc: _ }) => {
                if *value {
                    stdout_write("true");
                } else {
                    stdout_write("false");
                }
            }
            CodeExpr::CharLiteral(CharLiteralExpr {
                repr,
                value: _,
                loc: _,
            }) => {
                stdout_write(repr);
            }
            CodeExpr::NullLiteral(NullLiteralExpr { loc: _ }) => {
                stdout_write("null");
            }
            CodeExpr::IntLiteral(IntLiteralExpr {
                repr,
                tag,
                value: _,
                loc: _,
            }) => {
                stdout_write(repr);
                if let Some(tag) = tag {
                    stdout_write(tag);
                }
            }
            CodeExpr::StringLiteral(StringLiteralExpr {
                repr,
                value: _,
                loc: _,
            }) => {
                stdout_write(repr);
            }
            CodeExpr::ArrayLiteral(ArrayLiteralExpr {
                item_type,
                items,
                has_trailing_comma,
                loc,
            }) => {
                stdout_write("[");
                self.print_type_expr(item_type);
                stdout_write("]");

                stdout_write("[");
                self.print_expr_list(items, *has_trailing_comma, loc);
                stdout_write("]");
            }
            CodeExpr::ResultLiteral(ResultLiteralExpr {
                is_ok,
                result_type,
                value,
                loc: _,
            }) => {
                if *is_ok {
                    stdout_write("Ok")
                } else {
                    stdout_write("Err")
                }
                if let Some(result_type) = result_type {
                    stdout_write(":<");
                    self.print_type_expr(&result_type.ok);
                    stdout_write(", ");
                    self.print_type_expr(&result_type.err);
                    stdout_write(">");
                }
                stdout_write("(");
                if let Some(value) = value {
                    self.print_code_expr(value);
                }
                stdout_write(")");
            }

            CodeExpr::StructLiteral(StructLiteralExpr {
                struct_name,
                body,
                loc: _,
            }) => {
                stdout_write(&struct_name.repr);
                stdout_write(" ");
                self.print_code_expr_map(body);
            }

            CodeExpr::Ident(IdentExpr {
                repr,
                parts: _,
                loc: _,
            }) => {
                stdout_write(repr);
            }
            CodeExpr::Let(LetExpr {
                is_inline: inline,
                name,
                value,
                loc: _,
            }) => {
                if *inline {
                    stdout_write("inline ");
                }
                stdout_write("let ");
                stdout_write(&name.repr);
                stdout_write(" = ");
                self.print_code_expr(&value);
            }

            CodeExpr::Return(ReturnExpr { expr, loc: _ }) => {
                stdout_write("return");
                if let Some(expr) = expr {
                    stdout_write(" ");
                    self.print_code_expr(expr);
                }
            }
            CodeExpr::InfixOp(InfixOpExpr {
                lhs,
                rhs,
                op_tag: _,
                op_loc,
                loc: _,
            }) => {
                self.print_code_expr(lhs);
                if !self.print_backslashes_before(op_loc.pos.offset) {
                    stdout_write(" ");
                }
                stdout_write(op_loc.read_span(self.parser.lexer.source));
                stdout_write(" ");
                self.print_code_expr(rhs);
            }
            CodeExpr::ExprPipe(ExprPipeExpr {
                lhs,
                rhs,
                op_loc,
                loc: _,
            }) => {
                self.print_code_expr(lhs);
                if !self.print_backslashes_before(op_loc.pos.offset) {
                    stdout_write(" ");
                }
                stdout_write(op_loc.read_span(self.parser.lexer.source));
                stdout_write(" ");
                self.print_code_expr(rhs);
            }
            CodeExpr::PrefixOp(PrefixOpExpr {
                expr,
                op_tag: _,
                op_loc,
                loc: _,
            }) => {
                stdout_write(op_loc.read_span(self.parser.lexer.source));
                self.print_code_expr(expr);
            }
            CodeExpr::If(IfExpr {
                cond,
                then_block,
                else_block,
                loc: _,
            }) => {
                stdout_write("if");
                stdout_write(" ");

                match cond {
                    IfCond::Expr(expr) => {
                        self.print_code_expr(expr);
                    }
                    IfCond::Match(match_header) => {
                        self.print_match_header(match_header);
                    }
                }

                self.print_code_block(then_block);

                match else_block {
                    ElseBlock::None => {}
                    ElseBlock::Else(else_block) => {
                        stdout_write(" ");
                        stdout_write("else");
                        self.print_code_block(&else_block);
                    }
                    ElseBlock::ElseIf(if_expr) => {
                        stdout_write(" ");
                        stdout_write("else");
                        stdout_write(" ");
                        self.print_code_expr(&if_expr);
                    }
                }
            }
            CodeExpr::Match(MatchExpr {
                header,
                else_branch,
                loc: _,
            }) => {
                self.print_match_header(header);
                stdout_write(" else");
                self.print_code_block(else_branch);
            }
            CodeExpr::While(WhileExpr { cond, body, loc: _ }) => {
                if let Some(cond) = cond {
                    stdout_write("while ");
                    self.print_code_expr(cond);
                } else {
                    stdout_write("loop");
                }
                self.print_code_block(&body);
            }
            CodeExpr::For(ForExpr {
                counter,
                start,
                end,
                body,
                op_loc: _,
                loc: _,
            }) => {
                stdout_write("for ");
                stdout_write(&counter.repr);
                stdout_write(" in ");
                self.print_code_expr(&start);
                stdout_write("..");
                self.print_code_expr(&end);
                self.print_code_block(&body);
            }
            CodeExpr::Break(BreakExpr { loc: _ }) => {
                stdout_write("break");
            }
            CodeExpr::Continue(ContinueExpr { loc: _ }) => {
                stdout_write("continue");
            }
            CodeExpr::DoWith(DoWithExpr {
                body,
                args,
                with_loc: _,
                loc,
            }) => {
                stdout_write("do ");
                self.print_code_expr(body);
                stdout_write(" with ");
                self.print_args(args, loc);
            }
            CodeExpr::Defer(DeferExpr { expr, loc: _ }) => {
                stdout_write("defer ");
                self.print_code_expr(expr);
            }
            CodeExpr::Cast(CastExpr {
                expr,
                casted_to,
                loc: _,
            }) => {
                self.print_code_expr(expr);
                stdout_write(" as ");
                self.print_type_expr(casted_to);
            }
            CodeExpr::Assign(AssignExpr {
                op_loc: _,
                lhs,
                rhs,
                loc: _,
            }) => {
                self.print_code_expr(lhs);
                stdout_write(" = ");
                self.print_code_expr(rhs);
            }
            CodeExpr::FieldAccess(FieldAccessExpr {
                lhs,
                field_name,
                loc: _,
            }) => {
                self.print_code_expr(lhs);
                self.print_backslashes_before(field_name.loc.pos.offset);
                stdout_write(".");
                stdout_write(&field_name.repr);
            }
            CodeExpr::Catch(CatchExpr {
                lhs,
                error_bind,
                catch_body,
                catch_loc: _,
                loc: _,
            }) => {
                self.print_code_expr(lhs);
                stdout_write(" catch ");
                stdout_write(&error_bind.repr);
                self.print_code_block(catch_body);
            }
            CodeExpr::Paren(ParenExpr {
                expr,
                has_trailing_comma,
                loc,
            }) => {
                stdout_write("(");
                self.last_printed_item_line = loc.pos.line;

                if *has_trailing_comma {
                    stdout_write("\n");
                    self.indent += 1;
                    self.print_comments_before(expr.loc().pos);
                    self.print_indent();
                }

                self.print_code_expr(expr);

                if *has_trailing_comma {
                    self.indent -= 1;
                    stdout_writeln(",");
                    self.print_indent();
                }
                stdout_write(")");
            }
            CodeExpr::FnCall(FnCallExpr { fn_name, args, loc }) => {
                stdout_write(&fn_name.repr);
                self.print_args(args, loc);
            }
            CodeExpr::MethodCall(MethodCallExpr {
                lhs,
                field_name,
                args,
                loc: _,
            }) => {
                self.print_code_expr(lhs);
                self.print_backslashes_before(field_name.loc.pos.offset);
                stdout_write(".");
                stdout_write(&field_name.repr);
                self.print_args(args, &field_name.loc);
            }
            CodeExpr::InlineFnCall(InlineFnCallExpr {
                fn_name,
                args,
                type_args,
                loc,
            }) => {
                stdout_write(&fn_name.repr);
                self.print_type_args(type_args);
                self.print_args(args, loc);
            }
            CodeExpr::IntrinsicCall(InlineFnCallExpr {
                fn_name,
                args,
                type_args,
                loc,
            }) => {
                stdout_write("@");
                stdout_write(&fn_name.repr);
                self.print_type_args(type_args);
                self.print_args(args, loc);
            }
            CodeExpr::InlineMethodCall(InlineMethodCallExpr {
                lhs,
                field_name,
                args,
                type_args,
                loc: _,
            }) => {
                self.print_code_expr(lhs);
                self.print_backslashes_before(field_name.loc.pos.offset);
                stdout_write(".");
                stdout_write(&field_name.repr);
                self.print_type_args(type_args);
                self.print_args(args, &field_name.loc);
            }
            CodeExpr::Sizeof(SizeofExpr { type_expr, loc: _ }) => {
                stdout_write("sizeof ");
                self.print_type_expr(type_expr);
            }
            CodeExpr::PropagateError(PropagateErrorExpr { expr, loc: _ }) => {
                self.print_code_expr(expr);
                stdout_write("?");
            }
        }
    }

    fn print_code_expr_map(&mut self, map: &CodeExprMap) {
        stdout_write("{");
        self.last_printed_item_line = map.loc.pos.line;

        if map.has_trailing_comma {
            stdout_write("\n");

            self.indent += 1;

            for field in &map.fields {
                self.print_comments_before(field.loc.pos);
                self.print_indent();
                stdout_write(&field.key);
                stdout_write(": ");
                self.print_code_expr(&field.value);
                stdout_writeln(",");
            }

            self.print_comments_before(map.loc.end_pos);

            self.indent -= 1;
            self.print_indent();
            stdout_write("}");

            return;
        }

        if map.fields.len() > 0 {
            stdout_write(" ");
        }

        for i in 0..map.fields.len() {
            if i != 0 {
                stdout_write(", ");
            }

            let field = &map.fields[i];
            stdout_write(&field.key);
            stdout_write(": ");
            self.print_code_expr(&field.value);
        }

        if map.fields.len() > 0 {
            stdout_write(" ");
        }

        stdout_write("}");
    }

    fn print_match_header(&mut self, header: &MatchHeader) {
        stdout_write("match ");
        stdout_write(&header.variant_name.repr);
        stdout_write("(");
        stdout_write(&header.variant_bind.repr);
        stdout_write(") = ");
        self.print_code_expr(&header.expr_to_match);
    }

    fn print_args(&mut self, args: &CodeExprList, open_paren_loc: &Loc) {
        stdout_write("(");
        self.print_expr_list(&args.items, args.has_trailing_comma, open_paren_loc);
        stdout_write(")");
    }

    fn print_expr_list(&mut self, exprs: &Vec<CodeExpr>, is_multiline: bool, start_loc: &Loc) {
        if is_multiline {
            self.indent += 1;
            stdout_write("\n");
            self.last_printed_item_line = start_loc.pos.line;
        }

        let prev_backslashes_printed = self.backslashes_printed;
        for (arg, index) in exprs.iter().zip(0..) {
            if index != 0 {
                stdout_write(",");
            }

            let continues = self.print_double_backslashes_before(arg.loc().pos.offset);

            if is_multiline && !continues {
                if index != 0 {
                    stdout_write("\n");
                }

                self.print_comments_before(arg.loc().pos);
                self.print_indent();
            } else if index != 0 {
                stdout_write(" ");
            }

            self.print_code_expr(arg);
        }
        if !is_multiline && self.backslashes_printed != prev_backslashes_printed {
            stdout_write("\n");
            self.print_indent();
        }

        if is_multiline {
            stdout_writeln(",");
            self.indent -= 1;
            self.print_indent();
        }
    }

    fn print_type_args(&mut self, type_args: &Vec<TypeExpr>) {
        if type_args.len() == 0 {
            return;
        }

        stdout_write("<");
        for (arg, index) in type_args.iter().zip(0..) {
            if index != 0 {
                stdout_write(", ");
            }

            self.print_type_expr(arg);
        }
        stdout_write(">");
    }

    fn print_comments_before(&mut self, pos: Pos) {
        while self.comments_printed < self.parser.lexer.comments.len() {
            let comment = self.parser.lexer.comments[self.comments_printed].relax();
            if comment.end_pos.offset > pos.offset {
                break;
            }

            self.print_blank_line_before(comment.pos.line);
            self.last_printed_item_line = comment.end_pos.line;

            self.print_indent();
            stdout_writeln(&comment.read_span(self.parser.lexer.source));
            self.comments_printed += 1;
        }

        self.print_blank_line_before(pos.line);
        self.last_printed_item_line = pos.line;
    }

    fn print_blank_line_before(&mut self, line: usize) {
        let lines_since_last_item = line - self.last_printed_item_line;
        if lines_since_last_item > 1 {
            stdout_write("\n");
        }
    }

    fn print_backslashes_before(&mut self, offset: usize) -> bool {
        let mut printed = false;

        while self.backslashes_printed < self.parser.lexer.backslashes.len() {
            let backslash = &self.parser.lexer.backslashes[self.backslashes_printed];
            if backslash.end_pos.offset > offset {
                break;
            }

            self.backslashes_printed += 1;

            if !printed {
                printed = true;

                *self.backslash_stack.last_mut().unwrap() = true;
                self.last_stmt_had_backslash = true;

                stdout_writeln(" \\");
                self.print_indent();
            }
        }

        printed
    }

    fn print_double_backslashes_before(&mut self, offset: usize) -> bool {
        let mut printed = false;

        while self.double_backslashes_printed < self.parser.lexer.double_backslashes.len() {
            let dbs = &self.parser.lexer.double_backslashes[self.double_backslashes_printed];
            if dbs.end_pos.offset > offset {
                break;
            }

            self.double_backslashes_printed += 1;

            if !printed {
                printed = true;

                stdout_write(" \\\\");
            }
        }

        printed
    }

    fn print_indent(&self) {
        for _ in 0..self.indent {
            stdout_write("    ");
        }

        for indent in &self.backslash_stack {
            if *indent {
                stdout_write("    ");
            }
        }
    }
}
