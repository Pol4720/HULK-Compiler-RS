use super::node::ast_node_impl::{AstNode, AstNodeImpl};
use crate::regex_parser::node::regex_char::StarNode;

/// Parsea una expresión regular que solo contiene el metacarácter de inicio de línea '^'.
pub fn parse_start(input: &str) -> Option<AstNodeImpl> {
    if input.trim() == "^" {
        let node = StarNode::new();
        Some(node.to_ast())
    } else {
        None
    }
}
