use std::fmt;

use crate::hulk_tokens::hulk_expression::Expr;

#[derive(Debug, PartialEq,Clone)]
pub struct FunctionParams {
    pub name: String,
    pub signature: String,
}

impl FunctionParams {
    pub fn new(name: String, signature: String) -> Self {
        FunctionParams {
            name,
            signature,
        }
    }
}

impl fmt::Display for FunctionParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Adjust this to print your parameters as needed
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<FunctionParams>,
    pub return_type: String,
    pub body: Box<Expr>,
}

impl FunctionDef {
    pub fn new_expr(name: String, params: Vec<FunctionParams>, return_type: String, expr: Box<Expr>) -> Self {
        FunctionDef {
            name,
            params,
            return_type,
            body: expr,
        }
    }
}