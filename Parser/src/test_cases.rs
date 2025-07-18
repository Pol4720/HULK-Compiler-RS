use crate::token::{make_token, TokenType};

pub struct ParserTestCase {
    pub grammar_path: &'static str,
    pub start_symbol: &'static str,
    pub tokens: Vec<crate::token::Token>,
    pub description: &'static str,
}

use std::sync::LazyLock;

pub static TEST_CASES: LazyLock<Vec<ParserTestCase>> = LazyLock::new(|| {
    vec![
        ParserTestCase {
            grammar_path: "grammar.ll1",
            start_symbol: "Program",
            tokens: vec![
                make_token(TokenType::FUNCTION, "function", 1, 1),
                make_token(TokenType::IDENT, "main", 1, 10),
                make_token(TokenType::LPAREN, "(", 1, 14),
                // Falta RPAREN aquí (incorrecto)
                make_token(TokenType::LBRACE, "{", 1, 15),
                make_token(TokenType::LET, "let", 2, 2),
                make_token(TokenType::IDENT, "x", 2, 6),
                make_token(TokenType::ASSIGN, "=", 2, 8),
                // Falta valor de asignación (incorrecto)
                make_token(TokenType::SEMICOLON, ";", 2, 9),
                make_token(TokenType::IF, "if", 3, 2),
                // Falta condición después de 'if' (incorrecto)
                make_token(TokenType::LBRACE, "{", 3, 5),
                make_token(TokenType::IDENT, "y", 4, 3),
                make_token(TokenType::ASSIGN, "=", 4, 5),
                make_token(TokenType::NUMBER, "42", 4, 7),
                make_token(TokenType::SEMICOLON, ";", 4, 9),
                make_token(TokenType::RBRACE, "}", 5, 2),
                // Llave de cierre extra (incorrecto)
                make_token(TokenType::RBRACE, "}", 6, 1),
                make_token(TokenType::EOF, "", 7, 1),
            ], // ejemplo 1
            description: "Prueba con gramática HULK. Programa: \n\
    function main() {\n\
        let x = ;\n\
        if {\n\
            y = 42;\n\
        }\n\
    }\n",
        },
        ParserTestCase {
            grammar_path: "grammar.ll1",
            start_symbol: "Program",
            tokens: vec![
                make_token(TokenType::FUNCTION, "function", 1, 1),
                make_token(TokenType::IDENT, "foo", 1, 10),
                make_token(TokenType::LPAREN, "(", 1, 13),
                make_token(TokenType::RPAREN, ")", 1, 14),
                make_token(TokenType::LBRACE, "{", 1, 16),
                make_token(TokenType::LET, "let", 2, 2),
                make_token(TokenType::IDENT, "a", 2, 6),
                make_token(TokenType::ASSIGN, "=", 2, 8),
                make_token(TokenType::NUMBER, "10", 2, 10),
                make_token(TokenType::SEMICOLON, ";", 2, 12),
                make_token(TokenType::EOF, "", 3, 1),
            ],
            description: "Prueba con gramática HULK. Programa: \n\
    function foo() {\n\
        let a = 10;\n\
    \n",
        },
        ParserTestCase {
            grammar_path: "grammar.ll1",
            start_symbol: "Program",
            tokens: vec![
                make_token(TokenType::LET, "let", 1, 1),
                make_token(TokenType::IDENT, "b", 1, 5),
                make_token(TokenType::ASSIGN, "=", 1, 7),
                make_token(TokenType::PLUS, "+", 1, 9),
                make_token(TokenType::SEMICOLON, ";", 1, 10),
                make_token(TokenType::EOF, "", 2, 1),
            ],
            description: "Prueba con gramática HULK. Programa: \n\
    let b = +;\n\
",
        },
        ParserTestCase {
            grammar_path: "grammar.ll1",
            start_symbol: "Program",
            tokens:  vec![
        make_token(TokenType::LET, "let", 1, 1),
        make_token(TokenType::IDENT, "flag", 1, 5),
        make_token(TokenType::ASSIGN, "=", 1, 10),
        make_token(TokenType::OR, "||", 1, 12),
        make_token(TokenType::AND, "&&", 1, 15),
        make_token(TokenType::SEMICOLON, ";", 1, 17),
        make_token(TokenType::EOF, "", 2, 1),
    ],
            description: "Prueba con gramática HULK. Programa: \n\
    let flag = || && ;\n",
        },
        ParserTestCase {
            grammar_path: "grammar.ll1",
            start_symbol: "Program",
            tokens: vec![
                // type Animal { speak() : String => "Some sound" ; };
                make_token(TokenType::TYPE, "type", 1, 1),
                make_token(TokenType::IDENT, "Animal", 1, 6),
                make_token(TokenType::LBRACE, "{", 1, 13),
                make_token(TokenType::IDENT, "speak", 2, 13),
                make_token(TokenType::LPAREN, "(", 2, 18),
                make_token(TokenType::RPAREN, ")", 2, 19),
                make_token(TokenType::ARROW, "=>", 2, 30),
                make_token(TokenType::STRING, "\"Some sound\"", 2, 33),
                make_token(TokenType::SEMICOLON, ";", 2, 45),
                make_token(TokenType::RBRACE, "}", 3, 9),
                make_token(TokenType::SEMICOLON, ";", 3, 10),
                // type Dog (name: String) inherits Animal { ... };
                make_token(TokenType::TYPE, "type", 4, 9),
                make_token(TokenType::IDENT, "Dog", 4, 14),
                make_token(TokenType::LPAREN, "(", 4, 18),
                make_token(TokenType::IDENT, "name", 4, 19),
                make_token(TokenType::RPAREN, ")", 4, 31),
                make_token(TokenType::INHERITS, "inherits", 4, 33),
                make_token(TokenType::IDENT, "Animal", 4, 42),
                make_token(TokenType::LBRACE, "{", 4, 49),
                make_token(TokenType::IDENT, "name", 5, 13),
                make_token(TokenType::ASSIGN, "=", 5, 18),
                make_token(TokenType::IDENT, "name", 5, 20),
                make_token(TokenType::SEMICOLON, ";", 5, 24),
                make_token(TokenType::IDENT, "speak", 7, 13),
                make_token(TokenType::LPAREN, "(", 7, 18),
                make_token(TokenType::RPAREN, ")", 7, 19),
                make_token(TokenType::ARROW, "=>", 7, 30),
                make_token(TokenType::STRING, "\"Woof!\"", 7, 33),
                make_token(TokenType::SEMICOLON, ";", 7, 41),
                make_token(TokenType::RBRACE, "}", 8, 9),
                make_token(TokenType::SEMICOLON, ";", 8, 10),
                // type Cat (name: String) inherits Animal { ... };
                make_token(TokenType::TYPE, "type", 9, 9),
                make_token(TokenType::IDENT, "Cat", 9, 14),
                make_token(TokenType::LPAREN, "(", 9, 17),
                make_token(TokenType::IDENT, "name", 9, 18),
                make_token(TokenType::COLON, ":", 9, 22),
                make_token(TokenType::IDENT, "String", 9, 24),
                make_token(TokenType::RPAREN, ")", 9, 30),
                make_token(TokenType::INHERITS, "inherits", 9, 32),
                make_token(TokenType::IDENT, "Animal", 9, 41),
                make_token(TokenType::LBRACE, "{", 9, 48),
                make_token(TokenType::IDENT, "name", 10, 13),
                make_token(TokenType::ASSIGN, "=", 10, 18),
                make_token(TokenType::IDENT, "name", 10, 20),
                make_token(TokenType::SEMICOLON, ";", 10, 24),
                make_token(TokenType::IDENT, "speak", 12, 13),
                make_token(TokenType::LPAREN, "(", 12, 18),
                make_token(TokenType::RPAREN, ")", 12, 19),
                make_token(TokenType::ARROW, "=>", 12, 30),
                make_token(TokenType::STRING, "\"Meow!\"", 12, 33),
                make_token(TokenType::SEMICOLON, ";", 12, 41),
                make_token(TokenType::RBRACE, "}", 13, 9),
                make_token(TokenType::SEMICOLON, ";", 13, 10),
                // function testLCA(cond: Boolean): Animal { ... };
                make_token(TokenType::FUNCTION, "function", 15, 9),
                make_token(TokenType::IDENT, "testLCA", 15, 18),
                make_token(TokenType::LPAREN, "(", 15, 25),
                make_token(TokenType::IDENT, "cond", 15, 26),
                make_token(TokenType::RPAREN, ")", 15, 39),
                make_token(TokenType::LBRACE, "{", 15, 49),
                // if (cond) { new Dog("Buddy"); } else { new Cat("Whiskers"); }
                make_token(TokenType::IF, "if", 16, 13),
                make_token(TokenType::LPAREN, "(", 16, 16),
                make_token(TokenType::IDENT, "cond", 16, 17),
                make_token(TokenType::RPAREN, ")", 16, 21),
                make_token(TokenType::LBRACE, "{", 16, 23),
                make_token(TokenType::NEW, "new", 17, 17),
                make_token(TokenType::IDENT, "Dog", 17, 21),
                make_token(TokenType::LPAREN, "(", 17, 24),
                make_token(TokenType::STRING, "\"Buddy\"", 17, 25),
                make_token(TokenType::RPAREN, ")", 17, 32),
                make_token(TokenType::SEMICOLON, ";", 17, 33),
                make_token(TokenType::RBRACE, "}", 18, 13),
                make_token(TokenType::ELSE, "else", 18, 15),
                make_token(TokenType::LBRACE, "{", 18, 20),
                make_token(TokenType::NEW, "new", 19, 17),
                make_token(TokenType::IDENT, "Cat", 19, 21),
                make_token(TokenType::LPAREN, "(", 19, 24),
                make_token(TokenType::STRING, "\"Whiskers\"", 19, 25),
                make_token(TokenType::RPAREN, ")", 19, 35),
                make_token(TokenType::SEMICOLON, ";", 19, 36),
                make_token(TokenType::RBRACE, "}", 20, 13),
                make_token(TokenType::SEMICOLON, ";", 20, 14),
                make_token(TokenType::RBRACE, "}", 22, 9),
                make_token(TokenType::SEMICOLON, ";", 22, 10),
                // let a = testLCA(true).speak() in print(a);
                make_token(TokenType::LET, "let", 23, 1),
                make_token(TokenType::IDENT, "a", 23, 5),
                make_token(TokenType::ASSIGN, "=", 23, 7),
                make_token(TokenType::IDENT, "testLCA", 23, 9),
                make_token(TokenType::LPAREN, "(", 23, 16),
                make_token(TokenType::TRUE, "true", 23, 17),
                make_token(TokenType::RPAREN, ")", 23, 21),
                make_token(TokenType::DOT, ".", 23, 22),
                make_token(TokenType::IDENT, "speak", 23, 23),
                make_token(TokenType::LPAREN, "(", 23, 28),
                make_token(TokenType::RPAREN, ")", 23, 29),
                make_token(TokenType::IN, "in", 23, 31),
                make_token(TokenType::IDENT, "print", 23, 34),
                make_token(TokenType::LPAREN, "(", 23, 39),
                make_token(TokenType::IDENT, "a", 23, 40),
                make_token(TokenType::RPAREN, ")", 23, 41),
                make_token(TokenType::SEMICOLON, ";", 23, 42),
                // let b = testLCA(false).speak() in print(b);
                make_token(TokenType::LET, "let", 24, 1),
                make_token(TokenType::IDENT, "b", 24, 5),
                make_token(TokenType::ASSIGN, "=", 24, 7),
                make_token(TokenType::IDENT, "testLCA", 24, 9),
                make_token(TokenType::LPAREN, "(", 24, 16),
                make_token(TokenType::FALSE, "false", 24, 17),
                make_token(TokenType::RPAREN, ")", 24, 22),
                make_token(TokenType::DOT, ".", 24, 23),
                make_token(TokenType::IDENT, "speak", 24, 24),
                make_token(TokenType::LPAREN, "(", 24, 29),
                make_token(TokenType::RPAREN, ")", 24, 30),
                make_token(TokenType::IN, "in", 24, 32),
                make_token(TokenType::IDENT, "print", 24, 35),
                make_token(TokenType::LPAREN, "(", 24, 40),
                make_token(TokenType::IDENT, "b", 24, 41),
                make_token(TokenType::RPAREN, ")", 24, 42),
                make_token(TokenType::SEMICOLON, ";", 24, 43),
                make_token(TokenType::EOF, "", 25, 1),
            ], // ejemplo 2
            description: "Prueba con gramática HULK. Programa: \n\
    type Animal { speak()  => \"Some sound\"; };\n\
    type Dog (name) inherits Animal {\n\
        name = name;\n\
        speak() => \"Woof!\";\n\
    };\n\
    type Cat (name) inherits Animal {\n\
        name = name;\n\
        speak() => \"Meow!\";\n\
    };\n\
    function testLCA(cond): Animal {\n\
        if (cond) { new Dog(\"Buddy\"); } else { new Cat(\"Whiskers\"); }\n\
    };\n\
    let a = testLCA(true).speak() in print(a);\n\
    let b = testLCA(false).speak() in print(b);\n",
        },
        ParserTestCase {
            grammar_path: "grammar.ll1",
            start_symbol: "Program",
            tokens: vec![
                make_token(TokenType::IF, "if", 1, 1),
                make_token(TokenType::LPAREN, "(", 1, 4),
                make_token(TokenType::NUMBER, "2", 1, 5),
                make_token(TokenType::PLUS, "+", 1, 7),
                make_token(TokenType::NUMBER, "2", 1, 9),
                make_token(TokenType::GT, ">", 1, 11),
                make_token(TokenType::NUMBER, "4", 1, 13),
                make_token(TokenType::RPAREN, ")", 1, 14),
                make_token(TokenType::LBRACE, "{", 1, 16),
                make_token(TokenType::LET, "let", 2, 5),
                make_token(TokenType::IDENT, "a", 2, 9),
                make_token(TokenType::ASSIGN, "=", 2, 11),
                make_token(TokenType::STRING, "\"true\"", 2, 13),
                make_token(TokenType::IN, "in", 2, 20),
                make_token(TokenType::IDENT, "a", 2, 23),
                make_token(TokenType::SEMICOLON, ";", 2, 24),
                make_token(TokenType::RBRACE, "}", 3, 1),
                make_token(TokenType::ELIF, "elif", 4, 1),
                make_token(TokenType::LPAREN, "(", 4, 6),
                make_token(TokenType::NUMBER, "2", 4, 7),
                make_token(TokenType::PLUS, "+", 4, 9),
                make_token(TokenType::NUMBER, "2", 4, 11),
                make_token(TokenType::LT, "<", 4, 13),
                make_token(TokenType::NUMBER, "4", 4, 15),
                make_token(TokenType::RPAREN, ")", 4, 16),
                make_token(TokenType::LBRACE, "{", 4, 18),
                make_token(TokenType::LET, "let", 5, 5),
                make_token(TokenType::IDENT, "a", 5, 9),
                make_token(TokenType::ASSIGN, "=", 5, 11),
                make_token(TokenType::STRING, "\"true\"", 5, 13),
                make_token(TokenType::IN, "in", 5, 20),
                make_token(TokenType::IDENT, "a", 5, 23),
                make_token(TokenType::SEMICOLON, ";", 5, 24),
                make_token(TokenType::RBRACE, "}", 6, 1),
                make_token(TokenType::ELIF, "elif", 7, 1),
                make_token(TokenType::LPAREN, "(", 7, 6),
                make_token(TokenType::NUMBER, "2", 7, 7),
                make_token(TokenType::PLUS, "+", 7, 9),
                make_token(TokenType::NUMBER, "2", 7, 11),
                make_token(TokenType::LE, "<=", 7, 13),
                make_token(TokenType::NUMBER, "4", 7, 16),
                make_token(TokenType::RPAREN, ")", 7, 17),
                make_token(TokenType::LBRACE, "{", 7, 19),
                make_token(TokenType::LET, "let", 8, 5),
                make_token(TokenType::IDENT, "a", 8, 9),
                make_token(TokenType::ASSIGN, "=", 8, 11),
                make_token(TokenType::STRING, "\"true\"", 8, 13),
                make_token(TokenType::IN, "in", 8, 20),
                make_token(TokenType::IDENT, "a", 8, 23),
                make_token(TokenType::SEMICOLON, ";", 8, 24),
                make_token(TokenType::RBRACE, "}", 9, 1),
                make_token(TokenType::ELSE, "else", 10, 1),
                make_token(TokenType::LBRACE, "{", 10, 6),
                make_token(TokenType::STRING, "\"2\"", 11, 5),
                make_token(TokenType::SEMICOLON, ";", 11, 9),
                make_token(TokenType::RBRACE, "}", 12, 1),
                make_token(TokenType::SEMICOLON, ";", 12, 2),
                make_token(TokenType::EOF, "", 13, 1),
            ],
            description: "Prueba con gramática HULK. Programa:  \n\
    if (2 + 2 > 4) {\n\
        let a = \"true\" in a;\n\
    }\n\
    elif (2 + 2 < 4) {\n\
        let a = \"true\" in a;\n\
    }\n\
    elif (2 + 2 <= 4) {\n\
        let a = \"true\" in a;\n\
    }\n\
    else {\n\
        \"2\";\n\
    };\n",
        },
        ParserTestCase {
            grammar_path: "grammar.ll1",
            start_symbol: "Program",
            tokens: vec![
                // type Point (x, y) { ... }
                make_token(TokenType::TYPE, "type", 1, 1),
                make_token(TokenType::IDENT, "Point", 1, 6),
                make_token(TokenType::LPAREN, "(", 1, 12),
                make_token(TokenType::IDENT, "x", 1, 13),
                make_token(TokenType::COMMA, ",", 1, 22),
                make_token(TokenType::IDENT, "y", 1, 24),
                make_token(TokenType::RPAREN, ")", 1, 33),
                make_token(TokenType::LBRACE, "{", 1, 35),
                // x = x;
                make_token(TokenType::IDENT, "x", 2, 13),
                make_token(TokenType::ASSIGN, "=", 2, 15),
                make_token(TokenType::IDENT, "x", 2, 17),
                make_token(TokenType::SEMICOLON, ";", 2, 18),
                // y = y;
                make_token(TokenType::IDENT, "y", 3, 13),
                make_token(TokenType::ASSIGN, "=", 3, 15),
                make_token(TokenType::IDENT, "y", 3, 17),
                make_token(TokenType::SEMICOLON, ";", 3, 18),
                // getX()  => self.x;
                make_token(TokenType::IDENT, "getX", 5, 13),
                make_token(TokenType::LPAREN, "(", 5, 18),
                make_token(TokenType::RPAREN, ")", 5, 19),
                make_token(TokenType::ARROW, "=>", 5, 30),
                make_token(TokenType::SELF, "self", 5, 33),
                make_token(TokenType::DOT, ".", 5, 37),
                make_token(TokenType::IDENT, "x", 5, 38),
                make_token(TokenType::SEMICOLON, ";", 5, 39),
                // getY()  => self.y;
                make_token(TokenType::IDENT, "getY", 6, 13),
                make_token(TokenType::LPAREN, "(", 6, 18),
                make_token(TokenType::RPAREN, ")", 6, 19),
                make_token(TokenType::ARROW, "=>", 6, 30),
                make_token(TokenType::SELF, "self", 6, 33),
                make_token(TokenType::DOT, ".", 6, 37),
                make_token(TokenType::IDENT, "y", 6, 38),
                make_token(TokenType::SEMICOLON, ";", 6, 39),
                // setX(x)  => self.x := x ;
                make_token(TokenType::IDENT, "setX", 8, 13),
                make_token(TokenType::LPAREN, "(", 8, 18),
                make_token(TokenType::IDENT, "x", 8, 19),
                make_token(TokenType::RPAREN, ")", 8, 28),
                make_token(TokenType::ARROW, "=>", 8, 39),
                make_token(TokenType::SELF, "self", 8, 42),
                make_token(TokenType::DOT, ".", 8, 46),
                make_token(TokenType::IDENT, "x", 8, 47),
                make_token(TokenType::ASSIGN_DESTRUCT, ":=", 8, 49),
                make_token(TokenType::IDENT, "x", 8, 52),
                make_token(TokenType::SEMICOLON, ";", 8, 53),
                // setY(y)  => self.y := y ;
                make_token(TokenType::IDENT, "setY", 9, 13),
                make_token(TokenType::LPAREN, "(", 9, 18),
                make_token(TokenType::IDENT, "y", 9, 19),
                make_token(TokenType::RPAREN, ")", 9, 28),
                make_token(TokenType::ARROW, "=>", 9, 39),
                make_token(TokenType::SELF, "self", 9, 42),
                make_token(TokenType::DOT, ".", 9, 46),
                make_token(TokenType::IDENT, "y", 9, 47),
                make_token(TokenType::ASSIGN_DESTRUCT, ":=", 9, 49),
                make_token(TokenType::IDENT, "y", 9, 52),
                make_token(TokenType::SEMICOLON, ";", 9, 53),
                make_token(TokenType::RBRACE, "}", 10, 9),
                make_token(TokenType::SEMICOLON, ";", 10, 10),
                // let x = new Point(3, 4) in (x.getX() + x.getY());
                make_token(TokenType::LET, "let", 12, 1),
                make_token(TokenType::IDENT, "x", 12, 5),
                make_token(TokenType::ASSIGN, "=", 12, 7),
                make_token(TokenType::NEW, "new", 12, 9),
                make_token(TokenType::IDENT, "Point", 12, 13),
                make_token(TokenType::LPAREN, "(", 12, 18),
                make_token(TokenType::NUMBER, "3", 12, 19),
                make_token(TokenType::COMMA, ",", 12, 20),
                make_token(TokenType::NUMBER, "4", 12, 22),
                make_token(TokenType::RPAREN, ")", 12, 23),
                make_token(TokenType::IN, "in", 12, 25),
                make_token(TokenType::LPAREN, "(", 12, 28),
                make_token(TokenType::IDENT, "x", 12, 29),
                make_token(TokenType::DOT, ".", 12, 30),
                make_token(TokenType::IDENT, "getX", 12, 31),
                make_token(TokenType::LPAREN, "(", 12, 36),
                make_token(TokenType::RPAREN, ")", 12, 37),
                make_token(TokenType::PLUS, "+", 12, 39),
                make_token(TokenType::IDENT, "x", 12, 41),
                make_token(TokenType::DOT, ".", 12, 42),
                make_token(TokenType::IDENT, "getY", 12, 43),
                make_token(TokenType::LPAREN, "(", 12, 48),
                make_token(TokenType::RPAREN, ")", 12, 49),
                make_token(TokenType::RPAREN, ")", 12, 50),
                make_token(TokenType::SEMICOLON, ";", 12, 51),
                // for ( i in range(1,10) ) { ... }
                make_token(TokenType::FOR, "for", 15, 1),
                make_token(TokenType::LPAREN, "(", 15, 5),
                make_token(TokenType::IDENT, "i", 15, 7),
                make_token(TokenType::IN, "in", 15, 9),
                make_token(TokenType::IDENT, "range", 15, 12),
                make_token(TokenType::LPAREN, "(", 15, 17),
                make_token(TokenType::NUMBER, "1", 15, 18),
                make_token(TokenType::COMMA, ",", 15, 19),
                make_token(TokenType::NUMBER, "10", 15, 21),
                make_token(TokenType::RPAREN, ")", 15, 23),
                make_token(TokenType::RPAREN, ")", 15, 24),
                make_token(TokenType::LBRACE, "{", 15, 26),
                // if ( i > 5 ) { print("i"); } else { "hola"; }
                make_token(TokenType::IF, "if", 16, 5),
                make_token(TokenType::LPAREN, "(", 16, 8),
                make_token(TokenType::IDENT, "i", 16, 9),
                make_token(TokenType::GT, ">", 16, 11),
                make_token(TokenType::NUMBER, "5", 16, 13),
                make_token(TokenType::RPAREN, ")", 16, 14),
                make_token(TokenType::LBRACE, "{", 16, 16),
                make_token(TokenType::IDENT, "print", 17, 9),
                make_token(TokenType::LPAREN, "(", 17, 14),
                make_token(TokenType::STRING, "\"i\"", 17, 15),
                make_token(TokenType::RPAREN, ")", 17, 18),
                make_token(TokenType::SEMICOLON, ";", 17, 19),
                make_token(TokenType::RBRACE, "}", 18, 5),
                make_token(TokenType::ELSE, "else", 18, 7),
                make_token(TokenType::LBRACE, "{", 18, 12),
                make_token(TokenType::STRING, "\"hola\"", 19, 9),
                make_token(TokenType::SEMICOLON, ";", 19, 15),
                make_token(TokenType::RBRACE, "}", 20, 5),
                make_token(TokenType::SEMICOLON, ";", 20, 6),
                make_token(TokenType::RBRACE, "}", 21, 1),
                make_token(TokenType::SEMICOLON, ";", 21, 2),
                // let x = 5 in ( x + x );
                make_token(TokenType::LET, "let", 23, 1),
                make_token(TokenType::IDENT, "x", 23, 5),
                make_token(TokenType::ASSIGN, "=", 23, 7),
                make_token(TokenType::NUMBER, "5", 23, 9),
                make_token(TokenType::IN, "in", 23, 11),
                make_token(TokenType::LPAREN, "(", 23, 14),
                make_token(TokenType::IDENT, "x", 23, 15),
                make_token(TokenType::PLUS, "+", 23, 17),
                make_token(TokenType::IDENT, "x", 23, 19),
                make_token(TokenType::RPAREN, ")", 23, 20),
                make_token(TokenType::SEMICOLON, ";", 23, 21),
                // let y = 4 , z = 3 in ( y + z );
                make_token(TokenType::LET, "let", 24, 1),
                make_token(TokenType::IDENT, "y", 24, 5),
                make_token(TokenType::ASSIGN, "=", 24, 7),
                make_token(TokenType::NUMBER, "4", 24, 9),
                make_token(TokenType::COMMA, ",", 24, 10),
                make_token(TokenType::IDENT, "z", 24, 12),
                make_token(TokenType::ASSIGN, "=", 24, 14),
                make_token(TokenType::NUMBER, "3", 24, 16),
                make_token(TokenType::IN, "in", 24, 18),
                make_token(TokenType::LPAREN, "(", 24, 21),
                make_token(TokenType::IDENT, "y", 24, 22),
                make_token(TokenType::PLUS, "+", 24, 24),
                make_token(TokenType::IDENT, "z", 24, 26),
                make_token(TokenType::RPAREN, ")", 24, 27),
                make_token(TokenType::SEMICOLON, ";", 24, 28),
                // while ( !(3 < 4) ) { "hola"; };
                make_token(TokenType::WHILE, "while", 25, 1),
                make_token(TokenType::LPAREN, "(", 25, 7),
                make_token(TokenType::MINUS, "!", 25, 8),
                make_token(TokenType::LPAREN, "(", 25, 9),
                make_token(TokenType::NUMBER, "3", 25, 10),
                make_token(TokenType::LT, "<", 25, 12),
                make_token(TokenType::NUMBER, "4", 25, 14),
                make_token(TokenType::RPAREN, ")", 25, 15),
                make_token(TokenType::RPAREN, ")", 25, 16),
                make_token(TokenType::LBRACE, "{", 25, 18),
                make_token(TokenType::STRING, "\"hola\"", 26, 5),
                make_token(TokenType::SEMICOLON, ";", 26, 11),
                make_token(TokenType::RBRACE, "}", 27, 1),
                make_token(TokenType::SEMICOLON, ";", 27, 2),
                make_token(TokenType::EOF, "", 28, 1),
            ], // ejemplo 1
            description: "Prueba con gramática HULK. Programa:  \n\
    type Point(x, y) {\n\
        x = x;\n\
        y = y;\n\
        getX() => self.x;\n\
        getY() => self.y;\n\
        setX(x) => self.x := x;\n\
        setY(y) => self.y := y;\n\
    };\n\
    let x = new Point(3, 4) in (x.getX() + x.getY());\n\
    for (i in range(1, 10)) {\n\
        if (i > 5) { print(\"i\"); } else { \"hola\"; };\n\
    };\n\
    let x = 5 in (x + x);\n\
    let y = 4, z = 3 in (y + z);\n\
    while (!(3 < 4)) { \"hola\"; };\n",
        },
        ParserTestCase {
            grammar_path: "grammar.ll1",
            start_symbol: "Program",
            tokens: vec![
                make_token(TokenType::NUMBER, "2", 1, 1),
                make_token(TokenType::PLUS, "+", 1, 2),
                make_token(TokenType::NUMBER, "3", 1, 3),
                make_token(TokenType::MULT, "*", 1, 4),
                make_token(TokenType::NUMBER, "4", 1, 5),
                make_token(TokenType::SEMICOLON, ";", 1, 6),
                make_token(TokenType::EOF, "", 1, 7),
            ],
            description: "Prueba con gramática HULK. Expresión: 2+3*4; ",
        },
        ParserTestCase {
            grammar_path: "../Grammars/grammar_1.ll1",
            start_symbol: "Expr",
            tokens: vec![
                make_token(TokenType::NUMBER, "2", 1, 1),
                make_token(TokenType::PLUS, "+", 1, 3),
                make_token(TokenType::MULT, "*", 1, 5), // Error: operator without operand between
                make_token(TokenType::NUMBER, "3", 1, 7),
                make_token(TokenType::EOF, "", 1, 9),
            ],
            description: "Prueba gramática aritmética con cadena 2+*3",
        },
        ParserTestCase {
            grammar_path: "../Grammars/grammar_1.ll1",
            start_symbol: "Expr",
            tokens: vec![
                make_token(TokenType::NUMBER, "2", 1, 1),
                make_token(TokenType::MULT, "*", 1, 3),
                make_token(TokenType::LPAREN, "(", 1, 5),
                make_token(TokenType::NUMBER, "3", 1, 6),
                make_token(TokenType::PLUS, "+", 1, 8),
                make_token(TokenType::NUMBER, "4", 1, 10),
                make_token(TokenType::RPAREN, ")", 1, 11),
                make_token(TokenType::EOF, "", 1, 13),
            ],
            description: "Prueba gramática aritmética con cadena 2*(3+4)",
        },
        ParserTestCase {
            grammar_path: "../Grammars/grammar_2.ll1",
            start_symbol: "S",
            tokens: vec![
                make_token(TokenType::a, "a", 1, 1),
                make_token(TokenType::b, "b", 1, 2),
                make_token(TokenType::b, "b", 1, 3),
                make_token(TokenType::EOF, "", 1, 4),
            ],
            description: "Prueba gramática a^n b^n con cadena abb",
        },
        ParserTestCase {
            grammar_path: "../Grammars/grammar_2.ll1",
            start_symbol: "S",
            tokens: vec![
                make_token(TokenType::a, "a", 1, 1),
                make_token(TokenType::a, "a", 1, 2),
                make_token(TokenType::b, "b", 1, 3),
                make_token(TokenType::b, "b", 1, 4),
                make_token(TokenType::EOF, "", 1, 5),
            ],
            description: "Prueba gramática a^n b^n con cadena aabb",
        },
        ParserTestCase {
            grammar_path: "../Grammars/grammar_3.ll1",
            start_symbol: "S",
            tokens: vec![
                make_token(TokenType::LPAREN, "(", 1, 1),
                make_token(TokenType::RPAREN, ")", 1, 2),
                make_token(TokenType::LPAREN, "(", 1, 3),
                make_token(TokenType::EOF, "", 1, 4),
            ],
            description: "Prueba gramática de paréntesis balanceados con cadena ()(",
        },
        ParserTestCase {
            grammar_path: "../Grammars/grammar_3.ll1",
            start_symbol: "S",
            tokens: vec![
                make_token(TokenType::LPAREN, "(", 1, 1),
                make_token(TokenType::RPAREN, ")", 1, 2),
                make_token(TokenType::LPAREN, "(", 1, 3),
                make_token(TokenType::LPAREN, "(", 1, 4),
                make_token(TokenType::RPAREN, ")", 1, 5),
                make_token(TokenType::RPAREN, ")", 1, 6),
                make_token(TokenType::EOF, "", 1, 7),
            ],
            description: "Prueba gramática de paréntesis balanceados con cadena ()(())",
        },
    ]
});
