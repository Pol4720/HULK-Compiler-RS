#[derive(Debug, PartialEq)]
pub enum KeywordToken {
    Print,
    While,
    Elif,
    Else,
    If,
    In,
    Let,
    True,
    False,
}

#[derive(Debug, PartialEq)]
pub enum OperatorToken {
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

#[derive(Debug, PartialEq)]
pub enum DelimiterToken {
    Semicolon,
    Comma,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
}

#[derive(Debug, PartialEq)]
pub enum IdentifierToken {
    IDENTIFIER(String),
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(KeywordToken),
    Operator(OperatorToken),
    Delimiter(DelimiterToken),
    Identifier(IdentifierToken),
    EOF,
}
