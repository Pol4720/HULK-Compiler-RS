use inkwell::{builder::Builder, context::Context, execution_engine::ExecutionEngine, module::Module};

pub struct LLVMContext<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub engine: ExecutionEngine<'ctx>,
}

impl<'ctx> LLVMContext<'ctx> {
    pub fn new(name: &str, context: &'ctx Context) -> Self {
        let module = context.create_module(name);
        let builder = context.create_builder();
        let engine = module
            .create_jit_execution_engine(inkwell::OptimizationLevel::Default)
            .expect("Failed to create execution engine");

        Self {
            context,
            module,
            builder,
            engine,
        }
    }
}
