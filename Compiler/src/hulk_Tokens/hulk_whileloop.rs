use crate::hulk_tokens::hulk_expression::Expr;
use crate::visitor::hulk_accept::Accept;
use crate::visitor::hulk_visitor::Visitor;

#[derive(Debug, Clone, PartialEq)]
pub struct WhileLoop {
    pub condition: Box<Expr>,
    pub body: Box<Expr>,
}

impl WhileLoop {
    pub fn new(condition: Box<Expr>, body: Box<Expr>) -> Self {
        Self {
            condition,
            body,
        }
    }
}

impl Accept for WhileLoop {
    fn accept<V: Visitor<T>, T>(&self, visitor: &mut V) -> T {
        visitor.visit_while_loop(self)
    }
}