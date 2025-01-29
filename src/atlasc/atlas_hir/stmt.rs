use super::{expr::HirExpr, ty::HirTy};
use logos::Span;
use serde::Serialize;

/// Most of the statements could actually be replaced with
///
/// Statement::Expr(HirExpr)
/// Only the HirBlock & HirReturn is useful
#[derive(Debug, Clone, Serialize)]
pub enum HirStatement<'hir> {
    _Block(HirBlock<'hir>),
    Return(HirReturn<'hir>),
    Expr(HirExprStmt<'hir>),
    Let(HirLetStmt<'hir>),
    //tbf, no need to use anything else than a LetStmt for the const
    Const(HirLetStmt<'hir>),
    IfElse(HirIfElseStmt<'hir>),
    While(HirWhileStmt<'hir>),
    Break(Span),
    Continue(Span),
}

impl HirStatement<'_> {
    pub fn span(&self) -> Span {
        match self {
            HirStatement::_Block(block) => block.span.clone(),
            HirStatement::Return(ret) => ret.span.clone(),
            HirStatement::Expr(expr) => expr.span.clone(),
            HirStatement::Let(let_stmt) => let_stmt.span.clone(),
            HirStatement::Const(const_stmt) => const_stmt.span.clone(),
            HirStatement::IfElse(if_else) => if_else.span.clone(),
            HirStatement::While(while_stmt) => while_stmt.span.clone(),
            HirStatement::Break(span) => span.clone(),
            HirStatement::Continue(span) => span.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct HirExprStmt<'hir> {
    pub span: Span,
    pub expr: HirExpr<'hir>,
}
#[derive(Debug, Clone, Serialize)]
pub struct HirWhileStmt<'hir> {
    pub span: Span,
    pub condition: HirExpr<'hir>,
    pub body: HirBlock<'hir>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HirLetStmt<'hir> {
    pub span: Span,
    pub name: &'hir str,
    pub name_span: Span,
    pub ty: Option<&'hir HirTy<'hir>>,
    pub ty_span: Option<Span>,
    pub value: HirExpr<'hir>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HirIfElseStmt<'hir> {
    pub span: Span,
    pub condition: HirExpr<'hir>,
    pub then_branch: HirBlock<'hir>,
    pub else_branch: Option<HirBlock<'hir>>,
}
#[derive(Debug, Clone, Serialize)]
pub struct HirReturn<'hir> {
    pub span: Span,
    pub value: HirExpr<'hir>,
    pub ty: &'hir HirTy<'hir>,
}
#[derive(Debug, Clone, Serialize)]
pub struct HirBlock<'hir> {
    pub span: Span,
    pub statements: Vec<HirStatement<'hir>>,
}
