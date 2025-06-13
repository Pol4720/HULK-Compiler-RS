use crate::{hulk_ast_nodes::Expr, typings::types_node::TypeNode};

#[derive(Debug, PartialEq, Clone)]

pub struct PrintExpr {
    pub expr: Box<Expr>,
    pub _type: Option<TypeNode>
}

impl PrintExpr {
    pub fn new(
        expr: Box<Expr>,
        _type: Option<TypeNode>
    ) -> Self {
        PrintExpr {
            expr,
            _type
        }
    }

    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}