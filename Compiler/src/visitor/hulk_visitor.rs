use crate::hulk_ast_nodes::{hulk_if_exp::ElifBranch, hulk_print_expr::PrintExpr, *};
pub trait Visitor<T> {
    fn visit_program(&mut self, node: &mut ProgramNode) -> T;
    fn visit_function_def(&mut self, node: &mut FunctionDef) -> T;
    fn visit_code_block(&mut self, node: &mut Block) -> T;
    fn visit_expression_list(&mut self, node: &mut ExpressionList) -> T;
    fn visit_assignment(&mut self, node: &mut Assignment) -> T;
    fn visit_let_in(&mut self, node: &mut LetIn) -> T;
    fn visit_if_else(&mut self, node: &mut IfExpr) -> T;
    fn visit_elif_branch(&mut self, node: &mut ElifBranch) -> T;
    fn visit_else_branch(&mut self, node: &mut ElseBranch) -> T;
    fn visit_while_loop(&mut self, node: &mut WhileLoop) -> T;
    fn visit_function_call(&mut self, node: &mut FunctionCall) -> T;
    fn visit_identifier(&mut self, node: &mut Identifier) -> T;
    fn visit_number_literal(&mut self, node: &mut NumberLiteral) -> T;
    fn visit_boolean_literal(&mut self, node: &mut BooleanLiteral) -> T;
    fn visit_string_literal(&mut self, node: &mut StringLiteral) -> T;
    fn visit_binary_expr(&mut self, node: &mut BinaryExpr) -> T;
    fn visit_unary_expr(&mut self, node: &mut UnaryExpr) -> T;
    fn visit_for_expr(&mut self, node: &mut ForExpr) -> T;
    fn visit_type_def(&mut self, node: &mut HulkTypeNode) -> T;
    fn visit_new_type_instance(&mut self, node: &mut NewTypeInstance) -> T;
    fn visit_function_access(&mut self, node: &mut FunctionAccess) -> T;
    fn visit_member_access(&mut self, node: &mut MemberAccess) -> T;
    fn visit_destructive_assignment(&mut self, node: &mut DestructiveAssignment) -> T;
    fn visit_print_expr(&mut self, node: &mut PrintExpr) -> T;
}