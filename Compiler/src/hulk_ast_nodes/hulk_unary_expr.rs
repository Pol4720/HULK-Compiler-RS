//! # UnaryExpr AST Node
//!
//! Este módulo define el nodo `UnaryExpr` del AST para el compilador Hulk.
//! Permite representar expresiones unarias como la negación aritmética (`-x`), la negación lógica (`!x`) y el operador unario positivo (`+x`).
//! Incluye la estructura, métodos asociados y la generación de código LLVM IR.

use crate::hulk_tokens::hulk_operators::UnaryOperator;
use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::typings::types_node::TypeNode;

/// Representa una expresión unaria en el AST.
/// 
/// Por ejemplo: `-x`, `!flag`, `+y`
/// 
/// - `operator`: operador unario (`Minus`, `LogicalNot`, `Plus`).
/// - `operand`: expresión sobre la que se aplica el operador.
/// - `_type`: tipo inferido o declarado de la expresión (opcional).
#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr {
    pub operator: UnaryOperator,
    pub operand: Box<Expr>,
    pub _type: Option<TypeNode>,
}

impl UnaryExpr {
    /// Crea una nueva expresión unaria.
    ///
    /// # Arguments
    /// * `operator` - Operador unario.
    /// * `operand` - Expresión sobre la que se aplica el operador.
    pub fn new(operator: UnaryOperator, operand: Box<Expr>) -> Self {
        UnaryExpr { operator, operand , _type: None }
    }

    /// Establece el tipo de la expresión unaria.
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}

impl Codegen for UnaryExpr {
    /// Genera el código LLVM IR para la expresión unaria.
    ///
    /// Selecciona la instrucción LLVM adecuada según el operador:
    /// - `Minus`: negación aritmética (`sub i32 0, valor`)
    /// - `LogicalNot`: negación lógica (`xor i32 valor, -1`)
    /// - `Plus`: copia el valor (`add i32 0, valor`)
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // Genera el código del operando
        let operand_reg = self.operand.codegen(context);

        // Obtiene un nuevo registro temporal
        let result_reg = context.generate_temp();

        // Selecciona la operación LLVM correspondiente
        let line = match self.operator {
            UnaryOperator::Minus => {
                // Negación aritmética: -x
                format!("  {} = sub i32 0, {}", result_reg, operand_reg)
            }
            UnaryOperator::LogicalNot => {
                // Negación lógica: !x (bitwise not)
                format!("  {} = xor i32 {}, -1", result_reg, operand_reg)
            }
            UnaryOperator::Plus => {
                // Operador + unario: simplemente copia el valor
                format!("  {} = add i32 0, {}", result_reg, operand_reg)
            }
        };

        // Emite la instrucción LLVM IR
        context.emit(&line);

        result_reg
    }
}
