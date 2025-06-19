// Punto de entrada para el generador de analizadores léxicos

pub mod spec;
use spec::read_token_spec;

mod regex_parser;
use regex_parser::node::ast_node_impl::AstNode;
use regex_parser::regex_parser::parse_start;

fn main() {
    println!("Generador de Analizador Léxico");
    let specs = read_token_spec("tokens_spec.txt");
    for spec in specs {
        println!("Token: {} => {}", spec.name, spec.regex);
        // Generar el AST por cada expresión regular
        if let Some(ast) = parse_start(&spec.regex) {
            println!("  AST: {}", ast.to_repr());
        } else {
            println!("  AST: (no soportado por el parser actual)");
        }

        // Genero la NFA
        // Genero el DFA
        // Genro el código fuente del analizador léxico
    }
    // Aquí puedes continuar con la construcción del NFA, DFA, etc.
}
