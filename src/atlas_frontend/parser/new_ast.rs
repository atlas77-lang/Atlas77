// TODO!: Document this file

use serde::Serialize;

use atlas_core::utils::span::*;

/// An `AstProgram` is the top-level node of the AST and contains all the items.
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstProgram<'ast> {
    pub items: &'ast [&'ast AstItem<'ast>],
}

/// An `Item` is anything that can be declared at the top-level scope of a program.
/// This currently means variables & structs declarations
///
/// Enums & unions are also top-level items, but they are not yet supported
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub enum AstItem<'ast> {
    Struct(AstStruct<'ast>),
    ExternFunction(AstExternFunction<'ast>),
    Func(AstFunction<'ast>),
    Enum(AstEnum<'ast>),
    Union(AstUnion<'ast>),
    Include(AstInclude<'ast>),
}

impl Spanned for AstItem<'_> {
    fn span(&self) -> Span {
        match self {
            AstItem::Struct(v) => v.span,
            AstItem::ExternFunction(v) => v.span,
            AstItem::Func(v) => v.span,
            AstItem::Enum(v) => v.span,
            AstItem::Union(v) => v.span,
            AstItem::Include(v) => v.span,
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstFunction<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub args: &'ast [&'ast AstObjField<'ast>],
    pub ret: &'ast AstType<'ast>,
    pub body: &'ast AstExpr<'ast>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstUnion<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub variants: &'ast [&'ast AstUnionVariant<'ast>],
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstUnionVariant<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub fields: &'ast [&'ast AstObjField<'ast>],
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstEnum<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub variants: &'ast [&'ast AstEnumVariant<'ast>],
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
/// Enums currently don't support associated values
pub struct AstEnumVariant<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstStruct<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub fields: &'ast [&'ast AstObjField<'ast>],
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstObjField<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub ty: &'ast AstType<'ast>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstExternFunction<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub args: &'ast [&'ast AstType<'ast>],
    pub ret: &'ast AstType<'ast>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstInclude<'ast> {
    pub span: Span,
    pub path: &'ast str,
    pub alias: Option<&'ast AstIdentifier<'ast>>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub enum AstExpr<'ast> {
    Let(AstLetExpr<'ast>),
    Lambda(AstLambdaExpr<'ast>),
    CompTime(AstCompTimeExpr<'ast>),
    Match(AstMatchExpr<'ast>),
    IfElse(AstIfElseExpr<'ast>),
    BinaryOp(AstBinaryOpExpr<'ast>),
    UnaryOp(AstUnaryOpExpr<'ast>),
    Call(AstCallExpr<'ast>),
    Do(AstDoExpr<'ast>),
    Literal(AstLiteral<'ast>),
    Identifier(AstIdentifier<'ast>),
    Indexing(AstIndexingExpr<'ast>),
    FieldAccess(AstFieldAccessExpr<'ast>),
    NewObj(AstNewObjExpr<'ast>),
    //Tuple(AstTupleExpr<'ast>),
}

impl Spanned for AstExpr<'_> {
    fn span(&self) -> Span {
        match self {
            AstExpr::Let(e) => e.span,
            AstExpr::Lambda(e) => e.span,
            AstExpr::CompTime(e) => e.span,
            AstExpr::Match(e) => e.span,
            AstExpr::IfElse(e) => e.span,
            AstExpr::BinaryOp(e) => e.span,
            AstExpr::UnaryOp(e) => e.span,
            AstExpr::Call(e) => e.span,
            AstExpr::Do(e) => e.span,
            AstExpr::Literal(e) => e.span(),
            AstExpr::Identifier(e) => e.span,
            AstExpr::Indexing(e) => e.span,
            AstExpr::FieldAccess(e) => e.span,
            AstExpr::NewObj(e) => e.span,
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstNewObjExpr<'ast> {
    pub span: Span,
    pub ty: &'ast AstType<'ast>,
    pub fields: &'ast [&'ast AstFieldInit<'ast>],
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstFieldInit<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub value: &'ast AstExpr<'ast>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstFieldAccessExpr<'ast> {
    pub span: Span,
    pub target: &'ast AstExpr<'ast>,
    pub field: &'ast AstIdentifier<'ast>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstIndexingExpr<'ast> {
    pub span: Span,
    pub target: &'ast AstExpr<'ast>,
    pub index: &'ast AstExpr<'ast>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstDoExpr<'ast> {
    pub span: Span,
    pub exprs: &'ast [&'ast AstExpr<'ast>],
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstCallExpr<'ast> {
    pub span: Span,
    pub callee: &'ast AstExpr<'ast>,
    pub args: &'ast [&'ast AstExpr<'ast>],
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstUnaryOpExpr<'ast> {
    pub span: Span,
    pub expr: &'ast AstExpr<'ast>,
    pub op: Option<AstUnaryOp>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub enum AstUnaryOp {
    Neg,
    Not,
    Deref,
    AsRef,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstBinaryOpExpr<'ast> {
    pub span: Span,
    pub lhs: &'ast AstExpr<'ast>,
    pub rhs: &'ast AstExpr<'ast>,
    pub op: AstBinaryOp,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub enum AstBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Eq,
    NEq,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstIfElseExpr<'ast> {
    pub span: Span,
    pub condition: &'ast AstExpr<'ast>,
    pub then_expr: &'ast AstExpr<'ast>,
    pub else_expr: Option<&'ast AstExpr<'ast>>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstMatchExpr<'ast> {
    pub span: Span,
    pub expr: &'ast AstExpr<'ast>,
    pub arms: &'ast [&'ast AstMatchArm<'ast>],
    pub default: Option<&'ast AstExpr<'ast>>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstMatchArm<'ast> {
    pub span: Span,
    pub pattern: &'ast AstPattern<'ast>,
    pub expr: &'ast AstExpr<'ast>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstPattern<'ast> {
    pub span: Span,
    pub kind: AstPatternKind<'ast>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
// TODO: Add support for tuples, enums and structs
pub enum AstPatternKind<'ast> {
    Identifier(&'ast AstIdentifier<'ast>),
    Literal(AstLiteral<'ast>),
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstLetExpr<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub ty: Option<&'ast AstType<'ast>>,
    pub value: &'ast AstExpr<'ast>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstLambdaExpr<'ast> {
    pub span: Span,
    pub args: &'ast [&'ast AstIdentifier<'ast>],
    pub body: &'ast AstExpr<'ast>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstCompTimeExpr<'ast> {
    pub span: Span,
    pub expr: &'ast AstExpr<'ast>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstIdentifier<'ast> {
    pub span: Span,
    pub name: &'ast str,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub enum AstLiteral<'ast> {
    Integer(AstIntegerLiteral),
    UnsignedIntegerer(AstUnsignedIntegerLiteral),
    Float(AstFloatLiteral),
    String(AstStringLiteral<'ast>),
    Boolean(AstBooleanLiteral),
    List(AstListLiteral<'ast>),
}

impl Spanned for AstLiteral<'_> {
    fn span(&self) -> Span {
        match self {
            AstLiteral::Integer(l) => l.span,
            AstLiteral::UnsignedIntegerer(l) => l.span,
            AstLiteral::Float(l) => l.span,
            AstLiteral::String(l) => l.span,
            AstLiteral::Boolean(l) => l.span,
            AstLiteral::List(l) => l.span,
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstListLiteral<'ast> {
    pub span: Span,
    pub items: &'ast [&'ast AstExpr<'ast>],
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstBooleanLiteral {
    pub span: Span,
    pub value: bool,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstStringLiteral<'ast> {
    pub span: Span,
    pub value: &'ast str,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstFloatLiteral {
    pub span: Span,
    pub value: f64,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstUnsignedIntegerLiteral {
    pub span: Span,
    pub value: u64,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstIntegerLiteral {
    pub span: Span,
    pub value: i64,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub enum AstType<'ast> {
    Unit(AstUnitType),
    Boolean(AstBooleanType),
    Integer(AstIntegerType),
    Float(AstFloatType),
    UnsignedIntegerer(AstUnsignedIntegerType),
    String(AstStringType),
    Named(AstNamedType<'ast>),
    Pointer(AstPointerType<'ast>),
    Function(AstFunctionType<'ast>),
    List(AstListType<'ast>),
    Map(AstMapType<'ast>),
    //Tuple(AstTupleType<'ast>),
}

impl Spanned for AstType<'_> {
    fn span(&self) -> Span {
        match self {
            AstType::Unit(t) => t.span,
            AstType::Boolean(t) => t.span,
            AstType::Integer(t) => t.span,
            AstType::Float(t) => t.span,
            AstType::UnsignedIntegerer(t) => t.span,
            AstType::String(t) => t.span,
            AstType::Named(t) => t.span,
            AstType::Pointer(t) => t.span,
            AstType::Function(t) => t.span,
            AstType::List(t) => t.span,
            AstType::Map(t) => t.span,
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstMapType<'ast> {
    pub span: Span,
    pub key: &'ast AstType<'ast>,
    pub value: &'ast AstType<'ast>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstListType<'ast> {
    pub span: Span,
    pub inner: &'ast AstType<'ast>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstFunctionType<'ast> {
    pub span: Span,
    pub args: &'ast [&'ast AstType<'ast>],
    pub ret: &'ast AstType<'ast>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstPointerType<'ast> {
    pub span: Span,
    pub inner: &'ast AstType<'ast>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstStringType {
    pub span: Span,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstNamedType<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstIntegerType {
    pub span: Span,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstFloatType {
    pub span: Span,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstUnsignedIntegerType {
    pub span: Span,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstBooleanType {
    pub span: Span,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Copy)]
pub struct AstUnitType {
    pub span: Span,
}
