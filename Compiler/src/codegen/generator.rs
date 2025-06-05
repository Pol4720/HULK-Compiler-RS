use super::{context::LLVMContext, traits::CodegenNode, value_map::ValueMap};
use inkwell::context::Context;

pub struct CodeGenerator<'ctx> {
    llvm: LLVMContext<'ctx>,
    values: ValueMap<'ctx>,
}

impl<'ctx> CodeGenerator<'ctx> {
    pub fn new(module_name: &str, context: &'ctx Context) -> Self {
        let llvm = LLVMContext::new(module_name, context);
        Self {
            llvm,
            values: ValueMap::default(),
        }
    }

    pub fn generate<T: CodegenNode<'ctx>>(
        &mut self,
        root: &T,
    ) -> Result<(), String> {
        root.codegen(&mut self.llvm, &mut self.values)?;
        Ok(())
    }

    pub fn print_ir(&self) {
        self.llvm.module.print_to_stderr();
    }

    pub fn emit_machine_code(&self) -> Result<Vec<u8>, String> {
        // NOTE: This is a placeholder. Real machine code emission requires platform-specific logic.
        Err("Machine code emission not implemented.".into())
    }
}
