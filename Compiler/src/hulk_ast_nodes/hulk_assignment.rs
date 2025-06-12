//! # Assignment AST Node
//!
//! Este módulo define el nodo de asignación (`Assignment`) del AST para el compilador Hulk.
//! Incluye la estructura, métodos asociados, integración con el visitor pattern y generación de código LLVM IR.

use super::hulk_identifier::Identifier;
use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::typings::types_node::TypeNode;
use crate::visitor::hulk_accept::Accept;
use crate::visitor::hulk_visitor::Visitor;

/// Representa una asignación en el AST.
/// 
/// Por ejemplo: `x = 5`
/// 
/// - `identifier`: el identificador de la variable a asignar.
/// - `expression`: la expresión cuyo valor se asigna.
/// - `_type`: tipo inferido o declarado de la asignación (opcional).
#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub identifier: Identifier,
    pub expression: Box<Expr>,
    pub _type: Option<TypeNode>,
}

impl Assignment {
    /// Crea una nueva asignación.
    ///
    /// # Arguments
    /// * `identifier` - Identificador de la variable.
    /// * `expression` - Expresión a asignar.
    pub fn new(identifier: Identifier, expression: Box<Expr>) -> Self {
        Assignment { identifier, expression, _type: None }
    }

    /// Establece el tipo de la expresión asignada.
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}

impl Accept for Assignment {
    /// Permite que el nodo acepte un visitor.
    fn accept<V: Visitor<T>, T>(&mut self, visitor: &mut V) -> T {
        visitor.visit_assignment(self)
    }
}

impl Codegen for Assignment {
    /// Genera el código LLVM IR para la asignación.
    ///
    /// Busca el puntero de la variable en el contexto y almacena el valor generado por la expresión.
    /// Si la variable no existe en el contexto, lanza un panic.
    fn codegen(&self, context: &mut CodegenContext) -> String {
        let var_name = &self.identifier.id;
        let ptr = context.symbol_table.get(var_name).cloned();
        if let Some(ptr) = ptr {
            let value_reg = self.expression.codegen(context);
            context.emit(&format!("  store i32 {}, i32* {}", value_reg, ptr));
            value_reg
        } else {
            panic!(
                "Variable '{}' no definida en el contexto para asignación",
                var_name
            );
        }
    }
}
