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
    MacroDef(MacroDefExpr),
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
    pub param_name: String,
    pub param_type: FnParamType,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub enum FnParamType {
    Self_,
    SelfRef,
    Type { expr: TypeExpr },
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
    Memory(MemoryDefExpr),
}

impl Locatable for ImportItem {
    fn loc(&self) -> &LoLocation {
        match self {
            ImportItem::FnDecl(e) => &e.loc,
            ImportItem::Memory(e) => &e.loc,
        }
    }
}

#[derive(Debug)]
pub struct GlobalDefExpr {
    pub global_name: IdentExpr,
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
    pub data_start: Option<u32>,
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

#[derive(Debug)]
pub struct MacroDefExpr {
    pub macro_name: IdentExpr,
    pub macro_params: Vec<FnParam>,
    pub macro_type_params: Vec<String>,
    pub return_type: Option<TypeExpr>,
    pub body: CodeBlockExpr,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct CodeBlockExpr {
    pub exprs: Vec<CodeExpr>,
    pub loc: LoLocation,
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
            TopLevelExpr::MacroDef(e) => &e.loc,
        }
    }
}

#[derive(Debug)]
pub enum TypeExpr {
    Named {
        name: IdentExpr,
    },
    Pointer {
        pointee: Box<TypeExpr>,
    },
    SequencePointer {
        pointee: Box<TypeExpr>,
    },
    Result {
        ok_type: Box<TypeExpr>,
        err_type: Box<TypeExpr>,
    },
    Of {
        container_type: Box<TypeExpr>,
        item_type: Box<TypeExpr>,
    },
}

#[derive(Debug)]
pub enum CodeExpr {
    // literals
    BoolLiteral(BoolLiteralExpr),
    CharLiteral(CharLiteralExpr),
    IntLiteral(IntLiteralExpr),
    StringLiteral(StringLiteralExpr),
    StructLiteral(StructLiteralExpr),
    ArrayLiteral(ArrayLiteralExpr),

    // variables
    Ident(IdentExpr),
    Let(LetExpr),

    // operations
    InfixOp(InfixOpExpr),
    PrefixOp(PrefixOpExpr),
    Cast(CastExpr),
    Assign(AssignExpr),
    FieldAccess(FieldAccessExpr),
    PropagateError(PropagateErrorExpr),
    FnCall(FnCallExpr),
    MethodCall(MethodCallExpr),
    MacroFnCall(MacroFnCallExpr),
    MacroMethodCall(MacroMethodCallExpr),
    Dbg(DbgExpr),
    Sizeof(SizeofExpr),
    GetDataSize(GetDataSizeExpr),

    // control flow
    Return(ReturnExpr),
    If(IfExpr),
    Loop(LoopExpr),
    Break(BreakExpr),
    Unreachable(UnreachableExpr),
    ForLoop(ForLoopExpr),
    Continue(ContinueExpr),
    Defer(DeferExpr),
    Catch(CatchExpr),
    Paren(ParenExpr),
}

#[derive(Debug)]
pub struct BoolLiteralExpr {
    pub value: bool,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct CharLiteralExpr {
    pub repr: String,
    pub value: u32,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct IntLiteralExpr {
    pub repr: String,
    pub value: u32,
    pub tag: Option<String>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct StringLiteralExpr {
    pub repr: String,
    pub value: String,
    pub zero_terminated: bool,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct ReturnExpr {
    pub expr: Option<Box<CodeExpr>>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct IdentExpr {
    pub repr: String,
    pub parts: Vec<String>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct InfixOpExpr {
    pub op_tag: InfixOpTag,
    pub lhs: Box<CodeExpr>,
    pub rhs: Box<CodeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct PrefixOpExpr {
    pub op_tag: PrefixOpTag,
    pub expr: Box<CodeExpr>,
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
pub struct LetExpr {
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
pub struct UnreachableExpr {
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
pub struct StructLiteralExpr {
    pub struct_name: IdentExpr,
    pub fields: Vec<StructLiteralField>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct ArrayLiteralExpr {
    pub item_type: TypeExpr,
    pub items: Vec<CodeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct StructLiteralField {
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
pub struct PropagateErrorExpr {
    pub expr: Box<CodeExpr>,
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

#[derive(Debug)]
pub struct GetDataSizeExpr {
    pub loc: LoLocation,
}

impl Locatable for CodeExpr {
    fn loc(&self) -> &LoLocation {
        match self {
            CodeExpr::BoolLiteral(e) => &e.loc,
            CodeExpr::CharLiteral(e) => &e.loc,
            CodeExpr::IntLiteral(e) => &e.loc,
            CodeExpr::StringLiteral(e) => &e.loc,
            CodeExpr::ArrayLiteral(e) => &e.loc,
            CodeExpr::Return(e) => &e.loc,
            CodeExpr::Ident(e) => &e.loc,
            CodeExpr::InfixOp(e) => &e.loc,
            CodeExpr::If(e) => &e.loc,
            CodeExpr::Let(e) => &e.loc,
            CodeExpr::Loop(e) => &e.loc,
            CodeExpr::Break(e) => &e.loc,
            CodeExpr::ForLoop(e) => &e.loc,
            CodeExpr::Continue(e) => &e.loc,
            CodeExpr::Dbg(e) => &e.loc,
            CodeExpr::Defer(e) => &e.loc,
            CodeExpr::Cast(e) => &e.loc,
            CodeExpr::StructLiteral(e) => &e.loc,
            CodeExpr::Assign(e) => &e.loc,
            CodeExpr::FieldAccess(e) => &e.loc,
            CodeExpr::Catch(e) => &e.loc,
            CodeExpr::Paren(e) => &e.loc,
            CodeExpr::FnCall(e) => &e.loc,
            CodeExpr::MethodCall(e) => &e.loc,
            CodeExpr::MacroFnCall(e) => &e.loc,
            CodeExpr::MacroMethodCall(e) => &e.loc,
            CodeExpr::Sizeof(e) => &e.loc,
            CodeExpr::PropagateError(e) => &e.loc,
            CodeExpr::PrefixOp(e) => &e.loc,
            CodeExpr::GetDataSize(e) => &e.loc,
            CodeExpr::Unreachable(e) => &e.loc,
        }
    }
}
