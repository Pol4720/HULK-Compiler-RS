
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperatorToken {
    Mul,
    Div,
    Plus,
    Minus,
    Mod,
    Pow,
    Neg,
    Not,
    Eq,
    EqEq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    Concat,
    And,
    Or,
    DotEqual
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Plus,
    Minus,
    LogicalNot,
}


#[derive(Debug, PartialEq)]
pub enum DelimiterToken {
    Semicolon,
    Colon,
    Doubledot,
    Comma,
    Lparen,
    Rparen,
    Lbrace,
    Arrow,
    Rbrace,
}