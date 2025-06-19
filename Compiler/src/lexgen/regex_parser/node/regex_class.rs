use super::regex_char::RegexChar;

/// Representa una clase de caracteres en una expresión regular.
///
/// Puede ser:
/// - `Set(Vec<RegexChar>)`: conjunto de caracteres (ej: [abc]).
/// - `Range(char, char)`: rango de caracteres (ej: [a-z]).
/// - `Negated(Box<RegexClass>)`: negación de una clase ([^a], [^a-z]).
/// - `Dot`: el metacarácter punto, que representa cualquier carácter excepto salto de línea (.)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RegexClass {
    /// Un conjunto de caracteres explícitos, como [abc]
    Set(Vec<RegexChar>),
    /// Un rango de caracteres, como [a-z]
    Range(char, char),
    /// Una clase negada, como [^a] o [^a-z]
    Negated(Box<RegexClass>),
    /// El metacarácter punto, que representa cualquier carácter excepto salto de línea (.)
    Dot,
}

impl RegexClass {
    /// Devuelve true si la clase es una negación.
    pub fn is_negated(&self) -> bool {
        matches!(self, RegexClass::Negated(_))
    }
    /// Devuelve true si la clase es un conjunto explícito.
    pub fn is_set(&self) -> bool {
        matches!(self, RegexClass::Set(_))
    }
    /// Devuelve true si la clase es un rango.
    pub fn is_range(&self) -> bool {
        matches!(self, RegexClass::Range(_, _))
    }
    /// Devuelve true si la clase es el metacarácter punto.
    pub fn is_dot(&self) -> bool {
        matches!(self, RegexClass::Dot)
    }
}
