use core::usize;

use crate::{ast::*, core::*};
use alloc::{rc::Rc, vec::Vec};

pub struct Printer {
    ast: Rc<AST>,
    indent: usize,
    comments_printed: usize,
}

impl Printer {
    pub fn print(ast: Rc<AST>) {
        let mut printer = Printer {
            ast,
            indent: 0,
            comments_printed: 0,
        };

        printer.print_file();
    }

    // TODO: print all function declarations first in C mode
    fn print_file(&mut self) {
        for (expr, i) in self.ast.clone().exprs.iter().zip(0..) {
            self.print_comments_before_pos(expr.loc().pos.offset);
            self.print_top_level_expr(expr, i);
        }

        // print the rest of the comments
        self.print_comments_before_pos(usize::MAX);
    }

    fn print_top_level_expr(&mut self, expr: &TopLevelExpr, expr_index: usize) {
        match &expr {
            TopLevelExpr::FnDef(fn_def) => {
                self.print_fn_def(fn_def);
            }
            TopLevelExpr::Include(include) => {
                self.print_include(include);

                if let Some(TopLevelExpr::Include(_)) = self.ast.exprs.get(expr_index + 1) {
                    return;
                }
            }
            TopLevelExpr::Import(import) => {
                stdout_write("import from ");
                stdout_write(&import.module_name);
                stdout_write(" {\n");
                self.indent += 1;

                for item in &import.items {
                    self.print_comments_before_pos(item.loc().pos.offset);
                    self.print_indent();
                    match item {
                        ImportItem::FnDecl(decl) => self.print_fn_decl(decl),
                    }
                    stdout_writeln(";");
                }

                // print the rest of the comments
                self.print_comments_before_pos(import.loc.end_pos.offset);

                self.indent -= 1;
                self.print_indent();
                stdout_writeln("};");
            }
            TopLevelExpr::GlobalDef(global) => {
                stdout_write("global ");
                stdout_write(&global.global_name);
                stdout_write(" = ");
                self.print_code_expr(&global.expr);
                stdout_writeln(";");
            }
            TopLevelExpr::StructDef(struct_def) => {
                stdout_write("struct ");
                stdout_write(&struct_def.struct_name.repr);

                if struct_def.fields.len() == 0 {
                    stdout_writeln(" {};");
                } else {
                    stdout_writeln(" {");
                    self.indent += 1;
                    for field in &struct_def.fields {
                        self.print_comments_before_pos(field.loc.pos.offset);
                        self.print_indent();
                        stdout_write(&field.field_name);
                        stdout_write(": ");
                        self.print_type_expr(&field.field_type);
                        stdout_writeln(",");
                    }

                    // print the rest of the comments
                    self.print_comments_before_pos(struct_def.loc.end_pos.offset);

                    self.indent -= 1;
                    self.print_indent();

                    stdout_writeln("};");
                }
            }
            TopLevelExpr::TypeDef(type_def) => {
                stdout_write("type ");
                stdout_write(&type_def.type_name.repr);
                stdout_write(" = ");
                self.print_type_expr(&type_def.type_value);
                stdout_writeln(";");
            }
            TopLevelExpr::ConstDef(const_def) => {
                stdout_write("const ");
                stdout_write(&const_def.const_name.repr);
                stdout_write(" = ");
                self.print_code_expr(&const_def.const_value);
                stdout_writeln(";");
            }
        }

        if expr_index != self.ast.exprs.len() - 1 {
            stdout_writeln("");
        }
    }

    fn print_fn_def(&mut self, fn_def: &FnDefExpr) {
        if fn_def.exported {
            stdout_write("export ");
        }
        self.print_fn_decl(&fn_def.decl);
        stdout_write(" ");
        self.print_code_block_expr(&fn_def.body);
        stdout_writeln(";");
    }

    // TODO: figure out multiline param printing
    fn print_fn_decl(&mut self, fn_decl: &FnDeclExpr) {
        stdout_write("fn ");
        stdout_write(&fn_decl.fn_name.repr);
        stdout_write("(");
        for (fn_param, index) in fn_decl.fn_params.iter().zip(0..) {
            if index != 0 {
                stdout_write(", ");
            }

            stdout_write(&fn_param.name);
            if let Some(param_type_expr) = &fn_param.type_ {
                stdout_write(": ");
                self.print_type_expr(param_type_expr);
            }
        }
        stdout_write(")");

        let Some(return_type) = &fn_decl.return_type else {
            return;
        };

        stdout_write(": ");
        self.print_type_expr(&return_type);
    }

    fn print_include(&mut self, include: &IncludeExpr) {
        stdout_write("include ");
        stdout_write(&include.file_path);
        stdout_writeln(";");
    }

    fn print_type_expr(&mut self, type_expr: &TypeExpr) {
        match type_expr {
            TypeExpr::U32 => stdout_write("u32"),
            TypeExpr::AliasOrStruct { name } => stdout_write(&name.repr),
            TypeExpr::Result { ok_type, err_type } => {
                stdout_write("Result<");
                self.print_type_expr(&ok_type);
                stdout_write(", ");
                self.print_type_expr(&err_type);
                stdout_write(">");
            }
        }
    }

    fn print_code_block_expr(&mut self, code_block: &CodeBlockExpr) {
        stdout_writeln("{");

        self.indent += 1;

        for expr in &code_block.exprs {
            self.print_comments_before_pos(expr.loc().pos.offset);
            self.print_indent();
            self.print_code_expr(expr);
            stdout_writeln(";");
        }

        // print the rest of the comments
        self.print_comments_before_pos(code_block.loc.end_pos.offset);

        self.indent -= 1;

        self.print_indent();
        stdout_write("}");
    }

    fn print_code_expr(&mut self, expr: &CodeExpr) {
        match expr {
            CodeExpr::Return(ReturnExpr { expr, .. }) => {
                stdout_write("return");
                if let Some(expr) = expr {
                    stdout_write(" ");
                    self.print_code_expr(expr);
                }
            }
            CodeExpr::IntLiteral(IntLiteralExpr { repr, .. }) => {
                stdout_write(repr);
            }
            CodeExpr::StringLiteral(StringLiteralExpr { repr, .. }) => {
                stdout_write(repr);
            }
            CodeExpr::Ident(IdentExpr { repr: name, .. }) => {
                stdout_write(name);
            }
            CodeExpr::BinaryOp(BinaryOpExpr {
                op_tag, lhs, rhs, ..
            }) => {
                self.print_code_expr(lhs);
                stdout_write(" ");
                stdout_write(op_tag.to_str());
                stdout_write(" ");
                self.print_code_expr(rhs);
            }
            CodeExpr::If(IfExpr {
                cond,
                then_block,
                else_block,
                ..
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
            CodeExpr::BoolLiteral(BoolLiteralExpr { value, .. }) => {
                if *value {
                    stdout_write("true");
                } else {
                    stdout_write("false");
                }
            }
            // TODO: figure out multiline arg printing
            CodeExpr::FnCall(FnCallExpr {
                fn_name: ident,
                args,
                ..
            }) => {
                stdout_write(&ident.repr);
                self.print_args(args);
            }
            CodeExpr::Local(LocalExpr {
                local_name, value, ..
            }) => {
                stdout_write("let ");
                stdout_write(local_name);
                stdout_write(" = ");
                self.print_code_expr(&value);
            }
            CodeExpr::Loop(LoopExpr { body, .. }) => {
                stdout_write("loop ");
                self.print_code_block_expr(&body);
            }
            CodeExpr::ForLoop(ForLoopExpr {
                counter,
                start,
                end,
                body,
                ..
            }) => {
                stdout_write("for ");
                stdout_write(counter);
                stdout_write(" in ");
                self.print_code_expr(&start);
                stdout_write("..");
                self.print_code_expr(&end);
                stdout_write(" ");
                self.print_code_block_expr(&body);
            }
            CodeExpr::Break(BreakExpr { .. }) => {
                stdout_write("break");
            }
            CodeExpr::Continue(ContinueExpr { .. }) => {
                stdout_write("continue");
            }
            CodeExpr::Dbg(DbgExpr { message, .. }) => {
                stdout_write("dbg ");
                stdout_write(message);
            }
            CodeExpr::Defer(DeferExpr { expr, .. }) => {
                stdout_write("defer ");
                self.print_code_expr(expr);
            }
            CodeExpr::Cast(CastExpr {
                expr, casted_to, ..
            }) => {
                self.print_code_expr(expr);
                stdout_write(" as ");
                self.print_type_expr(casted_to);
            }
            CodeExpr::StructInit(StructInitExpr {
                struct_name,
                fields,
                loc,
            }) => {
                stdout_write(&struct_name.repr);
                stdout_writeln(" {");
                self.indent += 1;
                for field in fields {
                    self.print_indent();
                    stdout_write(&field.field_name);
                    stdout_write(": ");
                    self.print_code_expr(&field.value);
                    stdout_writeln(",");
                }

                // print the rest of the comments
                self.print_comments_before_pos(loc.end_pos.offset);

                self.indent -= 1;
                self.print_indent();

                stdout_write("}");
            }
            CodeExpr::Assign(AssignExpr { lhs, rhs, .. }) => {
                self.print_code_expr(lhs);
                stdout_write(" = ");
                self.print_code_expr(rhs);
            }
            CodeExpr::FieldAccess(FieldAccessExpr {
                lhs, field_name, ..
            }) => {
                self.print_code_expr(lhs);
                stdout_write(".");
                stdout_write(&field_name.repr);
            }
            CodeExpr::MethodCall(MethodCallExpr {
                lhs,
                field_name,
                args,
                ..
            }) => {
                self.print_code_expr(lhs);
                stdout_write(".");
                stdout_write(&field_name.repr);
                self.print_args(args);
            }
            CodeExpr::Catch(CatchExpr {
                lhs,
                error_bind,
                catch_body,
                ..
            }) => {
                self.print_code_expr(lhs);
                stdout_write(" catch ");
                stdout_write(error_bind);
                stdout_write(" ");
                self.print_code_block_expr(catch_body);
            }
        }
    }

    fn print_args(&mut self, args: &Vec<CodeExpr>) {
        stdout_write("(");
        for (arg, index) in args.iter().zip(0..) {
            if index != 0 {
                stdout_write(", ");
            }

            self.print_code_expr(arg);
        }
        stdout_write(")");
    }

    fn print_comments_before_pos(&mut self, offset: usize) {
        while self.comments_printed < self.ast.comments.len() {
            let comment = &self.ast.comments[self.comments_printed];
            if comment.loc.end_pos.offset > offset {
                break;
            }

            self.print_indent();
            stdout_writeln(&comment.content);
            self.comments_printed += 1;
        }
    }

    fn print_indent(&self) {
        stdout_write(" ".repeat(self.indent * 4));
    }
}
