#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    // Basic identifiers and literals
    IDENT,
    NUMBER,
    STRING,
    TRUE,
    FALSE,
    
    // Keywords
    FUNCTION,
    IF,
    ELSE,
    ELIF,
    WHILE,
    FOR,
    IN,
    LET,
    IS,
    AS,
    TYPE,
    INHERITS,
    NEW,
    SELF,
    BASE,
    
    // Brackets and delimiters
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    SEMICOLON,
    COMMA,
    COLON,
    DOT,
    
    // Operators
    PLUS,
    MINUS,
    MULT,
    DIV,
    MOD,
    POW,
    ASSIGN,
    ASSIGN_DESTRUCT,
    ARROW,
    LT,
    GT,
    LE,
    GE,
    EQ,
    NEQ,
    OR,
    AND,
    CONCAT,
    CONCAT_WS,
    
    // End of file
    EOF,

    a,
    b,

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

