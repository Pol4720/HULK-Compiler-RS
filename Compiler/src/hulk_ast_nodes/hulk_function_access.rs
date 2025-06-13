//! # FunctionAccess AST Node
//!
//! Este módulo define el nodo de acceso a función (`FunctionAccess`) del AST para el compilador Hulk.
//! Permite representar llamadas a métodos sobre objetos, como `obj.metodo()`.
//! Incluye la estructura, métodos asociados y el tipo inferido o declarado de la expresión.

use crate::{hulk_ast_nodes::{Expr, FunctionCall}, typings::types_node::TypeNode};

/// Representa el acceso a una función (método) de un objeto en el AST.
/// 
/// Por ejemplo: `obj.metodo()`
/// 
/// - `object`: expresión que representa el objeto sobre el que se accede al método.
/// - `member`: llamada a función que representa el método.
/// - `_type`: tipo inferido o declarado de la expresión (opcional).
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionAccess {
    pub object: Box<Expr>,
    pub member: Box<FunctionCall>,
    pub _type: Option<TypeNode>, 
}

impl FunctionAccess {
    /// Crea un nuevo acceso a función.
    ///
    /// # Arguments
    /// * `object` - Expresión del objeto.
    /// * `member` - Llamada a función (método) sobre el objeto.
    pub fn new(object: Expr, member: FunctionCall) -> Self {
        Self {
            object: Box::new(object),
            member: Box::new(member),
            _type: None,
        }
    }

    /// Establece el tipo de la expresión de acceso a función.
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}