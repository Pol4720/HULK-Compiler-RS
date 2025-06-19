//! # WhileLoop AST Node
//!
//! Este módulo define el nodo `WhileLoop` del AST para el compilador Hulk.
//! Permite representar bucles `while`, incluyendo la condición, el cuerpo y el tipo inferido o declarado de la expresión.
//! Incluye métodos asociados y la generación de código LLVM IR.

use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::hulk_tokens::{token_pos, TokenPos};
use crate::typings::types_node::TypeNode;

/// Representa una expresión de bucle `while` en el AST.
/// 
/// Por ejemplo:
/// ```hulk
/// while (condición) { ... }
/// ```
/// 
/// - `condition`: expresión booleana de condición.
/// - `body`: cuerpo del bucle.
/// - `_type`: tipo inferido o declarado de la expresión (opcional).
#[derive(Debug, Clone, PartialEq)]
pub struct WhileLoop {
    pub condition: Box<Expr>,
    pub body: Box<Expr>,
    pub _type: Option<TypeNode>,
    pub token_pos: TokenPos,
}

impl WhileLoop {
    /// Crea una nueva expresión de bucle `while`.
    ///
    /// # Arguments
    /// * `condition` - Expresión booleana de condición.
    /// * `body` - Cuerpo del bucle.
    pub fn new(condition: Box<Expr>, body: Box<Expr>, token_pos: TokenPos) -> Self {
        Self {
            condition,
            body,
            _type: None,
            token_pos:  token_pos,
        }
    }

    /// Establece el tipo de la expresión `while`.
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}

impl Codegen for WhileLoop {
    /// Genera el código LLVM IR para la expresión de bucle `while`.
    ///
    /// Crea etiquetas únicas para el inicio, cuerpo y fin del bucle, evalúa la condición,
    /// ejecuta el cuerpo y repite mientras la condición sea verdadera.
    /// El valor de un `while` como expresión suele ser 0 (o unit), aquí se devuelve 0.
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // Genera etiquetas únicas
        let start_label = context.generate_label("while_start");
        let body_label = context.generate_label("while_body");
        let end_label = context.generate_label("while_end");

        // Salto incondicional al inicio del bucle
        context.emit(&format!("  br label %{}", start_label));

        // Etiqueta de inicio
        context.emit(&format!("{}:", start_label));
        let cond_reg = self.condition.codegen(context);
        // Salto condicional al cuerpo o al final
        context.emit(&format!(
            "  br i1 {}, label %{}, label %{}",
            cond_reg, body_label, end_label
        ));

        // Etiqueta del cuerpo
        context.emit(&format!("{}:", body_label));
        let _body_reg = self.body.codegen(context);
        // Al terminar el cuerpo, vuelve a evaluar la condición
        context.emit(&format!("  br label %{}", start_label));

        // Etiqueta de fin
        context.emit(&format!("{}:", end_label));
        // El valor de un while como expresión suele ser 0 (o unit), aquí devolvemos 0
        let result_reg = context.generate_temp();
        context.emit(&format!("  {} = add i32 0, 0", result_reg));
        result_reg
    }
}
