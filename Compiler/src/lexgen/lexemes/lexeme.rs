/// Representa un lexema/token reconocido por el analizador léxico.
#[derive(Debug, Clone, PartialEq)]
pub struct Lexeme {
    pub token_type: String,
    pub value: String,
    pub line: usize,
    pub column_start: usize,
    pub column_end: usize,
}
