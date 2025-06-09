use crate::hulk_ast_nodes::*;
pub trait Visitor<T> {
    fn visit_program(&mut self, node: &ProgramNode) -> T;
    fn visit_function_def(&mut self, node: &FunctionDef) -> T;
    fn visit_code_block(&mut self, node: &Block) -> T;
    fn visit_expression_list(&mut self, node: &ExpressionList) -> T;
    fn visit_assignment(&mut self, node: &Assignment) -> T;
    fn visit_let_in(&mut self, node: &LetIn) -> T;
    fn visit_if_else(&mut self, node: &IfExpr) -> T;
    fn visit_else_branch(&mut self, node: &ElseBranch) -> T;
    fn visit_while_loop(&mut self, node: &WhileLoop) -> T;
    fn visit_function_call(&mut self, node: &FunctionCall) -> T;
    fn visit_identifier(&mut self, node: &Identifier) -> T;
    fn visit_number_literal(&mut self, node: &NumberLiteral) -> T;
    fn visit_boolean_literal(&mut self, node: &BooleanLiteral) -> T;
    fn visit_string_literal(&mut self, node: &StringLiteral) -> T;
    fn visit_binary_expr(&mut self, node: &BinaryExpr) -> T;
    fn visit_unary_expr(&mut self, node: &UnaryExpr) -> T;
    fn visit_for_expr(&mut self, node: &ForExpr) -> T;
    fn visit_destructive_assignment(&mut self, node: &DestructiveAssignment) -> T;
}