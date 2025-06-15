use crate::codegen::{
    context::CodegenContext, llvm_runner::run_llvm_ir, traits::Codegen as CodegenTrait,
    writer::write_to_file,
};

pub struct CodeGenerator;

impl CodeGenerator {
    pub fn generate_and_run<T: CodegenTrait>(node: &T, filename: &str) {
        let mut ctx = CodegenContext::new();

        // Ejecuta codegen y guarda el resultado
        let result_reg = node.codegen(&mut ctx);

        // Detecta tipo de resultado
        let mut result_type = "i32"; // por defecto
        if let Some(t) = ctx.symbol_table.get("__last_type__") {
            result_type = t;
        }
        if result_reg.starts_with("%t") {
            if let Some(t) = ctx.symbol_table.get(&format!("{}__type", result_reg)) {
                result_type = t;
            }
        }

        // Genera el cuerpo final del archivo LLVM IR
        let mut final_code = String::new();

        // Cabecera y formatos
        final_code.push_str("declare i32 @printf(i8*, ...)\n");
        final_code.push_str("@format_int = private constant [4 x i8] c\"%d\\0A\\00\"\n");
        final_code.push_str("@format_double = private constant [4 x i8] c\"%f\\0A\\00\"\n");
        final_code.push_str("@format_bool = private constant [4 x i8] c\"%d\\0A\\00\"\n"); // imprimimos i1 como %d
        final_code.push_str("@format_str = private constant [3 x i8] c\"%s\\00\"\n");

        // Definiciones globales (strings, etc.)
        if !ctx.globals.is_empty() {
            final_code.push_str("\n; Global definitions\n");
            final_code.push_str(&ctx.globals);
        }

        // Función main
        final_code.push_str("\ndefine i32 @main() {\n");
        final_code.push_str(&ctx.code);
        final_code.push_str("  ret i32 0\n");
        final_code.push_str("}\n");

        // Mostrar y guardar el código generado
        println!(
            "\n\x1b[35m--- LLVM IR Generado ---\x1b[0m\n{}\n\x1b[35m-------------------------\x1b[0m",
            final_code
        );

        write_to_file(&final_code, filename);
        run_llvm_ir(filename);
    }
}

