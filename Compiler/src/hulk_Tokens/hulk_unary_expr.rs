use crate::ast::Expr;
use super::hulk_operators::UnaryOperator;

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