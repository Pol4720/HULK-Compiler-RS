use lalrpop_util::lalrpop_mod;
mod ast;
pub mod hulk_Tokens;
lalrpop_mod!(pub parser);

use std::io::{self, Write};
use crate::parser::Expressions_ListParser;

fn main() {
    let parser = Expressions_ListParser::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }

        match parser.parse(&input) {
            Ok(ast) => {
            for expr in ast {
                println!("{}", expr.to_tree(0)); // Mostrar el árbol con sangría
                match expr.eval() {
                Ok(result) => println!("Resultado: {}", result),
                Err(err) => eprintln!("Error: {}", err),
                }
            }
            }
            Err(err) => {
            eprintln!("Error de análisis: {}", err);
            }
        }
    }
}
