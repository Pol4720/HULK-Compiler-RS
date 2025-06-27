use std::vec;

use crate::lexgen::lexer::Lexer;
use crate::lexgen::token_spec::TokenSpec;

#[test]
pub fn collision_case() {
    let rules = vec![
        TokenSpec::build("1".to_string(), r"a".to_string()),
        TokenSpec::build("2".to_string(), r"a*b".to_string()),
    ];

    let str = "aaaaaaaaaaaaaaaaaaaaaaa";
    let lexer = Lexer::new(rules);
    let result = lexer.split(str);

    assert!(result.is_ok());
    let tokens = result.unwrap();
    assert_eq!(tokens.len(), str.len());
    assert_eq!(
        tokens.iter().map(|t| t.kind.clone()).collect::<Vec<_>>(),
        (0..=(str.len() - 1))
            .into_iter()
            .map(|_| "1".to_string())
            .collect::<Vec<_>>()
    )
}

#[test]
pub fn lex_hulk_line() {
    let rules = vec![
        TokenSpec::build("EQUAL", r"="),
        TokenSpec::build("LET", r"let"),
        TokenSpec::build("IN", r"in"),
        TokenSpec::build("SEMICOLON", r";"),
        TokenSpec::build("COLON", r":"),
        TokenSpec::build("NUMBER", r"[0-9]+"),
        TokenSpec::build("IDENTIFIER", r"(_|[a-zA-Z])(_|[a-z0-9A-Z])*"),
        TokenSpec::build_ignorable("WhiteSpace", r"\s+"),
    ];

    let lexer = Lexer::new(rules);
    let result = lexer.split("let malanguita_frita: Number = 69 in x;");

    assert!(result.is_ok());
    let tokens = result.unwrap();

    assert_eq!(tokens.len(), 9);
    assert_eq!(
        tokens.iter().map(|t| t.kind).collect::<Vec<_>>(),
        vec![
            "LET",
            "IDENTIFIER",
            "COLON",
            "IDENTIFIER",
            "EQUAL",
            "NUMBER",
            "IN",
            "IDENTIFIER",
            "SEMICOLON"
        ]
    );
}

#[test]
pub fn lex_another_hulk_line() {
    let rules = vec![
        TokenSpec::build("EQUAL", r"="),
        TokenSpec::build("FUNCTION", r"function"),
        TokenSpec::build("ARROW", r"=>"),
        TokenSpec::build("SEMICOLON", r";"),
        TokenSpec::build("COLON", r":"),
        TokenSpec::build("PLUS", r"\+"),
        TokenSpec::build("LPAREN", r"\(".to_string()),
        TokenSpec::build("RPAREN", r"\)".to_string()),
        TokenSpec::build("COMMA", r","),
        TokenSpec::build("NUMBER", r"[0-9]+"),
        TokenSpec::build("IDENTIFIER", r"(_|[a-zA-Z])(_|[a-z0-9A-Z])*"),
        TokenSpec::build_ignorable("WhiteSpace", r"\s+"),
    ];

    let lexer = Lexer::new(rules);
    let result = lexer.split("function function_sexo (a:Number, b:Number): Number => a + b + 69;");

    assert!(result.is_ok());
    let tokens = result.unwrap();

    assert_eq!(tokens.len(), 20);
    assert_eq!(
        tokens.iter().map(|t| t.kind).collect::<Vec<_>>(),
        vec![
            "FUNCTION",
            "IDENTIFIER",
            "LPAREN",
            "IDENTIFIER",
            "COLON",
            "IDENTIFIER",
            "COMMA",
            "IDENTIFIER",
            "COLON",
            "IDENTIFIER",
            "RPAREN",
            "COLON",
            "IDENTIFIER",
            "ARROW",
            "IDENTIFIER",
            "PLUS",
            "IDENTIFIER",
            "PLUS",
            "NUMBER",
            "SEMICOLON"
        ]
    );
}
#[test]
pub fn lex_another_hulk_line_with_error() {
    let rules = vec![
        TokenSpec::build("EQUAL", r"="),
        TokenSpec::build("FUNCTION", r"function"),
        TokenSpec::build("ARROW", r"=>"),
        TokenSpec::build("SEMICOLON", r";"),
        TokenSpec::build("COLON", r":"),
        TokenSpec::build("PLUS", r"\+"),
        TokenSpec::build("LPAREN", r"\("),
        TokenSpec::build("RPAREN", r"\)"),
        TokenSpec::build("COMMA", r","),
        TokenSpec::build("NUMBER", r"[0-9]+"),
        TokenSpec::build("IDENTIFIER", r"(_|[a-zA-Z])(_|[a-z0-9A-Z])*"),
        TokenSpec::build_ignorable("WhiteSpace", r"\s+"),
    ];

    let lexer = Lexer::new(rules);
    let input = "function a() => b # c;";
    let result = lexer.split(input);

    assert!(result.is_err());

    let errors = result.err().unwrap();

    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0],
        "Lexical Error!: Unexpected character '#' at line: 0, column: 18"
    );
}

#[test]
pub fn lex_another_hulk_line_with_multiple_errors() {
    let rules = vec![
        TokenSpec::build("EQUAL", r"="),
        TokenSpec::build("FUNCTION", r"function"),
        TokenSpec::build("ARROW", r"=>"),
        TokenSpec::build("SEMICOLON", r";"),
        TokenSpec::build("COLON", r":"),
        TokenSpec::build("PLUS", r"\+"),
        TokenSpec::build("LPAREN", r"\("),
        TokenSpec::build("RPAREN", r"\)"),
        TokenSpec::build("COMMA", r","),
        TokenSpec::build("IDENTIFIER", r"(_|[a-zA-Z])(_|[a-z0-9A-Z])*"),
        TokenSpec::build_ignorable("WhiteSpace", r"(\s|\t|\n)+"),
    ];

    let lexer = Lexer::new(rules);
    let input = "function 2a () => b $ c;
function testicol () => d + e;
function minus() => f - g;";

    let result = lexer.split(input);

    assert!(result.is_err());

    let errors = result.err().unwrap();

    assert_eq!(errors.len(), 3);
    assert_eq!(
        errors,
        vec![
            "Lexical Error!: Unexpected character '2' at line: 0, column: 9",
            "Lexical Error!: Unexpected character '$' at line: 0, column: 20",
            "Lexical Error!: Unexpected character '-' at line: 4, column: 23"
        ]
    );
}
