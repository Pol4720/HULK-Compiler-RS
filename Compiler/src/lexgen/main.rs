// Punto de entrada para el generador de analizadores léxicos

pub mod spec;
use spec::read_token_spec;

// pub mod regex_parser;
// use regex_parser::parse_regex;

fn main() {
    println!("Generador de Analizador Léxico");
    let specs = read_token_spec("tokens_spec.txt");
    for spec in specs {
        println!("Token: {} => {}", spec.name, spec.regex);
        // Generar el AST por cada expresión regular
        // Genero la NFA
        // Genero el DFA
        // Genro el código fuente del analizador léxico
    }
    // Aquí puedes continuar con la construcción del NFA, DFA, etc.
}
