use crate::{core::LoLocation, lexer::*};
use alloc::{boxed::Box, string::String, vec::Vec};

pub trait Locatable {
    fn loc(&self) -> &LoLocation;
}

#[derive(Debug)]
pub struct AST {
    pub exprs: Vec<TopLevelExpr>,
    pub comments: Vec<Comment>,
}

#[derive(Debug)]
pub enum TopLevelExpr {
    FnDef(FnDefExpr),
}

impl Locatable for TopLevelExpr {
    fn loc(&self) -> &LoLocation {
        match self {
            TopLevelExpr::FnDef(fn_def) => &fn_def.loc,
        }
    }
}

#[derive(Debug)]
pub struct FnDefExpr {
    pub exported: bool,
    pub fn_name: String,
    pub return_type: TypeExpr,
    pub body: CodeBlockExpr,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub enum TypeExpr {
    U32,
}

#[derive(Debug)]
pub struct CodeBlockExpr {
    pub exprs: Vec<CodeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub enum CodeExpr {
    Return(ReturnExpr),
    IntLiteral(IntLiteralExpr),
}

#[derive(Debug)]
pub struct ReturnExpr {
    pub expr: Box<CodeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct IntLiteralExpr {
    pub value: String,
    pub loc: LoLocation,
}

impl Locatable for CodeExpr {
    fn loc(&self) -> &LoLocation {
        match self {
            CodeExpr::Return(e) => &e.loc,
            CodeExpr::IntLiteral(e) => &e.loc,
        }
    }
}
