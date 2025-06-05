use super::{context::LLVMContext, value_map::ValueMap};
use inkwell::values::BasicValueEnum;

pub trait CodegenNode<'ctx> {
    fn codegen(
        &self,
        llvm: &mut LLVMContext<'ctx>,
        values: &mut ValueMap<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String>;
}
