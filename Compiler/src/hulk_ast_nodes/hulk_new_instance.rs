//! # NewTypeInstance AST Node
//!
//! Este módulo define el nodo `NewTypeInstance` del AST para el compilador Hulk.
//! Permite representar la creación de nuevas instancias de tipos (clases/objetos) en el lenguaje Hulk,
//! incluyendo el nombre del tipo y los argumentos para el constructor.
//! Incluye la estructura, métodos asociados y el tipo inferido o declarado de la instancia.

use crate::hulk_ast_nodes::hulk_identifier::Identifier;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::typings::types_node::TypeNode;

/// Representa la creación de una nueva instancia de tipo (objeto) en el AST.
/// 
/// Por ejemplo: `Point(1, 2)`
/// 
/// - `type_name`: identificador del tipo a instanciar.
/// - `arguments`: lista de expresiones que representan los argumentos del constructor.
/// - `_type`: tipo inferido o declarado de la instancia (opcional).
#[derive(Debug, PartialEq, Clone)]
pub struct NewTypeInstance {
    pub type_name: Identifier,             
    pub arguments: Vec<Expr>,
    pub _type: Option<TypeNode>
}

impl NewTypeInstance {
    /// Crea una nueva instancia de tipo.
    ///
    /// # Arguments
    /// * `type_name` - Identificador del tipo a instanciar.
    /// * `arguments` - Vector de expresiones como argumentos del constructor.
    pub fn new(type_name: Identifier, arguments: Vec<Expr>) -> Self {
        NewTypeInstance { type_name, arguments, _type: None }
    }

    /// Establece el tipo de la instancia creada.
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}