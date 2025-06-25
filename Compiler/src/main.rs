//! # main.rs
//!
//! Punto de entrada del compilador Hulk. Este archivo orquesta el flujo principal de compilación, incluyendo:
//! - Lectura del archivo fuente Hulk (`script.hulk`)
//! - Parsing del código fuente a un AST usando LALRPOP y el parser personalizado (`HulkParser`)
//! - Reporte de errores de sintaxis con contexto y colores
//! - Análisis semántico del AST usando el visitor semántico (`SemanticVisitor`)
//! - Reporte de errores semánticos detallados
//! - Impresión del AST en formato legible
//! - Generación de código LLVM IR y ejecución del resultado
//!
//! ## Flujo principal
//! 1. Lee el archivo fuente Hulk desde `../script.hulk`
//! 2. Parsea el código fuente y reporta errores de sintaxis si existen
//! 3. Si el parseo es exitoso, realiza el análisis semántico y reporta errores si los hay
//! 4. Si no hay errores semánticos, imprime el AST y genera el código LLVM IR
//! 5. Ejecuta el código generado usando el runner de LLVM
//!
//! ## Módulos utilizados
//! - `codegen`: Generación de código LLVM IR
//! - `hulk_ast_nodes`: Definición de nodos del AST
//! - `hulk_tokens`: Definición de tokens y posiciones
//! - `semantic_visitor`: Análisis semántico y reporte de errores
//! - `typings`: Manejo de tipos y herencia
//! - `visitor`: Visitors para impresión y análisis
//! - `helper_error_reporter`: Reporte amigable de errores de parsing
//!
//! ## Ejecución
//! El archivo espera que exista un archivo `script.hulk` en el directorio superior al de `Compiler`.
//! El resultado del AST se guarda en `ast.txt` y el código LLVM IR generado se guarda y ejecuta como `out.ll`.
//!
//! ## Ejemplo de uso
//! ```sh
//! cargo run
//! ```
//!

use lalrpop_util::lalrpop_mod;
use semantic_visitor::hulk_semantic_visitor::SemanticVisitor;
use visitor::hulk_visitor::Visitor;

pub mod codegen;
pub mod hulk_ast_nodes;
pub mod hulk_tokens;
pub mod semantic_visitor;
pub mod typings;
pub mod visitor;

pub mod helper_error_reporter;

// #[cfg(test)]
// mod test {
//     mod code_block; 
// }

lalrpop_mod!(pub parser);

use crate::codegen::CodeGenerator;
use crate::helper_error_reporter::HulkParser;
use crate::visitor::hulk_ast_visitor_print::PreetyPrintVisitor;
use std::fs;
use std::fs::File;
use std::io::{self, Write};

fn main() {

    let ex = r#"

        function SumLet(a: Number, b: Number): Object {
            if ( a > b ) {
                5;
            } 
            else {
                \"hola\";
            }
        }

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

    print(is_prime(6));
    "#;

    let inp = r#"
        if (2 + 2 > 4) {
            let a = "true" in print(a);
        }
        elif (2 + 2 < 4) {
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

        let x = new Point(3, 4) in (x.getX() + x.getY());

        function SumLet (a: Number , b : Number) : Object {
            if ( a > b ) {
                5;
            } else {
                \"hola\";
            }
        }

        function SumPro ( a: Number , b : Number ) : Object {
            if ( a > b ) {
                5;
            } else {
                SumLet( a, b );
            }
        }

        for ( i in range(1,10) ) {
            if ( i > 5 ) {
                i;
            } else {
                \"hola\";
            }
        };

        let x = 5 in ( x + x );
        let y = 4 , z = 3 in ( y + z );
        while ( !(3 < 4) ) { 
            \"hola\";
        };

        let x = SumLet( 5, 5) in x;
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


    let recursive_test = r#"
        function factorial(n: Number): Number {
            if (n <= 1) {
                1;
            } else {
                n * factorial(n - 1);
            }
        }

        let result = factorial(5) in print(result);
    "#;

    let if_el = r#"
        function abs(x: Number): Number {
            if (x < 0) {
                -x;
            } else {
                x;
            }
        }

        function log10(x: Number): Number {
            if (x <= 0) {
                0;
            } else {
                let int_part = 0 in
                let temp = x in {
                    while (temp >= 10) {
                        temp := temp / 10;
                        int_part := int_part + 1;
                    };

                    while (temp < 1) {
                        temp := temp * 10;
                        int_part := int_part - 1;
                    };

                    let y = (temp - 1) / (temp + 1) in
                    let y2 = y * y in
                    let frac = 0.0, term = y, n = 0, epsilon = 0.0000000001, max_iter = 1000 in {
                        while ((abs(term) >= epsilon) & (n < max_iter)) {
                            frac := frac + term;
                            n := n + 1;
                            term := term * y2 * (2 * n - 1) / (2 * n + 1);
                        };
                        let fractional = 0.8685889638065035 * 2 * frac in
                            (int_part + fractional);
                    }
                }
            }
        }

        print(log10(1));
    "#;

    let boolean_test = r#"
            type Point (x: Number, y: Number) {
            x = x;
            y = y;
        } 
        
"#;


    let input_hulk = fs::read_to_string("../script.hulk")
        .expect("Failed to read input file");

    print!("> ");
    io::stdout().flush().unwrap();

    let parser = HulkParser::new();
    let parsed_result = parser.parse(&input_hulk);

    let mut parsed_expr = match parsed_result {
        Ok(expr) => expr,
        Err(parse_err) => {
            println!("\x1b[31mSyntax Error:\x1b[0m");
            for err in parse_err.iter() {
                println!("{}", err);
            }
            std::process::exit(1);
        }
    };

    let mut print_visitor = PreetyPrintVisitor;
    let mut semantic_visitor = SemanticVisitor::new();
    let res = semantic_visitor.check(&mut parsed_expr);

    match &res {
        Ok(_) => {
            println!("Parsed successfully And zero semantic errors!");
        }
        Err(errors) => {
            println!("\x1b[31mSemantic Errors:");
            for err in errors.iter() {
                println!("{}", err.report(&input_hulk));
            }
            println!("\x1b[0m");
            std::process::exit(3);
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
            CodeGenerator::generate_and_run(&mut parsed_expr, "out.ll");
        }
        Err(_) => {
            println!("\x1b[32mGenerando código y ejecutando...\x1b[0m");
            CodeGenerator::generate_and_run(&mut parsed_expr, "out.ll");
        }
    }

    println!("\n");
}