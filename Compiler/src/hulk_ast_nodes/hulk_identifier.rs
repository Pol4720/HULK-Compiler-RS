//! # Identifier AST Node
//!
//! Este módulo define el nodo `Identifier` del AST para el compilador Hulk.
//! Un identificador representa el nombre de una variable, parámetro o símbolo en el código fuente.
//! Incluye la estructura, métodos asociados y la generación de código LLVM IR.

use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::typings::types_node::TypeNode;
use std::fmt;

/// Representa un identificador en el AST.
/// 
/// Por ejemplo: `x`, `total`, `nombreVariable`
/// 
/// - `id`: nombre del identificador.
/// - `_type`: tipo inferido o declarado del identificador (opcional).
#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub id: String,
    pub _type: Option<TypeNode>
}

impl Identifier {
    /// Crea un nuevo identificador a partir de un string.
    ///
    /// # Arguments
    /// * `id` - Nombre del identificador.
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            _type: None,
        }
    }

    /// Establece el tipo del identificador.
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}

impl fmt::Display for Identifier {
    /// Permite mostrar el identificador como string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Codegen for Identifier {
    /// Genera el código LLVM IR para el identificador.
    ///
    /// Busca el puntero de la variable en la tabla de símbolos y genera una instrucción `load`.
    /// Si la variable no existe en el contexto, lanza un panic.
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // Busca el puntero de la variable en la tabla de símbolos
        let ptr = context.symbol_table.get(&self.id).cloned();
        if let Some(ptr) = ptr {
            let result_reg = context.generate_temp();
            // Asume tipo i32 (ajustar si soportas otros tipos)
            let line = format!("  {} = load i32, i32* {}", result_reg, ptr);
            context.emit(&line);
            result_reg
        } else {
            panic!("Variable '{}' no definida en el contexto", self.id);
        }
    }
}
