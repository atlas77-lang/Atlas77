use super::{signature::HirFunctionSignature, stmt::HirBlock};
use crate::atlasc::atlas_hir::signature::{HirClassFieldSignature, HirClassSignature};
use logos::Span;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct HirFunction<'hir> {
    pub span: Span,
    pub name: &'hir str,
    pub name_span: Span,
    pub signature: &'hir HirFunctionSignature<'hir>,
    pub body: HirBlock<'hir>,
}

/// Used by the type checker to import the API Signature of a module.
#[derive(Debug, Clone, Serialize)]
pub struct HirImport<'hir> {
    pub span: Span,
    pub path: &'hir str,
    pub path_span: Span,

    /// As of now the alias is unsupported.
    pub alias: Option<&'hir str>,
    pub alias_span: Option<Span>,
}
#[derive(Debug, Clone, Serialize)]
pub struct HirClass<'hir> {
    pub span: Span,
    pub name: &'hir str,
    pub name_span: Span,
    pub signature: &'hir HirClassSignature<'hir>,
    pub methods: Vec<HirFunction<'hir>>,
    pub fields: Vec<HirClassFieldSignature<'hir>>,
}