/// Representa un error léxico encontrado durante el análisis.
#[derive(Debug, Clone, PartialEq)]
pub struct LexicalError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}
