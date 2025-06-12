use crate::{hulk_ast_nodes::hulk_expression::Expr, typings::types_node::TypeNode};

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionList {
    pub expressions: Box<Vec<Expr>>,
}

impl ExpressionList {
    pub fn new(expressions: Vec<Expr>) -> Self {
        ExpressionList {
            expressions: Box::new(expressions),
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub expression_list: Box<ExpressionList>,
    pub _type: Option<TypeNode>
}

impl Block {
    pub fn new(expression_list: ExpressionList) -> Self {
        Block {
            expression_list: Box::new(expression_list),
            _type: None,
        }
    }

    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}