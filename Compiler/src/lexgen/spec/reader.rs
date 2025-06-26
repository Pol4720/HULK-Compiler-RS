use crate::spec::token_spec::TokenSpec;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Lee la especificación de tokens desde un archivo y devuelve un vector de TokenSpec
pub fn read_token_spec(path: &str) -> Vec<TokenSpec> {
    let file = File::open(path).expect("No se pudo abrir el archivo de especificación de tokens");
    let reader = BufReader::new(file);
    let mut specs = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((name, regex)) = line.split_once(':') {
                specs.push(TokenSpec {
                    name: name.trim().to_string(),
                    regex: regex.trim().to_string(),
                });
            }
        }
    }
    specs
}
