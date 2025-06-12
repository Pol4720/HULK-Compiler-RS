use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::hulk_tokens::hulk_operators::BinaryOperatorToken;
use crate::typings::types_node::TypeNode;

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: BinaryOperatorToken,
    pub right: Box<Expr>,
    pub _type: Option<TypeNode>,
}
impl BinaryExpr {
    pub fn new(left: Box<Expr>, operator: BinaryOperatorToken, right: Box<Expr>) -> Self {
        BinaryExpr { left, operator, right, _type: None }
    }

    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}