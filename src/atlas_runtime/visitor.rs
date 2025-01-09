use crate::atlas_frontend::parser::ast::{
    BinaryExpression, DoExpression, Expression, FieldAccessExpression, FunctionCall,
    FunctionExpression, IdentifierNode, IfElseNode, IndexExpression, MatchExpression,
    NewObjectExpression, UnaryExpression, VariableDeclaration,
};
use crate::atlas_memory::vm_data::VMData;

pub type Program = Vec<Expression>;

//TODO: visit() should return a Result<VMData, crate::errors::RuntimeError>
pub trait Visitor<'visitor> {
    type CallBack;
    // Entry point
    fn visit(&mut self, program: &'visitor Program) -> VMData;

    // Expressions
    fn visit_expression(&mut self, expression: &'visitor Expression) -> VMData;
    fn visit_binary_expression(&mut self, expression: &'visitor BinaryExpression) -> VMData;
    fn visit_unary_expression(&mut self, expression: &'visitor UnaryExpression) -> VMData;
    fn visit_function_expression(
        &mut self,
        function_expression: &'visitor FunctionExpression,
    ) -> VMData;
    fn visit_function_call(&mut self, function_call: &'visitor FunctionCall) -> VMData;
    fn visit_index_expression(&mut self, index_expression: &'visitor IndexExpression) -> VMData;
    fn visit_field_access_expression(
        &mut self,
        field_access_expression: &'visitor FieldAccessExpression,
    ) -> VMData;
    fn visit_new_object_expression(
        &mut self,
        new_object_expression: &'visitor NewObjectExpression,
    ) -> VMData;

    // Variables and Identifiers
    fn visit_variable_declaration(
        &mut self,
        variable_declaration: &'visitor VariableDeclaration,
    ) -> VMData;
    fn visit_identifier(&mut self, identifier: &'visitor IdentifierNode) -> VMData;

    // Control flow
    fn visit_if_else_node(&mut self, if_else_node: &'visitor IfElseNode) -> VMData;
    fn visit_do_expression(&mut self, do_expression: &'visitor DoExpression) -> VMData;
    fn visit_match_expression(&mut self, match_expression: &'visitor MatchExpression) -> VMData;
}
