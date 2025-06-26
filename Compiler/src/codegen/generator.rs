//! # CodeGenerator
//!
//! Este módulo define la estructura principal para la generación de código LLVM IR a partir del AST de Hulk.
//!
//! ## Funcionalidad principal
//! - Generar el código LLVM IR a partir de cualquier nodo del AST que implemente el trait `Codegen`.
//! - Permitir obtener el código generado como un `String` (útil para pruebas).
//! - Permitir generar, guardar y ejecutar el código LLVM IR en un archivo temporal.
//!
//! ## Métodos
//!
//! - `generate_only<T: CodegenTrait>(node: &T) -> String`  
//!   Genera el código LLVM IR para el nodo dado y lo retorna como un string. No ejecuta ni guarda el resultado.
//!
//! - `generate_and_run<T: CodegenTrait>(node: &T, filename: &str)`  
//!   Genera el código LLVM IR para el nodo dado, lo guarda en el archivo especificado y ejecuta el resultado usando el runner de LLVM.
//!
//! ## Detalles de implementación
//! - Ambos métodos construyen el contexto de generación (`CodegenContext`) y ejecutan el codegen del nodo raíz.
//! - Se agregan cabeceras, declaraciones y formatos estándar de LLVM IR para soportar operaciones y funciones comunes (como impresión y manejo de strings).
//! - El método `generate_and_run` utiliza utilidades para escribir el archivo y ejecutar el código generado.
//! - El método `generate_only` es útil para pruebas unitarias y para inspeccionar el IR generado sin ejecutarlo.
//!
//! ## Ejemplo de uso
//! ```rust
//! let ir_code = CodeGenerator::generate_only(&mi_ast);
//! CodeGenerator::generate_and_run(&mi_ast,

use crate::codegen::{
    context::CodegenContext, llvm_runner::run_llvm_ir, traits::Codegen as CodegenTrait,
    writer::write_to_file,
};

pub struct CodeGenerator;

impl CodeGenerator {

    /// Genera el código LLVM IR y lo retorna como String (útil para tests)
    pub fn generate_only<T: CodegenTrait>(node: &T) -> String {
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
      final_code.push_str(
        r#"
      define i8* @hulk_str_concat(i8* %s1, i8* %s2) {
      entry:
        %len1 = call i64 @strlen(i8* %s1)
        %len2 = call i64 @strlen(i8* %s2)
        %totallen = add i64 %len1, %len2
        %totallen1 = add i64 %totallen, 1
        %buf = call i8* @malloc(i64 %totallen1)
        call void @llvm.memcpy.p0i8.p0i8.i64(i8* %buf, i8* %s1, i64 %len1, i1 false)
        %buf_offset = getelementptr i8, i8* %buf, i64 %len1
        call void @llvm.memcpy.p0i8.p0i8.i64(i8* %buf_offset, i8* %s2, i64 %len2, i1 false)
        %last = getelementptr i8, i8* %buf, i64 %totallen
        store i8 0, i8* %last
        ret i8* %buf
      }

      ; Compara si dos strings son iguales (devuelve i1)
      define i1 @hulk_str_eq(i8* %s1, i8* %s2) {
      entry:
        %cmp = call i32 @strcmp(i8* %s1, i8* %s2)
        %is_eq = icmp eq i32 %cmp, 0
        ret i1 %is_eq
      }

      ; Compara si s1 > s2 (por longitud)
      define i1 @hulk_str_gt(i8* %s1, i8* %s2) {
      entry:
        %len1 = call i64 @strlen(i8* %s1)
        %len2 = call i64 @strlen(i8* %s2)
        %gt = icmp ugt i64 %len1, %len2
        ret i1 %gt
      }

      ; Compara si s1 < s2 (por longitud)
      define i1 @hulk_str_lt(i8* %s1, i8* %s2) {
      entry:
        %len1 = call i64 @strlen(i8* %s1)
        %len2 = call i64 @strlen(i8* %s2)
        %lt = icmp ult i64 %len1, %len2
        ret i1 %lt
      }

      ; Compara si s1 >= s2 (por longitud)
      define i1 @hulk_str_ge(i8* %s1, i8* %s2) {
      entry:
        %len1 = call i64 @strlen(i8* %s1)
        %len2 = call i64 @strlen(i8* %s2)
        %ge = icmp uge i64 %len1, %len2
        ret i1 %ge
      }

      ; Compara si s1 <= s2 (por longitud)
      define i1 @hulk_str_le(i8* %s1, i8* %s2) {
      entry:
        %len1 = call i64 @strlen(i8* %s1)
        %len2 = call i64 @strlen(i8* %s2)
        %le = icmp ule i64 %len1, %len2
        ret i1 %le
      }

      declare i64 @strlen(i8*)
      declare i8* @malloc(i64)
      declare void @llvm.memcpy.p0i8.p0i8.i64(i8*, i8*, i64, i1)
      declare i32 @strcmp(i8*, i8*)
      "#
      );
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

      // Mostrar el código generado (opcional, puedes quitarlo en tests)
      println!(
        "\n\x1b[35m--- LLVM IR Generado ---\x1b[0m\n{}\n\x1b[35m-------------------------\x1b[0m",
        final_code
      );

      final_code
    }
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
        final_code.push_str(
            r#"
        define i8* @hulk_str_concat(i8* %s1, i8* %s2) {
        entry:
          %len1 = call i64 @strlen(i8* %s1)
          %len2 = call i64 @strlen(i8* %s2)
          %totallen = add i64 %len1, %len2
          %totallen1 = add i64 %totallen, 1
          %buf = call i8* @malloc(i64 %totallen1)
          call void @llvm.memcpy.p0i8.p0i8.i64(i8* %buf, i8* %s1, i64 %len1, i1 false)
          %buf_offset = getelementptr i8, i8* %buf, i64 %len1
          call void @llvm.memcpy.p0i8.p0i8.i64(i8* %buf_offset, i8* %s2, i64 %len2, i1 false)
          %last = getelementptr i8, i8* %buf, i64 %totallen
          store i8 0, i8* %last
          ret i8* %buf
        }

        ; Compara si dos strings son iguales (devuelve i1)
        define i1 @hulk_str_eq(i8* %s1, i8* %s2) {
        entry:
          %cmp = call i32 @strcmp(i8* %s1, i8* %s2)
          %is_eq = icmp eq i32 %cmp, 0
          ret i1 %is_eq
        }

        ; Compara si s1 > s2 (por longitud)
        define i1 @hulk_str_gt(i8* %s1, i8* %s2) {
        entry:
          %len1 = call i64 @strlen(i8* %s1)
          %len2 = call i64 @strlen(i8* %s2)
          %gt = icmp ugt i64 %len1, %len2
          ret i1 %gt
        }

        ; Compara si s1 < s2 (por longitud)
        define i1 @hulk_str_lt(i8* %s1, i8* %s2) {
        entry:
          %len1 = call i64 @strlen(i8* %s1)
          %len2 = call i64 @strlen(i8* %s2)
          %lt = icmp ult i64 %len1, %len2
          ret i1 %lt
        }

        ; Compara si s1 >= s2 (por longitud)
        define i1 @hulk_str_ge(i8* %s1, i8* %s2) {
        entry:
          %len1 = call i64 @strlen(i8* %s1)
          %len2 = call i64 @strlen(i8* %s2)
          %ge = icmp uge i64 %len1, %len2
          ret i1 %ge
        }

        ; Compara si s1 <= s2 (por longitud)
        define i1 @hulk_str_le(i8* %s1, i8* %s2) {
        entry:
          %len1 = call i64 @strlen(i8* %s1)
          %len2 = call i64 @strlen(i8* %s2)
          %le = icmp ule i64 %len1, %len2
          ret i1 %le
        }

        declare i64 @strlen(i8*)
        declare i8* @malloc(i64)
        declare void @llvm.memcpy.p0i8.p0i8.i64(i8*, i8*, i64, i1)
        declare i32 @strcmp(i8*, i8*)
        "#
        );
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

