use super::bin_op::RegexBinOp;
use super::group::RegexGroup;
use super::regex_char::LiteralNode;
use super::regex_class::RegexClass;
use super::un_op::RegexUnOp;
use std::fmt::Debug;

/// Enum que representa los diferentes tipos de nodos del AST de una expresión regular.
#[derive(Debug, Clone, PartialEq)]
pub enum AstNodeKind {
    Literal(LiteralNode),
    BinOp {
        op: RegexBinOp,
        left: Box<AstNode>,
        right: Box<AstNode>,
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

pub trait AstNode: Debug + Clone + PartialEq {
    fn children(&self) -> Vec<&AstNodeImpl>;
    fn to_ast(&self) -> AstNodeImpl;
    fn to_repr(&self) -> String;
}

impl AstNode for AstNodeImpl {
    fn children(&self) -> Vec<&AstNodeImpl> {
        match &self.kind {
            AstNodeKind::Literal(_) => vec![],
            AstNodeKind::BinOp { left, right, .. } => vec![left, right],
            AstNodeKind::UnOp { expr, .. } => vec![expr],
            AstNodeKind::Group(group) => group.children(),
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
