#[derive(Debug, Clone)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Option<Span>,
}

#[derive(Debug)]
pub enum StmtKind {
    ExprStmt(Expr),
    FunctionDecl {
        name: String,
        params: Vec<(String, Option<Type>)>,
        body: Box<Stmt>,
        return_type: Option<Type>,
    },
    TypeDecl {
        name: String,
        type_params: Vec<String>,
        attributes: Vec<AttributeDecl>,
        methods: Vec<MethodDecl>,
        base_type: String,
        base_args: Vec<Expr>,
    },
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Option<Span>,
}

#[derive(Debug)]
pub enum ExprKind {
    Number(f64),
    String(String),
    Boolean(bool),
    Variable(String),
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Block(Vec<Stmt>),  // Changed from Block { stmts: Vec<Stmt> }
    Let {
        var_name: String,  // Changed from name
        value: Box<Expr>,
        body: Option<Box<Expr>>,  // Made optional since some implementations don't provide it
        declared_type: Option<Type>,
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
    For {
        var_name: String,  // Changed from iterator
        iterable: Box<Expr>,  // Changed from collection
        body: Box<Expr>,
    },
    Call {
        function: String,
        args: Vec<Expr>,
    },
    MethodCall {
        object: Box<Expr>,
        method: String,
        args: Vec<Expr>,
    },
    GetAttr {
        object: Box<Expr>,
        attr: String,
    },
    SetAttr {
        object: Box<Expr>,
        attr: String,
        value: Box<Expr>,
    },
    Assign {
        var_name: String,
        value: Box<Expr>,
    },
    Is {
        expr: Box<Expr>,
        type_name: String,
    },
    As {
        expr: Box<Expr>,
        type_name: String,
    },
    SelfExpr,
    BaseCall {
        args: Vec<Expr>,
    },
    New {
        type_name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug)]
pub struct AttributeDecl {
    pub name: String,
    pub initializer: Option<Expr>,
    pub declared_type: Option<Type>,
}

#[derive(Debug)]
pub struct MethodDecl {
    pub name: String,
    pub params: Vec<(String, Option<Type>)>,
    pub body: Box<Stmt>,
    pub return_type: Option<Type>,
}

#[derive(Debug, Clone)]
pub enum Type {
    Number,
    String,
    Boolean,
    Object(String),
    Unknown,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
    Concat,
    ConcatWs,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UnaryOp {
    Neg,
}