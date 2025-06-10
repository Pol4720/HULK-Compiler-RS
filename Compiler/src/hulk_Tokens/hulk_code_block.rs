use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
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

impl Codegen for ExpressionList {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        let mut last_reg = String::new();
        for expr in self.expressions.iter() {
            last_reg = expr.codegen(context);
        }
        last_reg
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub expression_list: Box<ExpressionList>,
}

impl Block {
    pub fn new(expression_list: ExpressionList) -> Self {
        Block {
            expression_list: Box::new(expression_list),
        }
    }
}

impl Codegen for Block {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        let exprs = &self.expression_list.expressions;
        let mut last_reg = String::new();
        for expr in exprs.iter() {
            last_reg = expr.codegen(context);
        }
        last_reg
    }
}
