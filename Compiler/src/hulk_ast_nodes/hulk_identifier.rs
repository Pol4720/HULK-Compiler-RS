//! # Identifier AST Node
//!
//! Este módulo define el nodo `Identifier` del AST para el compilador Hulk.
//! Un identificador representa el nombre de una variable, parámetro o símbolo en el código fuente.
//! Incluye la estructura, métodos asociados y la generación de código LLVM IR.

use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_tokens::TokenPos;
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
    pub _type: Option<TypeNode>,
    pub token_pos: TokenPos
}

impl Identifier {
    /// Crea un nuevo identificador a partir de un string.
    ///
    /// # Arguments
    /// * `id` - Nombre del identificador.
    pub fn new(id: &str, token_pos: TokenPos) -> Self {
        Self {
            id: id.to_string(),
            _type: None,
            token_pos:  token_pos
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
        // Usa el método get_variable en lugar de acceder directamente a symbol_table
        let ptr = context.get_variable(&self.id).cloned();
        if ptr.is_none() {
            panic!("Variable '{}' no definida en el contexto", self.id);
        }
        let ptr = ptr.unwrap();

        // Asegura que el tipo del identificador esté definido
    let hulk_type = self._type.clone().expect(&format!(
        "El tipo del identificador '{}' no ha sido inferido",
        self.id
    ));
    let type_name = hulk_type.type_name.clone();
    let llvm_type = CodegenContext::to_llvm_type(type_name.clone());

    // Si el tipo es un puntero (como i8* para strings), la instrucción es 'load ptr, ptr'
    match llvm_type.as_str() {
        "ptr" => {
            let result_reg = context.generate_temp();
            context.add_register_hulk_type(result_reg.clone(), type_name);
            let line = format!("  {} = load ptr, ptr {}", result_reg.clone(), ptr);
            context.emit(&line);
            result_reg
        }
        _ => {
            let result_reg = context.generate_temp();
            context.add_register_hulk_type(result_reg.clone(), type_name);
            let line = format!("  {} = load {}, {}* {}", result_reg.clone(), llvm_type, llvm_type, ptr);
            context.emit(&line);
            result_reg
        }
    }
    }
}



