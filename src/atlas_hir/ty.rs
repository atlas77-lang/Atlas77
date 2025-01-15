use atlas_core::prelude::Span;

pub enum HirTy<'hir> {
    Integer(HirIntegerTy),

    Named(HirNamedTy<'hir>),
}

pub struct HirIntegerTy {}

pub struct HirNamedTy<'hir> {
    pub name: &'hir str,
    pub span: Span,
}