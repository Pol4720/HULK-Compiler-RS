use crate::hulk_tokens::hulk_expression::Expr;
use crate::codegen::traits::Codegen;
use crate::codegen::context::CodegenContext;

#[derive(Debug, Clone, PartialEq)]
pub struct WhileLoop {
    pub condition: Box<Expr>,
    pub body: Box<Expr>,
}

impl WhileLoop {
    pub fn new(condition: Box<Expr>, body: Box<Expr>) -> Self {
        Self {
            condition,
            body,
        }
    }
}

impl Codegen for WhileLoop {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // TODO: Implement codegen for WhileLoop
        String::new()
    }
}
