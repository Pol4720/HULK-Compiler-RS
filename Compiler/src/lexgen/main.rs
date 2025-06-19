// Punto de entrada para el generador de analizadores léxicos

pub mod spec;
use spec::read_token_spec;

mod regex_parser;
use regex_parser::node::ast_node_impl::AstNode;
use regex_parser::regex_parser::parse_start;
mod nfa;
use nfa::join_nfa::JoinedNFA;
use nfa::nfa::NFA;

fn main() {
    println!("Generador de Analizador Léxico");
    let specs = read_token_spec("tokens_spec.txt");
    let mut nfas = Vec::new();
    let mut priority = 0;
    for spec in specs {
        println!("Token: {} => {}", spec.name, spec.regex);
        // Generar el AST por cada expresión regular
        if let Some(ast) = parse_start(&spec.regex) {
            println!("  AST: {}", ast.to_repr());
            // Genero la NFA
            println!("  Generando NFA para el token '{}':", spec.name);
            let nfa = NFA::from_ast(&ast);
            println!("{}", nfa.to_string());
            // Guardar NFA, tipo de token y prioridad
            nfas.push((nfa, spec.name.clone(), priority));
            priority += 1;
        } else {
            println!("  AST: (no soportado por el parser actual)");
            println!("  No se puede generar NFA para el token '{}'.", spec.name);
        }

        // Genero el DFA
        // Genro el código fuente del analizador léxico
    }
    // Combinar todos los NFAs en uno solo
    if !nfas.is_empty() {
        let joined_nfa = JoinedNFA::join(nfas);
        println!("\nNFA combinado:");
        println!("{}", joined_nfa.to_string());
        // Puedes imprimir transiciones o estados si lo deseas
    }
    // Aquí puedes continuar con la construcción del DFA, etc.
}
