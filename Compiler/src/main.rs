use lalrpop_util::lalrpop_mod;
mod ast;
pub mod hulk_tokens;
lalrpop_mod!(pub parser);

use std::io::{self, Write};
use crate::parser::ProgramParser;

fn main() {
    let parser = ProgramParser::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }

        match parser.parse(&input) {
            Ok(ast) => {
            for instr in ast.instructions {
                println!("{}", instr.to_tree(0)); 
                match instr.eval() {
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
