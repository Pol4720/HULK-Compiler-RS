use lalrpop_util::lalrpop_mod;
mod ast;

lalrpop_mod!(pub parser);

use std::io::{self, Write};
use crate::parser::ExpParserParser;

fn main() {
    let parser = ExpParserParser::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }

        match parser.parse(&input) {
            Ok(expr) => println!("= {}", expr.eval()),
            Err(err) => eprintln!("Error de sintaxis: {:?}", err),
        }
    }
}