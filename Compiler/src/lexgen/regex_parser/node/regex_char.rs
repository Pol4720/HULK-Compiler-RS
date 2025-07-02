use super::ast_node_impl::{AstNode, AstNodeImpl, AstNodeKind};
use super::regex_escape::RegexEscape;

/// Representa un carácter en una expresión regular.
///
/// Puede ser:
/// - `Literal(char)`: un carácter literal específico.
/// - `Escape(RegexEscape)`: un carácter de escape (\n, \t, etc.).
/// - `Epsilon`: la transición vacía (ε), usada en autómatas para representar ausencia de consumo de carácter.
/// - `Start`: el carácter de inicio de línea (^).
/// - `End`: el carácter de final de línea ($).
/// - `Any`: representa cualquier carácter (como el punto en regex).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum RegexChar {
    /// Un carácter literal en la expresión regular.
    Literal(char),
    /// Un carácter de escape (\n, \t, etc.)
    Escape(RegexEscape),
    /// La transición vacía (ε).
    Epsilon,
    /// El carácter de inicio de línea (^).
    Start,
    /// El carácter de final de línea ($).
    End,
    /// Representa cualquier carácter (como el punto en regex).
    Any,
}

impl RegexChar {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EpsilonNode {
    pub value: RegexChar,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StarNode {
    pub value: RegexChar,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EndNode {
    pub value: RegexChar,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnyNode {
    pub value: RegexChar,
}

impl EpsilonNode {
    /// Crea un nuevo nodo para la transición vacía (ε).
    pub fn new() -> Self {
        EpsilonNode {
            value: RegexChar::Epsilon,
        }
    }
}

impl AstNode for EpsilonNode {
    fn children(&self) -> Vec<&AstNodeImpl> {
        vec![]
    }

    fn to_ast(&self) -> AstNodeImpl {
        AstNodeImpl {
            kind: AstNodeKind::RegexChar(self.value.clone()),
        }
    }

    fn to_repr(&self) -> String {
        format!("Epsilon({:?})", self.value)
    }
}

impl StarNode {
    /// Crea un nuevo nodo para el carácter de inicio de línea (^).
    pub fn new() -> Self {
        StarNode {
            value: RegexChar::Start,
        }
    }
}

impl AstNode for StarNode {
    fn children(&self) -> Vec<&AstNodeImpl> {
        vec![]
    }

    fn to_ast(&self) -> AstNodeImpl {
        AstNodeImpl {
            kind: AstNodeKind::RegexChar(self.value.clone()),
        }
    }

    fn to_repr(&self) -> String {
        format!("Star({:?})", self.value)
    }
}

impl EndNode {
    /// Crea un nuevo nodo para el carácter de final de línea ($).
    pub fn new() -> Self {
        EndNode {
            value: RegexChar::End,
        }
    }
}

impl AstNode for EndNode {
    fn children(&self) -> Vec<&AstNodeImpl> {
        vec![]
    }

    fn to_ast(&self) -> AstNodeImpl {
        AstNodeImpl {
            kind: AstNodeKind::RegexChar(self.value.clone()),
        }
    }

    fn to_repr(&self) -> String {
        format!("End({:?})", self.value)
    }
}

impl LiteralNode {
    /// Crea un nuevo nodo para un carácter literal.
    pub fn new(value: char) -> Self {
        LiteralNode {
            value: RegexChar::Literal(value),
        }
    }
}

impl AstNode for LiteralNode {
    fn children(&self) -> Vec<&AstNodeImpl> {
        vec![]
    }

    fn to_ast(&self) -> AstNodeImpl {
        AstNodeImpl {
            kind: AstNodeKind::RegexChar(self.value.clone()),
        }
    }

    fn to_repr(&self) -> String {
        format!("Literal({})", self.value.as_char().unwrap_or(' '))
    }
}

impl AnyNode {
    /// Crea un nuevo nodo para cualquier carácter (.).
    pub fn new() -> Self {
        AnyNode {
            value: RegexChar::Any,
        }
    }
}

impl AstNode for AnyNode {
    fn children(&self) -> Vec<&AstNodeImpl> {
        vec![]
    }

    fn to_ast(&self) -> AstNodeImpl {
        AstNodeImpl {
            kind: AstNodeKind::RegexChar(self.value.clone()),
        }
    }

    fn to_repr(&self) -> String {
        format!("Any({:?})", self.value)
    }
}
