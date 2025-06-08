use lalrpop_util::lalrpop_mod;
use semantic_visitor::hulk_semantic_visitor::SemanticVisitor;
use visitor::hulk_visitor::Visitor;

pub mod hulk_tokens;
pub mod visitor;
pub mod typings;
pub mod semantic_visitor;
pub mod codegen;

lalrpop_mod!(pub parser);

use std::io::{self, Write};
use crate::parser::ProgramParser;
use crate::visitor::hulk_ast_visitor_print::PreetyPrintVisitor;
use crate::codegen::CodeGenerator;

fn main() {
    let parser = ProgramParser::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }

        // Parse input
        let parsed_expr = match parser.parse(&input) {
            Ok(ast) => ast,
            Err(e) => {
                eprintln!("\x1b[31mError de parsing: {:?}\x1b[0m", e);
                continue;
            }
        };

        // Pretty print del AST
        let mut print_visitor = PreetyPrintVisitor;
        println!("\n\x1b[34m{}\x1b[0m", print_visitor.visit_program(&parsed_expr));

        // Análisis semántico
        let mut semantic_visitor = SemanticVisitor::new();
        match semantic_visitor.analyze(&parsed_expr) {
            Ok(_) => {
                println!("Análisis semántico exitoso.");
            }
            Err(errors) => {
                println!("\x1b[31mErrores semánticos:");
                for err in errors.iter() {
                    println!("{}", err.message());
                }
                println!("\x1b[0m");
                continue;
            }
        }

        // Codegen y ejecución
        println!("\x1b[32mGenerando código y ejecutando...\x1b[0m");
        CodeGenerator::generate_and_run(&parsed_expr, "out.ll");

        println!("\n");
    }
}
