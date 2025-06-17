/// Enum que representa las operaciones unarias en una expresión regular.
///
/// - `Star`: Cero o más repeticiones (`*`)
/// - `Plus`: Una o más repeticiones (`+`)
/// - `Optional`: Cero o una repetición (`?`)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RegexUnOp {
    /// Cero o más repeticiones: *
    Star,
    /// Una o más repeticiones: +
    Plus,
    /// Cero o una repetición: ?
    Optional,
}

impl RegexUnOp {
    /// Devuelve el símbolo asociado a la operación unaria.
    pub fn symbol(&self) -> char {
        match self {
            RegexUnOp::Star => '*',
            RegexUnOp::Plus => '+',
            RegexUnOp::Optional => '?',
        }
    }
}

