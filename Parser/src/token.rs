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
        make_token(TokenType::FUNCTION, "function", 1, 1),
        make_token(TokenType::IDENT, "main", 1, 10),
        make_token(TokenType::LPAREN, "(", 1, 14),
        make_token(TokenType::RPAREN, ")", 1, 15),
        make_token(TokenType::LBRACE, "{", 2, 1),
        make_token(TokenType::RBRACE, "}", 3, 1),
        make_token(TokenType::EOF, "", 4, 1),
        make_token(TokenType::SEMICOLON, ";", 3, 2)
    ]
}
