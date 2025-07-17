use crate::grammar::load_grammar;
use crate::token::mock_tokens;
use crate::ll1::{compute_first, compute_follow, build_ll1_table, LL1Table};
use crate::parser::Parser;
use crate::cst_to_ast::{convert_to_ast, print_ast};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

mod grammar;
mod utils;
mod token;
mod ll1;
mod parser;
mod cst;
mod ast;
mod cst_to_ast;

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
    let grammar = load_grammar("grammar.ll1");
    let first = compute_first(&grammar);
    let follow = compute_follow(&grammar, &first, "Program");
    let table = build_ll1_table(&grammar, &first, &follow);

    // Save LL(1) table to CSV
    let csv_path = "ll1_table.csv";
    if let Err(e) = save_ll1_table_csv(&table, csv_path) {
        eprintln!("Error saving LL(1) table to CSV: {}", e);
    }

    let tokens = mock_tokens();
    let mut parser = Parser::new(tokens, table, grammar.clone(), "Program")
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
           
        },
        Err(error) => {
            eprintln!("Parsing failed: {}", error);
        }
    }
}

