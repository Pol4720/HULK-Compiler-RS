use super::hulk_identifier::Identifier;
use crate::hulk_tokens::hulk_expression::Expr;
use crate::visitor::hulk_accept::Accept;
use crate::visitor::hulk_visitor::Visitor;



#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub identifier: Identifier,
    pub expression: Box<Expr>,
}

impl Assignment {
    pub fn new(identifier: Identifier, expression: Box<Expr>) -> Self {
        Assignment { identifier, expression }
    }
}



impl Accept for Assignment {
    fn accept<V: Visitor<T>, T>(&self, visitor: &mut V) -> T {
        visitor.visit_assignment(self)
    }
}