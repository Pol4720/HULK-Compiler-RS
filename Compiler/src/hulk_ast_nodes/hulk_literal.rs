//! # Literales AST Nodes
//!
//! Este módulo define los nodos de literales (`NumberLiteral`, `BooleanLiteral`, `StringLiteral`) del AST para el compilador Hulk.
//! Permite representar valores literales numéricos, booleanos y de cadena en el AST, así como su generación de código LLVM IR.

use crate::{codegen::context::CodegenContext, hulk_tokens::TokenPos};
use crate::codegen::traits::Codegen;
use std::fmt::{self, Display, Formatter};
use crate::typings::types_node::TypeNode;

/// Representa un literal numérico en el AST.
/// 
/// Por ejemplo: `42`, `3.14`
/// 
/// - `value`: valor numérico.
/// - `_type`: tipo inferido o declarado del literal (opcional).
#[derive(Debug, Clone, PartialEq)]
pub struct NumberLiteral {
    pub value: f64,
    pub _type: Option<TypeNode>,
    pub token_pos: TokenPos,
}

impl NumberLiteral {
    /// Crea un nuevo literal numérico a partir de un string.
    pub fn new(value: &str, token_pos: TokenPos) -> Self {
        Self {
            value: value.parse().unwrap(),
            _type: None, 
            token_pos
        }
    }
    /// Establece el tipo del literal numérico.
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}

impl Display for NumberLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Codegen for NumberLiteral {
    /// Genera el código LLVM IR para el literal numérico.
    ///
    /// Usa una instrucción `fadd double 0.0, valor` para asignar el valor a un registro temporal.
    fn codegen(&self, context: &mut CodegenContext) -> String {
        let result_reg = context.generate_temp();
        let formatted = format!("{:.16E}", self.value);
        let line = format!("  {} = fadd double 0.0, {}", result_reg, formatted);
        context.emit(&line);
        context.add_register_hulk_type(result_reg.clone(), "Number".to_string());
        context.symbol_table.insert("__last_type__".to_string(), "double".to_string());
        result_reg
    }
}

/// Representa un literal booleano en el AST.
/// 
/// Por ejemplo: `true`, `false`
/// 
/// - `value`: valor booleano.
/// - `_type`: tipo inferido o declarado del literal (opcional).
#[derive(Debug, Clone, PartialEq)]
pub struct BooleanLiteral {
    pub value: bool,
    pub _type: Option<TypeNode>,
    pub token_pos: TokenPos,
}

impl BooleanLiteral {
    /// Crea un nuevo literal booleano a partir de un string.
    pub fn new(value: &str, token_pos: TokenPos) -> Self {
        Self {
            value: value.parse().unwrap(),
            _type: None,
            token_pos
        }
    }
    /// Establece el tipo del literal booleano.
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}

impl Display for BooleanLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Codegen for BooleanLiteral {
    /// Genera el código LLVM IR para el literal booleano.
    ///
    /// Convierte el valor a 1 o 0 y lo asigna a un registro temporal usando `add i1 0, valor`.
    fn codegen(&self, context: &mut CodegenContext) -> String {
        let llvm_bool = if self.value { "1" } else { "0" };
        let result_reg = context.generate_temp();
        let line = format!("  {} = add i1 0, {}", result_reg, llvm_bool);
        context.emit(&line);
        context.symbol_table.insert("__last_type__".to_string(), "i1".to_string());
        context.add_register_hulk_type(result_reg.clone(), "Boolean".to_string());
        result_reg
    }
}

/// Representa un literal de cadena en el AST.
/// 
/// Por ejemplo: `"hola mundo"`
/// 
/// - `value`: valor de la cadena.
/// - `_type`: tipo inferido o declarado del literal (opcional).
#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral {
    pub value: String,
    pub _type: Option<TypeNode>,
    pub token_pos: TokenPos,
}

impl StringLiteral {
    /// Crea un nuevo literal de cadena a partir de un string.
    pub fn new(value: &str, token_pos: TokenPos) -> Self {
        Self {
            value: value.to_string(),
            _type: None,
            token_pos
        }
    }
    /// Establece el tipo del literal de cadena.
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type)
    }
}

impl Display for StringLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Codegen for StringLiteral {
    /// Genera el código LLVM IR para el literal de cadena.
    ///
    /// Escapa caracteres especiales, define una constante global y obtiene un puntero a la cadena.
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // Escape comillas, saltos de línea, etc.
        let escaped = self
            .value
            .replace("\\", "\\5C")
            .replace("\n", "\\0A")
            .replace("\"", "\\22");

        // Asegura que termine en nulo para C strings
        let null_terminated = format!("{}\\00", escaped);
        let byte_count = escaped.len() + 1; // Tamaño del string más el terminador nulo.

        // Nombre único para la constante
        let const_name = context.generate_string_const_name();

        // Define la constante global
        let global_def = format!(
            "@{} = private unnamed_addr constant [{} x i8] c\"{}\"",
            const_name, byte_count, null_terminated
        );
        context.emit_global(&global_def);

        // Prepara el puntero con getelementptr
        let ptr_reg = context.generate_temp();
        let gep_inst = format!(
            "  {} = getelementptr inbounds [{} x i8], [{} x i8]* @{}, i32 0, i32 0",
            ptr_reg, byte_count, byte_count, const_name
        );
        context.emit(&gep_inst);
        context.add_register_hulk_type(ptr_reg.clone(), "String".to_string());
        context.symbol_table.insert("__last_type__".to_string(), "ptr".to_string());

        ptr_reg // Devuelve el nombre del registro con la dirección del string
    }
}

