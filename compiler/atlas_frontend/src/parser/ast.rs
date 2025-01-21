use serde::Serialize;

use atlas_core::utils::span::*;

/// An `AstProgram` is the top-level node of the AST and contains all the items.
#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstProgram<'ast> {
    pub items: &'ast [&'ast AstItem<'ast>],
}

/// An `Item` is anything that can be declared at the top-level scope of a program.
/// This currently means functions, classes & structs declarations
///
/// Enums & unions are also top-level items, but they are not yet supported
#[derive(Debug, Clone, Serialize, Copy)]
//todo: Add classes and a trait-ish stuff
pub enum AstItem<'ast> {
    Struct(AstStruct<'ast>),
    ExternFunction(AstExternFunction<'ast>),
    Func(AstFunction<'ast>),
    _Enum(AstEnum<'ast>),
    ///Should unions be supported?
    Import(AstImport<'ast>),
}

impl Spanned for AstItem<'_> {
    fn span(&self) -> Span {
        match self {
            AstItem::Struct(v) => v.span,
            AstItem::ExternFunction(v) => v.span,
            AstItem::Func(v) => v.span,
            AstItem::_Enum(v) => v.span,
            AstItem::Import(v) => v.span,
        }
    }
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstFunction<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub args: &'ast [&'ast AstObjField<'ast>],
    pub ret: &'ast AstType<'ast>,
    pub body: &'ast AstBlock<'ast>,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstUnionVariant<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub fields: &'ast [&'ast AstObjField<'ast>],
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstEnum<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub variants: &'ast [&'ast AstEnumVariant<'ast>],
}

#[derive(Debug, Clone, Serialize, Copy)]
/// Enums currently don't support associated values
pub struct AstEnumVariant<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstStruct<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub fields: &'ast [&'ast AstObjField<'ast>],
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstObjField<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub ty: &'ast AstType<'ast>,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstExternFunction<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub args_name: &'ast [&'ast AstIdentifier<'ast>],
    pub args_ty: &'ast [&'ast AstType<'ast>],
    pub ret: &'ast AstType<'ast>,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstImport<'ast> {
    pub span: Span,
    pub path: &'ast str,
    pub alias: Option<&'ast AstIdentifier<'ast>>,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub enum AstStatement<'ast> {
    Let(AstLetExpr<'ast>),
    Const(AstConstExpr<'ast>),
    IfElse(AstIfElseExpr<'ast>),
    _InnerFunc(AstFunction<'ast>),
    _Block(AstBlock<'ast>),
    _Call(AstCallExpr<'ast>),
    While(AstWhileExpr<'ast>),
    Expr(AstExpr<'ast>),
    Break(AstBreakStmt),
    Continue(AstContinueStmt),
    Return(AstReturnStmt<'ast>),
}

impl AstStatement<'_> {
    pub fn span(&self) -> Span {
        match self {
            AstStatement::Let(e) => e.span,
            AstStatement::Const(e) => e.span,
            AstStatement::IfElse(e) => e.span,
            AstStatement::_InnerFunc(e) => e.span,
            AstStatement::_Block(e) => e.span,
            AstStatement::_Call(e) => e.span,
            AstStatement::While(e) => e.span,
            AstStatement::Expr(e) => e.span(),
            AstStatement::Break(e) => e.span,
            AstStatement::Continue(e) => e.span,
            AstStatement::Return(e) => e.span,
        }
    }
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstContinueStmt {
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstBreakStmt {
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstConstExpr<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub ty: Option<&'ast AstType<'ast>>,
    pub value: &'ast AstExpr<'ast>,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstWhileExpr<'ast> {
    pub span: Span,
    pub condition: &'ast AstExpr<'ast>,
    pub body: &'ast AstBlock<'ast>,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstAssignExpr<'ast> {
    pub span: Span,
    pub target: &'ast AstExpr<'ast>,
    pub value: &'ast AstExpr<'ast>,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub enum AstExpr<'ast> {
    _Let(AstLetExpr<'ast>),
    _Lambda(AstLambdaExpr<'ast>),
    _CompTime(AstCompTimeExpr<'ast>),
    IfElse(AstIfElseExpr<'ast>),
    BinaryOp(AstBinaryOpExpr<'ast>),
    UnaryOp(AstUnaryOpExpr<'ast>),
    Call(AstCallExpr<'ast>),
    Literal(AstLiteral<'ast>),
    Identifier(AstIdentifier<'ast>),
    Indexing(AstIndexingExpr<'ast>),
    FieldAccess(AstFieldAccessExpr<'ast>),
    _NewObj(AstNewObjExpr<'ast>),
    _Block(AstBlock<'ast>),
    Assign(AstAssignExpr<'ast>),
    //Tuple(AstTupleExpr<'ast>),
}

impl Spanned for AstExpr<'_> {
    fn span(&self) -> Span {
        match self {
            AstExpr::_Let(e) => e.span,
            AstExpr::_Lambda(e) => e.span,
            AstExpr::_CompTime(e) => e.span,
            AstExpr::IfElse(e) => e.span,
            AstExpr::BinaryOp(e) => e.span,
            AstExpr::UnaryOp(e) => e.span,
            AstExpr::Call(e) => e.span,
            AstExpr::Literal(e) => e.span(),
            AstExpr::Identifier(e) => e.span,
            AstExpr::Indexing(e) => e.span,
            AstExpr::FieldAccess(e) => e.span,
            AstExpr::_NewObj(e) => e.span,
            AstExpr::_Block(e) => e.span,
            AstExpr::Assign(e) => e.span,
        }
    }
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstReturnStmt<'ast> {
    pub span: Span,
    pub value: &'ast AstExpr<'ast>,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstBlock<'ast> {
    pub span: Span,
    pub stmts: &'ast [&'ast AstStatement<'ast>],
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstNewObjExpr<'ast> {
    pub span: Span,
    pub ty: &'ast AstType<'ast>,
    pub fields: &'ast [&'ast AstFieldInit<'ast>],
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstFieldInit<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub value: &'ast AstExpr<'ast>,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstFieldAccessExpr<'ast> {
    pub span: Span,
    pub target: &'ast AstExpr<'ast>,
    pub field: &'ast AstIdentifier<'ast>,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstIndexingExpr<'ast> {
    pub span: Span,
    pub target: &'ast AstExpr<'ast>,
    pub index: &'ast AstExpr<'ast>,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstCallExpr<'ast> {
    pub span: Span,
    pub callee: &'ast AstExpr<'ast>,
    pub args: &'ast [&'ast AstExpr<'ast>],
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstUnaryOpExpr<'ast> {
    pub span: Span,
    pub expr: &'ast AstExpr<'ast>,
    pub op: Option<AstUnaryOp>,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub enum AstUnaryOp {
    Neg,
    Not,
    _Deref,
    _AsRef,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstBinaryOpExpr<'ast> {
    pub span: Span,
    pub lhs: &'ast AstExpr<'ast>,
    pub rhs: &'ast AstExpr<'ast>,
    pub op: AstBinaryOp,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub enum AstBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    NEq,
    Lt,
    Lte,
    Gt,
    Gte,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstIfElseExpr<'ast> {
    pub span: Span,
    pub condition: &'ast AstExpr<'ast>,
    pub body: &'ast AstBlock<'ast>,
    pub else_body: Option<&'ast AstBlock<'ast>>,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstLetExpr<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub ty: Option<&'ast AstType<'ast>>,
    pub value: &'ast AstExpr<'ast>,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstLambdaExpr<'ast> {
    pub span: Span,
    pub args: &'ast [&'ast AstIdentifier<'ast>],
    pub body: &'ast AstExpr<'ast>,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstCompTimeExpr<'ast> {
    pub span: Span,
    pub expr: &'ast AstExpr<'ast>,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstIdentifier<'ast> {
    pub span: Span,
    pub name: &'ast str,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub enum AstLiteral<'ast> {
    Integer(AstIntegerLiteral),
    UnsignedIntegerer(AstUnsignedIntegerLiteral),
    Float(AstFloatLiteral),
    String(AstStringLiteral<'ast>),
    Boolean(AstBooleanLiteral),
    _List(AstListLiteral<'ast>),
}

impl Spanned for AstLiteral<'_> {
    fn span(&self) -> Span {
        match self {
            AstLiteral::Integer(l) => l.span,
            AstLiteral::UnsignedIntegerer(l) => l.span,
            AstLiteral::Float(l) => l.span,
            AstLiteral::String(l) => l.span,
            AstLiteral::Boolean(l) => l.span,
            AstLiteral::_List(l) => l.span,
        }
    }
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstListLiteral<'ast> {
    pub span: Span,
    pub items: &'ast [&'ast AstExpr<'ast>],
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstBooleanLiteral {
    pub span: Span,
    pub value: bool,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstStringLiteral<'ast> {
    pub span: Span,
    pub value: &'ast str,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstFloatLiteral {
    pub span: Span,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstUnsignedIntegerLiteral {
    pub span: Span,
    pub value: u64,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstIntegerLiteral {
    pub span: Span,
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, Copy)]
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
    _List(AstListType<'ast>),
    _Map(AstMapType<'ast>),
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
            AstType::_List(t) => t.span,
            AstType::_Map(t) => t.span,
        }
    }
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstMapType<'ast> {
    pub span: Span,
    pub key: &'ast AstType<'ast>,
    pub value: &'ast AstType<'ast>,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstListType<'ast> {
    pub span: Span,
    pub inner: &'ast AstType<'ast>,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstFunctionType<'ast> {
    pub span: Span,
    pub args: &'ast [&'ast AstType<'ast>],
    pub ret: &'ast AstType<'ast>,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstPointerType<'ast> {
    pub span: Span,
    pub inner: &'ast AstType<'ast>,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstStringType {
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstNamedType<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstIntegerType {
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstFloatType {
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstUnsignedIntegerType {
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstBooleanType {
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstUnitType {
    pub span: Span,
}
