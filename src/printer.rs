use core::usize;

use crate::{ast::*, core::*};
use alloc::rc::Rc;

use PrintFormat::*;

#[derive(PartialEq)]
pub enum PrintFormat {
    PrettyPrint,
    TranspileToC,
}

pub struct Printer {
    format: PrintFormat,
    bundle: bool,
    ast: Rc<AST>,
    indent: usize,
    comments_printed: usize,
}

impl Printer {
    pub fn print(ast: Rc<AST>, format: PrintFormat, bundle: bool) {
        let mut printer = Printer {
            format,
            bundle,
            ast,
            indent: 0,
            comments_printed: 0,
        };

        printer.print_file();
    }

    // TODO: print all function declarations first in C mode
    fn print_file(&mut self) {
        if self.format == TranspileToC && !self.bundle {
            stdout_writeln("#pragma once");
            stdout_writeln("");
        }

        let mut should_add_newline = false;
        for expr in &self.ast.clone().exprs {
            if should_add_newline {
                stdout_writeln("");
            };

            self.print_comments_before_pos(expr.loc().pos.offset);
            let printed = self.print_top_level_expr(expr);
            should_add_newline = printed;
        }

        // print the rest of the comments
        self.print_comments_before_pos(usize::MAX);
    }

    fn print_top_level_expr(&mut self, expr: &TopLevelExpr) -> bool {
        match &expr {
            TopLevelExpr::FnDef(fn_def) => {
                self.print_fn_def(fn_def);
            }
            TopLevelExpr::Include(include) => {
                if self.bundle {
                    return false;
                }

                self.print_include(include);
            }
            TopLevelExpr::Import(import) => {
                if self.format != TranspileToC {
                    stdout_write("import from \"");
                    stdout_write(&import.module_name);
                    stdout_write("\" {\n");
                    self.indent += 1;
                }

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

                if self.format != TranspileToC {
                    self.indent -= 1;
                    self.print_indent();
                    stdout_write("};\n");
                }
            }
        }

        true
    }

    fn print_fn_def(&mut self, fn_def: &FnDefExpr) {
        if self.format == PrettyPrint && fn_def.exported {
            stdout_write("export ");
        }
        self.print_fn_decl(&fn_def.decl);
        stdout_write(" ");
        self.print_code_block_expr(&fn_def.body);
        stdout_writeln(";");
    }

    // TODO: figure out multiline param printing
    fn print_fn_decl(&mut self, fn_decl: &FnDeclExpr) {
        if self.format == TranspileToC {
            match &fn_decl.return_type {
                Some(return_type) => self.print_type(return_type),
                _ => stdout_write("void"),
            }

            stdout_write(" ");
        } else {
            stdout_write("fn ");
        }

        stdout_write(&fn_decl.fn_name);
        stdout_write("(");
        for (fn_param, index) in fn_decl.fn_params.iter().zip(0..) {
            if index != 0 {
                stdout_write(", ");
            }

            if self.format == TranspileToC {
                self.print_type(&fn_param.type_);
                stdout_write(" ");
                stdout_write(&fn_param.name);
                continue;
            }

            stdout_write(&fn_param.name);
            stdout_write(": ");
            self.print_type(&fn_param.type_);
        }
        stdout_write(")");

        if self.format != TranspileToC {
            let Some(return_type) = &fn_decl.return_type else {
                return;
            };

            stdout_write(": ");
            self.print_type(&return_type);
        }
    }

    fn print_include(&mut self, include: &IncludeExpr) {
        if self.format == TranspileToC {
            stdout_write("#include \"");
            stdout_write(drop_file_extension(include.file_path.as_str()));
            stdout_write(".c\"");
            return;
        }

        stdout_write("include \"");
        stdout_write(&include.file_path);
        stdout_write("\";\n");
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
                stdout_write("return ");
                self.print_code_expr(&expr);
            }
            CodeExpr::IntLiteral(IntLiteralExpr { repr, .. }) => {
                stdout_write(repr);
            }
            CodeExpr::VarLoad(VarLoadExpr { name, .. }) => {
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
                if self.format == TranspileToC {
                    stdout_write("(");
                }
                self.print_code_expr(cond);
                if self.format == TranspileToC {
                    stdout_write(")");
                }
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
            // TODO: figure out multiline arg printing
            CodeExpr::Call(CallExpr { fn_name, args, .. }) => {
                stdout_write(fn_name);
                stdout_write("(");
                for (arg, index) in args.iter().zip(0..) {
                    if index != 0 {
                        stdout_write(",");
                    }

                    self.print_code_expr(arg);
                }
                stdout_write(")");
            }
        }
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

fn drop_file_extension(file_name: &str) -> &str {
    if let Some(last_dot_pos) = file_name.rfind('.') {
        if let Some(last_slash_pos) = file_name.rfind('/') {
            if last_slash_pos > last_dot_pos {
                return file_name; // no extension
            }
        }

        return &file_name[0..last_dot_pos];
    }

    file_name
}
