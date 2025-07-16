#[derive(Debug)]
pub enum Stmt {
    ExprStmt(Expr),
    FunctionDecl {
        name: String,
        params: Vec<String>,
        body: Expr,
    },
    TypeDecl {
        name: String,
        attributes: Vec<String>, // placeholder
        methods: Vec<String>,    // placeholder
    },
    // ...otros tipos de declaración
}

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    String(String),
    Variable(String),
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Block(Vec<Stmt>),
    Let {
        name: String,
        value: Box<Expr>,
        body: Box<Expr>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },
    While {
        condition: Box<Expr>,
        body: Box<Expr>,
    },
    // ...
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
}
