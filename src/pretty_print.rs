use crate::{ast::*, core::*};

struct PrintContext {
    indent: usize,
}

pub fn pretty_print(ast: &AST) {
    let ctx = PrintContext { indent: 0 };

    for expr in &ast.exprs {
        print_top_level_expr(&ctx, expr);
        stdout_writeln(";");
        stdout_writeln("");
    }
}

fn print_top_level_expr(ctx: &PrintContext, expr: &TopLevelExpr) {
    match expr {
        TopLevelExpr::FnDef(fn_def) => print_fn_def(ctx, fn_def),
    }
}

fn print_fn_def(ctx: &PrintContext, fn_def: &FnDefExpr) {
    if fn_def.exported {
        stdout_write("export ");
    }

    stdout_write("fn ");

    stdout_write(&fn_def.fn_name);

    stdout_write("(): ");

    print_type(&fn_def.return_type);

    stdout_write(" ");

    print_code_block_expr(ctx, &fn_def.body);
}

fn print_type(type_expr: &TypeExpr) {
    match type_expr {
        TypeExpr::U32 => stdout_write("u32"),
    }
}

fn print_code_block_expr(ctx: &PrintContext, code_block: &CodeBlockExpr) {
    let block_ctx = PrintContext {
        indent: ctx.indent + 1,
        ..*ctx
    };

    stdout_writeln("{");

    for expr in &code_block.exprs {
        print_indent(&block_ctx);
        print_code_expr(&block_ctx, expr);
        stdout_writeln(";");
    }

    stdout_write("}");
}

fn print_code_expr(ctx: &PrintContext, expr: &CodeExpr) {
    match expr {
        CodeExpr::Return(ReturnExpr { expr }) => {
            stdout_write("return ");
            print_code_expr(ctx, &expr);
        }
        CodeExpr::IntLiteral(IntLiteralExpr { value }) => {
            stdout_write(value);
        }
    }
}

fn print_indent(ctx: &PrintContext) {
    stdout_write(" ".repeat(ctx.indent * 4));
}
