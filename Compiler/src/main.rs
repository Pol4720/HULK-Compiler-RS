use lalrpop_util::lalrpop_mod;
use semantic_visitor::hulk_semantic_visitor::SemanticVisitor;
use visitor::hulk_visitor::Visitor;

pub mod codegen;
pub mod hulk_ast_nodes;
pub mod hulk_tokens;
pub mod semantic_visitor;
pub mod typings;
pub mod visitor;

lalrpop_mod!(pub parser);

use crate::codegen::CodeGenerator;
use crate::parser::ProgramParser;
use crate::visitor::hulk_ast_visitor_print::PreetyPrintVisitor;
use std::fs;
use std::fs::File;
use std::io::{self, Write};

fn main() {
    let parser = ProgramParser::new();

    let ex = r#"
        type Point (x: Number, y: Number) {
            x = x;
            y = y;

            getX() : Number => self.x;
            getY() : Number => self.y;

            setX(x: Number) : Number => self.x := x ;
            setY(y: Number) : Number => self.y := y ;
        }

        let x = new Point(3, 4) in (x.getX() + x.getY()) ;

        function SumLet (a: Number , b : Number) : Object {
            if ( a > b ) {
                5 ;
            } else {
                \"hola\" ;
            }
        } 

        function SumPro ( a: Number , b : Number ) : Object {
            if ( a > b ) {
                5 ;
            } else {
                SumLet( a, b ) ;
            }
        }

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
    "#;

    let test_lca = r#"
        type Animal {
            speak() : String => \"Some sound\" ;
        } 
        type Dog (name: String) inherits Animal {
            name = name ;

            speak() : String => \"Woof!\" ;
        } 
        type Cat (name: String) inherits Animal {
            name = name ;

            speak() : String => \"Meow!\" ;
        } 

        function testLCA(cond: Boolean): Animal {
            if (2 < 3) {
                new Dog(\"Buddy\");
            }
            elif(2 > 3){
                new Cat(\"Whiskers\");
            }
            else {
                new Animal();
            }
        }
    "#;

    let a = r#"
    function is_prime(n: Number): Boolean {
        if (n <= 1) {
            false;
        } elif (n == 2) {
            true;
        } elif (n % 2 == 0) {
            false;
        } else {
            let divisor = 3 in {
                while (divisor * divisor <= n) {
                    if (n % divisor == 0) {
                        false;
                    };
                    divisor := divisor + 2;
                };
            };
            true;
        };
    }

    print(is_prime(5));
    "#;

    let inp = r#"
        if (2 + 2 > 4) {
            let a = "true" in print(a);
        }
        elif (2 + 2 <= 4) {
            let a = "true" in print(a);
        }
        else{
            print("2");
        };
    "#;

    let input = r#"
        type Point (x: Number, y: Number) {
            x = x;
            y = y;

            getX() : Number => self.x;
            getY() : Number => self.y;

            setX(x: Number) : Number => self.x := x ;
            setY(y: Number) : Number => self.y := y ;
        }

        let x = new Point(3, 4) in (x.getX() + x.getY()) ;

        function SumLet (a: Number , b : Number) : Object {
            if ( a > b ) {
                5 ;
            } else {
                \"hola\" ;
            }
        }
        function SumPro ( a: Number , b : Number ) : Object {
            if ( a > b ) {
                5 ;
            } else {
                SumLet( a, b ) ;
            }
        }

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
    "#;

    let test_type = r#"
        type Point (x: Number, y: Number) {
            x = x;
            y = y;

            getX() : Number => self.x;
            getY() : Number => self.y;

            setX(x: Number) : Number => self.x := x ;
            setY(y: Number) : Number => self.y := y ;
        }
        
        function SumLet(a: Number , b: Number): Object {
            if ( a > b ) {
                5 ;
            } else {
                \"hola\" ;
            }
        }
    "#;

    let function_test = r#"
        function sum(a: Number, b: Number): Number {
            print(a);
            a + b ;
        }
        print(sum(3, 4) + 2);

        let a = 2 in (a + 2);
    "#;

    let boolean_test = r#"
        let a = "2" in print(a);
    "#;

<<<<<<< HEAD
    ";

    let recursive_test = "
=======
    let recursive_test = r#"
>>>>>>> da81e4b (changing if expr)
        function factorial(n: Number): Number {
            if (n <= 1) {
                1;
            } else {
                n * factorial(n - 1);
            }
        }

        let result = factorial(5) in print(result);
    "#;

<<<<<<< HEAD
    let input_hulk = fs::read_to_string("../script.hulk").expect("Failed to read input file");
=======
    let input_hulk = fs::read_to_string("../script.hulk")
        .expect("Failed to read input file");
>>>>>>> da81e4b (changing if expr)

    print!("> ");
    io::stdout().flush().unwrap();

<<<<<<< HEAD
    // let mut input = String::new();
    // if io::stdin().read_line(&mut input).unwrap() == 0 {
    //     break;
    // }

    let mut parsed_expr = parser.parse(&input_hulk).unwrap();
=======
    let mut parsed_expr = parser.parse(&inp).unwrap();
>>>>>>> da81e4b (changing if expr)
    let mut print_visitor = PreetyPrintVisitor;
    let mut semantic_visitor = SemanticVisitor::new();
    let res = semantic_visitor.check(&mut parsed_expr);

    match &res {
        Ok(_) => {
            println!("Parsed successfully And zero semantic errors!");
        }
        Err(errors) => {
            println!("\x1b[31mErrors:");
            for err in errors.iter() {
                println!("{}", err.message());
            }
            println!("\x1b[0m");
        }
    }
    println!("");

    let mut ast_file = File::create("ast.txt").expect("No se pudo crear ast.txt");

    match &res {
        Ok(_) => {
            println!("Parsed successfully And zero semantic errors!");
            let ast_str = print_visitor.visit_program(&mut parsed_expr);
            println!("\x1b[34m{}\x1b[0m", ast_str);
            ast_file
                .write_all(ast_str.as_bytes())
                .expect("No se pudo escribir en ast.txt");
            // Codegen y ejecución
            println!("\x1b[32mGenerando código y ejecutando...\x1b[0m");
            CodeGenerator::generate_and_run(&parsed_expr, "out.ll");
        }
<<<<<<< HEAD
        Err(errors) => {
            println!("\x1b[31mErrors:");
            for err in errors.iter() {
                println!("{}", err.message());
                ast_file
                    .write_all(format!("{}\n", err.message()).as_bytes())
                    .expect("No se pudo escribir en ast.txt");
            }
            println!("\x1b[0m");
=======
        Err(_) => {
            println!("\x1b[32mGenerando código y ejecutando...\x1b[0m");
            CodeGenerator::generate_and_run(&parsed_expr, "out.ll");
>>>>>>> da81e4b (changing if expr)
        }
    }

    println!("\n");
}