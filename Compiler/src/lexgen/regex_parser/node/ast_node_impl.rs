use super::bin_op::RegexBinOp;
use super::group::RegexGroup;
use super::regex_char::RegexChar;
use super::regex_class::RegexClass;
use super::un_op::RegexUnOp;
use std::fmt::Debug;

/// Enum que representa los diferentes tipos de nodos del AST de una expresión regular.
#[derive(Debug, Clone, PartialEq)]
pub enum AstNodeKind {
    RegexChar(RegexChar), // Nodo hoja: carácter, escape, inicio/fin de línea, etc.
    BinOp {
        op: RegexBinOp,
        left: Box<AstNodeImpl>,
        right: Box<AstNodeImpl>,
    },
    UnOp {
        op: RegexUnOp,
        expr: Box<AstNodeImpl>,
    },
    Group(RegexGroup<Box<AstNodeImpl>>),
    Class(RegexClass),
}

/// Nodo del AST para expresiones regulares.
#[derive(Debug, Clone, PartialEq)]
pub struct AstNodeImpl {
    pub kind: AstNodeKind,
}

/// Trait común para todos los nodos del AST de regex.
pub trait AstNode: Debug + Clone + PartialEq {
    /// Devuelve los hijos inmediatos del nodo (para recorridos).
    fn children(&self) -> Vec<&AstNodeImpl>;
    /// Devuelve el nodo como AstNodeImpl (útil para conversión).
    fn to_ast(&self) -> AstNodeImpl;
    /// Devuelve una representación string del nodo (debug).
    fn to_repr(&self) -> String;
}

impl AstNode for AstNodeImpl {
    fn children(&self) -> Vec<&AstNodeImpl> {
        match &self.kind {
            AstNodeKind::RegexChar(_) => vec![],
            AstNodeKind::BinOp { left, right, .. } => vec![left, right],
            AstNodeKind::UnOp { expr, .. } => vec![expr],
            AstNodeKind::Group(group) => group.expr.children(),
            AstNodeKind::Class(_) => vec![],
        }
    }

    fn to_ast(&self) -> AstNodeImpl {
        self.clone()
    }

    fn to_repr(&self) -> String {
        format!("{:?}", self)
    }
}
