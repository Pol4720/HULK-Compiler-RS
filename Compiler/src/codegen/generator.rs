use crate::codegen::{
    context::CodegenContext, llvm_runner::run_llvm_ir, traits::Codegen as CodegenTrait,
    writer::write_to_file,
};

pub struct CodeGenerator;

impl CodeGenerator {
    pub fn generate_and_run<T: CodegenTrait>(node: &T, filename: &str) {
        let mut ctx = CodegenContext::new();

        // Header básico LLVM
        ctx.emit("declare i32 @printf(i8*, ...)");
        ctx.emit("@format_int = private constant [4 x i8] c\"%d\\0A\\00\"");
        ctx.emit("@format_double = private constant [4 x i8] c\"%f\\0A\\00\"");
        ctx.emit("");
        ctx.emit("define i32 @main() {");

        let result_reg = node.codegen(&mut ctx);

        // Determinar el tipo del resultado para printf
        // Por defecto, asume i32
        let mut result_type = "i32";
        if let Some(last_type) = ctx.symbol_table.get("__last_type__") {
            result_type = last_type;
        }
        // Si el registro de resultado comienza con '%t' y hay un tipo guardado, úsalo
        if result_reg.starts_with("%t") {
            if let Some(last_type) = ctx.symbol_table.get(&format!("{}__type", result_reg)) {
                result_type = last_type;
            }
        }

        if !result_reg.is_empty() {
            if result_type == "double" {
                ctx.emit(&format!(
                    "  call i32 (i8*, ...) @printf(i8* getelementptr ([4 x i8], [4 x i8]* @format_double, i32 0, i32 0), double {})",
                    result_reg
                ));
            } else {
                ctx.emit(&format!(
                    "  call i32 (i8*, ...) @printf(i8* getelementptr ([4 x i8], [4 x i8]* @format_int, i32 0, i32 0), i32 {})",
                    result_reg
                ));
            }
        } else {
            ctx.emit("  call i32 (i8*, ...) @printf(i8* getelementptr ([4 x i8], [4 x i8]* @format_int, i32 0, i32 0), i32 0)");
        }

        ctx.emit("  ret i32 0");
        ctx.emit("}");

        //  Aquí imprimimos el código LLVM generado
        println!(
            "\n\x1b[35m--- LLVM IR Generado ---\x1b[0m\n{}\n\x1b[35m-------------------------\x1b[0m",
            ctx.code
        );

        write_to_file(&ctx.code, filename);
        run_llvm_ir(filename);
    }
}
