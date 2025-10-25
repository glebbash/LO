use crate::{core::*, lexer::*};
use alloc::{boxed::Box, string::String, vec::Vec};

pub trait Locatable {
    fn loc(&self) -> &LoLocation;
}

#[derive(Debug, Default)]
pub struct AST {
    pub exprs: Vec<TopLevelExpr>,
    pub comments: Vec<LoLocation>,
    pub backslashes: Vec<LoLocation>,
    pub double_backslashes: Vec<LoLocation>,
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
    TryExport(TryExportExpr),
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
    pub fn_params_trailing_comma: bool,
    pub return_type: Option<TypeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct FnParam {
    pub param_name: IdentExpr,
    pub param_type: FnParamType,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub enum FnParamType {
    Self_,
    SelfRef,
    Type { expr: TypeExpr },
    Infer { name: String },
}

/// DOC: `include "<module path>" [as <alias>] [with extern]` syntax was chosen
///   because it reuses existing rust keywords and reads nicely (mostly).
///
/// Another option was `import "..." [as ...] [and expose]`
///   but `import` is already a WASM concept and `and expose` is two new keyword for just one flag
#[derive(Debug)]
pub struct IncludeExpr {
    pub file_path: EscapedString,
    pub alias: Option<IdentExpr>,
    pub with_extern: bool,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct ImportExpr {
    pub module_name: EscapedString,
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
    pub global_value: GlobalDefValue,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub enum GlobalDefValue {
    Expr(CodeExpr),
    DataSize,
}

#[derive(Debug)]
pub struct StructDefExpr {
    pub struct_name: IdentExpr,
    pub fields: Vec<StructDefField>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct StructDefField {
    pub field_name: IdentExpr,
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

#[derive(Debug, Clone)]
pub struct MemoryDefExpr {
    pub exported: bool,
    pub min_pages: Option<u32>,
    pub data_start: Option<u32>,
    pub loc: LoLocation,
}

/// DOC: `try export <in> as "<out>" [from root]` syntax was chosen
///   because it reuses existing rust keywords and reads nicely
#[derive(Debug)]
pub struct TryExportExpr {
    pub in_name: IdentExpr,
    pub out_name: EscapedString,
    pub from_root: bool,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct MacroDefExpr {
    pub macro_name: IdentExpr,
    pub macro_params: Vec<FnParam>,
    pub macro_params_trailing_comma: bool,
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
            TopLevelExpr::TryExport(e) => &e.loc,
            TopLevelExpr::MacroDef(e) => &e.loc,
        }
    }
}

#[derive(Debug)]
pub enum TypeExpr {
    Named(TypeExprNamed),
    Pointer(TypeExprPointer),
    SequencePointer(TypeExprSequencePointer),
    Result(TypeExprResult),
    Of(TypeExprOf),
}

#[derive(Debug)]
pub struct TypeExprNamed {
    pub name: IdentExpr,
}

#[derive(Debug)]
pub struct TypeExprPointer {
    pub pointee: Box<TypeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct TypeExprSequencePointer {
    pub pointee: Box<TypeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct TypeExprResult {
    pub ok_type: Box<TypeExpr>,
    pub err_type: Box<TypeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct TypeExprOf {
    pub container_type: Box<TypeExpr>,
    pub item_type: Box<TypeExpr>,
    pub loc: LoLocation,
}

impl Locatable for TypeExpr {
    fn loc(&self) -> &LoLocation {
        match self {
            TypeExpr::Named(e) => &e.name.loc,
            TypeExpr::Pointer(e) => &e.loc,
            TypeExpr::SequencePointer(e) => &e.loc,
            TypeExpr::Result(e) => &e.loc,
            TypeExpr::Of(e) => &e.loc,
        }
    }
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
    ResultLiteral(ResultLiteralExpr),

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
    IntrinsicCall(MacroFnCallExpr),

    // control flow
    Return(ReturnExpr),
    If(IfExpr),
    Loop(LoopExpr),
    Break(BreakExpr),
    ForLoop(ForLoopExpr),
    Continue(ContinueExpr),
    With(WithExpr),
    Defer(DeferExpr),
    Catch(CatchExpr),
    Paren(ParenExpr),

    // TODO?: should these use intrinsic syntax?
    Dbg(DbgExpr),
    Sizeof(SizeofExpr),
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
    pub op_loc: LoLocation,
    pub lhs: Box<CodeExpr>,
    pub rhs: Box<CodeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct PrefixOpExpr {
    pub op_tag: PrefixOpTag,
    pub expr: Box<CodeExpr>,
    pub op_loc: LoLocation,
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
    pub local_name: IdentExpr,
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
    pub counter: IdentExpr,
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
pub struct CodeExprList {
    pub items: Vec<CodeExpr>,
    pub has_trailing_comma: bool,
}

#[derive(Debug)]
pub struct WithExpr {
    pub bind: IdentExpr,
    pub args: CodeExprList,
    pub body: CodeBlockExpr,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct DbgExpr {
    pub message: EscapedString,
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
    pub has_trailing_comma: bool,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct ArrayLiteralExpr {
    pub item_type: TypeExpr,
    pub items: Vec<CodeExpr>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct ResultLiteralExpr {
    pub is_ok: bool,
    pub result_type: Option<ResultTypeExpr>,
    pub value: Option<Box<CodeExpr>>,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct ResultTypeExpr {
    pub ok: TypeExpr,
    pub err: TypeExpr,
}

#[derive(Debug)]
pub struct StructLiteralField {
    pub field_name: String,
    pub value: CodeExpr,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct AssignExpr {
    pub op_loc: LoLocation,
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
    pub error_bind: IdentExpr,
    pub catch_body: CodeBlockExpr,
    pub catch_loc: LoLocation, // on `catch` keyword used to report catch errors
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
    pub has_trailing_comma: bool,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct FnCallExpr {
    pub fn_name: IdentExpr,
    pub args: CodeExprList,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct MethodCallExpr {
    pub lhs: Box<CodeExpr>,
    pub field_name: IdentExpr,
    pub args: CodeExprList,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct MacroFnCallExpr {
    pub fn_name: IdentExpr,
    pub type_args: Vec<TypeExpr>,
    pub args: CodeExprList,
    pub loc: LoLocation,
}

#[derive(Debug)]
pub struct MacroMethodCallExpr {
    pub lhs: Box<CodeExpr>,
    pub field_name: IdentExpr,
    pub type_args: Vec<TypeExpr>,
    pub args: CodeExprList,
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
            CodeExpr::BoolLiteral(e) => &e.loc,
            CodeExpr::CharLiteral(e) => &e.loc,
            CodeExpr::IntLiteral(e) => &e.loc,
            CodeExpr::StringLiteral(e) => &e.loc,
            CodeExpr::ArrayLiteral(e) => &e.loc,
            CodeExpr::ResultLiteral(e) => &e.loc,
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
            CodeExpr::IntrinsicCall(e) => &e.loc,
            CodeExpr::MacroMethodCall(e) => &e.loc,
            CodeExpr::Sizeof(e) => &e.loc,
            CodeExpr::PropagateError(e) => &e.loc,
            CodeExpr::PrefixOp(e) => &e.loc,
            CodeExpr::With(e) => &e.loc,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EscapedString(pub LoLocation);

impl EscapedString {
    pub fn get_raw(&self, source: &'static [u8]) -> &str {
        return self.0.read_span(source);
    }

    pub fn unescape(&self, source: &'static [u8]) -> String {
        Lexer::unescape_string(self.get_raw(source))
    }
}
