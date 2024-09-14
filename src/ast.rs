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
    StructDef(StructDefExpr),
    TypeDef(TypeDefExpr),
    ConstDef(ConstDefExpr),
    MemoryDef(MemoryDefExpr),
    StaticDataStore(StaticDataStoreExpr),
    ExportExistingFn(ExportExistingFnExpr),
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
    pub fn_name: IdentExpr,
    pub fn_params: Vec<FnParam>,
    pub return_type: Option<TypeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct FnParam {
    pub name: String,
    pub type_: Option<TypeExpr>,
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

#[derive(Debug)]
pub struct StructDefExpr {
    pub struct_name: IdentExpr,
    pub fields: Vec<StructDefField>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct StructDefField {
    pub field_name: String,
    pub field_type: TypeExpr,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct TypeDefExpr {
    pub type_name: IdentExpr,
    pub type_value: TypeExpr,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct ConstDefExpr {
    pub const_name: IdentExpr,
    pub const_value: CodeExpr,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct MemoryDefExpr {
    pub exported: bool,
    pub min_pages: Option<u32>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct StaticDataStoreExpr {
    pub addr: CodeExpr,
    pub data: StaticDataStorePayload,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub enum StaticDataStorePayload {
    String { value: String },
}

#[derive(Debug)]
pub struct ExportExistingFnExpr {
    pub in_fn_name: IdentExpr,
    pub out_fn_name: String,
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
            TopLevelExpr::StructDef(e) => &e.loc,
            TopLevelExpr::TypeDef(e) => &e.loc,
            TopLevelExpr::ConstDef(e) => &e.loc,
            TopLevelExpr::MemoryDef(e) => &e.loc,
            TopLevelExpr::StaticDataStore(e) => &e.loc,
            TopLevelExpr::ExportExistingFn(e) => &e.loc,
        }
    }
}

#[derive(Debug)]
pub enum TypeExpr {
    U32,
    Pointer {
        pointee: Box<TypeExpr>,
    },
    SequencePointer {
        pointee: Box<TypeExpr>,
    },
    AliasOrStruct {
        name: IdentExpr,
    },
    Result {
        ok_type: Box<TypeExpr>,
        err_type: Box<TypeExpr>,
    },
}

#[derive(Debug)]
pub enum CodeExpr {
    Return(ReturnExpr),
    IntLiteral(IntLiteralExpr),
    StringLiteral(StringLiteralExpr),
    Ident(IdentExpr),
    BinaryOp(BinaryOpExpr),
    If(IfExpr),
    BoolLiteral(BoolLiteralExpr),
    Local(LocalExpr),
    Loop(LoopExpr),
    Break(BreakExpr),
    ForLoop(ForLoopExpr),
    Continue(ContinueExpr),
    Dbg(DbgExpr),
    Defer(DeferExpr),
    Cast(CastExpr),
    StructInit(StructInitExpr),
    Assign(AssignExpr),
    FieldAccess(FieldAccessExpr),
    Catch(CatchExpr),
    Dereference(DereferenceExpr),
    Paren(ParenExpr),
    FnCall(FnCallExpr),
    MethodCall(MethodCallExpr),
    MacroFnCall(MacroFnCallExpr),
    MacroMethodCall(MacroMethodCallExpr),
    Sizeof(SizeofExpr),
}

#[derive(Debug)]
pub struct CodeBlockExpr {
    pub exprs: Vec<CodeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct ReturnExpr {
    pub expr: Option<Box<CodeExpr>>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct IntLiteralExpr {
    pub repr: String,
    pub value: u32,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct StringLiteralExpr {
    pub repr: String,
    pub value: String,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct IdentExpr {
    pub repr: String,
    pub parts: Vec<String>,
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
pub struct BoolLiteralExpr {
    pub value: bool,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub enum ElseBlock {
    None,
    Else(Box<CodeBlockExpr>),
    ElseIf(Box<CodeExpr>),
}

#[derive(Debug)]
pub struct LocalExpr {
    pub local_name: String,
    pub value: Box<CodeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct LoopExpr {
    pub body: Box<CodeBlockExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct BreakExpr {
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct ForLoopExpr {
    pub counter: String,
    pub start: Box<CodeExpr>,
    pub end: Box<CodeExpr>,
    pub body: Box<CodeBlockExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct ContinueExpr {
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct DbgExpr {
    pub message: String,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct DeferExpr {
    pub expr: Box<CodeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct CastExpr {
    pub expr: Box<CodeExpr>,
    pub casted_to: TypeExpr,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct StructInitExpr {
    pub struct_name: IdentExpr,
    pub fields: Vec<StructInitField>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct StructInitField {
    pub field_name: String,
    pub value: CodeExpr,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct AssignExpr {
    pub lhs: Box<CodeExpr>,
    pub rhs: Box<CodeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct FieldAccessExpr {
    pub lhs: Box<CodeExpr>,
    pub field_name: IdentExpr,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct CatchExpr {
    pub lhs: Box<CodeExpr>,
    pub error_bind: String,
    pub catch_body: CodeBlockExpr,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct DereferenceExpr {
    pub referenced: Box<CodeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct ParenExpr {
    pub expr: Box<CodeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct FnCallExpr {
    pub fn_name: IdentExpr,
    pub args: Vec<CodeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct MethodCallExpr {
    pub lhs: Box<CodeExpr>,
    pub field_name: IdentExpr,
    pub args: Vec<CodeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct MacroFnCallExpr {
    pub fn_name: IdentExpr,
    pub type_args: Vec<TypeExpr>,
    pub args: Vec<CodeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct MacroMethodCallExpr {
    pub lhs: Box<CodeExpr>,
    pub field_name: IdentExpr,
    pub type_args: Vec<TypeExpr>,
    pub args: Vec<CodeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct SizeofExpr {
    pub type_expr: TypeExpr,
    pub loc: LoLocation,
}

impl Locatable for CodeExpr {
    fn loc(&self) -> &LoLocation {
        match self {
            CodeExpr::Return(e) => &e.loc,
            CodeExpr::IntLiteral(e) => &e.loc,
            CodeExpr::StringLiteral(e) => &e.loc,
            CodeExpr::Ident(e) => &e.loc,
            CodeExpr::BinaryOp(e) => &e.loc,
            CodeExpr::If(e) => &e.loc,
            CodeExpr::BoolLiteral(e) => &e.loc,
            CodeExpr::Local(e) => &e.loc,
            CodeExpr::Loop(e) => &e.loc,
            CodeExpr::Break(e) => &e.loc,
            CodeExpr::ForLoop(e) => &e.loc,
            CodeExpr::Continue(e) => &e.loc,
            CodeExpr::Dbg(e) => &e.loc,
            CodeExpr::Defer(e) => &e.loc,
            CodeExpr::Cast(e) => &e.loc,
            CodeExpr::StructInit(e) => &e.loc,
            CodeExpr::Assign(e) => &e.loc,
            CodeExpr::FieldAccess(e) => &e.loc,
            CodeExpr::Catch(e) => &e.loc,
            CodeExpr::Dereference(e) => &e.loc,
            CodeExpr::Paren(e) => &e.loc,
            CodeExpr::FnCall(e) => &e.loc,
            CodeExpr::MethodCall(e) => &e.loc,
            CodeExpr::MacroFnCall(e) => &e.loc,
            CodeExpr::MacroMethodCall(e) => &e.loc,
            CodeExpr::Sizeof(e) => &e.loc,
        }
    }
}
