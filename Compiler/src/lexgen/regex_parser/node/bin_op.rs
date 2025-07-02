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

