use crate::{ hulk_ast_nodes::hulk_expression::Expr, typings::types_node::TypeNode};

#[derive(Debug, PartialEq,Clone)]
pub struct DestructiveAssignment {
    pub identifier: Box<Expr>,
    pub expression: Box<Expr>,
    pub _type: Option<TypeNode>,
}

impl DestructiveAssignment {
    pub fn new(identifier: Box<Expr>, expression: Expr) -> Self {
        Self {
            identifier,
            expression: Box::new(expression),
            _type: None,
        }
    }
    pub fn set_expression_type(&mut self, _type: TypeNode){
        self._type = Some(_type)
    }
}