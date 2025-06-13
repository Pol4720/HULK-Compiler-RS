//! # Operadores y Delimitadores Tokens
//!
//! Este módulo define los enums `BinaryOperatorToken`, `UnaryOperator` y `DelimiterToken` para el compilador Hulk.
//! Representan los operadores binarios, unarios y delimitadores soportados por el lenguaje Hulk.
//! Son utilizados en el análisis léxico, sintáctico y en la generación de código para identificar y manipular operaciones y símbolos especiales.

//// Enum que representa los operadores binarios del lenguaje Hulk.
//// 
//// Ejemplos: suma, resta, multiplicación, comparación, concatenación, etc.
//#[derive(Debug, Clone, Copy, PartialEq, Eq)]
//pub enum BinaryOperatorToken {
//    Mul,        // *
//    Div,        // /
//    Plus,       // +
//    Minus,      // -
//    Mod,        // %
//    Pow,        // **
//    Neg,        // Negación (no estándar, puede usarse para - unario)
//    Not,        // ! (no estándar, puede usarse para not binario)
//    Eq,         // =
//    EqEq,       // ==
//    Neq,        // !=
//    Gt,         // >
//    Gte,        // >=
//    Lt,         // <
//    Lte,        // <=
//    Concat,     // ++ (concatenación de strings)
//    And,        // &&
//    Or,         // ||
//    DotEqual    // .= (asignación a miembro)
//}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperatorToken {
    Mul,
    Div,
    Plus,
    Minus,
    Mod,
    Pow,
    Neg,
    Not,
    Eq,
    EqEq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    Concat,
    And,
    Or,
    DotEqual
}


/// Enum que representa los operadores unarios del lenguaje Hulk.
/// 
/// Ejemplos: negación aritmética, negación lógica, operador positivo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Plus,        // +x
    Minus,       // -x
    LogicalNot,  // !x
}

/// Enum que representa los delimitadores del lenguaje Hulk.
/// 
/// Ejemplos: punto y coma, paréntesis, llaves, coma, dos puntos, flecha, etc.
#[derive(Debug, PartialEq)]
pub enum DelimiterToken {
    Semicolon,    // ;
    Colon,        // :
    Doubledot,    // ..
    Comma,        // ,
    Lparen,       // (
    Rparen,       // )
    Lbrace,       // {
    Arrow,        // ->
    DotAccess,    // .
    Rbrace,       // }
}