/// Enum que representa las operaciones binarias en una expresión regular.
///
/// - `Concat`: Concatenación de expresiones.
/// - `Or`: Alternancia (|).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RegexBinOp {
    /// Concatenación de expresiones regulares.
    Concat,
    /// Alternancia: |
    Or,
}

impl RegexBinOp {
    /// Devuelve el símbolo asociado a la operación binaria, si existe.
    pub fn symbol(&self) -> Option<char> {
        match self {
            RegexBinOp::Concat => None, // La concatenación no tiene símbolo explícito
            RegexBinOp::Or => Some('|'),
        }
    }
}
