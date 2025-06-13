use crate::codegen::context::CodegenContext;

pub trait Codegen {
    fn codegen(&self, context: &mut CodegenContext) -> String;
}
