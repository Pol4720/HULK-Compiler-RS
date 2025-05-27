use super::hulk_operators::*;
use crate::hulk_tokens::hulk_expression::Expr;
use crate::visitor::hulk_accept::Accept;
use crate::visitor::hulk_visitor::Visitor;


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

impl Accept for BinaryExpr {
    fn accept<V: Visitor<T>, T>(&self, visitor: &mut V) -> T {
        visitor.visit_binary_expr(self)
    }
}