use crate::grammar::load_grammar;
use crate::token::mock_tokens;
use crate::ll1::{compute_first, compute_follow, build_ll1_table};
use crate::parser::Parser;
use crate::cst_to_ast::convert_to_ast;

mod grammar;
mod utils;
mod token;
mod ll1;
mod parser;
mod cst;
mod ast;
mod cst_to_ast;

fn main() {
    let grammar = load_grammar("grammar.ll1");
    let first = compute_first(&grammar);
    let follow = compute_follow(&grammar, &first, "Program");
    let table = build_ll1_table(&grammar, &first, &follow);

    let tokens = mock_tokens();
    let mut parser = Parser::new(tokens, table, grammar, "Program");
    let cst = parser.parse();

    println!("\nCST Tree:");
    println!("{}", cst);

    let ast = convert_to_ast(&cst);
    println!("\nAST Tree:");
    for stmt in ast {
        println!("{:?}", stmt);
    }
}
