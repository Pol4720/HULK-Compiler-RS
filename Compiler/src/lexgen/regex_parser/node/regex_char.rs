use super::AstNode;
use super::AstNodeImpl;
use super::AstNodeKind;
use crate::lexgen::regex_parser::regex_char::RegexChar;

/// Representa un carácter en una expresión regular.
///
/// Puede ser:
/// - `Literal(char)`: un carácter literal específico.
/// - `Epsilon`: la transición vacía (ε), usada en autómatas para representar ausencia de consumo de carácter.
/// - `Start`: el carácter de inicio de línea (^).
/// - `End`: el carácter de final de línea ($).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RegexChar {
    /// Un carácter literal en la expresión regular.
    Literal(char),
    /// La transición vacía (ε).
    Epsilon,
    /// El carácter de inicio de línea (^).
    Start,
    /// El carácter de final de línea ($).
    End,
}

impl RegexChar {
    /// Devuelve `true` si el carácter es una transición vacía (ε).
    pub fn is_epsilon(&self) -> bool {
        matches!(self, RegexChar::Epsilon)
    }

    /// Devuelve `true` si el carácter es un literal.
    pub fn is_literal(&self) -> bool {
        matches!(self, RegexChar::Literal(_))
    }

    /// Devuelve `true` si el carácter es de inicio de línea (^).
    pub fn is_start(&self) -> bool {
        matches!(self, RegexChar::Start)
    }

    /// Devuelve `true` si el carácter es de final de línea ($).
    pub fn is_end(&self) -> bool {
        matches!(self, RegexChar::End)
    }

    /// Si es un literal, retorna el carácter; si no, retorna `None`.
    pub fn as_char(&self) -> Option<char> {
        if let RegexChar::Literal(c) = self {
            Some(*c)
        } else {
            None
        }
    }
}

/// Nodo AST para un literal en una expresión regular.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LiteralNode {
    pub value: RegexChar,
}

impl LiteralNode {
    pub fn new(value: RegexChar) -> Self {
        LiteralNode { value }
    }
}

impl AstNode for LiteralNode {
    fn children(&self) -> Vec<&AstNodeImpl> {
        vec![]
    }
    fn to_ast(&self) -> AstNodeImpl {
        AstNodeImpl::new(super::AstNodeKind::Literal(self.clone()))
    }
    fn to_repr(&self) -> String {
        format!("Literal({:?})", self.value)
    }
}
