use crate::{hulk_ast_nodes::hulk_expression::Expr, typings::types_node::TypeNode};
#[derive(Debug, PartialEq,Clone)]
pub struct ForExpr {
    pub variable: String,
    pub start: Box<Expr>,
    pub end: Box<Expr>,
    pub body: Box<Expr>,
    pub _type: Option<TypeNode>,
}

impl ForExpr {
    pub fn new(variable: String, start: Expr, end: Expr, body: Expr) -> Self {
        ForExpr {
            variable,
            start: Box::new(start),
            end: Box::new(end),
            body: Box::new(body),
            _type: None,
        }
    }
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}