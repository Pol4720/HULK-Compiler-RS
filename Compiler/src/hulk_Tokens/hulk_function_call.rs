use crate::hulk_tokens::hulk_expression::Expr;
use crate::visitor::hulk_accept::Accept;
use crate::visitor::hulk_visitor::Visitor;

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

impl Accept for FunctionCall {
    fn accept<V: Visitor<T>, T>(&self, visitor: &mut V) -> T {
        visitor.visit_function_call(self)
    }
}
