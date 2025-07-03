// ===============================
// main.rs: Generador de Analizador Léxico
// ===============================
// Orquesta la construcción de NFAs, combinación, conversión a DFA, simulación y extracción de lexemas.

pub mod spec;
use spec::read_token_spec;

mod regex_parser;
use regex_parser::node::ast_node_impl::AstNode;
use regex_parser::regex_parser::parse_regex;
mod nfa;
use nfa::join_nfa::JoinedNFA;
use nfa::nfa::NFA;
mod dfa;
use dfa::dfa::DFA;
mod lexemes;
mod simulation;
use lexemes::extract_lexemes;
use std::fs;

/// Lee la especificación de tokens y construye los NFAs individuales.
fn construir_nfas(path: &str) -> Vec<(NFA, String, usize)> {
    let specs = read_token_spec(path);
    let mut nfas = Vec::new();
    let mut priority = 0;
    for spec in specs {
        println!("Token: {} => {}", spec.name, spec.regex);
        // Generar el AST por cada expresión regular
        if let Some(ast) = parse_regex(&spec.regex) {
            // println!("  AST: {}", ast.to_repr());
            // Generar la NFA
            // println!("  Generando NFA para el token '{}':", spec.name);
            let nfa = NFA::from_ast(&ast);
            // println!("{}", nfa.to_string());
            // Guardar NFA, tipo de token y prioridad
            nfas.push((nfa, spec.name.clone(), priority));
            priority += 1;
        } else {
            println!("  AST: (no soportado por el parser actual)");
            println!("  No se puede generar NFA para el token '{}'.", spec.name);
        }
    }
    nfas
}

/// Combina los NFAs individuales en un solo NFA etiquetado y lo imprime.
fn combinar_nfas(nfas: Vec<(NFA, String, usize)>) -> Option<JoinedNFA> {
    if nfas.is_empty() {
        println!("No se generaron NFAs válidos. Abortando.");
        return None;
    }
    let joined_nfa = JoinedNFA::join(nfas);
    // println!("\nNFA combinado:");
    // println!("{}", joined_nfa.to_string());
    Some(joined_nfa)
}

/// Construye el DFA a partir del NFA combinado y lo retorna.
fn construir_dfa(joined_nfa: &JoinedNFA) -> DFA {
    DFA::from_joined_nfa(joined_nfa)
}

/// Orquesta el proceso de generación léxica, simulación y extracción de tokens.
fn main() {
    println!("==============================");
    println!("  Generador de Analizador Léxico");
    println!("==============================");
    // 1. Construir NFAs individuales a partir de la especificación de tokens
    let nfas = construir_nfas("tokens_spec.txt");
    // 2. Combinar los NFAs en un solo NFA etiquetado
    if let Some(joined_nfa) = combinar_nfas(nfas) {
        // 3. Construir el DFA resultante
        let dfa = construir_dfa(&joined_nfa);
        // 4. Leer la cadena de prueba desde un archivo externo
        let test_str = fs::read_to_string("test_input.txt")
            .expect("No se pudo leer test_input.txt")
            .trim_end_matches(['\n', '\r'])
            .to_string();
        // 5. Visualización y simulación del NFA
        // let nfa_for_sim = NFA {
        //     states: joined_nfa.states.clone(),
        //     start: joined_nfa.start.clone(),
        //     accepts: joined_nfa.accepts.keys().cloned().collect(),
        // };
        // nfa_for_sim.visualize();
        // println!("\nTabla de transición del NFA:");
        // nfa_for_sim.print_transition_table();
        // let accepted = nfa_for_sim.accepts(&test_str);
        // println!(
        //     "¿La cadena '{test_str}' es aceptada por el NFA? {}",
        //     if accepted { "Sí" } else { "No" }
        // );
        // 6. Probar extracción de lexemas sobre el texto leído
        match extract_lexemes(&test_str, &dfa) {
            Ok(lexs) => {
                println!("\nLexemas reconocidos:");
                for lex in lexs {
                    println!("{:?}", lex);
                }
            }
            Err(errors) => {
                println!("\nErrores léxicos encontrados:");
                for err in errors {
                    println!("  Línea {}, columna {}: {}", err.line, err.column, err.message);
                }
            }
        }
    }
}
