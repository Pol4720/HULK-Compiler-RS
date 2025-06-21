//! # llvm_runner
//!
//! Este módulo proporciona utilidades para compilar y ejecutar código LLVM IR generado por el compilador Hulk.
//!
//! ## Funciones
//!
//! - `run_llvm_ir(filename: &str)`  
//!   Compila el archivo LLVM IR especificado usando `clang` y ejecuta el binario resultante.
//!
//! ## Detalles de implementación
//!
//! - Detecta el sistema operativo (`windows`, `macos` o `linux`) y ajusta los argumentos de compilación y el nombre del ejecutable de salida.
//! - Usa el comando `clang` para compilar el archivo LLVM IR a un ejecutable nativo.
//! - Si la compilación falla, muestra un mensaje de error.
//! - Si la compilación es exitosa, ejecuta el binario generado y muestra un mensaje si la ejecución falla.
//!
//! ## Ejemplo de uso
//! ```rust
//! run_llvm_ir("out.ll");
//! ```
//!
//! ## Notas
//! - Requiere que `clang` esté instalado y disponible en el PATH del sistema.
//! - El ejecutable generado se llama `output.exe`, `output_macos` o `output_linux` según el sistema operativo.
//! - Los argumentos de compilación incluyen el target adecuado para cada plataforma.

use std::process::Command;
// use std::env;

pub fn run_llvm_ir(filename: &str) {
    let (output, clang_args): (&str, Vec<&str>) = if cfg!(target_os = "windows") {
        (
            "output.exe",
            vec![
                filename,
                "-o",
                "output.exe",
                "-fuse-ld=lld",
                "--target=x86_64-w64-windows-gnu",
            ],
        )
    } else if cfg!(target_os = "macos") {
        (
            "output_macos",
            vec![
                filename,
                "-o",
                "output_macos",
                "--target=x86_64-apple-darwin",
            ],
        )
    } else {
        // Assume Linux
        (
            "output_linux",
            vec![
                filename,
                "-o",
                "output_linux",
                "-fuse-ld=lld",
                "--target=x86_64-pc-linux-gnu",
            ],
        )
    };

    if !Command::new("clang")
        .args(&clang_args)
        .status()
        .map_or(false, |s| s.success())
    {
        eprintln!("Falló la compilación con clang");
        return;
    }

    let exec_cmd = if cfg!(target_os = "windows") {
        format!(".\\{}", output)
    } else {
        format!("./{}", output)
    };

    if !Command::new(exec_cmd)
        .status()
        .map_or(false, |s| s.success())
    {
        eprintln!("Falló la ejecución del ejecutable generado");
    }
}
