use std::process::Command;

pub fn run_llvm_ir(filename: &str) {
    let status = Command::new("lli")
        .arg(filename)
        .status()
        .expect("Falló la ejecución con lli");

    if !status.success() {
        eprintln!("Ejecución fallida con código: {:?}", status);
    }
}
