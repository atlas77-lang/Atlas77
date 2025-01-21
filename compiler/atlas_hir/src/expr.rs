use atlas_core::prelude::{Span, Spanned};
use serde::Serialize;

use super::ty::HirTy;

#[derive(Debug, Clone, Serialize)]
//todo: Add arrays/struct & class init literal
pub enum HirExpr<'hir> {
    Assign(HirAssignExpr<'hir>),
    HirBinaryOp(HirBinaryOpExpr<'hir>),
    Call(HirFunctionCallExpr<'hir>),
    Ident(HirIdentExpr<'hir>),
    Unary(UnaryOpExpr<'hir>),
    FloatLiteral(HirFloatLiteralExpr<'hir>),
    IntegerLiteral(HirIntegerLiteralExpr<'hir>),
    BooleanLiteral(HirBooleanLiteralExpr<'hir>),
    UnsignedIntegerLiteral(HirUnsignedIntegerLiteralExpr<'hir>),
    _StringLiteral(HirStringLiteralExpr<'hir>),
}

impl Spanned for HirExpr<'_> {
    fn span(&self) -> Span {
        match self {
            HirExpr::Ident(expr) => expr.span,
            HirExpr::IntegerLiteral(expr) => expr.span,
            HirExpr::UnsignedIntegerLiteral(expr) => expr.span,
            HirExpr::BooleanLiteral(expr) => expr.span,
            HirExpr::FloatLiteral(expr) => expr.span,
            HirExpr::Unary(expr) => expr.span,
            HirExpr::HirBinaryOp(expr) => expr.span,
            HirExpr::Call(expr) => expr.span,
            HirExpr::Assign(expr) => expr.span,
            HirExpr::_StringLiteral(expr) => expr.span,
        }
    }
}

impl<'hir> HirExpr<'hir> {
    pub fn ty(&self) -> &'hir HirTy<'hir> {
        match self {
            HirExpr::Ident(expr) => expr.ty,
            HirExpr::IntegerLiteral(expr) => expr.ty,
            HirExpr::UnsignedIntegerLiteral(expr) => expr.ty,
            HirExpr::BooleanLiteral(expr) => expr.ty,
            HirExpr::FloatLiteral(expr) => expr.ty,
            HirExpr::Unary(expr) => expr.ty,
            HirExpr::HirBinaryOp(expr) => expr.ty,
            HirExpr::Call(expr) => expr.ty,
            HirExpr::Assign(expr) => expr.ty,
            HirExpr::_StringLiteral(expr) => expr.ty,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct HirBooleanLiteralExpr<'hir> {
    pub value: bool,
    pub span: Span,
    pub ty: &'hir HirTy<'hir>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HirStringLiteralExpr<'hir> {
    pub value: &'hir str,
    pub span: Span,
    pub ty: &'hir HirTy<'hir>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HirAssignExpr<'hir> {
    pub span: Span,
    pub lhs: Box<HirExpr<'hir>>,
    pub rhs: Box<HirExpr<'hir>>,
    pub ty: &'hir HirTy<'hir>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HirFunctionCallExpr<'hir> {
    pub span: Span,
    /// The callee can be any kind of expression (e.g. ``Rectangle::new()`` or ``MyStruct.some_fn_ptr()`` or ``MyOtherStruct.some_array_of_fn[0]()``)
    pub callee: Box<HirExpr<'hir>>,
    pub callee_span: Span,
    pub args: Vec<HirExpr<'hir>>,
    pub args_ty: Vec<&'hir HirTy<'hir>>,
    /// Result type of the call
    pub ty: &'hir HirTy<'hir>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HirBinaryOpExpr<'hir> {
    pub span: Span,
    pub op: HirBinaryOp,
    pub op_span: Span,
    pub lhs: Box<HirExpr<'hir>>,
    pub rhs: Box<HirExpr<'hir>>,
    /// The type of the result of the expression.
    pub ty: &'hir HirTy<'hir>,
}

#[derive(Debug, Clone, Serialize)]
pub enum HirBinaryOp {
    Add,
    And,
    Div,
    Eq,
    Gt,
    Gte,
    Lt,
    Lte,
    Mod,
    Mul,
    Neq,
    Or,
    Sub,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnaryOpExpr<'hir> {
    pub span: Span,
    pub op: Option<UnaryOp>,
    pub expr: Box<HirExpr<'hir>>,
    /// The type of the result of the expression.
    pub ty: &'hir HirTy<'hir>,
}

#[derive(Debug, Clone, Serialize)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, Serialize)]
pub struct HirFloatLiteralExpr<'hir> {
    pub value: f64,
    pub span: Span,
    pub ty: &'hir HirTy<'hir>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HirUnsignedIntegerLiteralExpr<'hir> {
    pub value: u64,
    pub span: Span,
    pub ty: &'hir HirTy<'hir>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HirIntegerLiteralExpr<'hir> {
    pub value: i64,
    pub span: Span,
    pub ty: &'hir HirTy<'hir>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HirIdentExpr<'hir> {
    pub name: &'hir str,
    pub span: Span,
    pub ty: &'hir HirTy<'hir>,
}
