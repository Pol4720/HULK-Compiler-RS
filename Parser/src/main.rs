use crate::cst_to_ast::{convert_to_ast, print_ast};
use crate::grammar::load_grammar;
use crate::ll1::{build_ll1_table, compute_first, compute_follow, validate_ll1_grammar, LL1Table};
use crate::parser::Parser;
// use crate::token::{make_token, mock_tokens, TokenType};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

mod ast;
mod cst;
mod cst_to_ast;
mod grammar;
mod ll1;
mod parser;
mod token;
mod utils;
mod test_cases; // Nuevo módulo

fn save_ll1_table_csv(table: &LL1Table, filepath: &str) -> io::Result<()> {
    // Collect all terminal symbols
    let mut terminals = HashSet::new();
    for (_, inner_map) in table {
        for terminal in inner_map.keys() {
            terminals.insert(terminal.clone());
        }
    }
    let mut terminals: Vec<String> = terminals.into_iter().collect();
    terminals.sort(); // Sort terminals for consistent output

    // Create file
    let path = Path::new(filepath);
    let mut file = File::create(path)?;

    // Write CSV header
    write!(file, "Non-Terminal")?;
    for terminal in &terminals {
        write!(file, ",{}", terminal)?;
    }
    writeln!(file)?;

    // Write CSV content
    let mut non_terminals: Vec<String> = table.keys().cloned().collect();
    non_terminals.sort(); // Sort non-terminals for consistent output

    for nt in non_terminals {
        write!(file, "{}", nt)?;

        for terminal in &terminals {
            write!(file, ",")?;
            if let Some(inner_map) = table.get(&nt) {
                if let Some(production) = inner_map.get(terminal) {
                    let prod_str = if production.is_empty() {
                        "ε".to_string()
                    } else {
                        // Escape commas by wrapping in quotes
                        format!("\"{}\"", production.join(" "))
                    };
                    write!(file, "{}", prod_str)?;
                }
            }
        }
        writeln!(file)?;
    }

    println!("LL(1) table saved to: {}", filepath);
    Ok(())
}


fn main() {
    use test_cases::TEST_CASES; // Importa la lista

    let test_cases = &TEST_CASES;

    for (i, test) in test_cases.iter().enumerate() {
        println!("\n==============================");
        println!("Ejecutando prueba {}: {}", i + 1, test.description);
        println!(
            "Gramática: {}, Símbolo inicial: {}",
            test.grammar_path, test.start_symbol
        );

        let grammar = load_grammar(test.grammar_path);
        let first = compute_first(&grammar);
        let follow = compute_follow(&grammar, &first, test.start_symbol);
        let table = build_ll1_table(&grammar, &first, &follow);
        validate_ll1_grammar(&grammar, &table);

        // Guardar tabla LL(1) CSV (opcional, puedes comentar si no quieres sobrescribir)
        let csv_path = format!("ll1_table_{}.csv", i + 1);
        if let Err(e) = save_ll1_table_csv(&table, &csv_path) {
            eprintln!("Error guardando tabla LL(1) CSV: {}", e);
        }

        let mut parser = Parser::new(
            test.tokens.clone(),
            table,
            grammar.clone(),
            test.start_symbol,
        )
        .with_error_recovery(false);

        match parser.parse() {
            Ok(cst) => {
                println!("\nCST Tree:");
                println!("{}", cst);

                let ast = convert_to_ast(&cst);
                println!("\nAST Tree:");
                if let Ok(ref program) = ast {
                    print_ast(program);
                } else {
                    eprintln!("{:?}", ast);
                }
            }
            Err(error) => {
                eprintln!("Parsing failed: {}", error);
            }
        }
    }
}
              