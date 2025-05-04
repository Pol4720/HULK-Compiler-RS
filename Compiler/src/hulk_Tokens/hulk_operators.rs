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
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
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
    Comma,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
}