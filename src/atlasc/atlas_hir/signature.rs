use super::ty::{HirTy, HirUnitTy};
use logos::Span;
use serde::Serialize;
use std::collections::BTreeMap;

/// An HirModuleSignature represents the API of a module.
///
/// Currently only functions exist in the language.
#[derive(Debug, Clone, Serialize, Default)]
pub struct HirModuleSignature<'hir> {
    pub functions: BTreeMap<&'hir str, &'hir HirFunctionSignature<'hir>>,
    pub classes: BTreeMap<&'hir str, &'hir HirClassSignature<'hir>>,
}

#[derive(Debug, Clone, Serialize)]
/// As of now, classes don't inherit from other classes or extend interfaces.
///
/// Generic classes are not supported yet.
pub struct HirClassSignature<'hir> {
    pub span: Span,
    pub methods: BTreeMap<&'hir str, &'hir HirFunctionSignature<'hir>>,
    pub fields: BTreeMap<&'hir str, HirClassFieldSignature<'hir>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HirClassFieldSignature<'hir> {
    pub span: Span,
    pub name: &'hir str,
    pub ty: &'hir HirTy<'hir>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HirFunctionSignature<'hir> {
    pub span: Span,
    pub params: Vec<&'hir HirFunctionParameterSignature<'hir>>,
    pub generics: Option<Vec<&'hir HirTypeParameterItemSignature<'hir>>>,
    pub type_params: Vec<&'hir HirTypeParameterItemSignature<'hir>>,
    /// The user can declare a function without a return type, in which case the return type is `()`.
    pub return_ty: &'hir HirTy<'hir>,
    /// The span of the return type, if it exists.
    pub return_ty_span: Option<Span>,
    pub is_external: bool,
}

impl Default for HirFunctionSignature<'_> {
    fn default() -> Self {
        Self {
            span: Span::default(),
            params: Vec::new(),
            generics: None,
            type_params: Vec::new(),
            return_ty: &HirTy::Unit(HirUnitTy {}),
            return_ty_span: None,
            is_external: false,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct HirTypeParameterItemSignature<'hir> {
    pub span: Span,
    pub name: &'hir str,
    pub name_span: Span,
}
#[derive(Debug, Clone, Serialize)]
pub struct HirFunctionParameterSignature<'hir> {
    pub span: Span,
    pub name: &'hir str,
    pub name_span: Span,
    pub ty: &'hir HirTy<'hir>,
    pub ty_span: Span,
}
