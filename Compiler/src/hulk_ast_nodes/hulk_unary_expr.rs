use crate::hulk_tokens::hulk_operators::UnaryOperator;
use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::typings::types_node::TypeNode;

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr {
    pub operator: UnaryOperator,
    pub operand: Box<Expr>,
    pub _type: Option<TypeNode>,
}
impl UnaryExpr {
    pub fn new(operator: UnaryOperator, operand: Box<Expr>) -> Self {
        UnaryExpr { operator, operand , _type: None }
    }
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}


impl Codegen for UnaryExpr {
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
