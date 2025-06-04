use super::hulk_operators::*;
use crate::hulk_tokens::hulk_expression::Expr;

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