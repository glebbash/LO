use crate::{ast::*, core::*, lexer::Comment};
use alloc::vec::Vec;

use PrintFormat::*;

#[derive(PartialEq)]
pub enum PrintFormat {
    PrettyPrint,
    TranspileToC,
}

pub struct Printer {
    format: PrintFormat,
    indent: usize,
    comments: Vec<Comment>,
    comments_printed: usize,
}

impl Printer {
    pub fn print(ast: AST, format: PrintFormat) {
        let mut printer = Printer {
            format,
            indent: 0,
            comments: ast.comments,
            comments_printed: 0,
        };

        printer.print_file(&ast.exprs);
    }

    fn print_file(&mut self, exprs: &Vec<TopLevelExpr>) {
        if self.format == TranspileToC {
            stdout_writeln("#pragma once");
        }

        for expr in exprs {
            self.print_comments_before_pos(&expr.loc().pos);
            self.print_top_level_expr(expr);

            if self.should_print_semi_top_level(expr) {
                stdout_writeln(";");
            } else {
                stdout_writeln("");
            }
            stdout_writeln("");
        }

        // print the rest of the comments
        if self.comments_printed != self.comments.len() {
            for comment in self.comments.iter().skip(self.comments_printed) {
                stdout_writeln(&comment.content);
            }
            stdout_writeln("");
        }
    }

    fn print_top_level_expr(&mut self, expr: &TopLevelExpr) {
        match expr {
            TopLevelExpr::FnDef(fn_def) => self.print_fn_def(fn_def),
            TopLevelExpr::Include(include) => self.print_include(include),
        }
    }

    fn print_fn_def(&mut self, fn_def: &FnDefExpr) {
        if self.format == TranspileToC {
            self.print_type(&fn_def.return_type);
            stdout_write(" ");
            stdout_write(&fn_def.fn_name);
            stdout_write("()");
            stdout_writeln("");
            self.print_code_block_expr(&fn_def.body);
            return;
        }

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

    fn print_include(&mut self, include: &IncludeExpr) {
        if self.format == TranspileToC {
            stdout_write("#");
        }
        stdout_write("include ");
        stdout_write("\"");
        stdout_write(&include.file_path);
        stdout_write("\"");
    }

    fn print_type(&mut self, type_expr: &TypeExpr) {
        if self.format == TranspileToC {
            match type_expr {
                TypeExpr::U32 => stdout_write("unsigned int"),
            }
            return;
        }

        match type_expr {
            TypeExpr::U32 => stdout_write("u32"),
        }
    }

    fn print_code_block_expr(&mut self, code_block: &CodeBlockExpr) {
        stdout_writeln("{");

        self.indent += 1;

        for expr in &code_block.exprs {
            self.print_comments_before_pos(&expr.loc().pos);
            self.print_indent();
            self.print_code_expr(expr);

            if self.should_print_semi(expr) {
                stdout_writeln(";");
            } else {
                stdout_writeln("");
            }
        }

        // print the rest of the comments
        self.print_comments_before_pos(&code_block.loc.end_pos);

        self.indent -= 1;

        stdout_write("}");
    }

    fn print_code_expr(&mut self, expr: &CodeExpr) {
        match expr {
            CodeExpr::Return(ReturnExpr { expr, .. }) => {
                stdout_write("return ");
                self.print_code_expr(&expr);
            }
            CodeExpr::IntLiteral(IntLiteralExpr { value, .. }) => {
                stdout_write(value);
            }
        }
    }

    fn print_comments_before_pos(&mut self, pos: &LoPosition) {
        while self.comments_printed < self.comments.len() {
            let comment = &self.comments[self.comments_printed];
            if comment.loc.end_pos.offset > pos.offset {
                break;
            }

            self.print_indent();
            stdout_writeln(&comment.content);
            self.comments_printed += 1;
        }
    }

    fn should_print_semi_top_level(&mut self, expr: &TopLevelExpr) -> bool {
        if self.format == TranspileToC {
            match expr {
                TopLevelExpr::FnDef(_) => false,
                TopLevelExpr::Include(_) => false,
            }
        } else {
            true
        }
    }

    fn should_print_semi(&mut self, expr: &CodeExpr) -> bool {
        if self.format == TranspileToC {
            match expr {
                CodeExpr::Return(_) | CodeExpr::IntLiteral(_) => true,
            }
        } else {
            true
        }
    }

    fn print_indent(&self) {
        stdout_write(" ".repeat(self.indent * 4));
    }
}
