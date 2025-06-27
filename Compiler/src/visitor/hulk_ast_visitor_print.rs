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
        Assignment, DestructiveAssignment, BinaryExpr, ForExpr, Block, BooleanLiteral,
        ExpressionList, FunctionCall, FunctionDef, Identifier, LetIn, NumberLiteral,
        ProgramNode, StringLiteral, UnaryExpr, WhileLoop, HulkTypeNode,
    },
    visitor::hulk_accept::Accept,
};

use crate::hulk_ast_nodes::hulk_if_exp::IfExpr;

use super::hulk_visitor::Visitor;

/// Visitor que recorre el AST y genera una representación legible de cada nodo.
pub struct PreetyPrintVisitor;

impl Visitor<String> for PreetyPrintVisitor {
    fn visit_program(&mut self, program: &mut ProgramNode) -> String {
        let definitions = program.definitions.iter_mut()
            .map(|def| def.accept(self))
            .collect::<Vec<_>>()
            .join("\n");
        let instructions = program.instructions.iter_mut()
            .map(|instr| instr.accept(self))
            .collect::<Vec<_>>()
            .join("\n");
        format!("Program:\n{}\n{}", definitions, instructions)
    }

    fn visit_identifier(&mut self, identifier: &mut Identifier) -> String {
        match &identifier._type {
            Some(ty) => format!("Identifier: {} : {}", identifier.id, ty.type_name),
            None => format!("Identifier: {}", identifier.id),
        }
    }

    fn visit_number_literal(&mut self, number: &mut NumberLiteral) -> String {
        match &number._type {
            Some(ty) => format!("NumberLiteral: {} : {}", number.value, ty.type_name),
            None => format!("NumberLiteral: {}", number.value),
        }
    }
    fn visit_boolean_literal(&mut self, boolean: &mut BooleanLiteral) -> String {
        match &boolean._type {
            Some(ty) => format!("BooleanLiteral: {} : {}", boolean.value, ty.type_name),
            None => format!("BooleanLiteral: {}", boolean.value),
        }
    }

    fn visit_string_literal(&mut self, string: &mut StringLiteral) -> String {
        match &string._type {
            Some(ty) => format!("StringLiteral: {} : {}", string.value, ty.type_name),
            None => format!("StringLiteral: {}", string.value),
        }
    }

    fn visit_function_def(&mut self, function_def: &mut FunctionDef) -> String {
        let params = function_def.params.iter()
            .map(|param| {
                // Asumiendo que param tiene campos 'name' y 'param_type'
                format!("{}: {}", param.name, param.param_type)
            })
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
        let type_str = match &function_call._type {
            Some(ty) => format!(" : {}", ty.type_name),
            None => "".to_string(),
        };
        format!("FunctionCall: {}({}){}", function_call.funct_name, args, type_str)
    }

    fn visit_assignment(&mut self, assignment: &mut Assignment) -> String {
        let id = &assignment.identifier;
        let expr_str = assignment.expression.accept(self);
        let type_str = match &assignment._type {
            Some(ty) => format!(" : {}", ty.type_name),
            None => "".to_string(),
        };
        format!(
            "Assignment: {}{} = {}",
            id.id,
            type_str,
            expr_str
        )
    }

    fn visit_let_in(&mut self, node: &mut LetIn) -> String {
        let assignments: Vec<String> = node.assignment.iter_mut()
            .map(|assignment| format!("{} = {}", assignment.identifier, assignment.expression.accept(self)))
            .collect();
        let body = node.body.accept(self);
        format!("let {} in {}", assignments.join(", "), body)
    }

    fn visit_if_else(&mut self, node: &mut IfExpr) -> String {
        let condition = node.condition.accept(self);
        let if_body = node.then_branch.accept(self);
        let mut result = format!("if ({}) {{\n{}\n}}",condition,if_body);
        for (condition , body) in node.else_branch.iter() {
            let expr_body = body.clone().accept(self);
            if let Some(cond) = condition {
                let elif_condition = cond.clone().accept(self);
                result.push_str(&format!(" elif ({}) {{\n{}\n}}", elif_condition,expr_body));
            }else {
                result.push_str(&format!(" else {{\n{}\n}}", expr_body));
            }
        }
        result
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
        let type_str = match &binary_expr._type {
            Some(ty) => format!(" : {}", ty.type_name),
            None => "".to_string(),
        };
        format!("BinaryExpr: {} {:?} {}{}", left, binary_expr.operator, right, type_str)
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
