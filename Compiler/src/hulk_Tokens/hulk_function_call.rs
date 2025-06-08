use crate::hulk_tokens::hulk_expression::Expr;
use crate::codegen::traits::Codegen;
use crate::codegen::context::CodegenContext;

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub funct_name: String,             
    pub arguments: Vec<Expr>,
}

impl FunctionCall {
    pub fn new(funct_name: String, arguments: Vec<Expr>) -> Self {
        FunctionCall { funct_name, arguments }
    }
}

impl Codegen for FunctionCall {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // TODO: Implement codegen for FunctionCall
        String::new()
    }
}
