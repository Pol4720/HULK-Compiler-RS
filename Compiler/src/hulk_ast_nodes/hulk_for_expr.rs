use crate::hulk_ast_nodes::hulk_expression::Expr;
#[derive(Debug, PartialEq,Clone)]
pub struct ForExpr {
    pub variable: String,
    pub start: Box<Expr>,
    pub end: Box<Expr>,
    pub body: Box<Expr>,
}

impl ForExpr {
    pub fn new(variable: String, start: Expr, end: Expr, body: Expr) -> Self {
        ForExpr {
            variable,
            start: Box::new(start),
            end: Box::new(end),
            body: Box::new(body),
        }
    }
}