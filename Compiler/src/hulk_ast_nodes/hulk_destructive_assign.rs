//! # DestructiveAssignment AST Node
//!
//! Este módulo define el nodo de asignación destructiva (`DestructiveAssignment`) del AST para el compilador Hulk.
//! Una asignación destructiva permite modificar el valor de una variable o propiedad existente, por ejemplo: `x := 5`.
//! Incluye la estructura, métodos asociados y la generación de código LLVM IR.

use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::hulk_ast_nodes::hulk_expression::ExprKind;
use crate::typings::types_node::TypeNode;

/// Representa una asignación destructiva en el AST.
/// 
/// Por ejemplo: `x := 5`
/// 
/// - `identifier`: expresión que representa el identificador o propiedad a modificar.
/// - `expression`: expresión cuyo valor se asigna.
/// - `_type`: tipo inferido o declarado de la asignación (opcional).
#[derive(Debug, PartialEq, Clone)]
pub struct DestructiveAssignment {
    pub identifier: Box<Expr>,
    pub expression: Box<Expr>,
    pub _type: Option<TypeNode>,
}

impl DestructiveAssignment {
    /// Crea una nueva asignación destructiva.
    ///
    /// # Arguments
    /// * `identifier` - Expresión que representa el identificador o propiedad.
    /// * `expression` - Expresión a asignar.
    pub fn new(identifier: Box<Expr>, expression: Expr) -> Self {
        Self {
            identifier,
            expression: Box::new(expression),
            _type: None,
        }
    }

    /// Establece el tipo de la expresión asignada.
    pub fn set_expression_type(&mut self, _type: TypeNode){
        self._type = Some(_type)
    }
}

impl Codegen for DestructiveAssignment {
    /// Genera el código LLVM IR para la asignación destructiva.
    ///
    /// Busca el puntero de la variable en el contexto y almacena el valor generado por la expresión.
    /// Si la variable no existe en el contexto, lanza un panic.
    fn codegen(&self, context: &mut CodegenContext) -> String {

        let var_name = match *self.identifier {
            Expr {
                kind: ExprKind::Identifier(ref name),
                ..
            } => name,
            _ => panic!("Expected identifier on left side of destructive assignment"),
        };
        let ptr = context.symbol_table.get(&var_name.to_string()).cloned();
        if let Some(ptr) = ptr {
            let value_reg = self.expression.codegen(context);
            context.emit(&format!("  store i32 {}, i32* {}", value_reg, ptr));
            value_reg
        } else {
            panic!(
                "Variable '{}' no definida en el contexto para asignación destructiva",
                var_name
            );
        }
    }
}
