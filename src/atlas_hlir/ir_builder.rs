use atlas_frontend::parser::ast::AbstractSyntaxTree;

use crate::context::HlirContext;
use crate::errors::{HlirError, HlirResult};
use crate::HlirTree;

pub struct HlirBuilder {
    ctx: HlirContext,
    program: HlirTree,
}
impl HlirBuilder {
    pub fn new() -> HlirBuilder {
        HlirBuilder {
            ctx: HlirContext::default(),
            program: HlirTree::default(),
        }
    }
    pub fn build(&mut self, ast: AbstractSyntaxTree) -> Result<HlirResult, HlirError> {
        Ok(HlirResult::Success)
    }
}