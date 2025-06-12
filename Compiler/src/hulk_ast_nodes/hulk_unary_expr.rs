use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::hulk_tokens::hulk_operators::UnaryOperator;
use crate::typings::types_node::TypeNode;

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr {
    pub operator: UnaryOperator,
    pub operand: Box<Expr>,
    pub _type: Option<TypeNode>,
}
impl UnaryExpr {
    pub fn new(operator: UnaryOperator, operand: Box<Expr>) -> Self {
        UnaryExpr { operator, operand , _type: None }
    }
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}

