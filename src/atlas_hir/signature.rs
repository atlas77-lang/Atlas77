use std::collections::BTreeMap;

use atlas_core::prelude::Span;
use serde::Serialize;

use super::ty::HirTy;

/// An HirModuleSignature represents the API of a module.
///
/// Currently only functions exist in the language.
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize, Default)]
pub struct HirModuleSignature<'hir> {
    pub functions: BTreeMap<&'hir str, &'hir HirFunctionSignature<'hir>>,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize)]
pub struct HirFunctionSignature<'hir> {
    pub span: Span,
    pub params: Vec<&'hir HirFunctionParameterSignature<'hir>>,
    pub type_params: Vec<&'hir HirTypeParameterItemSignature<'hir>>,
    /// The user can declare a function without a return type, in which case the return type is `()`.
    pub return_ty: &'hir HirTy<'hir>,
    /// The span of the return type, if it exists.
    pub return_ty_span: Option<Span>,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize)]
pub struct HirTypeParameterItemSignature<'hir> {
    pub span: Span,
    pub name: &'hir str,
    pub name_span: Span,
}
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Serialize)]
pub struct HirFunctionParameterSignature<'hir> {
    pub span: Span,
    pub name: &'hir str,
    pub name_span: Span,
    pub ty: &'hir HirTy<'hir>,
    pub ty_span: Span,
}
