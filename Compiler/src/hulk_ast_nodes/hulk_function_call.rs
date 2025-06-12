use crate::{hulk_ast_nodes::hulk_expression::Expr, typings::types_node::TypeNode};

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub funct_name: String,             
    pub arguments: Vec<Expr>,
    pub _type: Option<TypeNode>,
}


impl FunctionCall {
    pub fn new(funct_name: String, arguments: Vec<Expr>) -> Self {
        FunctionCall { funct_name, arguments, _type: None }
    }
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}
