//! # Scope
//!
//! Este módulo define la estructura `Scope` para el compilador Hulk.
//! Un `Scope` representa un entorno de alcance léxico durante el análisis semántico.
//! Permite almacenar y consultar variables, funciones y tipos declarados en el contexto actual,
//! así como rastrear el tipo y la función activa durante el recorrido del AST.

use std::collections::HashMap;

use crate::hulk_ast_nodes::HulkTypeNode;
use crate::hulk_ast_nodes::hulk_function_info::HulkFunctionInfo;

/// Representa un entorno de alcance léxico (scope) durante el análisis semántico.
/// 
/// - `variables`: tabla de variables locales (nombre → tipo como string).
/// - `declared_functions`: funciones declaradas en el scope (nombre → información de función).
/// - `declared_types_def`: tipos definidos en el scope (nombre → definición de tipo).
/// - `current_type_def`: nombre del tipo actualmente en análisis (si aplica).
/// - `current_function`: nombre de la función actualmente en análisis (si aplica).
#[derive(Debug, Clone)]
pub struct Scope{
    pub variables: HashMap<String, String>,
    pub declared_functions: HashMap<String, HulkFunctionInfo>,
    pub declared_types_def: HashMap<String, HulkTypeNode>,
    pub current_type_def: Option<String>,
    pub current_function: Option<String>
}