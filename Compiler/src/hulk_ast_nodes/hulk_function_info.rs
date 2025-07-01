//! # HulkFunctionInfo AST Node
//!
//! Este módulo define la estructura `HulkFunctionInfo` para el AST del compilador Hulk.
//! Se utiliza para almacenar información relevante sobre funciones declaradas, como su nombre, tipos de argumentos y tipo de retorno.
//! También permite la generación del prototipo de la función en LLVM IR.

use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;

/// Contiene la información de una función declarada en el AST.
/// 
/// - `function_name`: nombre de la función.
/// - `argument_types`: vector de tuplas (nombre, tipo) de los argumentos.
/// - `return_type`: tipo de retorno de la función.
#[derive(Debug, Clone)]
pub struct HulkFunctionInfo {
    pub function_name: String,
    pub argument_types: Vec<(String, String)>,
    pub return_type: String,
}

impl HulkFunctionInfo {
    /// Crea una nueva instancia de información de función.
    ///
    /// # Arguments
    /// * `function_name` - Nombre de la función.
    /// * `argument_types` - Vector de tuplas (nombre, tipo) de los argumentos.
    /// * `return_type` - Tipo de retorno de la función.
    pub fn new(
        function_name: String,
        argument_types: Vec<(String, String)>,
        return_type: String,
    ) -> Self {
        HulkFunctionInfo {
            function_name,
            argument_types,
            return_type,
        }
    }
}

impl Codegen for HulkFunctionInfo {
    /// Genera el prototipo de la función en LLVM IR.
    ///
    /// Convierte los tipos de argumentos y retorno a LLVM (asume `i32` por simplicidad).
    /// Emite la declaración del prototipo (sin cuerpo) en el contexto global.
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // Genera el prototipo de la función en LLVM IR
        // Convierte los tipos de argumentos y retorno a LLVM (asume i32 por simplicidad)
        let args_ir: Vec<String> = self
            .argument_types
            .iter()
            .map(|_| "i32".to_string())
            .collect();
        let args_str = args_ir.join(", ");
        let ret_type = "i32"; // Puedes mapear self.return_type a LLVM IR si tienes más tipos
        // Emite la declaración del prototipo (sin cuerpo)
        let proto = format!("declare {} @{}({})", ret_type, self.function_name, args_str);
        context.emit_global(&proto);
        String::new()
    }
}
