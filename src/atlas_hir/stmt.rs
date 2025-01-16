use atlas_core::prelude::Span;
use serde::Serialize;

use super::{expr::HirExpr, ty::HirTy};

/// Most of the statements could actually be replaced with
///
/// Statement::Expr(HirExpr)
/// Only the HirBlock & HirReturn is useful
#[derive(Debug, Clone, Serialize)]
pub(crate) enum HirStatement<'hir> {
    Block(HirBlock<'hir>),
    Return(HirReturn<'hir>),
    Expr(HirExprStmt<'hir>),
    Let(HirLetStmt<'hir>),
    IfElse(HirIfElseStmt<'hir>),
    While(HirWhileStmt<'hir>),
    Break(Span),
    Continue(Span),
}
#[derive(Debug, Clone, Serialize)]
pub(crate) struct HirExprStmt<'hir> {
    pub span: Span,
    pub expr: &'hir HirExpr<'hir>,
}
#[derive(Debug, Clone, Serialize)]
pub(crate) struct HirWhileStmt<'hir> {
    pub span: Span,
    pub condition: &'hir HirExpr<'hir>,
    pub body: &'hir HirBlock<'hir>,
}

/// Types will become optional in the future.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct HirLetStmt<'hir> {
    pub span: Span,
    pub name: &'hir str,
    pub name_span: Span,
    pub ty: &'hir HirTy<'hir>,
    pub value: &'hir HirExpr<'hir>,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct HirIfElseStmt<'hir> {
    pub span: Span,
    pub condition: &'hir HirExpr<'hir>,
    pub then_branch: &'hir HirBlock<'hir>,
    pub else_branch: Option<&'hir HirBlock<'hir>>,
}
#[derive(Debug, Clone, Serialize)]
pub(crate) struct HirReturn<'hir> {
    pub span: Span,
    pub value: &'hir HirExpr<'hir>,
    pub ty: &'hir HirTy<'hir>,
}
#[derive(Debug, Clone, Serialize)]
pub(crate) struct HirBlock<'hir> {
    pub span: Span,
    pub statements: Vec<&'hir HirStatement<'hir>>,
}
