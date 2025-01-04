use node::HlirTree;

pub mod node;
pub mod ir_builder;
pub mod data_type;
pub mod context;
pub mod errors;

pub fn translate(_ast: &atlas_frontend::parser::ast::AbstractSyntaxTree) -> HlirTree {
    HlirTree::new()
}