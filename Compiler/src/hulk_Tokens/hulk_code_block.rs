use crate::hulk_tokens::hulk_expression::Expr;

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
}

impl Block {
    pub fn new(expression_list: ExpressionList) -> Self {
        Block {
            expression_list: Box::new(expression_list)
        }
    }
}