//! # MemberAccess AST Node
//!
//! Este módulo define el nodo `MemberAccess` del AST para el compilador Hulk.
//! Permite representar el acceso a miembros o propiedades de un objeto, como `obj.prop`.
//! Incluye la estructura, métodos asociados y el tipo inferido o declarado de la expresión.

use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::hulk_ast_nodes::hulk_identifier::Identifier;
use crate::hulk_tokens::TokenPos;
use crate::typings::types_node::TypeNode;

/// Representa el acceso a un miembro (propiedad o campo) de un objeto en el AST.
/// 
/// Por ejemplo: `obj.prop`
/// 
/// - `object`: expresión que representa el objeto.
/// - `member`: identificador del miembro al que se accede.
/// - `_type`: tipo inferido o declarado del acceso (opcional).
#[derive(Debug, Clone, PartialEq)]
pub struct MemberAccess {
    pub object: Box<Expr>,
    pub member: Identifier,
    pub _type: Option<TypeNode>, // Tipo opcional para el acceso al miembro
    pub token_pos: TokenPos,
}

impl MemberAccess {
    /// Crea un nuevo acceso a miembro.
    ///
    /// # Arguments
    /// * `object` - Expresión del objeto.
    /// * `member` - Identificador del miembro.
    pub fn new(object: Expr, member: Identifier, token_pos: TokenPos) -> Self {
        Self {
            object: Box::new(object),
            member,
            _type: None,
            token_pos,
        }
    }

    /// Establece el tipo de la expresión de acceso a miembro.
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}