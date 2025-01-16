use std::collections::BTreeMap;

use item::{HirFunction, HirImport};
use serde::Serialize;
use signature::HirModuleSignature;

pub mod syntax_lowering_pass;
pub mod type_check_pass;

pub mod arena;
pub mod error;
pub mod expr;
pub mod item;
pub mod signature;
pub mod stmt;
pub mod ty;

#[derive(Debug, Clone, Serialize, Default)]
pub struct HirModuleBody<'hir> {
    pub functions: BTreeMap<&'hir str, HirFunction<'hir>>,
    pub imports: Vec<&'hir HirImport<'hir>>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct HirModule<'hir> {
    pub body: HirModuleBody<'hir>,
    pub signature: HirModuleSignature<'hir>,
}
