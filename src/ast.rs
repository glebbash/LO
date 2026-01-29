use crate::{core::*, lexer::*};
use alloc::{boxed::Box, string::String, vec::Vec};

pub enum TopLevelExpr {
    Let(LetExpr),
    FnDef(FnDefExpr),
    InlineFnDef(InlineFnDefExpr),

    StructDef(StructDefExpr),
    EnumDef(EnumDefExpr),
    TypeDef(TypeDefExpr),

    IntrinsicCall(InlineFnCallExpr),

    Import(ImportExpr),
    Include(IncludeExpr),
    MemoryDef(MemoryDefExpr),
}

pub struct FnDefExpr {
    pub exported: bool,
    pub decl: FnDeclExpr,
    pub body: CodeBlock,
    pub loc: Loc,
}

pub struct FnDeclExpr {
    pub fn_name: IdentExpr,
    pub fn_params: Vec<FnParam>,
    pub fn_params_trailing_comma: bool,
    pub return_type: Option<TypeExpr>,
    pub loc: Loc,
}

pub struct FnParam {
    pub param_name: IdentExpr,
    pub param_type: FnParamType,
    pub loc: Loc,
}

pub enum FnParamType {
    Self_,
    SelfRef,
    Type { expr: TypeExpr },
}

/// DOC: `include "<module path>" [as <alias>] [with extern]` syntax was chosen
///   because it reuses existing rust keywords and reads nicely (mostly).
///
/// Another option was `import "..." [as ...] [and expose]`
///   but `import` is already a WASM concept and `and expose` is two new keywords for just one flag

pub struct IncludeExpr {
    pub file_path: QuotedString,
    pub alias: Option<IdentExpr>,
    pub with_extern: bool,
    pub loc: Loc,
}

pub struct ImportExpr {
    pub module_name: QuotedString,
    pub items: Vec<ImportItem>,
    pub loc: Loc,
}

pub enum ImportItem {
    FnDecl(FnDeclExpr),
    Memory(MemoryDefExpr),
}

impl ImportItem {
    pub fn loc(&self) -> &Loc {
        match self {
            ImportItem::FnDecl(e) => &e.loc,
            ImportItem::Memory(e) => &e.loc,
        }
    }
}

pub struct LetExpr {
    pub is_inline: bool,
    pub name: IdentExpr,
    pub value: Box<CodeExpr>,
    pub loc: Loc,
}

pub struct StructDefExpr {
    pub struct_name: IdentExpr,
    pub fields: Vec<StructDefField>,
    pub loc: Loc,
}

pub struct StructDefField {
    pub field_name: IdentExpr,
    pub field_type: TypeExpr,
    pub loc: Loc,
}

pub struct EnumDefExpr {
    pub enum_name: IdentExpr,
    pub variant_type: Option<TypeExpr>,
    pub variants: Vec<EnumDefVariant>,
    pub loc: Loc,
}

pub struct EnumDefVariant {
    pub variant_name: IdentExpr,
    pub variant_type: Option<TypeExpr>,
    pub loc: Loc,
}

pub struct TypeDefExpr {
    pub type_name: IdentExpr,
    pub type_value: TypeExpr,
    pub loc: Loc,
}

pub struct MemoryDefExpr {
    pub exported: bool,
    pub params: CodeExprMap,
    pub loc: Loc,
}

pub struct InlineFnDefExpr {
    pub inline_fn_name: IdentExpr,
    pub type_params: Vec<&'static str>,
    pub params: Vec<FnParam>,
    pub params_trailing_comma: bool,
    pub return_type: Option<TypeExpr>,
    pub body: CodeBlock,
    pub loc: Loc,
}

pub struct CodeBlock {
    pub exprs: Vec<CodeExpr>,
    pub loc: Loc,
}

impl TopLevelExpr {
    pub fn loc(&self) -> &Loc {
        match self {
            TopLevelExpr::Let(e) => &e.loc,
            TopLevelExpr::FnDef(e) => &e.loc,
            TopLevelExpr::Include(e) => &e.loc,
            TopLevelExpr::Import(e) => &e.loc,
            TopLevelExpr::StructDef(e) => &e.loc,
            TopLevelExpr::EnumDef(e) => &e.loc,
            TopLevelExpr::TypeDef(e) => &e.loc,
            TopLevelExpr::MemoryDef(e) => &e.loc,
            TopLevelExpr::InlineFnDef(e) => &e.loc,
            TopLevelExpr::IntrinsicCall(e) => &e.loc,
        }
    }
}

pub enum TypeExpr {
    Named(TypeExprNamed),
    Pointer(TypeExprPointer),
    SequencePointer(TypeExprSequencePointer),
    Container(TypeExprContainer),
}

pub struct TypeExprNamed {
    pub name: IdentExpr,
}

pub struct TypeExprPointer {
    pub pointee: Box<TypeExpr>,
    pub loc: Loc,
}

pub struct TypeExprSequencePointer {
    pub pointee: Box<TypeExpr>,
    pub loc: Loc,
}

pub struct TypeExprContainer {
    pub container: Box<TypeExpr>,
    pub items: Vec<TypeExpr>,
    pub loc: Loc,
}

impl TypeExpr {
    pub fn loc(&self) -> &Loc {
        match self {
            TypeExpr::Named(e) => &e.name.loc,
            TypeExpr::Pointer(e) => &e.loc,
            TypeExpr::SequencePointer(e) => &e.loc,
            TypeExpr::Container(e) => &e.loc,
        }
    }
}

pub enum CodeExpr {
    // literals
    BoolLiteral(BoolLiteralExpr),
    CharLiteral(CharLiteralExpr),
    NullLiteral(NullLiteralExpr),
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
    InlineFnCall(InlineFnCallExpr),
    InlineMethodCall(InlineMethodCallExpr),
    IntrinsicCall(InlineFnCallExpr),

    // control flow
    Return(ReturnExpr),
    If(IfExpr),
    While(WhileExpr),
    For(ForExpr),
    Break(BreakExpr),
    Continue(ContinueExpr),
    Defer(DeferExpr),
    Catch(CatchExpr),
    Match(MatchExpr),

    // other
    Paren(ParenExpr),
    DoWith(DoWithExpr),
    ExprPipe(ExprPipeExpr),
    Sizeof(SizeofExpr),
}

pub struct BoolLiteralExpr {
    pub value: bool,
    pub loc: Loc,
}

pub struct CharLiteralExpr {
    pub repr: &'static str,
    pub value: u32,
    pub loc: Loc,
}

pub struct NullLiteralExpr {
    pub loc: Loc,
}

pub struct IntLiteralExpr {
    pub repr: &'static str,
    pub value: i64,
    pub tag: Option<&'static str>,
    pub loc: Loc,
}

pub struct StringLiteralExpr {
    pub repr: &'static str,
    pub value: String,
    pub loc: Loc,
}

pub struct ReturnExpr {
    pub expr: Option<Box<CodeExpr>>,
    pub loc: Loc,
}

pub struct IdentExpr {
    pub repr: &'static str,
    pub parts: Vec<Loc>,
    pub loc: Loc,
}

pub struct InfixOpExpr {
    pub lhs: Box<CodeExpr>,
    pub rhs: Box<CodeExpr>,
    pub op_tag: InfixOpTag,
    pub op_loc: Loc,
    pub loc: Loc,
}

pub struct PrefixOpExpr {
    pub op_tag: PrefixOpTag,
    pub expr: Box<CodeExpr>,
    pub op_loc: Loc,
    pub loc: Loc,
}

pub struct IfExpr {
    pub cond: IfCond,
    pub then_block: Box<CodeBlock>,
    pub else_block: ElseBlock,
    pub loc: Loc,
}

pub enum IfCond {
    Expr(Box<CodeExpr>),
    Match(Box<MatchHeader>),
}

pub struct MatchHeader {
    pub variant_name: IdentExpr,
    pub variant_bind: IdentExpr,
    pub expr_to_match: CodeExpr,
}

pub enum ElseBlock {
    None,
    Else(Box<CodeBlock>),
    ElseIf(Box<CodeExpr>),
}

pub struct WhileExpr {
    pub cond: Option<Box<CodeExpr>>,
    pub body: Box<CodeBlock>,
    pub loc: Loc,
}

pub struct BreakExpr {
    pub loc: Loc,
}

pub struct ForExpr {
    pub counter: IdentExpr,
    pub start: Box<CodeExpr>,
    pub end: Box<CodeExpr>,
    pub body: Box<CodeBlock>,
    pub op_loc: Loc,
    pub loc: Loc,
}

pub struct ContinueExpr {
    pub loc: Loc,
}

pub struct CodeExprList {
    pub items: Vec<CodeExpr>,
    pub has_trailing_comma: bool,
}

pub struct DoWithExpr {
    pub body: Box<CodeExpr>,
    pub args: CodeExprList,
    pub with_loc: Loc,
    pub loc: Loc,
}

pub struct ExprPipeExpr {
    pub lhs: Box<CodeExpr>,
    pub rhs: Box<CodeExpr>,
    pub op_loc: Loc,
    pub loc: Loc,
}

pub struct DeferExpr {
    pub expr: Box<CodeExpr>,
    pub loc: Loc,
}

pub struct CastExpr {
    pub expr: Box<CodeExpr>,
    pub casted_to: TypeExpr,
    pub loc: Loc,
}

pub struct StructLiteralExpr {
    pub struct_name: IdentExpr,
    pub body: CodeExprMap,
    pub loc: Loc,
}

pub struct CodeExprMap {
    pub fields: Vec<CodeExprMapField>,
    pub has_trailing_comma: bool,
    pub loc: Loc,
}

pub struct ArrayLiteralExpr {
    pub item_type: TypeExpr,
    pub items: Vec<CodeExpr>,
    pub has_trailing_comma: bool,
    pub loc: Loc,
}

pub struct ResultLiteralExpr {
    pub is_ok: bool,
    pub result_type: Option<ResultTypeExpr>,
    pub value: Option<Box<CodeExpr>>,
    pub loc: Loc,
}

pub struct ResultTypeExpr {
    pub ok: TypeExpr,
    pub err: TypeExpr,
}

pub struct CodeExprMapField {
    pub key: &'static str,
    pub value: CodeExpr,
    pub loc: Loc,
}

pub struct AssignExpr {
    pub op_loc: Loc,
    pub lhs: Box<CodeExpr>,
    pub rhs: Box<CodeExpr>,
    pub loc: Loc,
}

pub struct FieldAccessExpr {
    pub lhs: Box<CodeExpr>,
    pub field_name: IdentExpr,
    pub loc: Loc,
}

pub struct CatchExpr {
    pub lhs: Box<CodeExpr>,
    pub error_bind: IdentExpr,
    pub catch_body: CodeBlock,
    pub catch_loc: Loc,
    pub loc: Loc,
}

pub struct MatchExpr {
    pub header: Box<MatchHeader>,
    pub else_branch: CodeBlock,
    pub loc: Loc,
}

pub struct PropagateErrorExpr {
    pub expr: Box<CodeExpr>,
    pub loc: Loc,
}

pub struct ParenExpr {
    pub expr: Box<CodeExpr>,
    pub has_trailing_comma: bool,
    pub loc: Loc,
}

pub struct FnCallExpr {
    pub fn_name: IdentExpr,
    pub args: CodeExprList,
    pub loc: Loc,
}

pub struct MethodCallExpr {
    pub lhs: Box<CodeExpr>,
    pub field_name: IdentExpr,
    pub args: CodeExprList,
    pub loc: Loc,
}

pub struct InlineFnCallExpr {
    pub fn_name: IdentExpr,
    pub type_args: Vec<TypeExpr>,
    pub args: CodeExprList,
    pub loc: Loc,
}

pub struct InlineMethodCallExpr {
    pub lhs: Box<CodeExpr>,
    pub field_name: IdentExpr,
    pub type_args: Vec<TypeExpr>,
    pub args: CodeExprList,
    pub loc: Loc,
}

pub struct SizeofExpr {
    pub type_expr: TypeExpr,
    pub loc: Loc,
}

impl CodeExpr {
    pub fn loc(&self) -> Loc {
        match self {
            CodeExpr::BoolLiteral(e) => e.loc,
            CodeExpr::CharLiteral(e) => e.loc,
            CodeExpr::NullLiteral(e) => e.loc,
            CodeExpr::IntLiteral(e) => e.loc,
            CodeExpr::StringLiteral(e) => e.loc,
            CodeExpr::ArrayLiteral(e) => e.loc,
            CodeExpr::ResultLiteral(e) => e.loc,
            CodeExpr::Return(e) => e.loc,
            CodeExpr::Ident(e) => e.loc,
            CodeExpr::InfixOp(e) => e.loc,
            CodeExpr::If(e) => e.loc,
            CodeExpr::Let(e) => e.loc,
            CodeExpr::While(e) => e.loc,
            CodeExpr::Break(e) => e.loc,
            CodeExpr::For(e) => e.loc,
            CodeExpr::Continue(e) => e.loc,
            CodeExpr::Defer(e) => e.loc,
            CodeExpr::Cast(e) => e.loc,
            CodeExpr::StructLiteral(e) => e.loc,
            CodeExpr::Assign(e) => e.loc,
            CodeExpr::FieldAccess(e) => e.loc,
            CodeExpr::Catch(e) => e.loc,
            CodeExpr::Match(e) => e.loc,
            CodeExpr::Paren(e) => e.loc,
            CodeExpr::FnCall(e) => e.loc,
            CodeExpr::MethodCall(e) => e.loc,
            CodeExpr::InlineFnCall(e) => e.loc,
            CodeExpr::IntrinsicCall(e) => e.loc,
            CodeExpr::InlineMethodCall(e) => e.loc,
            CodeExpr::Sizeof(e) => e.loc,
            CodeExpr::PropagateError(e) => e.loc,
            CodeExpr::PrefixOp(e) => e.loc,
            CodeExpr::DoWith(e) => e.loc,
            CodeExpr::ExprPipe(e) => e.loc,
        }
    }
}
