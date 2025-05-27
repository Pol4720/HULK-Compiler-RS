use crate::hulk_tokens::hulk_expression::Expr;
use crate::visitor::hulk_accept::Accept;
use crate::visitor::hulk_visitor::Visitor;

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDef{
    pub name: String,
    pub params: Vec<String>,
    pub body: Box<Expr>,
}

impl FunctionDef {
    pub fn new(name: String, params: Vec<String>, expr: Box<Expr>) -> Self {
        FunctionDef {
            name,
            params,
            body: expr,
        }
    }
}

impl Accept for FunctionDef {
    fn accept<V: Visitor<T>, T>(&self, visitor: &mut V) -> T {
        visitor.visit_function_def(self)
    }
}