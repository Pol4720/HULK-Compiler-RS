use crate::hulk_ast_nodes::hulk_identifier::Identifier;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::typings::types_node::TypeNode;

#[derive(Debug, PartialEq,Clone)]
pub struct NewTypeInstance {
    pub type_name: Identifier,             
    pub arguments: Vec<Expr>,
    pub _type: Option<TypeNode>
}

impl NewTypeInstance {
    pub fn new(type_name: Identifier, arguments: Vec<Expr>) -> Self {
        NewTypeInstance { type_name, arguments, _type:None}
    }
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}