use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::typings::types_node::TypeNode;

#[derive(Debug, Clone)]
pub struct HulkFunctionInfo {
    pub function_name: String,
    pub argument_types: Vec<TypeNode>,
    pub return_type: TypeNode,
}

impl HulkFunctionInfo {
    pub fn new(
        function_name: String,
        argument_types: Vec<TypeNode>,
        return_type: TypeNode,
    ) -> Self {
        HulkFunctionInfo {
            function_name,
            argument_types,
            return_type,
        }
    }
}

impl Codegen for HulkFunctionInfo {
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
