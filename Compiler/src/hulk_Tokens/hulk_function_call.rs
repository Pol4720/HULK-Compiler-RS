use crate::ast::Expr;

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
