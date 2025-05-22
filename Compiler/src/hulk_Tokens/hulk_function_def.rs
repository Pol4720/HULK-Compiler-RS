use crate::ast::Expr;

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