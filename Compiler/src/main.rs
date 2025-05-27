use lalrpop_util::lalrpop_mod;
use visitor::hulk_accept::Accept;
pub mod hulk_tokens;
pub mod visitor;
lalrpop_mod!(pub parser);

use std::io::{self, Write};
use crate::parser::ProgramParser;
use crate::visitor::hulk_ast_visitor_print::PreetyPrintVisitor;

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
                // 2. Usa el visitor para imprimir el AST bonito
                let mut printer = PreetyPrintVisitor;
                println!("{}", ast.accept(&mut printer));

                // Si quieres, puedes dejar el to_tree para debug tradicional:
                // println!("{}", ast.to_tree(0));

                for instr in ast.instructions {
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
