use logos::Span;
use serde::Serialize;

/// An `AstProgram` is the top-level node of the AST and contains all the items.
#[derive(Debug, Clone, Serialize, Copy)]
pub struct AstProgram<'ast> {
    pub items: &'ast [&'ast AstItem<'ast>],
}

/// An `Item` is anything that can be declared at the top-level scope of a program.
/// This currently means functions, classes & structs declarations
///
/// Enums & unions are also top-level items, but they are not yet supported
#[derive(Debug, Clone, Serialize)]
//todo: Add classes and a trait-ish stuff
pub enum AstItem<'ast> {
    Import(AstImport<'ast>),
    Enum(AstEnum<'ast>),
    Class(AstClass<'ast>),
    Struct(AstStruct<'ast>),
    ExternFunction(AstExternFunction<'ast>),
    Func(AstFunction<'ast>),
}

impl AstItem<'_> {
    pub fn set_vis(&mut self, vis: AstVisibility) {
        match self {
            AstItem::Import(_) => {}
            AstItem::Enum(v) => v.vis = vis,
            AstItem::Class(v) => v.vis = vis,
            AstItem::Struct(v) => v.vis = vis,
            AstItem::ExternFunction(v) => v.vis = vis,
            AstItem::Func(v) => v.vis = vis,
        }
    }
    fn span(&self) -> Span {
        match self {
            AstItem::Import(v) => v.span.clone(),
            AstItem::Enum(v) => v.span.clone(),
            AstItem::Class(v) => v.span.clone(),
            AstItem::Struct(v) => v.span.clone(),
            AstItem::ExternFunction(v) => v.span.clone(),
            AstItem::Func(v) => v.span.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Default, Copy)]
pub enum AstVisibility {
    Public,
    #[default]
    Private,
}
#[derive(Debug, Clone, Serialize)]
pub struct AstClass<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub name_span: Span,
    pub vis: AstVisibility,
    pub fields: &'ast [&'ast AstObjField<'ast>],
    pub field_span: Span,
    pub methods: &'ast [&'ast AstFunction<'ast>],
}

#[derive(Debug, Clone, Serialize)]
pub struct AstFunction<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub args: &'ast [&'ast AstObjField<'ast>],
    pub ret: &'ast AstType<'ast>,
    pub body: &'ast AstBlock<'ast>,
    pub vis: AstVisibility,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstUnionVariant<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub fields: &'ast [&'ast AstObjField<'ast>],
}

#[derive(Debug, Clone, Serialize)]
pub struct AstEnum<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub variants: &'ast [&'ast AstEnumVariant<'ast>],
    pub vis: AstVisibility,
}

#[derive(Debug, Clone, Serialize)]
/// Enums currently don't support associated values
pub struct AstEnumVariant<'ast> {
    pub span: Span,
    ///todo: Maybe change from [``AstIdentifier``] to &'ast str
    pub name: &'ast AstIdentifier<'ast>,
    /// Some(i32) if the user specified a value for the variant
    ///
    /// None if the user didn't specify a value
    ///
    /// During ast lowering, the compiler will assign a value to the variant
    pub val: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstStruct<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub fields: &'ast [&'ast AstObjField<'ast>],
    pub vis: AstVisibility,
}

#[derive(Debug, Clone, Serialize)]
///todo: Rename because it's also used by functions
pub struct AstObjField<'ast> {
    pub span: Span,
    /// In a function or a struct the visibility is always public
    pub name: &'ast AstIdentifier<'ast>,
    pub ty: &'ast AstType<'ast>,
    pub vis: AstVisibility,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstExternFunction<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub args_name: &'ast [&'ast AstIdentifier<'ast>],
    pub args_ty: &'ast [&'ast AstType<'ast>],
    pub ret: &'ast AstType<'ast>,
    pub vis: AstVisibility,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstImport<'ast> {
    pub span: Span,
    pub path: &'ast str,
    pub alias: Option<&'ast AstIdentifier<'ast>>,
}

#[derive(Debug, Clone, Serialize)]
pub enum AstStatement<'ast> {
    Let(AstLet<'ast>),
    Const(AstConst<'ast>),
    IfElse(AstIfElseExpr<'ast>),
    Block(AstBlock<'ast>),
    While(AstWhileExpr<'ast>),
    Expr(AstExpr<'ast>),
    Break(AstBreakStmt),
    Continue(AstContinueStmt),
    Return(AstReturnStmt<'ast>),
    _InnerFunc(AstFunction<'ast>),
}

impl AstStatement<'_> {
    pub fn span(&self) -> Span {
        match self {
            AstStatement::Let(e) => e.span.clone(),
            AstStatement::Const(e) => e.span.clone(),
            AstStatement::IfElse(e) => e.span.clone(),
            AstStatement::_InnerFunc(e) => e.span.clone(),
            AstStatement::Block(e) => e.span.clone(),
            AstStatement::While(e) => e.span.clone(),
            AstStatement::Expr(e) => e.span(),
            AstStatement::Break(e) => e.span.clone(),
            AstStatement::Continue(e) => e.span.clone(),
            AstStatement::Return(e) => e.span.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AstContinueStmt {
    pub span: Span,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstBreakStmt {
    pub span: Span,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstConst<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub ty: Option<&'ast AstType<'ast>>,
    pub value: &'ast AstExpr<'ast>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstWhileExpr<'ast> {
    pub span: Span,
    pub condition: &'ast AstExpr<'ast>,
    pub body: &'ast AstBlock<'ast>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstAssignExpr<'ast> {
    pub span: Span,
    pub target: &'ast AstExpr<'ast>,
    pub value: &'ast AstExpr<'ast>,
}

#[derive(Debug, Clone, Serialize)]
pub enum AstExpr<'ast> {
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
    StaticAccess(AstStaticAccessExpr<'ast>),
    NewObj(AstNewObjExpr<'ast>),
    _Block(AstBlock<'ast>),
    Assign(AstAssignExpr<'ast>),
    Casting(AstCastingExpr<'ast>),
    //Tuple(AstTupleExpr<'ast>),
}

impl AstExpr<'_> {
    pub(crate) fn span(&self) -> Span {
        match self {
            AstExpr::_Lambda(e) => e.span.clone(),
            AstExpr::_CompTime(e) => e.span.clone(),
            AstExpr::IfElse(e) => e.span.clone(),
            AstExpr::BinaryOp(e) => e.span.clone(),
            AstExpr::UnaryOp(e) => e.span.clone(),
            AstExpr::Call(e) => e.span.clone(),
            AstExpr::Literal(e) => e.span(),
            AstExpr::Identifier(e) => e.span.clone(),
            AstExpr::Indexing(e) => e.span.clone(),
            AstExpr::FieldAccess(e) => e.span.clone(),
            AstExpr::StaticAccess(e) => e.span.clone(),
            AstExpr::NewObj(e) => e.span.clone(),
            AstExpr::_Block(e) => e.span.clone(),
            AstExpr::Assign(e) => e.span.clone(),
            AstExpr::Casting(e) => e.span.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
/// i.e. ``5 as f64``
pub struct AstCastingExpr<'ast> {
    pub span: Span,
    pub ty: &'ast AstType<'ast>,
    pub value: &'ast AstExpr<'ast>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstReturnStmt<'ast> {
    pub span: Span,
    pub value: &'ast AstExpr<'ast>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstBlock<'ast> {
    pub span: Span,
    pub stmts: &'ast [&'ast AstStatement<'ast>],
}

#[derive(Debug, Clone, Serialize)]
pub struct AstNewObjExpr<'ast> {
    pub span: Span,
    pub ty: &'ast AstIdentifier<'ast>,
    pub fields: &'ast [&'ast AstFieldInit<'ast>],
}

#[derive(Debug, Clone, Serialize)]
pub struct AstFieldInit<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub value: &'ast AstExpr<'ast>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstStaticAccessExpr<'ast> {
    pub span: Span,
    pub target: &'ast AstExpr<'ast>,
    pub field: &'ast AstIdentifier<'ast>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstFieldAccessExpr<'ast> {
    pub span: Span,
    pub target: &'ast AstExpr<'ast>,
    pub field: &'ast AstIdentifier<'ast>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstIndexingExpr<'ast> {
    pub span: Span,
    pub target: &'ast AstExpr<'ast>,
    pub index: &'ast AstExpr<'ast>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstCallExpr<'ast> {
    pub span: Span,
    pub callee: &'ast AstExpr<'ast>,
    pub args: &'ast [&'ast AstExpr<'ast>],
}

#[derive(Debug, Clone, Serialize)]
pub struct AstUnaryOpExpr<'ast> {
    pub span: Span,
    pub expr: &'ast AstExpr<'ast>,
    pub op: Option<AstUnaryOp>,
}

#[derive(Debug, Clone, Serialize)]
pub enum AstUnaryOp {
    Neg,
    Not,
    _Deref,
    _AsRef,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstBinaryOpExpr<'ast> {
    pub span: Span,
    pub lhs: &'ast AstExpr<'ast>,
    pub rhs: &'ast AstExpr<'ast>,
    pub op: AstBinaryOp,
}

#[derive(Debug, Clone, Serialize)]
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

#[derive(Debug, Clone, Serialize)]
pub struct AstIfElseExpr<'ast> {
    pub span: Span,
    pub condition: &'ast AstExpr<'ast>,
    pub body: &'ast AstBlock<'ast>,
    pub else_body: Option<&'ast AstBlock<'ast>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstLet<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
    pub ty: Option<&'ast AstType<'ast>>,
    pub value: &'ast AstExpr<'ast>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstLambdaExpr<'ast> {
    pub span: Span,
    pub args: &'ast [&'ast AstIdentifier<'ast>],
    pub body: &'ast AstExpr<'ast>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstCompTimeExpr<'ast> {
    pub span: Span,
    pub expr: &'ast AstExpr<'ast>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstIdentifier<'ast> {
    pub span: Span,
    pub name: &'ast str,
}

#[derive(Debug, Clone, Serialize)]
pub enum AstLiteral<'ast> {
    Integer(AstIntegerLiteral),
    UnsignedInteger(AstUnsignedIntegerLiteral),
    Float(AstFloatLiteral),
    String(AstStringLiteral<'ast>),
    Boolean(AstBooleanLiteral),
    List(AstListLiteral<'ast>),
}

impl AstLiteral<'_> {
    pub(crate) fn span(&self) -> Span {
        match self {
            AstLiteral::Integer(l) => l.span.clone(),
            AstLiteral::UnsignedInteger(l) => l.span.clone(),
            AstLiteral::Float(l) => l.span.clone(),
            AstLiteral::String(l) => l.span.clone(),
            AstLiteral::Boolean(l) => l.span.clone(),
            AstLiteral::List(l) => l.span.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AstListLiteral<'ast> {
    pub span: Span,
    pub items: &'ast [&'ast AstExpr<'ast>],
}

#[derive(Debug, Clone, Serialize)]
pub struct AstBooleanLiteral {
    pub span: Span,
    pub value: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstStringLiteral<'ast> {
    pub span: Span,
    pub value: &'ast str,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstFloatLiteral {
    pub span: Span,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstUnsignedIntegerLiteral {
    pub span: Span,
    pub value: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstIntegerLiteral {
    pub span: Span,
    pub value: i64,
}

#[derive(Debug, Clone, Serialize)]
pub enum AstType<'ast> {
    Unit(AstUnitType),
    Boolean(AstBooleanType),
    Integer(AstIntegerType),
    Float(AstFloatType),
    UnsignedInteger(AstUnsignedIntegerType),
    String(AstStringType),
    Named(AstNamedType<'ast>),
    Pointer(AstPointerType<'ast>),
    Function(AstFunctionType<'ast>),
    List(AstListType<'ast>),
    Generic(AstGenericType<'ast>),
}

impl AstType<'_> {
    pub(crate) fn span(&self) -> Span {
        match self {
            AstType::Unit(t) => t.span.clone(),
            AstType::Boolean(t) => t.span.clone(),
            AstType::Integer(t) => t.span.clone(),
            AstType::Float(t) => t.span.clone(),
            AstType::UnsignedInteger(t) => t.span.clone(),
            AstType::String(t) => t.span.clone(),
            AstType::Named(t) => t.span.clone(),
            AstType::Pointer(t) => t.span.clone(),
            AstType::Function(t) => t.span.clone(),
            AstType::List(t) => t.span.clone(),
            AstType::Generic(t) => t.span.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
/// A generic type in atlas as the form of `@T`
pub struct AstGenericType<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
}

#[derive(Debug, Clone, Serialize)]
///A List type in atlas as the form of `[T]`
pub struct AstListType<'ast> {
    pub span: Span,
    pub inner: &'ast AstType<'ast>,
}

#[derive(Debug, Clone, Serialize)]
///todo: Add support for generic types and constraints (i.e. `T: Display`)
pub struct AstFunctionType<'ast> {
    pub span: Span,
    pub args: &'ast [&'ast AstType<'ast>],
    pub ret: &'ast AstType<'ast>,
}

#[derive(Debug, Clone, Serialize)]
///A pointer type in atlas as the form of `&T`
pub struct AstPointerType<'ast> {
    pub span: Span,
    pub inner: &'ast AstType<'ast>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstStringType {
    pub span: Span,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstNamedType<'ast> {
    pub span: Span,
    pub name: &'ast AstIdentifier<'ast>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstIntegerType {
    pub span: Span,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstFloatType {
    pub span: Span,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstUnsignedIntegerType {
    pub span: Span,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstBooleanType {
    pub span: Span,
}

#[derive(Debug, Clone, Serialize)]
pub struct AstUnitType {
    pub span: Span,
}
