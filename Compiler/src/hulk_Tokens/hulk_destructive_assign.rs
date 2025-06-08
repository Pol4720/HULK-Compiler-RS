use crate ::hulk_tokens::hulk_expression::Expr;
use crate::codegen::traits::Codegen;
use crate::codegen::context::CodegenContext;

#[derive(Debug, PartialEq,Clone)]
pub struct DestructiveAssignment {
    pub identifier: String,
    pub expression: Box<Expr>,
}

impl DestructiveAssignment {
    pub fn new(identifier: String, expression: Expr) -> Self {
        Self {
            identifier,
            expression: Box::new(expression),
        }
    }
}

impl Codegen for DestructiveAssignment {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // TODO: Implement codegen for DestructiveAssignment
        String::new()
    }
}