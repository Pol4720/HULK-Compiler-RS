use crate::hulk_tokens::hulk_expression::Expr;
use crate::visitor::hulk_accept::Accept;
use crate::visitor::hulk_visitor::Visitor;

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionList {
    pub expressions: Box<Vec<Expr>>,
}

impl ExpressionList {
    pub fn new(expressions: Vec<Expr>) -> Self {
        ExpressionList {
            expressions: Box::new(expressions),
        }
    }
}

impl Accept for ExpressionList {
    fn accept<V: Visitor<T>, T>(&self, visitor: &mut V) -> T {
        visitor.visit_expression_list(self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub expression_list: Box<ExpressionList>,
}

impl Block {
    pub fn new(expression_list: ExpressionList) -> Self {
        Block {
            expression_list: Box::new(expression_list)
        }
    }
}

impl Accept for Block {
    fn accept<V: Visitor<T>, T>(&self, visitor: &mut V) -> T {
        visitor.visit_code_block(self)
    }
}