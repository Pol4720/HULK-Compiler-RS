use crate::hulk_ast_nodes::hulk_expression::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct Inheritance {
    pub parent_type: String,
    pub arguments: Vec<Expr>
}
impl Inheritance {
    pub fn new(parent_type: String, arguments: Vec<Expr>) -> Self {
        Inheritance {
            parent_type,
            arguments,
        }
    }
}