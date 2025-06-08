use crate::hulk_tokens::hulk_expression::Expr;
use crate::codegen::traits::Codegen;
use crate::codegen::context::CodegenContext;

#[derive(Debug, PartialEq,Clone)]
pub struct ForExpr {
    pub variable: String,
    pub start: Box<Expr>,
    pub end: Box<Expr>,
    pub body: Box<Expr>,
}

impl ForExpr {
    pub fn new(variable: String, start: Expr, end: Expr, body: Expr) -> Self {
        ForExpr {
            variable,
            start: Box::new(start),
            end: Box::new(end),
            body: Box::new(body),
        }
    }
}

impl Codegen for ForExpr {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // TODO: Implement codegen for ForExpr
        String::new()
    }
}