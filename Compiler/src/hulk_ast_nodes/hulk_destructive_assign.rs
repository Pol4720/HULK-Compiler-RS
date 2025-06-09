use crate ::hulk_ast_nodes::hulk_expression::Expr;

#[derive(Debug, PartialEq,Clone)]
pub struct DestructiveAssignment {
    pub identifier: String,
    pub expression: Box<Expr>,
}

impl DestructiveAssignment {
    pub fn new(identifier: String, expression: Expr) -> Self {
        Self {
            identifier,
            expression: Box::new(expression),
        }
    }
}