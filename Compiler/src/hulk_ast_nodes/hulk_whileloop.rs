use crate::{hulk_ast_nodes::hulk_expression::Expr, typings::types_node::TypeNode};

#[derive(Debug, Clone, PartialEq)]
pub struct WhileLoop {
    pub condition: Box<Expr>,
    pub body: Box<Expr>,
    pub _type: Option<TypeNode>
}

impl WhileLoop {
    pub fn new(condition: Box<Expr>, body: Box<Expr>) -> Self {
        Self {
            condition,
            body,
            _type: None,
        }
    }
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}
