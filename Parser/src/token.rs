#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    IDENT,
    FUNCTION,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    SEMICOLON,
    EOF,
    // ... add others as needed
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

// Helper function to create a token
pub fn make_token(token_type: TokenType, lexeme: &str, line: usize, column: usize) -> Token {
    Token {
        token_type,
        lexeme: lexeme.to_string(),
        line,
        column,
    }
}

/// Returns a list of mock tokens for testing
pub fn mock_tokens() -> Vec<Token> {
    vec![

        make_token(TokenType::IDENT, "a", 1, 5),
        make_token(TokenType::IDENT, "+", 1, 7),
        make_token(TokenType::IDENT, "b", 1, 9),
        make_token(TokenType::SEMICOLON, ";", 1, 10),
        make_token(TokenType::EOF, "", 2, 1)
    ]
}
