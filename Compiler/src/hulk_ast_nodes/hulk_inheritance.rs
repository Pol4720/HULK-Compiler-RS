//! # Inheritance AST Node
//!
//! Este módulo define el nodo `Inheritance` del AST para el compilador Hulk.
//! Permite representar la herencia de tipos (clases) en el lenguaje Hulk, incluyendo el tipo padre y los argumentos de inicialización.
//! Es útil para modelar la relación de herencia y la inicialización de clases base en el AST.

use crate::{hulk_ast_nodes::hulk_expression::Expr, hulk_tokens::TokenPos};

/// Representa una relación de herencia en el AST.
/// 
/// Por ejemplo: `type Hijo : Padre(1, 2)`
/// 
/// - `parent_type`: nombre del tipo padre.
/// - `arguments`: lista de expresiones que representan los argumentos para el constructor del padre.
#[derive(Debug, Clone, PartialEq)]
pub struct Inheritance {
    pub parent_type: String,
    pub arguments: Vec<Expr>,
    pub token_pos: TokenPos,

}

impl Inheritance {
    /// Crea una nueva relación de herencia.
    ///
    /// # Arguments
    /// * `parent_type` - Nombre del tipo padre.
    /// * `arguments` - Vector de expresiones para inicializar el padre.
    pub fn new(parent_type: String, arguments: Vec<Expr> , token_pos: TokenPos) -> Self {
        Inheritance {
            parent_type,
            arguments,
            token_pos
        }
    }
}