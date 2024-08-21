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
    Include(IncludeExpr),
}

#[derive(Debug)]
pub struct FnDefExpr {
    pub exported: bool,
    pub fn_name: String,
    pub fn_params: Vec<FnParam>,
    pub return_type: TypeExpr,
    pub body: CodeBlockExpr,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct FnParam {
    pub name: String,
    pub type_: TypeExpr,
}

#[derive(Debug, Clone)]
pub struct IncludeExpr {
    pub file_path: String,
    pub loc: LoLocation,
}

impl Locatable for TopLevelExpr {
    fn loc(&self) -> &LoLocation {
        match self {
            TopLevelExpr::FnDef(e) => &e.loc,
            TopLevelExpr::Include(e) => &e.loc,
        }
    }
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
    VarLoad(VarLoadExpr),
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

#[derive(Debug)]
pub struct VarLoadExpr {
    pub name: String,
    pub loc: LoLocation,
}

impl Locatable for CodeExpr {
    fn loc(&self) -> &LoLocation {
        match self {
            CodeExpr::Return(e) => &e.loc,
            CodeExpr::IntLiteral(e) => &e.loc,
            CodeExpr::VarLoad(e) => &e.loc,
        }
    }
}
