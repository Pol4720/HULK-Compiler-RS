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
        // TODO: Implement codegen for BinaryExpr
        String::new()
    }
}