use crate::{hulk_ast_nodes::{Expr, FunctionCall}, typings::types_node::TypeNode};


#[derive(Debug, Clone, PartialEq)]
pub struct FunctionAccess {
    pub object: Box<Expr>,
    pub member: Box<FunctionCall>,
    pub _type: Option<TypeNode>, 
}

impl FunctionAccess {
    pub fn new(object: Expr, member: FunctionCall) -> Self {
        Self {
            object: Box::new(object),
            member: Box::new(member),
            _type: None,
        }
    }

    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}