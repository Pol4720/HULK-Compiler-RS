//! # Expr y ExprKind AST Nodes
//!
//! Este módulo define los nodos de expresión (`Expr` y `ExprKind`) del AST para el compilador Hulk.
//! Permite representar y manipular cualquier tipo de expresión del lenguaje, incluyendo literales, operaciones, llamadas a función, bloques, etc.
//! Provee integración con el visitor pattern, evaluación directa y generación de código LLVM IR.

use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_ast_nodes::Block;
use crate::hulk_ast_nodes::DestructiveAssignment;
use crate::hulk_ast_nodes::FunctionCall;
use crate::hulk_ast_nodes::hulk_assignment::Assignment;
use crate::hulk_ast_nodes::hulk_binary_expr::*;
use crate::hulk_ast_nodes::hulk_for_expr::ForExpr;
use crate::hulk_ast_nodes::hulk_identifier::*;
use crate::hulk_ast_nodes::hulk_if_exp::*;
use crate::hulk_ast_nodes::hulk_let_in::*;
use crate::hulk_ast_nodes::hulk_literal::*;
use crate::hulk_tokens::hulk_operators::*;
use crate::hulk_ast_nodes::hulk_unary_expr::*;
use crate::hulk_ast_nodes::hulk_whileloop::*;
use crate::hulk_ast_nodes::NewTypeInstance;
use crate::visitor::hulk_accept::Accept;
use crate::visitor::hulk_visitor::Visitor;
use crate::hulk_ast_nodes::hulk_function_access::FunctionAccess;
use crate::hulk_ast_nodes::hulk_member_access::MemberAccess;

/// Nodo de expresión general del AST.
/// 
/// Contiene un `ExprKind` que determina el tipo específico de la expresión.
/// 
/// Ejemplos: literales, operaciones binarias, llamadas a función, bloques, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
}

impl Accept for Expr {
    /// Permite que el nodo acepte un visitor.
    fn accept<V: Visitor<T>, T>(&mut self, visitor: &mut V) -> T {
        self.kind.accept(visitor)
    }
}

/// Enum que representa todos los tipos de expresiones posibles en el lenguaje Hulk.
/// 
/// - Literales: `Number`, `Boolean`, `String`
/// - Identificadores y operaciones: `Identifier`, `BinaryOp`, `UnaryOp`
/// - Control de flujo: `If`, `WhileLoop`, `ForExp`, `CodeBlock`
/// - Asignaciones: `Assignment`, `DestructiveAssign`, `LetIn`
/// - Llamadas y acceso: `FunctionCall`, `FunctionAccess`, `MemberAccess`
/// - Instanciación de tipos: `NewTypeInstance`
#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Number(NumberLiteral),
    Boolean(BooleanLiteral),
    String(StringLiteral),
    Identifier(Identifier),
    BinaryOp(BinaryExpr),
    UnaryOp(UnaryExpr),
    If(IfExpr),
    FunctionCall(FunctionCall),
    Assignment(Assignment),
    LetIn(LetIn),
    WhileLoop(WhileLoop),
    ForExp(ForExpr),
    CodeBlock(Block),
    DestructiveAssign(DestructiveAssignment),
    NewTypeInstance(NewTypeInstance),
    FunctionAccess(FunctionAccess),
    MemberAccess(MemberAccess),
}

impl Expr {
    /// Crea una nueva expresión a partir de un `ExprKind`.
    pub fn new(kind: ExprKind) -> Self {
        Expr { kind }
    }

    /// Evalúa la expresión si es posible (solo para expresiones aritméticas y booleanas simples).
    /// 
    /// Retorna el resultado como `f64` o un error si la expresión no es evaluable directamente.
    pub fn eval(&self) -> Result<f64, String> {
        match &self.kind {
            ExprKind::Number(n) => Ok(n.value),
            ExprKind::Boolean(b) => Ok(if b.value { 1.0 } else { 0.0 }),
            ExprKind::BinaryOp(binary_expr) => {
                let left_val = binary_expr.left.eval()?;
                let right_val = binary_expr.right.eval()?;
                match &binary_expr.operator {
                    BinaryOperatorToken::Plus => Ok(left_val + right_val),
                    BinaryOperatorToken::Minus => Ok(left_val - right_val),
                    BinaryOperatorToken::Mul => Ok(left_val * right_val),
                    BinaryOperatorToken::Div => {
                        if right_val == 0.0 {
                            Err("Error: División por cero".to_string())
                        } else {
                            Ok(left_val / right_val)
                        }
                    }
                    BinaryOperatorToken::Mod => Ok(left_val % right_val),
                    BinaryOperatorToken::Pow => Ok(left_val.powf(right_val)),
                    BinaryOperatorToken::Eq => Ok((left_val == right_val) as i64 as f64),
                    BinaryOperatorToken::Neq => Ok((left_val != right_val) as i64 as f64),
                    BinaryOperatorToken::Gt => Ok((left_val > right_val) as i64 as f64),
                    BinaryOperatorToken::Gte => Ok((left_val >= right_val) as i64 as f64),
                    BinaryOperatorToken::Lt => Ok((left_val < right_val) as i64 as f64),
                    BinaryOperatorToken::Lte => Ok((left_val <= right_val) as i64 as f64),
                    _ => Err("Operador no soportado".to_string()),
                }
            }
            ExprKind::UnaryOp(unary_expr) => {
                let val = unary_expr.operand.eval()?;
                match &unary_expr.operator {
                    UnaryOperator::Plus => Ok(val),
                    UnaryOperator::Minus => Ok(-val),
                    UnaryOperator::LogicalNot => Ok((val == 0.0) as i64 as f64),
                }
            }
            _ => Err("Expresión no soportada".to_string()),
        }
    }
}

impl Accept for ExprKind {
    /// Permite que el nodo acepte un visitor.
    fn accept<V: Visitor<T>, T>(&mut self, visitor: &mut V) -> T {
        match self {
            ExprKind::Number(node) => visitor.visit_number_literal(node),
            ExprKind::Boolean(node) => visitor.visit_boolean_literal(node),
            ExprKind::String(node) => visitor.visit_string_literal(node),
            ExprKind::Identifier(node) => visitor.visit_identifier(node),
            ExprKind::FunctionCall(node) => visitor.visit_function_call(node),
            ExprKind::WhileLoop(node) => visitor.visit_while_loop(node),
            ExprKind::CodeBlock(node) => visitor.visit_code_block(node),
            ExprKind::BinaryOp(node) => visitor.visit_binary_expr(node),
            ExprKind::ForExp(node) => visitor.visit_for_expr(node),
            ExprKind::UnaryOp(node) => visitor.visit_unary_expr(node),
            ExprKind::If(node) => visitor.visit_if_else(node),
            ExprKind::DestructiveAssign(node) => visitor.visit_destructive_assignment(node),
            ExprKind::LetIn(node) => visitor.visit_let_in(node),
            ExprKind::Assignment(node) => visitor.visit_assignment(node),
            ExprKind::NewTypeInstance(node) => visitor.visit_new_type_instance(node),
            ExprKind::FunctionAccess(node) => visitor.visit_function_access(node),
            ExprKind::MemberAccess(node) => visitor.visit_member_access(node),
        }
    }
}

impl Codegen for Expr {
    /// Genera el código LLVM IR para la expresión.
    fn codegen(&self, context: &mut CodegenContext) -> String {
        self.kind.codegen(context)
    }
}

impl Codegen for ExprKind {
    /// Genera el código LLVM IR para el tipo específico de expresión.
    fn codegen(&self, context: &mut CodegenContext) -> String {
        match self {
            ExprKind::Number(n) => n.codegen(context),
            ExprKind::Boolean(b) => b.codegen(context),
            ExprKind::String(s) => s.codegen(context),
            ExprKind::Identifier(id) => id.codegen(context),
            ExprKind::BinaryOp(bin) => bin.codegen(context),
            ExprKind::UnaryOp(un) => un.codegen(context),
            ExprKind::If(ifexp) => ifexp.codegen(context),
            ExprKind::FunctionCall(call) => call.codegen(context),
            ExprKind::Assignment(assign) => assign.codegen(context),
            ExprKind::LetIn(letin) => letin.codegen(context),
            ExprKind::WhileLoop(whileloop) => whileloop.codegen(context),
            ExprKind::ForExp(forexp) => forexp.codegen(context),
            ExprKind::CodeBlock(block) => block.codegen(context),
            ExprKind::DestructiveAssign(destruct) => destruct.codegen(context),
            ExprKind::NewTypeInstance(new_type_instance) => todo!(),
            ExprKind::FunctionAccess(function_access) => todo!(),
            ExprKind::MemberAccess(member_access) => todo!(),
        }
    }
}
