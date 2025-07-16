/// Token types for the HULK language
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Number,
    String,
    Boolean,
    Identifier,
    
    // Keywords
    Let,
    In,
    If,
    Else,
    While,
    For,
    Function,
    Type,
    Protocol,
    New,
    Is,
    As,
    
    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Assign,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    Not,
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Colon,
    Arrow,
    
    // Special
    Eof,
    Error,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize, column: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
            column,
        }
    }
}
