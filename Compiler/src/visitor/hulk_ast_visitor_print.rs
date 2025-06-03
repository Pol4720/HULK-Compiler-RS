use crate::{hulk_tokens::{Assignment, DestructiveAssignment ,BinaryExpr, ForExpr,Block, BooleanLiteral, ElseBranch, ExpressionList, FunctionCall, FunctionDef, Identifier, IfExpr, LetIn, NumberLiteral, ProgramNode, StringLiteral, UnaryExpr, WhileLoop}};
use crate::visitor::hulk_accept::Accept;

use super::hulk_visitor::Visitor;

pub struct PreetyPrintVisitor;

impl Visitor<String> for PreetyPrintVisitor {
    fn visit_program(&mut self, program: &ProgramNode) -> String {
        let instructions = program.instructions.iter()
            .map(|instr| instr.accept(self))
            .collect::<Vec<_>>()
            .join("\n");
        format!("Program:\n{}", instructions)
    }

    fn visit_identifier(&mut self, identifier: &Identifier) -> String {
        format!("Identifier: {}", identifier.id)
    }

    fn visit_number_literal(&mut self, number: &NumberLiteral) -> String {
        format!("NumberLiteral: {}", number.value)
    }

    fn visit_boolean_literal(&mut self, boolean: &BooleanLiteral) -> String {
        format!("BooleanLiteral: {}", boolean.value)
    }

    fn visit_string_literal(&mut self, string: &StringLiteral) -> String {
        format!("StringLiteral: {}", string.value)
    }

    fn visit_function_def(&mut self, function_def: &FunctionDef) -> String {
        let params = function_def.params.iter()
            .map(|param| param.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "FunctionDef: {}({}) -> {{\n  {}\n}}",
            function_def.name,
            params,
            function_def.body.accept(self)
        )
    }

    fn visit_function_call(&mut self, function_call: &FunctionCall) -> String {
        let args = function_call.arguments.iter()
            .map(|arg| arg.accept(self))
            .collect::<Vec<_>>()
            .join(", ");
        format!("FunctionCall: {}({})", function_call.funct_name, args)
    }

    fn visit_assignment(&mut self, assignment: &Assignment) -> String {
        format!("Assignment: {} = {}", assignment.identifier.id, assignment.expression.accept(self))
    }

    fn visit_let_in(&mut self, let_in: &LetIn) -> String {
        let assignments = let_in.assignment.iter()
            .map(|a| a.accept(self))
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "LetIn:\nAssignments:\n{}\nBody:\n{}",
            assignments,
            let_in.body.accept(self)
        )
    }

    fn visit_if_else(&mut self, if_expr: &IfExpr) -> String {
        let condition = if_expr.condition.accept(self);
        let then_branch = if_expr.then_branch.accept(self);
        let else_branch = if_expr.else_branch.as_ref()
            .map_or("None".to_string(), |e| e.accept(self));
        format!(
            "IfExpr:\nCondition: {}\nThen: {}\nElse: {}",
            condition, then_branch, else_branch
        )
    }

    fn visit_else_branch(&mut self, else_branch: &ElseBranch) -> String {
        format!("ElseBranch:\n{}", else_branch.body.accept(self))
    }

    fn visit_while_loop(&mut self, while_loop: &WhileLoop) -> String {
        let condition = while_loop.condition.accept(self);
        let body = while_loop.body.accept(self);
        format!("WhileLoop:\nCondition: {}\nBody: {}", condition, body)
    }

    fn visit_code_block(&mut self, block: &Block) -> String {
        let expressions = block.expression_list.expressions.iter()
            .map(|expr| expr.accept(self))
            .collect::<Vec<_>>()
            .join("\n");
        format!("CodeBlock:\n{}", expressions)
    }

    fn visit_binary_expr(&mut self, binary_expr: &BinaryExpr) -> String {
        let left = binary_expr.left.accept(self);
        let right = binary_expr.right.accept(self);
        format!("BinaryExpr: {} {:?} {}", left, binary_expr.operator, right)
    }

    fn visit_unary_expr(&mut self, unary_expr: &UnaryExpr) -> String {
        let operand = unary_expr.operand.accept(self);
        format!("UnaryExpr: {:?}{}", unary_expr.operator, operand)
    }

    fn visit_expression_list(&mut self, expression_list: &ExpressionList) -> String {
        let expressions = expression_list.expressions.iter()
            .map(|expr| expr.accept(self))
            .collect::<Vec<_>>()
            .join("\n");
        format!("ExpressionList:\n{}", expressions)
    }
    
    fn visit_for_expr(&mut self, node: &ForExpr) -> String {
        let variable = &node.variable;
        let start = node.start.accept(self);
        let end = node.end.accept(self);
        let body = node.body.accept(self);
        format!("for ({} in range({}, {})) {{\n{}\n}}", variable, start, end, body)
    }
    
    fn visit_destructive_assignment(&mut self, node: &DestructiveAssignment) -> String {
        let id = &node.identifier;
        let expr = node.expression.accept(self);
        format!("{} := {}", id, expr)
    }
    
}