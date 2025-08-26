use crate::{ast::*, core::*};
use alloc::{string::ToString, vec::Vec};

pub struct Printer {
    ast: UBox<AST>,
    source: UBox<[u8]>,
    indent: usize,
    comments_printed: usize,
    backslashes_printed: usize,
    is_multiline_stmt: bool,
    multiline_stmt_indent: usize,
    last_printed_item_line: usize,
}

impl Printer {
    pub fn print(ast: UBox<AST>, source: UBox<[u8]>) {
        let mut printer = Printer {
            ast,
            source,
            indent: 0,
            comments_printed: 0,
            backslashes_printed: 0,
            is_multiline_stmt: false,
            multiline_stmt_indent: 0,
            last_printed_item_line: 1,
        };

        stdout_enable_buffering();
        printer.print_file();
        stdout_disable_buffering();
    }

    fn print_file(&mut self) {
        for expr in &self.ast.clone().exprs {
            self.print_top_level_expr(expr);
        }

        // print the rest of the comments
        self.print_comments_before(LoPosition {
            offset: usize::MAX,
            line: self.last_printed_item_line,
            col: usize::MAX,
        });
    }

    fn print_top_level_expr(&mut self, expr: &TopLevelExpr) {
        self.print_comments_before(expr.loc().pos);

        match &expr {
            TopLevelExpr::FnDef(FnDefExpr {
                exported,
                decl,
                body,
                loc: _,
            }) => {
                if *exported {
                    stdout_write("export ");
                }
                self.print_fn_decl(decl);
                stdout_write(" ");
                self.print_code_block_expr(body);
                stdout_writeln("");
            }
            TopLevelExpr::Include(IncludeExpr {
                file_path,
                alias,
                loc: _,
            }) => {
                stdout_write("include ");
                stdout_write(file_path.get_raw(self.source));
                if let Some(alias) = alias {
                    stdout_write(" as ");
                    stdout_write(&alias.repr);
                }
                stdout_writeln("");
            }
            TopLevelExpr::Import(ImportExpr {
                module_name,
                items,
                loc,
            }) => {
                stdout_write("import from ");
                stdout_write(module_name.get_raw(self.source));
                stdout_write(" {\n");
                self.indent += 1;

                for item in items {
                    self.print_comments_before(item.loc().pos);
                    self.print_indent();
                    match item {
                        ImportItem::FnDecl(decl) => self.print_fn_decl(decl),
                        ImportItem::Memory(memory_def) => self.print_memory_def(memory_def),
                    }
                    stdout_writeln("");
                }

                // print the rest of the comments
                self.print_comments_before(loc.end_pos);

                self.indent -= 1;
                self.print_indent();
                stdout_write("}");
                stdout_writeln("");
            }
            TopLevelExpr::GlobalDef(GlobalDefExpr {
                global_name,
                global_value,
                loc: _,
            }) => {
                stdout_write("global ");
                stdout_write(&global_name.repr);
                stdout_write(" = ");
                match global_value {
                    GlobalDefValue::Expr(expr) => self.print_code_expr(expr),
                    GlobalDefValue::DataSize => stdout_write("@data_size"),
                }
                stdout_writeln("");
            }
            TopLevelExpr::StructDef(StructDefExpr {
                struct_name,
                fields,
                loc,
            }) => {
                stdout_write("struct ");
                stdout_write(&struct_name.repr);

                if fields.len() == 0 {
                    stdout_write(" {}");
                    stdout_writeln("");
                } else {
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
                    stdout_writeln("");
                }
            }
            TopLevelExpr::TypeDef(TypeDefExpr {
                type_name,
                type_value,
                loc: _,
            }) => {
                stdout_write("type ");
                stdout_write(&type_name.repr);
                stdout_write(" = ");
                self.print_type_expr(type_value);
                stdout_writeln("");
            }
            TopLevelExpr::ConstDef(ConstDefExpr {
                const_name,
                const_value,
                loc: _,
            }) => {
                stdout_write("const ");
                stdout_write(&const_name.repr);
                stdout_write(" = ");
                self.print_code_expr(const_value);
                stdout_writeln("");
            }
            TopLevelExpr::MemoryDef(memory_def) => {
                self.print_memory_def(memory_def);
                stdout_writeln("");
            }
            TopLevelExpr::StaticDataStore(StaticDataStoreExpr { addr, data, loc: _ }) => {
                stdout_write("*");
                self.print_code_expr(addr);
                stdout_write(" = ");
                match data {
                    StaticDataStorePayload::String { value } => {
                        stdout_write(value.get_raw(self.source));
                    }
                }
                stdout_writeln("");
            }
            TopLevelExpr::ExportExistingFn(ExportExistingFnExpr {
                in_fn_name,
                out_fn_name,
                loc: _,
            }) => {
                stdout_write("export existing fn ");
                stdout_write(&in_fn_name.repr);
                stdout_write(" as ");
                stdout_write(out_fn_name.get_raw(self.source));
                stdout_writeln("");
            }
            TopLevelExpr::MacroDef(MacroDefExpr {
                macro_name,
                macro_params,
                macro_type_params,
                return_type,
                body,
                loc: _,
            }) => {
                stdout_write("macro ");
                stdout_write(&macro_name.repr);
                stdout_write("!");
                if macro_type_params.len() != 0 {
                    stdout_write("<");
                    for (type_param, i) in macro_type_params.iter().zip(0..) {
                        stdout_write(type_param);
                        if i != macro_type_params.len() - 1 {
                            stdout_write(",");
                        }
                    }
                    stdout_write(">");
                }
                self.print_fn_params(macro_params);
                if let Some(return_type) = return_type {
                    stdout_write(": ");
                    self.print_type_expr(return_type);
                }
                stdout_write(" ");
                self.print_code_block_expr(body);
                stdout_writeln("");
            }
        }

        self.last_printed_item_line = expr.loc().end_pos.line;
    }

    // TODO: figure out multiline param printing
    fn print_fn_decl(&mut self, fn_decl: &FnDeclExpr) {
        stdout_write("fn ");
        stdout_write(&fn_decl.fn_name.repr);
        self.print_fn_params(&fn_decl.fn_params);

        let Some(return_type) = &fn_decl.return_type else {
            return;
        };

        stdout_write(": ");
        self.print_type_expr(&return_type);
    }

    fn print_fn_params(&mut self, fn_params: &Vec<FnParam>) {
        stdout_write("(");
        for (fn_param, index) in fn_params.iter().zip(0..) {
            if index != 0 {
                stdout_write(", ");
            }

            match &fn_param.param_type {
                FnParamType::Self_ => {
                    stdout_write(&fn_param.param_name.repr);
                }
                FnParamType::SelfRef => {
                    stdout_write("&");
                    stdout_write(&fn_param.param_name.repr);
                }
                FnParamType::Type { expr } => {
                    stdout_write(&fn_param.param_name.repr);
                    stdout_write(": ");
                    self.print_type_expr(&expr);
                }
                FnParamType::Infer { name } => {
                    stdout_write(&fn_param.param_name.repr);
                    stdout_write(": infer ");
                    stdout_write(name);
                }
            }
        }
        stdout_write(")");
    }

    fn print_memory_def(&mut self, mem: &MemoryDefExpr) {
        if mem.exported {
            stdout_write("export ");
        }
        if mem.loc.pos.line == mem.loc.end_pos.line {
            return stdout_write("memory {}");
        }

        stdout_writeln("memory {");
        self.last_printed_item_line = mem.loc.end_pos.line;

        self.indent += 1;
        if let Some(min_pages) = mem.min_pages {
            self.print_indent();
            stdout_write("min_pages: ");
            stdout_write(min_pages.to_string());
            stdout_writeln(",");
        }
        if let Some(data_start) = mem.data_start {
            self.print_indent();
            stdout_write("data_start: ");
            stdout_write(data_start.to_string());
            stdout_writeln(",");
        }
        self.indent -= 1;
        self.print_indent();
        stdout_write("}");
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
            TypeExpr::Result(TypeExprResult {
                ok_type,
                err_type,
                loc: _,
            }) => {
                stdout_write("Result<");
                self.print_type_expr(ok_type);
                stdout_write(", ");
                self.print_type_expr(err_type);
                stdout_write(">");
            }
            TypeExpr::Of(TypeExprOf {
                container_type,
                item_type,
                loc: _,
            }) => {
                self.print_type_expr(container_type);
                stdout_write(" of ");
                self.print_type_expr(item_type);
            }
        }
    }

    fn print_code_block_expr(&mut self, code_block: &CodeBlockExpr) {
        if code_block.loc.pos.line == code_block.loc.end_pos.line {
            return stdout_write("{}");
        }

        stdout_writeln("{");
        self.last_printed_item_line = code_block.loc.pos.line;

        self.indent += 1;

        for expr in &code_block.exprs {
            self.print_comments_before(expr.loc().pos);
            self.print_indent();
            self.print_code_expr(expr);
            stdout_writeln("");
            self.is_multiline_stmt = false;
            self.multiline_stmt_indent = 0;
        }

        // print the rest of the comments
        self.print_comments_before(code_block.loc.end_pos);

        self.indent -= 1;

        self.print_indent();
        stdout_write("}");
    }

    fn print_code_expr(&mut self, expr: &CodeExpr) {
        self.print_backslashes_before(expr.loc().pos.offset);

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
                zero_terminated,
                value: _,
                loc: _,
            }) => {
                stdout_write(repr);
                if *zero_terminated {
                    stdout_write("0");
                }
            }
            CodeExpr::ArrayLiteral(ArrayLiteralExpr {
                item_type,
                items,
                loc,
            }) => {
                stdout_write("[");
                self.print_type_expr(item_type);
                stdout_write("]");
                stdout_writeln("[");
                self.indent += 1;
                for item in items {
                    self.print_comments_before(item.loc().pos);
                    self.print_indent();
                    self.print_code_expr(item);
                    stdout_writeln(",");
                }
                // print the rest of the comments
                self.print_comments_before(loc.end_pos);
                self.indent -= 1;
                self.print_indent();
                stdout_write("]");
            }
            CodeExpr::ResultLiteral(ResultLiteralExpr {
                is_ok,
                result_type,
                value,
                loc: _,
            }) => {
                stdout_write(if *is_ok { "Ok" } else { "Err" });
                if let Some(result_type) = result_type {
                    stdout_write("::<");
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

            CodeExpr::Ident(IdentExpr {
                repr,
                parts: _,
                loc: _,
            }) => {
                stdout_write(repr);
            }
            CodeExpr::Let(LetExpr {
                local_name,
                value,
                loc: _,
            }) => {
                stdout_write("let ");
                stdout_write(&local_name.repr);
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
                op_tag,
                op_loc: _,
                lhs,
                rhs,
                loc: _,
            }) => {
                self.print_code_expr(lhs);
                stdout_write(" ");
                stdout_write(op_tag.to_str());
                stdout_write(" ");
                self.print_code_expr(rhs);
            }
            CodeExpr::PrefixOp(PrefixOpExpr {
                expr,
                op_tag,
                op_loc: _,
                loc: _,
            }) => {
                stdout_write(op_tag.to_str());
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
                self.print_code_expr(cond);
                stdout_write(" ");
                self.print_code_block_expr(then_block);
                match else_block {
                    ElseBlock::None => {}
                    ElseBlock::Else(else_block) => {
                        stdout_write(" ");
                        stdout_write("else");
                        stdout_write(" ");
                        self.print_code_block_expr(&else_block);
                    }
                    ElseBlock::ElseIf(if_expr) => {
                        stdout_write(" ");
                        stdout_write("else");
                        stdout_write(" ");
                        self.print_code_expr(&if_expr);
                    }
                }
            }
            CodeExpr::Loop(LoopExpr { body, loc: _ }) => {
                stdout_write("loop ");
                self.print_code_block_expr(&body);
            }
            CodeExpr::ForLoop(ForLoopExpr {
                counter,
                start,
                end,
                body,
                loc: _,
            }) => {
                stdout_write("for ");
                stdout_write(&counter.repr);
                stdout_write(" in ");
                self.print_code_expr(&start);
                stdout_write("..");
                self.print_code_expr(&end);
                stdout_write(" ");
                self.print_code_block_expr(&body);
            }
            CodeExpr::Break(BreakExpr { loc: _ }) => {
                stdout_write("break");
            }
            CodeExpr::Continue(ContinueExpr { loc: _ }) => {
                stdout_write("continue");
            }
            CodeExpr::With(WithExpr {
                bind,
                args,
                body,
                loc: _,
            }) => {
                stdout_write("with ");
                stdout_write(&bind.repr);
                stdout_write(" in ");
                self.print_args(args);
                stdout_write(" do ");
                self.print_code_block_expr(body);
            }
            CodeExpr::Unreachable(UnreachableExpr { loc: _ }) => {
                stdout_write("unreachable");
            }
            CodeExpr::Dbg(DbgExpr {
                message,
                message_unescaped: _,
                loc: _,
            }) => {
                stdout_write("dbg ");
                stdout_write(message.get_raw(self.source));
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
            CodeExpr::StructLiteral(StructLiteralExpr {
                struct_name,
                fields,
                has_trailing_comma,
                loc,
            }) => {
                stdout_write(".");
                stdout_write(&struct_name.repr);
                stdout_write(" {");
                self.last_printed_item_line = loc.pos.line;

                if *has_trailing_comma {
                    stdout_writeln("");

                    self.indent += 1;

                    for field in fields {
                        self.print_comments_before(field.loc.pos);
                        self.print_indent();
                        stdout_write(&field.field_name);
                        stdout_write(": ");
                        self.print_code_expr(&field.value);
                        stdout_writeln(",");
                    }

                    // print the rest of the comments
                    self.print_comments_before(loc.end_pos);

                    self.indent -= 1;
                    self.print_indent();
                    stdout_write("}");
                    return;
                }

                if fields.len() == 0 {
                    stdout_write("}");
                    return;
                }

                stdout_write(" ");

                for i in 0..fields.len() {
                    if i != 0 {
                        stdout_write(", ");
                    }

                    let field = &fields[i];
                    stdout_write(&field.field_name);
                    stdout_write(": ");
                    self.print_code_expr(&field.value);
                }

                stdout_write(" }");
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
                stdout_write(".");
                stdout_write(&field_name.repr);
            }
            CodeExpr::Catch(CatchExpr {
                lhs,
                error_bind,
                catch_body,
                loc: _,
            }) => {
                self.print_code_expr(lhs);
                stdout_write(" catch ");
                stdout_write(&error_bind.repr);
                stdout_write(" ");
                self.print_code_block_expr(catch_body);
            }
            CodeExpr::Paren(ParenExpr { expr, loc: _ }) => {
                stdout_write("(");
                let prev_backslashes_printed = self.backslashes_printed;
                self.multiline_stmt_indent += 1;
                self.print_code_expr(expr);
                self.multiline_stmt_indent -= 1;
                if self.backslashes_printed != prev_backslashes_printed {
                    stdout_writeln("");
                    self.print_indent();
                }
                stdout_write(")");
            }
            // TODO: figure out multiline arg printing
            CodeExpr::FnCall(FnCallExpr {
                fn_name,
                args,
                loc: _,
            }) => {
                stdout_write(&fn_name.repr);
                self.print_args(args);
            }
            CodeExpr::MethodCall(MethodCallExpr {
                lhs,
                field_name,
                args,
                loc: _,
            }) => {
                self.print_code_expr(lhs);
                stdout_write(".");
                stdout_write(&field_name.repr);
                self.print_args(args);
            }
            CodeExpr::MacroFnCall(MacroFnCallExpr {
                fn_name,
                args,
                type_args,
                loc: _,
            }) => {
                stdout_write(&fn_name.repr);
                stdout_write("!");
                self.print_type_args(type_args);
                self.print_args(args);
            }
            CodeExpr::MacroMethodCall(MacroMethodCallExpr {
                lhs,
                field_name,
                args,
                type_args,
                loc: _,
            }) => {
                self.print_code_expr(lhs);
                stdout_write(".");
                stdout_write(&field_name.repr);
                stdout_write("!");
                self.print_type_args(type_args);
                self.print_args(args);
            }
            CodeExpr::Sizeof(SizeofExpr { type_expr, loc: _ }) => {
                stdout_write("sizeof ");
                self.print_type_expr(type_expr);
            }
            CodeExpr::PropagateError(PropagateErrorExpr { expr, loc: _ }) => {
                self.print_code_expr(expr);
                stdout_write("?");
            }
            CodeExpr::MemorySize(MemorySizeExpr { loc: _ }) => {
                stdout_write("__memory_size()");
            }
            CodeExpr::MemoryGrow(MemoryGrowExpr { args, loc: _ }) => {
                stdout_write("__memory_grow");
                self.print_args(args);
            }
            CodeExpr::MemoryCopy(MemoryCopyExpr { args, loc: _ }) => {
                stdout_write("__memory_copy");
                self.print_args(args);
            }
        }

        self.last_printed_item_line = expr.loc().end_pos.line
    }

    fn print_args(&mut self, args: &Vec<CodeExpr>) {
        stdout_write("(");
        let prev_backslashes_printed = self.backslashes_printed;
        self.multiline_stmt_indent += 1;
        for (arg, index) in args.iter().zip(0..) {
            if index != 0 {
                stdout_write(", ");
            }

            self.print_code_expr(arg);
        }
        self.multiline_stmt_indent -= 1;
        if self.backslashes_printed != prev_backslashes_printed {
            stdout_writeln(",");
            self.print_indent();
        }
        stdout_write(")");
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

    fn print_comments_before(&mut self, pos: LoPosition) {
        while self.comments_printed < self.ast.comments.len() {
            let comment = UBox::relax(&self.ast.comments[self.comments_printed]);
            if comment.loc.end_pos.offset > pos.offset {
                break;
            }

            self.print_blank_line_before(comment.loc.pos.line);
            self.last_printed_item_line = comment.loc.end_pos.line;

            self.print_indent();
            stdout_writeln(&comment.loc.read_span(self.source));
            self.comments_printed += 1;
        }

        self.print_blank_line_before(pos.line);
        self.last_printed_item_line = pos.line;
    }

    fn print_blank_line_before(&mut self, line: usize) {
        let lines_since_last_item = line - self.last_printed_item_line;
        if lines_since_last_item > 1 {
            stdout_writeln("");
        }
    }

    fn print_backslashes_before(&mut self, offset: usize) {
        while self.backslashes_printed < self.ast.backslashes.len() {
            let backslash = &self.ast.backslashes[self.backslashes_printed];
            if backslash.loc.end_pos.offset > offset {
                break;
            }

            // implicit indent
            if !self.is_multiline_stmt && self.multiline_stmt_indent == 0 {
                self.multiline_stmt_indent = 1;
            }

            self.is_multiline_stmt = true;
            self.backslashes_printed += 1;

            stdout_writeln("");
            self.print_indent();
            stdout_write("\\ ");
        }
    }

    fn print_indent(&self) {
        for _ in 0..self.indent {
            stdout_write("    ");
        }
        if self.is_multiline_stmt {
            for _ in 0..self.multiline_stmt_indent {
                stdout_write("    ");
            }
        }
    }
}
