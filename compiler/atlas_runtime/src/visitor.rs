use atlas_frontend::parser::ast::AstLiteral;
use atlas_vm::memory::vm_data::VMData;

use atlas_frontend::parser::ast::{
    AstBinaryOpExpr, AstBlock, AstCallExpr, AstExpr, AstFieldAccessExpr, AstFunction,
    AstIdentifier, AstIfElseExpr, AstIndexingExpr, AstLetExpr, AstProgram, AstUnaryOpExpr,
};
use atlas_vm::RuntimeResult;

//TODO: visit() should return a Result<VMData, crate::errors::RuntimeError>
#[deprecated = r#"This trait will be removed in favor of the VM.
It will still be used for compile time evaluation, but will be reworked.
The rework will retarget it to the typed High-level Intermediate Representation"#]
pub(crate) trait _Visitor<'visitor> {
    type CallBack;
    type RuntimeResult<T>;
    // Entry point
    fn visit(&mut self, program: &'visitor AstProgram, entry_point: &str) -> RuntimeResult<VMData>;

    // Expressions
    fn visit_expression(&mut self, expression: &'visitor AstExpr) -> RuntimeResult<VMData>;
    fn visit_binary_expression(
        &mut self,
        expression: &'visitor AstBinaryOpExpr,
    ) -> RuntimeResult<VMData>;
    fn visit_unary_expression(
        &mut self,
        expression: &'visitor AstUnaryOpExpr,
    ) -> RuntimeResult<VMData>;
    fn visit_function_expression(
        &mut self,
        function_expression: &'visitor AstFunction,
    ) -> RuntimeResult<VMData>;
    fn visit_function_call(
        &mut self,
        function_call: &'visitor AstCallExpr,
    ) -> RuntimeResult<VMData>;
    fn visit_index_expression(
        &mut self,
        index_expression: &'visitor AstIndexingExpr,
    ) -> RuntimeResult<VMData>;
    fn visit_field_access_expression(
        &mut self,
        field_access_expression: &'visitor AstFieldAccessExpr,
    ) -> RuntimeResult<VMData>;
    // Variables and Identifiers
    fn visit_variable_declaration(
        &mut self,
        variable_declaration: &'visitor AstLetExpr,
    ) -> RuntimeResult<VMData>;
    fn visit_identifier(&mut self, identifier: &'visitor AstIdentifier) -> RuntimeResult<VMData>;
    fn visit_literal(&mut self, literal: &'visitor AstLiteral) -> RuntimeResult<VMData>;

    // Control flow
    fn visit_if_else_node(
        &mut self,
        if_else_node: &'visitor AstIfElseExpr,
    ) -> RuntimeResult<VMData>;
    fn visit_block_expression(&mut self, block: &'visitor AstBlock) -> RuntimeResult<VMData>;
}
