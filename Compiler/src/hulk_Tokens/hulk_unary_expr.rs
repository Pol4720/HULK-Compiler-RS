use crate::hulk_tokens::hulk_expression::Expr;
use super::hulk_operators::UnaryOperator;
use crate::codegen::traits::Codegen;
use crate::codegen::context::CodegenContext;

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr {
    pub operator: UnaryOperator,
    pub operand: Box<Expr>,
}
impl UnaryExpr {
    pub fn new(operator: UnaryOperator, operand: Box<Expr>) -> Self {
        UnaryExpr { operator, operand }
    }
}

impl Codegen for UnaryExpr {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // TODO: Implement codegen for UnaryExpr
        String::new()
    }
}

