use crate::codegen::context::CodegenContext;

pub trait Codegen {
    /// Genera LLVM IR para el nodo y devuelve el registro del valor generado
    fn codegen(&self, context: &mut CodegenContext) -> String;
}
