use crate::{common::*, lexer::*};

pub type ExprId = usize;

pub enum TopLevelExpr {
    Let(LetExpr),
    Fn(FnExpr),
    Type(TypeDefExpr),
    Intrinsic(InlineFnCallExpr),
}

pub struct FnExpr {
    pub exported: bool,
    pub is_inline: bool,
    pub decl: FnDeclExpr,
    pub value: FnExprValue,
    pub loc: Loc,
}

pub struct FnDeclExpr {
    pub fn_name: IdentExpr,
    pub type_params: Vec<&'static str>,
    pub params: Vec<FnParam>,
    pub params_trailing_comma: bool,
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

pub enum FnExprValue {
    Body(CodeBlock),
    ImportFrom(QuotedString),
}

pub struct LetExpr {
    pub id: ExprId,
    pub is_inline: bool,
    pub name: IdentExpr,
    pub value: Box<CodeExpr>,
    pub loc: Loc,
}

pub struct TypeDefExpr {
    pub name: IdentExpr,
    pub value: TypeDefValue,
    pub loc: Loc,
}

pub enum TypeDefValue {
    Struct {
        fields: Vec<StructDefField>,
    },
    Enum {
        variant_type: Option<TypeExpr>,
        variants: Vec<EnumDefVariant>,
    },
    Alias(TypeExpr),
}

pub struct StructDefField {
    pub field_name: IdentExpr,
    pub field_type: TypeExpr,
    pub loc: Loc,
}

pub struct EnumDefVariant {
    pub variant_name: IdentExpr,
    pub variant_type: Option<TypeExpr>,
    pub loc: Loc,
}

pub struct CodeBlock {
    pub exprs: Vec<CodeExpr>,
    pub expr_id_start: usize,
    pub expr_id_count: usize,
    pub loc: Loc,
}

impl TopLevelExpr {
    pub fn loc(&self) -> Loc {
        match self {
            TopLevelExpr::Let(e) => e.loc,
            TopLevelExpr::Fn(e) => e.loc,
            TopLevelExpr::Type(e) => e.loc,
            TopLevelExpr::Intrinsic(e) => e.loc,
        }
    }
}

pub enum TypeExpr {
    Named(IdentExpr),
    Pointer(TypeExprPointer),
    Container(TypeExprContainer),
}

pub struct TypeExprPointer {
    pub id: ExprId,
    pub pointee: Box<TypeExpr>,
    pub is_sequence: bool,
    pub loc: Loc,
}

pub struct TypeExprContainer {
    pub id: ExprId,
    pub container: Box<TypeExpr>,
    pub items: Vec<TypeExpr>,
    pub loc: Loc,
}

impl TypeExpr {
    pub fn id(&self) -> ExprId {
        match self {
            TypeExpr::Named(e) => e.id,
            TypeExpr::Pointer(e) => e.id,
            TypeExpr::Container(e) => e.id,
        }
    }

    pub fn loc(&self) -> Loc {
        match self {
            TypeExpr::Named(e) => e.loc,
            TypeExpr::Pointer(e) => e.loc,
            TypeExpr::Container(e) => e.loc,
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
    Pipe(PipeExpr),
    Sizeof(SizeofExpr),
}

pub struct BoolLiteralExpr {
    pub id: ExprId,
    pub value: bool,
    pub loc: Loc,
}

pub struct CharLiteralExpr {
    pub id: ExprId,
    pub repr: &'static str,
    pub value: u32,
    pub loc: Loc,
}

pub struct NullLiteralExpr {
    pub id: ExprId,
    pub loc: Loc,
}

pub struct IntLiteralExpr {
    pub id: ExprId,
    pub repr: &'static str,
    pub value: i64,
    pub tag: Option<&'static str>,
    pub loc: Loc,
}

pub struct StringLiteralExpr {
    pub id: ExprId,
    pub repr: &'static str,
    pub value: String,
    pub loc: Loc,
}

pub struct ReturnExpr {
    pub id: ExprId,
    pub expr: Option<Box<CodeExpr>>,
    pub loc: Loc,
}

pub struct IdentExpr {
    pub id: ExprId,
    pub repr: &'static str,
    pub parts: Vec<Loc>,
    pub loc: Loc,
}

pub struct InfixOpExpr {
    pub id: ExprId,
    pub lhs: Box<CodeExpr>,
    pub rhs: Box<CodeExpr>,
    pub op_tag: InfixOpTag,
    pub op_loc: Loc,
    pub loc: Loc,
}

pub struct PrefixOpExpr {
    pub id: ExprId,
    pub op_tag: PrefixOpTag,
    pub expr: Box<CodeExpr>,
    pub op_loc: Loc,
    pub loc: Loc,
}

pub struct IfExpr {
    pub id: ExprId,
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
    pub id: ExprId,
    pub cond: Option<Box<CodeExpr>>,
    pub body: Box<CodeBlock>,
    pub loc: Loc,
}

pub struct BreakExpr {
    pub id: ExprId,
    pub loc: Loc,
}

pub struct ForExpr {
    pub id: ExprId,
    pub counter: IdentExpr,
    pub start: Box<CodeExpr>,
    pub end: Box<CodeExpr>,
    pub body: Box<CodeBlock>,
    pub op_loc: Loc,
    pub loc: Loc,
}

pub struct ContinueExpr {
    pub id: ExprId,
    pub loc: Loc,
}

pub struct CodeExprList {
    pub items: Vec<CodeExpr>,
    pub has_trailing_comma: bool,
}

pub struct DoWithExpr {
    pub id: ExprId,
    pub body: Box<CodeExpr>,
    pub args: CodeExprList,
    pub with_loc: Loc,
    pub loc: Loc,
}

pub struct PipeExpr {
    pub id: ExprId,
    pub lhs: Box<CodeExpr>,
    pub rhs: Box<CodeExpr>,
    pub op_loc: Loc,
    pub loc: Loc,
}

pub struct DeferExpr {
    pub id: ExprId,
    pub expr: Box<CodeExpr>,
    pub loc: Loc,
}

pub struct CastExpr {
    pub id: ExprId,
    pub expr: Box<CodeExpr>,
    pub casted_to: TypeExpr,
    pub loc: Loc,
}

pub struct StructLiteralExpr {
    pub id: ExprId,
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
    pub id: ExprId,
    pub item_type: TypeExpr,
    pub items: Vec<CodeExpr>,
    pub has_trailing_comma: bool,
    pub loc: Loc,
}

pub struct ResultLiteralExpr {
    pub id: ExprId,
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
    pub id: ExprId,
    pub op_loc: Loc,
    pub lhs: Box<CodeExpr>,
    pub rhs: Box<CodeExpr>,
    pub loc: Loc,
}

pub struct FieldAccessExpr {
    pub id: ExprId,
    pub lhs: Box<CodeExpr>,
    pub field_name: IdentExpr,
    pub loc: Loc,
}

pub struct CatchExpr {
    pub id: ExprId,
    pub lhs: Box<CodeExpr>,
    pub error_bind: IdentExpr,
    pub catch_body: CodeBlock,
    pub catch_loc: Loc,
    pub loc: Loc,
}

pub struct MatchExpr {
    pub id: ExprId,
    pub header: Box<MatchHeader>,
    pub else_branch: CodeBlock,
    pub loc: Loc,
}

pub struct PropagateErrorExpr {
    pub id: ExprId,
    pub expr: Box<CodeExpr>,
    pub loc: Loc,
}

pub struct ParenExpr {
    pub id: ExprId,
    pub expr: Box<CodeExpr>,
    pub has_trailing_comma: bool,
    pub loc: Loc,
}

pub struct FnCallExpr {
    pub id: ExprId,
    pub fn_name: IdentExpr,
    pub args: CodeExprList,
    pub loc: Loc,
}

pub struct MethodCallExpr {
    pub id: ExprId,
    pub lhs: Box<CodeExpr>,
    pub field_name: IdentExpr,
    pub args: CodeExprList,
    pub loc: Loc,
}

pub struct InlineFnCallExpr {
    pub id: ExprId,
    pub fn_name: IdentExpr,
    pub type_args: Vec<TypeExpr>,
    pub args: CodeExprList,
    pub loc: Loc,
}

pub struct InlineMethodCallExpr {
    pub id: ExprId,
    pub lhs: Box<CodeExpr>,
    pub field_name: IdentExpr,
    pub type_args: Vec<TypeExpr>,
    pub args: CodeExprList,
    pub loc: Loc,
}

pub struct SizeofExpr {
    pub id: ExprId,
    pub type_expr: TypeExpr,
    pub loc: Loc,
}

impl CodeExpr {
    pub fn id(&self) -> usize {
        match self {
            CodeExpr::BoolLiteral(e) => e.id,
            CodeExpr::CharLiteral(e) => e.id,
            CodeExpr::NullLiteral(e) => e.id,
            CodeExpr::IntLiteral(e) => e.id,
            CodeExpr::StringLiteral(e) => e.id,
            CodeExpr::ArrayLiteral(e) => e.id,
            CodeExpr::ResultLiteral(e) => e.id,
            CodeExpr::Return(e) => e.id,
            CodeExpr::Ident(e) => e.id,
            CodeExpr::InfixOp(e) => e.id,
            CodeExpr::If(e) => e.id,
            CodeExpr::Let(e) => e.id,
            CodeExpr::While(e) => e.id,
            CodeExpr::Break(e) => e.id,
            CodeExpr::For(e) => e.id,
            CodeExpr::Continue(e) => e.id,
            CodeExpr::Defer(e) => e.id,
            CodeExpr::Cast(e) => e.id,
            CodeExpr::StructLiteral(e) => e.id,
            CodeExpr::Assign(e) => e.id,
            CodeExpr::FieldAccess(e) => e.id,
            CodeExpr::Catch(e) => e.id,
            CodeExpr::Match(e) => e.id,
            CodeExpr::Paren(e) => e.id,
            CodeExpr::FnCall(e) => e.id,
            CodeExpr::MethodCall(e) => e.id,
            CodeExpr::InlineFnCall(e) => e.id,
            CodeExpr::IntrinsicCall(e) => e.id,
            CodeExpr::InlineMethodCall(e) => e.id,
            CodeExpr::Sizeof(e) => e.id,
            CodeExpr::PropagateError(e) => e.id,
            CodeExpr::PrefixOp(e) => e.id,
            CodeExpr::DoWith(e) => e.id,
            CodeExpr::Pipe(e) => e.id,
        }
    }

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
            CodeExpr::Pipe(e) => e.loc,
        }
    }
}
