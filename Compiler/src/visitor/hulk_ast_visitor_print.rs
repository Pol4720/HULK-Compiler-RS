//! # PreetyPrintVisitor
//!
//! Este módulo define el visitor `PreetyPrintVisitor` para el compilador Hulk.
//! Implementa el trait `Visitor<String>` para recorrer el AST y generar una representación legible (pretty print) de cada nodo.
//! Es útil para depuración, visualización y pruebas del árbol de sintaxis abstracta generado por el parser.
//!
//! ## Características principales
//! - Recorre todos los nodos relevantes del AST de Hulk.
//! - Genera una cadena formateada que describe la estructura y los valores de cada nodo.
//! - Soporta expresiones, declaraciones, bloques, funciones, tipos, ciclos, condicionales, literales, asignaciones y más.
//! - Permite visualizar el árbol de sintaxis de manera jerárquica y comprensible.

use crate::{
    hulk_ast_nodes::{
        Assignment, DestructiveAssignment, BinaryExpr, ForExpr, Block, BooleanLiteral, ElseBranch,
        ExpressionList, FunctionCall, FunctionDef, Identifier, LetIn, NumberLiteral,
        ProgramNode, StringLiteral, UnaryExpr, WhileLoop, HulkTypeNode,
    },
    visitor::hulk_accept::Accept,
};

use crate::hulk_ast_nodes::hulk_if_exp::IfExpr;
use crate::hulk_ast_nodes::hulk_if_exp::ElseOrElif;

use super::hulk_visitor::Visitor;

/// Visitor que recorre el AST y genera una representación legible de cada nodo.
pub struct PreetyPrintVisitor;

impl Visitor<String> for PreetyPrintVisitor {
    fn visit_program(&mut self, program: &mut ProgramNode) -> String {
        let instructions = program.instructions.iter_mut()
            .map(|instr| instr.accept(self))
            .collect::<Vec<_>>()
            .join("\n");
        format!("Program:\n{}", instructions)
    }

    fn visit_identifier(&mut self, identifier: &mut Identifier) -> String {
        format!("Identifier: {}", identifier.id)
    }

    fn visit_number_literal(&mut self, number: &mut NumberLiteral) -> String {
        format!("NumberLiteral: {}", number.value)
    }

    fn visit_boolean_literal(&mut self, boolean: &mut BooleanLiteral) -> String {
        format!("BooleanLiteral: {}", boolean.value)
    }

    fn visit_string_literal(&mut self, string: &mut StringLiteral) -> String {
        format!("StringLiteral: {}", string.value)
    }

    fn visit_function_def(&mut self, function_def: &mut FunctionDef) -> String {
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

    fn visit_function_call(&mut self, function_call: &mut FunctionCall) -> String {
        let args = function_call.arguments.iter_mut()
            .map(|arg| arg.accept(self))
            .collect::<Vec<_>>()
            .join(", ");
        format!("FunctionCall: {}({})", function_call.funct_name, args)
    }

    fn visit_assignment(&mut self, assignment: &mut Assignment) -> String {
        format!("Assignment: {} = {}", assignment.identifier.id, assignment.expression.accept(self))
    }

    fn visit_let_in(&mut self, let_in: &mut LetIn) -> String {
        let assignments = let_in.assignment.iter_mut()
            .map(|a| a.accept(self))
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "LetIn:\nAssignments:\n{}\nBody:\n{}",
            assignments,
            let_in.body.accept(self)
        )
    }

    fn visit_if_else(&mut self, if_expr: &mut IfExpr) -> String {
        let condition = if_expr.condition.accept(self);
        let then_branch = if_expr.then_branch.accept(self);
        let else_branch = if_expr.else_branch.as_mut()
                    .map_or("None".to_string(), |e| match e {
                        ElseOrElif::Else(else_branch) => else_branch.accept(self),
                        ElseOrElif::Elif(elif_branch) => elif_branch.accept(self),
                    });
        format!(
            "IfExpr:\nCondition: {}\nThen: {}\nElse: {}",
            condition, then_branch, else_branch
        )
    }

    fn visit_else_branch(&mut self, else_branch: &mut ElseBranch) -> String {
        format!("ElseBranch:\n{}", else_branch.body.accept(self))
    }

    fn visit_while_loop(&mut self, while_loop: &mut WhileLoop) -> String {
        let condition = while_loop.condition.accept(self);
        let body = while_loop.body.accept(self);
        format!("WhileLoop:\nCondition: {}\nBody: {}", condition, body)
    }

    fn visit_code_block(&mut self, block: &mut Block) -> String {
        let expressions = block.expression_list.expressions.iter_mut()
            .map(|expr| expr.accept(self))
            .collect::<Vec<_>>()
            .join("\n");
        format!("CodeBlock:\n{}", expressions)
    }

    fn visit_binary_expr(&mut self, binary_expr: &mut BinaryExpr) -> String {
        let left = binary_expr.left.accept(self);
        let right = binary_expr.right.accept(self);
        format!("BinaryExpr: {} {:?} {}", left, binary_expr.operator, right)
    }

    fn visit_unary_expr(&mut self, unary_expr: &mut UnaryExpr) -> String {
        let operand = unary_expr.operand.accept(self);
        format!("UnaryExpr: {:?}{}", unary_expr.operator, operand)
    }

    fn visit_expression_list(&mut self, expression_list: &mut ExpressionList) -> String {
        let expressions = expression_list.expressions.iter_mut()
            .map(|expr| expr.accept(self))
            .collect::<Vec<_>>()
            .join("\n");
        format!("ExpressionList:\n{}", expressions)
    }
    
    fn visit_for_expr(&mut self, node: &mut ForExpr) -> String {
        let variable = &node.variable;
        let start = node.start.accept(self);
        let end = node.end.accept(self);
        let body = node.body.accept(self);
        format!("for ({} in range({}, {})) {{\n{}\n}}", variable, start, end, body)
    }
    
    fn visit_destructive_assignment(&mut self, node: &mut DestructiveAssignment) -> String {
        let id = &node.identifier.accept(self);
        let expr = &node.expression.accept(self);
        format!("{} := {}", id, expr)
    }
    
fn visit_type_def(&mut self, node: &mut HulkTypeNode) -> String {
    let type_name = node.type_name.clone();
        let type_params: Vec<String> = node.parameters.iter()
            .map(|param| format!("{}: {}", param.name, param.param_type))
            .collect();

    let members: Vec<String> = node.methods.iter_mut().map(|(_, method)| {
        let method_def = self.visit_function_def(method);
        format!("{}", method_def)
    }).collect();

    let attributes: Vec<String> = node.attributes.iter_mut().map(|(_, attr)| {
        let init_expr = attr.init_expr.accept(self);
        format!("{}: {}", attr.name.id, init_expr)
    }).collect();

    if let Some(parent) = &node.parent {
        let parent_args: Vec<String> = node.parent_args.iter_mut()
            .map(|arg| arg.accept(self))
            .collect();
        return format!(
            "type {} {} inherits {}({}) {{\n{}\n{}\n}}",
            type_name,
            if type_params.is_empty() { "".to_string() } else { format!("( {} )", type_params.join(", ")) },
            parent,
            parent_args.join(", "),
            attributes.join("\n"),
            members.join("\n")
        );
    }
    format!(
        "type {} {} {{\n{}\n{}\n}}",
        type_name,
        if type_params.is_empty() { "".to_string() } else { format!("( {} )", type_params.join(", ")) },
        attributes.join("\n"),
        members.join("\n")
    )
}

    fn visit_new_type_instance(&mut self, node: &mut crate::hulk_ast_nodes::NewTypeInstance) -> String {
        let type_name = &node.type_name;
        let type_args: Vec<String> = node.arguments.iter_mut()
            .map(|arg| arg.accept(self))
            .collect();
        format!("new {}({})", type_name, type_args.join(", "))
    }
    
    fn visit_function_access(&mut self, node: &mut crate::hulk_ast_nodes::FunctionAccess) -> String {
        let object = node.object.accept(self);
        let member_call = self.visit_function_call(&mut node.member);
        format!("{}.{}", object, member_call)
    }

    fn visit_member_access(&mut self, node: &mut crate::hulk_ast_nodes::MemberAccess) -> String {
        let object = node.object.accept(self);
        let member = &node.member;
        format!("{}.{}", object, member)
    }
    
    fn visit_elif_branch(&mut self, node: &mut crate::hulk_ast_nodes::hulk_if_exp::ElifBranch) -> String {
        let condition = node.condition.accept(self);
        let then_branch = node.body.accept(self);
        let else_branch = node.next.as_mut()
            .map_or("None".to_string(), |e| match &mut **e {
                crate::hulk_ast_nodes::hulk_if_exp::ElseOrElif::Else(else_branch) => else_branch.accept(self),
                crate::hulk_ast_nodes::hulk_if_exp::ElseOrElif::Elif(elif_branch) => elif_branch.accept(self),
            });
        format!(
            "ElifBranch:\nCondition: {}\nThen: {}\nElse: {}",
            condition, then_branch, else_branch
        )
    }
    
    fn visit_print_expr(&mut self, node: &mut crate::hulk_ast_nodes::hulk_print_expr::PrintExpr) -> String {
        let expr = node.expr.accept(self);
        format!("Print: {}", expr)
    }

    fn visit_function_body(&mut self, node: &mut crate::hulk_ast_nodes::hulk_function_def::FunctionBody) -> String {
        match node {
            crate::hulk_ast_nodes::hulk_function_def::FunctionBody::Block(block) => self.visit_code_block(block),
            crate::hulk_ast_nodes::hulk_function_def::FunctionBody::ArrowExpression(arrow_expr) => {
                arrow_expr.expression.accept(self)
            }
        }
    }
}
