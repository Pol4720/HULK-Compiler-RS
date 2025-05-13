use super::hulk_identifier::Identifier;
use crate::ast::Expr;

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

impl Assignment {
    pub fn to_tree(&self, indent: usize) -> String {
        let padding = "  ".repeat(indent);
        format!(
            "{}Assignment({})\n{}",
            padding,
            self.identifier,
            self.expression.to_tree(indent + 1)
        )
    }
}