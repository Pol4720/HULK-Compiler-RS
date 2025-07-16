use super::{AstNode, Position, Visitor, Expr};

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression {
        expr: Expr,
        pos: Position,
    },
    Function {
        name: String,
        params: Vec<Parameter>,
        body: Expr,
        pos: Position,
    },
    Type {
        name: String,
        attributes: Vec<Attribute>,
        methods: Vec<Method>,
        pos: Position,
    },
    Protocol {
        name: String,
        methods: Vec<MethodSignature>,
        pos: Position,
    },
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Option<String>,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub type_annotation: Option<String>,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct Method {
    pub name: String,
    pub params: Vec<Parameter>,
    pub body: Expr,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct MethodSignature {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Option<String>,
    pub pos: Position,
}

impl AstNode for Stmt {
    fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_stmt(self)
    }
}
