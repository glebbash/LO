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
    Import(ImportExpr),
    GlobalDef(GlobalDefExpr),
}

#[derive(Debug)]
pub struct FnDefExpr {
    pub exported: bool,
    pub decl: FnDeclExpr,
    pub body: CodeBlockExpr,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct FnDeclExpr {
    pub fn_name: String,
    pub fn_params: Vec<FnParam>,
    pub return_type: Option<TypeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct FnParam {
    pub name: String,
    pub type_: TypeExpr,
    pub loc: LoLocation,
}

#[derive(Debug, Clone)]
pub struct IncludeExpr {
    pub file_path: String,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct ImportExpr {
    pub module_name: String,
    pub items: Vec<ImportItem>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub enum ImportItem {
    FnDecl(FnDeclExpr),
}

#[derive(Debug)]
pub struct GlobalDefExpr {
    pub global_name: String,
    pub expr: CodeExpr,
    pub loc: LoLocation,
}

impl Locatable for ImportItem {
    fn loc(&self) -> &LoLocation {
        match self {
            ImportItem::FnDecl(e) => &e.loc,
        }
    }
}

impl Locatable for TopLevelExpr {
    fn loc(&self) -> &LoLocation {
        match self {
            TopLevelExpr::FnDef(e) => &e.loc,
            TopLevelExpr::Include(e) => &e.loc,
            TopLevelExpr::Import(e) => &e.loc,
            TopLevelExpr::GlobalDef(e) => &e.loc,
        }
    }
}

#[derive(Debug)]
pub enum TypeExpr {
    U32,
}

#[derive(Debug)]
pub enum CodeExpr {
    Return(ReturnExpr),
    IntLiteral(IntLiteralExpr),
    VarLoad(VarLoadExpr),
    BinaryOp(BinaryOpExpr),
    If(IfExpr),
    Call(CallExpr),
    Local(LocalExpr),
}

#[derive(Debug)]
pub struct CodeBlockExpr {
    pub exprs: Vec<CodeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct ReturnExpr {
    pub expr: Box<CodeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct IntLiteralExpr {
    pub repr: String,
    pub value: u32,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct VarLoadExpr {
    pub name: String,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct BinaryOpExpr {
    pub op_tag: InfixOpTag,
    pub lhs: Box<CodeExpr>,
    pub rhs: Box<CodeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct IfExpr {
    pub cond: Box<CodeExpr>,
    pub then_block: Box<CodeBlockExpr>,
    pub else_block: ElseBlock,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub enum ElseBlock {
    None,
    Else(Box<CodeBlockExpr>),
    ElseIf(Box<CodeExpr>),
}

#[derive(Debug)]
pub struct CallExpr {
    pub fn_name: String,
    pub args: Vec<CodeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct LocalExpr {
    pub local_name: String,
    pub value: Box<CodeExpr>,
    pub loc: LoLocation,
}

impl Locatable for CodeExpr {
    fn loc(&self) -> &LoLocation {
        match self {
            CodeExpr::Return(e) => &e.loc,
            CodeExpr::IntLiteral(e) => &e.loc,
            CodeExpr::VarLoad(e) => &e.loc,
            CodeExpr::BinaryOp(e) => &e.loc,
            CodeExpr::If(e) => &e.loc,
            CodeExpr::Call(e) => &e.loc,
            CodeExpr::Local(e) => &e.loc,
        }
    }
}
