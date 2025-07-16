use super::{AstNode, Position, Visitor};

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
        pos: Position,
    },
    Unary {
        operator: UnaryOp,
        operand: Box<Expr>,
        pos: Position,
    },
    Literal {
        value: LiteralValue,
        pos: Position,
    },
    Variable {
        name: String,
        pos: Position,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        pos: Position,
    },
    Let {
        bindings: Vec<LetBinding>,
        body: Box<Expr>,
        pos: Position,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
        pos: Position,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Pow,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Not, Neg,
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub struct LetBinding {
    pub name: String,
    pub value: Expr,
    pub pos: Position,
}

impl AstNode for Expr {
    fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_expr(self)
    }
}
