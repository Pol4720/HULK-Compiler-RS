use crate::token::{make_token, mock_tokens, TokenType};

pub struct ParserTestCase {
    pub grammar_path: &'static str,
    pub start_symbol: &'static str,
    pub tokens: Vec<crate::token::Token>,
    pub description: &'static str,
}

use std::sync::LazyLock;

pub static TEST_CASES: LazyLock<Vec<ParserTestCase>> = LazyLock::new(|| vec![
    ParserTestCase {
        grammar_path: "grammar.ll1",
        start_symbol: "Program",
        tokens: mock_tokens(), // ejemplo 1
        description: "Prueba con gramática HULK y expresión aritmética simple",
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
]);
