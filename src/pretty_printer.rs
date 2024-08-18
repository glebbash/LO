use alloc::{boxed::Box, vec::Vec};

use crate::{ast::*, core::*, lexer::Comment};

pub struct PrettyPrinter {
    indent: usize,
    comments: Box<Vec<Comment>>,
    comments_printed: usize,
}

impl PrettyPrinter {
    pub fn print(ast: Box<AST>) {
        let mut printer = PrettyPrinter {
            indent: 0,
            comments: Box::new(ast.comments),
            comments_printed: 0,
        };

        printer.print_file(&ast.exprs);
    }

    fn print_file(&mut self, exprs: &Vec<TopLevelExpr>) {
        for expr in exprs {
            self.print_top_level_expr(expr);
            stdout_writeln(";");
            stdout_writeln("");
        }

        for comment in self.comments.iter() {
            stdout_writeln(&comment.content);
        }
        self.comments_printed += self.comments.len();
        stdout_writeln("");
    }

    fn print_top_level_expr(&mut self, expr: &TopLevelExpr) {
        match expr {
            TopLevelExpr::FnDef(fn_def) => self.print_fn_def(fn_def),
        }
    }

    fn print_fn_def(&mut self, fn_def: &FnDefExpr) {
        if fn_def.exported {
            stdout_write("export ");
        }

        stdout_write("fn ");

        stdout_write(&fn_def.fn_name);

        stdout_write("(): ");

        self.print_type(&fn_def.return_type);

        stdout_write(" ");

        self.print_code_block_expr(&fn_def.body);
    }

    fn print_type(&mut self, type_expr: &TypeExpr) {
        match type_expr {
            TypeExpr::U32 => stdout_write("u32"),
        }
    }

    fn print_code_block_expr(&mut self, code_block: &CodeBlockExpr) {
        stdout_writeln("{");

        self.indent += 1;

        for expr in &code_block.exprs {
            self.print_indent();
            self.print_code_expr(expr);
            stdout_writeln(";");
        }

        self.indent -= 1;

        stdout_write("}");
    }

    fn print_code_expr(&mut self, expr: &CodeExpr) {
        match expr {
            CodeExpr::Return(ReturnExpr { expr }) => {
                stdout_write("return ");
                self.print_code_expr(&expr);
            }
            CodeExpr::IntLiteral(IntLiteralExpr { value }) => {
                stdout_write(value);
            }
        }
    }

    fn print_indent(&mut self) {
        stdout_write(" ".repeat(self.indent * 4));
    }
}
