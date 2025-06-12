//! # Block y ExpressionList AST Nodes
//!
//! Este módulo define los nodos `Block` y `ExpressionList` del AST para el compilador Hulk.
//! Un `Block` representa un bloque de código (por ejemplo, el cuerpo de una función o una rama de un if).
//! Un `ExpressionList` representa una lista de expresiones evaluadas secuencialmente dentro de un bloque.

use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::typings::types_node::TypeNode;

/// Representa una lista de expresiones en el AST.
/// 
/// Por ejemplo: `{ x = 5; y = x + 1; y }`
/// 
/// - `expressions`: lista de expresiones a evaluar en orden.
#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionList {
    pub expressions: Box<Vec<Expr>>,
}

impl ExpressionList {
    /// Crea una nueva lista de expresiones.
    ///
    /// # Arguments
    /// * `expressions` - Vector de expresiones.
    pub fn new(expressions: Vec<Expr>) -> Self {
        ExpressionList {
            expressions: Box::new(expressions),
        }
    }
}

impl Codegen for ExpressionList {
    /// Genera el código LLVM IR para la lista de expresiones.
    ///
    /// Evalúa cada expresión en orden y retorna el registro del último resultado.
    fn codegen(&self, context: &mut CodegenContext) -> String {
        let mut last_reg = String::new();
        for expr in self.expressions.iter() {
            last_reg = expr.codegen(context);
        }
        last_reg
    }
}

/// Representa un bloque de código en el AST.
/// 
/// Por ejemplo: el cuerpo de una función, un if, un while, etc.
/// 
/// - `expression_list`: lista de expresiones dentro del bloque.
/// - `_type`: tipo inferido o declarado del bloque (opcional).
#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub expression_list: Box<ExpressionList>,
    pub _type: Option<TypeNode>
}

impl Block {
    /// Crea un nuevo bloque de código.
    ///
    /// # Arguments
    /// * `expression_list` - Lista de expresiones del bloque.
    pub fn new(expression_list: ExpressionList) -> Self {
        Block {
            expression_list: Box::new(expression_list),
            _type: None,
        }
    }

    /// Establece el tipo del bloque.
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}

impl Codegen for Block {
    /// Genera el código LLVM IR para el bloque.
    ///
    /// Evalúa todas las expresiones del bloque y retorna el registro del último resultado.
    fn codegen(&self, context: &mut CodegenContext) -> String {
        let exprs = &self.expression_list.expressions;
        let mut last_reg = String::new();
        for expr in exprs.iter() {
            last_reg = expr.codegen(context);
        }
        last_reg
    }
}
