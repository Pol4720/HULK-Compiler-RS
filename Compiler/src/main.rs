use lalrpop_util::lalrpop_mod;
use semantic_visitor::hulk_semantic_visitor::SemanticVisitor;
use visitor::hulk_visitor::Visitor;
pub mod hulk_tokens;
pub mod visitor;
pub mod typings;
pub mod semantic_visitor;
lalrpop_mod!(pub parser);

use std::io::{self, Write};
use crate::parser::ProgramParser;
use crate::visitor::hulk_ast_visitor_print::PreetyPrintVisitor;

use inkwell::context::Context;
use codegen::generator::CodeGenerator;

fn main() {
    let parser = ProgramParser::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }

        let parsed_expr = parser.parse(&input).unwrap();
        let mut print_visitor = PreetyPrintVisitor;
        let mut semantic_visitor = SemanticVisitor::new();
        let res = semantic_visitor.analyze(&parsed_expr);
        match res {
            Ok(_) => {
                println!("Parsed successfully!");
                println!("");
                println!("\x1b[34m{}\x1b[0m", print_visitor.visit_program(&parsed_expr));

                // --- INTEGRACIÓN CODEGEN ---
                // Solo ejecutar codegen si el análisis semántico fue exitoso
                let context = Context::create();
                let mut generator = CodeGenerator::new("hulk_module", &context);
                match generator.generate(&parsed_expr) {
                    Ok(_) => generator.print_ir(), // Muestra el LLVM IR generado
                    Err(e) => eprintln!("Error en codegen: {}", e),
                }
                // --- FIN CODEGEN ---
            }
            Err(errors) => {
                println!("\x1b[31mErrors:");
                for err in errors.iter() {
                    println!("{}", err.message());
                }
                println!("\x1b[0m");
            }
        }
}
}
