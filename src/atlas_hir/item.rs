use atlas_core::prelude::Span;
use serde::Serialize;

use super::{signature::HirFunctionSignature, stmt::HirBlock};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize)]
pub enum HirItem<'hir> {
    Function(HirFunction<'hir>),
    Import(HirImport<'hir>),
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize)]
pub struct HirFunction<'hir> {
    pub span: Span,
    pub name: &'hir str,
    pub name_span: Span,
    pub signature: &'hir HirFunctionSignature<'hir>,
    pub body: HirBlock<'hir>,
}

/// Used by the type checker to import the API Signature of a module.
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize)]
pub struct HirImport<'hir> {
    pub span: Span,
    pub path: &'hir str,
    pub path_span: Span,

    /// As of now the alias is unsupported.
    pub alias: Option<&'hir str>,
    pub alias_span: Option<Span>,
}
