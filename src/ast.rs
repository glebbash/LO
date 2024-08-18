use alloc::{boxed::Box, string::String, vec::Vec};

#[derive(Debug)]
pub struct AST {
    pub exprs: Vec<TopLevelExpr>,
}

#[derive(Debug)]
pub enum TopLevelExpr {
    FnDef(FnDefExpr),
}

#[derive(Debug)]
pub struct FnDefExpr {
    pub exported: bool,
    pub fn_name: String,
    pub return_type: TypeExpr,
    pub body: CodeBlockExpr,
}

#[derive(Debug)]
pub enum TypeExpr {
    U32,
}

#[derive(Debug)]
pub struct CodeBlockExpr {
    pub exprs: Vec<CodeExpr>,
}

#[derive(Debug)]
pub enum CodeExpr {
    Return(ReturnExpr),
    IntLiteral(IntLiteralExpr),
}

#[derive(Debug)]
pub struct ReturnExpr {
    pub expr: Box<CodeExpr>,
}

#[derive(Debug)]
pub struct IntLiteralExpr {
    pub value: String,
}
