use crate::hulk_ast_nodes::hulk_expression::Expr;

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
