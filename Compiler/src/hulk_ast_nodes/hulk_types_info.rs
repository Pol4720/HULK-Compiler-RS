//! # HulkTypesInfo Enum
//!
//! Este módulo define el enum `HulkTypesInfo` para el compilador Hulk.
//! Representa los tipos básicos soportados por el lenguaje Hulk a nivel de información de tipos en el AST.
//! Es útil para anotaciones, inferencia y chequeo de tipos en el análisis semántico y generación de código.


/// Enum que representa los tipos básicos del lenguaje Hulk.
/// 
/// - `Object`: tipo objeto genérico.
/// - `String`: tipo cadena de texto.
/// - `Number`: tipo numérico.
/// - `Boolean`: tipo booleano.
/// - `Unknown`: tipo desconocido o no inferido.
#[derive(Debug, Clone)]
pub enum HulkTypesInfo {
    Object,
    String,
    Number,
    Boolean,
    Unknown,
}

impl HulkTypesInfo {
    /// Devuelve el nombre del tipo como string.
    pub fn as_str(&self) -> &str {
        match self {
            HulkTypesInfo::Object => "Object",
            HulkTypesInfo::String => "String",
            HulkTypesInfo::Number => "Number",
            HulkTypesInfo::Boolean => "Boolean",
            HulkTypesInfo::Unknown => "Unknown",
        }
    }
}
