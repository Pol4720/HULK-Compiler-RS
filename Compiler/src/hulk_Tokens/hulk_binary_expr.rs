use super::hulk_operators::*;
use crate::hulk_tokens::hulk_expression::Expr;
use crate::codegen::traits::Codegen;
use crate::codegen::context::CodegenContext;

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: BinaryOperatorToken,
    pub right: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(left: Box<Expr>, operator: BinaryOperatorToken, right: Box<Expr>) -> Self {
        BinaryExpr { left, operator, right }
    }
}

impl Codegen for BinaryExpr {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // Genera el código de los operandos izquierdo y derecho
        let left_reg = self.left.codegen(context);
        let right_reg = self.right.codegen(context);

        // Obtiene un nuevo registro temporal
        let result_reg = context.generate_temp();

        // Selecciona la operación LLVM correspondiente
        let op_ir = match self.operator {
            BinaryOperatorToken::Plus => "add",
            BinaryOperatorToken::Minus => "sub",
            BinaryOperatorToken::Mul => "mul",
            BinaryOperatorToken::Div => "sdiv",
            BinaryOperatorToken::Mod => "srem",
            BinaryOperatorToken::Eq | BinaryOperatorToken::EqEq => "icmp eq",
            BinaryOperatorToken::Neq => "icmp ne",
            BinaryOperatorToken::Lt => "icmp slt",
            BinaryOperatorToken::Gt => "icmp sgt",
            BinaryOperatorToken::Lte => "icmp sle",
            BinaryOperatorToken::Gte => "icmp sge",
            BinaryOperatorToken::And => "and",
            BinaryOperatorToken::Or => "or",
            BinaryOperatorToken::Pow => "pow",
            BinaryOperatorToken::Concat => "concat",
            BinaryOperatorToken::DotEqual => panic!("Operador 'DotEqual' no soportado en Codegen"),
            BinaryOperatorToken::Neg => panic!("Operador 'Neg' no soportado en Codegen"),
            BinaryOperatorToken::Not => panic!("Operador 'Not' no soportado en Codegen"),
        };

        // Determina el tipo del resultado
        let line = match op_ir {
            "pow" => {
                // Llama a la función externa de potencia (debes declarar 'llvm.powi.i32' en tu IR)
                format!("  {} = call i32 @llvm.powi.i32(i32 {}, i32 {})", result_reg, left_reg, right_reg)
            }
            "concat" => {
                // Llama a la función auxiliar de concatenación de cadenas (debes implementarla en tu runtime)
                // Supone que left_reg y right_reg son i8* (punteros a cadenas)
                format!("  {} = call i8* @hulk_str_concat(i8* {}, i8* {})", result_reg, left_reg, right_reg)
            }
            _ if op_ir.starts_with("icmp") => {
                // Operaciones de comparación -> i1
                format!("  {} = {} i32 {}, {}", result_reg, op_ir, left_reg, right_reg)
            }
            _ => {
                // Operaciones aritméticas -> i32
                format!("  {} = {} i32 {}, {}", result_reg, op_ir, left_reg, right_reg)
            }
        };

        // Emite la instrucción LLVM IR
        context.emit(&line);

        result_reg
    }
}