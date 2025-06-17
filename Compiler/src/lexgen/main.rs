// Punto de entrada para el generador de analizadores léxicos

mod lexgen;

fn main() {
    println!("Generador de Analizador Léxico");
    let specs = lexgen::read_token_spec("src/lexgen/tokens_spec.txt");
    for spec in specs {
        println!("Token: {} => {}", spec.name, spec.regex);
    }
    // Aquí puedes continuar con la construcción del NFA, DFA, etc.
}
