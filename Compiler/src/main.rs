use lalrpop_util::lalrpop_mod;
use semantic_visitor::hulk_semantic_visitor::SemanticVisitor;
use visitor::hulk_visitor::Visitor;

pub mod hulk_ast_nodes;
pub mod hulk_tokens;
pub mod visitor;
pub mod typings;
pub mod semantic_visitor;
pub mod codegen;

lalrpop_mod!(pub parser);

use std::fs;
use std::fs::File;
use std::io::{self, Write};
use crate::parser::ProgramParser;
use crate::visitor::hulk_ast_visitor_print::PreetyPrintVisitor;
use crate::codegen::CodeGenerator;

fn main() {
    let parser = ProgramParser::new();

    let ex = "
        type Point (x: Number, y: Number) {
        x = x;
        y = y;

        getX() : Number => self.x;
        getY() : Number => self.y;

        setX(x: Number) : Number => self.x := x ;
        setY(y: Number) : Number => self.y := y ;
    }; 


    let x = new Point(3, 4) in (x.getX() + x.getY()) ;
    
    function SumLet (a: Number , b : Number) : Object {
        if ( a > b ) {
            5 ;
        } else {
            \"hola\" ;
        }
    } ;
    function SumPro ( a: Number , b : Number ) : Object {
        if ( a > b ) {
            5 ;
        } else {
            SumLet( a, b ) ;
        }
    } ;

    for ( i in range(1,10) ) {
        if ( i > 5 ) {
            i;
        } else {
            \"hola\";
        }
    };

    let x = 5 in ( x + x ) ;
    let y = 4 , z = 3 in ( y + z ) ;
    while ( !(3 < 4) ) { 
        \"hola\" ;
    };

    let x = SumLet( 5, 5) in x ;";

    
    let a = "
        if (true) {
            if (true) {
                2;
            }
        }
        elif (false){
            3;
        }
        else{ 
            2;
        };
        
        let a = 2, a = 3 in (print(a) + b);

    ";
    let inp = "
    function SumLet(a: Number , b: Number): Object {
        if ( a > b ) {
            5 ;
        } else {
            \"hola\" ;
        }
    } ;
    ";
    let input = "
    type Point (x: Number, y: Number) {
        x = x;
        y = y;

        getX() : Number => self.x;
        getY() : Number => self.y;

        setX(x: Number) : Number => self.x := x ;
        setY(y: Number) : Number => self.y := y ;
    }; 


    let x = new Point(3, 4) in (x.getX() + x.getY()) ;
    
    function SumLet (a: Number , b : Number) : Object {
        if ( a > b ) {
            5 ;
        } else {
            \"hola\" ;
        }
    } ;
    function SumPro ( a: Number , b : Number ) : Object {
        if ( a > b ) {
            5 ;
        } else {
            SumLet( a, b ) ;
        }
    } ;

    for ( i in range(1,10) ) {
        if ( i > 5 ) {
            i;
        } else {
            \"hola\";
        }
    };

    let x = 5 in ( x + x ) ;
    let y = 4 , z = 3 in ( y + z ) ;
    while ( !(3 < 4) ) { 
        \"hola\" ;
    };

    let x = SumLet( 5, 5) in x ;
    ";

    let input_hulk = fs::read_to_string("../../script.hulk")
        .expect("Failed to read input file");

    // loop {
        print!("> ");
        io::stdout().flush().unwrap();

        // let mut input = String::new();
        // if io::stdin().read_line(&mut input).unwrap() == 0 {
        //     break;
        // }

        let mut parsed_expr = parser.parse(&input_hulk).unwrap();
        let mut print_visitor = PreetyPrintVisitor;
        let mut semantic_visitor = SemanticVisitor::new();
        let res = semantic_visitor.check(&mut parsed_expr);
        let mut ast_file = File::create("ast.txt").expect("No se pudo crear ast.txt");

        let ast_str = print_visitor.visit_program(&mut parsed_expr);
        println!("\x1b[34m{}\x1b[0m", ast_str);
        ast_file.write_all(ast_str.as_bytes()).expect("No se pudo escribir en ast.txt");

        match res {
            Ok(_) => {
            println!("Parsed successfully And zero semantic errors!");
            }
            Err(errors) => {
            println!("\x1b[31mErrors:");
            for err in errors.iter() {
                println!("{}", err.message());
                ast_file.write_all(format!("{}\n", err.message()).as_bytes())
                .expect("No se pudo escribir en ast.txt");
            }
            println!("\x1b[0m");
            }
        }
        println!("");
        // Codegen y ejecución
        println!("\x1b[32mGenerando código y ejecutando...\x1b[0m");
        CodeGenerator::generate_and_run(&parsed_expr, "out.ll");

        println!("\n");
    // }
}
