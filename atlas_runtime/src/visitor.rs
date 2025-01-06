use atlas_frontend::parser::ast::{
    BinaryExpression, DoExpression, Expression, FieldAccessExpression, FunctionCall,
    FunctionExpression, IdentifierNode, IfElseNode, IndexExpression, MatchExpression,
    NewObjectExpression, UnaryExpression, VariableDeclaration,
};
use atlas_memory::vm_data::VMData;

pub type Program = Vec<Expression>;

pub trait Visitor {
    // Entry point
    fn visit(&mut self, program: &Program) -> VMData;

    // Expressions
    fn visit_expression(&mut self, expression: &Expression) -> VMData;
    fn visit_binary_expression(&mut self, expression: &BinaryExpression) -> VMData;
    fn visit_unary_expression(&mut self, expression: &UnaryExpression) -> VMData;
    fn visit_function_expression(&mut self, function_expression: &FunctionExpression) -> VMData;
    fn visit_function_call(&mut self, function_call: &FunctionCall) -> VMData;
    fn visit_index_expression(&mut self, index_expression: &IndexExpression) -> VMData;
    fn visit_field_access_expression(
        &mut self,
        field_access_expression: &FieldAccessExpression,
    ) -> VMData;
    fn visit_new_object_expression(
        &mut self,
        new_object_expression: &NewObjectExpression,
    ) -> VMData;

    // Variables and Identifiers
    fn visit_variable_declaration(&mut self, variable_declaration: &VariableDeclaration) -> VMData;
    fn visit_identifier(&mut self, identifier: &IdentifierNode) -> VMData;

    // Control flow
    fn visit_if_else_node(&mut self, if_else_node: &IfElseNode) -> VMData;
    fn visit_do_expression(&mut self, do_expression: &DoExpression) -> VMData;
    fn visit_match_expression(&mut self, match_expression: &MatchExpression) -> VMData;
}
