use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::hulk_ast_nodes::hulk_identifier::Identifier;
use crate::typings::types_node::TypeNode;



#[derive(Debug, Clone, PartialEq)]
pub struct MemberAccess {
    pub object: Box<Expr>,
    pub member: Identifier,
    pub _type: Option<TypeNode>, // Optional type for the member access
}

impl MemberAccess {
    pub fn new(object: Expr, member: Identifier) -> Self {
        Self {
            object: Box::new(object),
            member,
            _type: None,
        }
    }
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }

}