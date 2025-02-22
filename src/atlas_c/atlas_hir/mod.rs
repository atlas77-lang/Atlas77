use std::collections::BTreeMap;

use crate::atlas_c::atlas_hir::item::HirClass;
use item::{HirFunction, HirImport};
use serde::Serialize;
use signature::HirModuleSignature;

//Should try to run even with a faulty AST
/// Pass not run in debug mode
pub mod constant_folding;
/// Pass not run in debug mode
pub mod dead_code;
pub mod syntax_lowering_pass;
/// Always run
pub mod type_check_pass;

pub mod arena;
pub mod error;
//todo: The Hir needs a little rework to correctly define what is an item, a statement, an expression, a type, etc.
pub mod expr;
pub mod item;
pub mod signature;
pub mod stmt;
pub mod ty;

#[derive(Debug, Clone, Serialize, Default)]
pub struct HirModuleBody<'hir> {
    pub functions: BTreeMap<&'hir str, HirFunction<'hir>>,
    pub classes: BTreeMap<&'hir str, HirClass<'hir>>,
    pub imports: Vec<&'hir HirImport<'hir>>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct HirModule<'hir> {
    pub body: HirModuleBody<'hir>,
    pub signature: HirModuleSignature<'hir>,
}
