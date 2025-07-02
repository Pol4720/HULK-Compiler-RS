use super::regex_char::RegexChar;
use super::regex_escape::RegexEscape;

/// Representa una clase de caracteres en una expresión regular.
///
/// Puede ser:
/// - `Set(Vec<RegexChar>)`: conjunto de caracteres (ej: [abc]).
/// - `Ranges(Vec<(char, char)>)`: múltiples rangos de caracteres (ej: [a-zA-Z0-9]).
/// - `Mixed { ranges: Vec<(char, char)>, singles: Vec<RegexChar> }`: mezcla de rangos y caracteres individuales (ej: [a-zA-Z_]).
/// - `Negated(Box<RegexClass>)`: negación de una clase ([^a], [^a-z]).
/// - `Dot`: el metacarácter punto, que representa cualquier carácter excepto salto de línea (.)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RegexClass {
    /// Un conjunto de caracteres explícitos, como [abc]
    Set(Vec<RegexChar>),
    /// Uno o más rangos de caracteres, como [a-zA-Z0-9]
    Ranges(Vec<(char, char)>),
    /// Mezcla de rangos y caracteres individuales, como [a-zA-Z_]
    Mixed {
        ranges: Vec<(char, char)>,
        singles: Vec<RegexChar>,
    },
    /// Una clase negada, como [^a] o [^a-z]
    Negated(Box<RegexClass>),
    /// El metacarácter punto, que representa cualquier carácter excepto salto de línea (.)
    Dot,
}

// Implementación para RegexClass
impl RegexClass {
    pub fn to_repr(&self) -> String {
        match self {
            RegexClass::Set(chars) => {
                let inner = chars
                    .iter()
                    .map(|c| format!("{:?}", c))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("Set[{}]", inner)
            }
            RegexClass::Ranges(ranges) => {
                let inner = ranges
                    .iter()
                    .map(|(a, b)| format!("{}-{}", a, b))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("Ranges[{}]", inner)
            }
            RegexClass::Mixed { ranges, singles } => {
                let ranges_repr = ranges
                    .iter()
                    .map(|(a, b)| format!("{}-{}", a, b))
                    .collect::<Vec<_>>()
                    .join(", ");
                let singles_repr = singles
                    .iter()
                    .map(|c| format!("{:?}", c))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("Mixed[ranges: {}, singles: {}]", ranges_repr, singles_repr)
            }
            RegexClass::Negated(inner) => {
                format!("Negated[{}]", inner.to_repr())
            }
            RegexClass::Dot => "Dot".to_string(),
        }
    }
}
