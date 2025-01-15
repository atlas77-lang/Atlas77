use std::collections::BTreeMap;

use atlas_core::prelude::Span;

use super::ty::HirTy;

/// An HirModuleSignature represents the API of a module.
/// 
/// Currently only functions exist in the language.
pub struct HirModuleSignature<'hir> {
    pub functions: BTreeMap<&'hir str, &'hir HirFunctionSignature<'hir>>,
}

pub struct HirFunctionSignature<'hir> {
    pub span: Span,
    pub params: Vec<&'hir HirFunctionParameterSignature<'hir>>,
    /// The user can declare a function without a return type, in which case the return type is `()`.
    pub return_ty: &'hir HirTy<'hir>,
    /// The span of the return type, if it exists.
    pub return_ty_span: Option<Span>,
}

pub struct HirFunctionParameterSignature<'hir> {
    pub span: Span,
    pub name: &'hir str,
    pub name_span: Span,
    pub ty: HirTy<'hir>,
    pub ty_span: Span,
}


