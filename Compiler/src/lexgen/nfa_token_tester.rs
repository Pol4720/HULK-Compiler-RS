// nfa_token_tester.rs: Prueba individual de NFAs para cada token
// Uso: Ejecuta este archivo como binario independiente para probar tokens y sus NFAs

mod spec;
use spec::read_token_spec;
mod regex_parser;
use regex_parser::regex_parser::parse_regex;
mod nfa;
use nfa::nfa::NFA;
mod simulation;
use regex_parser::node::ast_node_impl::AstNode;
use std::io::{self, Write};

fn main() {
    println!("==============================");
    println!("  Prueba de NFA para un Token");
    println!("==============================");
    // Define aquí el nombre y la regex del token a probar
    let token_name = "Token de prueba";
    let token_regex = "^a.*";
    println!("Token: {} => {}", token_name, token_regex);
    if let Some(ast) = parse_regex(token_regex) {
        println!("  AST: {}", ast.to_repr());

        let nfa = NFA::from_ast(&ast);
        println!("NFA generado para '{}':", token_name);

        nfa.print_transition_table();

        // println!("{}", nfa.to_string());
        
        // Permitir probar cadenas de entrada
        loop {
            print!(
                "Ingresa una cadena para probar con '{}', o 'salir' para terminar: ",
                token_name
            );
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if input == "salir" {
                break;
            }
            let aceptado = nfa.accepts(input);
            println!(
                "¿La cadena '{}' es aceptada? {}",
                input,
                if aceptado { "Sí" } else { "No" }
            );
        }
    } else {
        println!("No se pudo generar AST/NFA para el token '{}'.", token_name);
    }
    println!("\nPrueba finalizada.");
}
