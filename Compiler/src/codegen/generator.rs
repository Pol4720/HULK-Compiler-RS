use crate::codegen::{
    context::CodegenContext,
    traits::Codegen as CodegenTrait,
    writer::write_to_file,
    llvm_runner::run_llvm_ir,
};

pub struct CodeGenerator;

impl CodeGenerator {
    pub fn generate_and_run<T: CodegenTrait>(node: &T, filename: &str) {
        let mut ctx = CodegenContext::new();

        // Header básico LLVM
        ctx.emit("declare i32 @printf(i8*, ...)");
        ctx.emit("@format = private constant [4 x i8] c\"%d\\0A\\00\"");
        ctx.emit("");
        ctx.emit("define i32 @main() {");

        let result_reg = node.codegen(&mut ctx);

        if !result_reg.is_empty() {
            ctx.emit(&format!(
            "  call i32 (i8*, ...) @printf(i8* getelementptr ([4 x i8], [4 x i8]* @format, i32 0, i32 0), i32 {})",
            result_reg
            ));
        } else {
            // Si no hay resultado, imprime 0 para evitar error de LLVM
            ctx.emit("  call i32 (i8*, ...) @printf(i8* getelementptr ([4 x i8], [4 x i8]* @format, i32 0, i32 0), i32 0)");
        }

        ctx.emit("  ret i32 0");
        ctx.emit("}");


        //  Aquí imprimimos el código LLVM generado
        println!("\n\x1b[35m--- LLVM IR Generado ---\x1b[0m\n{}\n\x1b[35m-------------------------\x1b[0m", ctx.code);

        write_to_file(&ctx.code, filename);
        run_llvm_ir(filename);
    }
}
