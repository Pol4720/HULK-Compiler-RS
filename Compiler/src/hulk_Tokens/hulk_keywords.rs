//! # KeywordToken Enum
//!
//! Este módulo define el enum `KeywordToken` para el compilador Hulk.
//! Representa todas las palabras clave reservadas del lenguaje Hulk, como `if`, `let`, `while`, `function`, etc.
//! Es utilizado en el análisis léxico y sintáctico para identificar y manejar correctamente las palabras clave del lenguaje.

use std::fmt::Display;

/// Enum que representa las palabras clave reservadas del lenguaje Hulk.
/// 
/// - Cada variante corresponde a una palabra clave específica del lenguaje.
/// - Se utiliza en el lexer y parser para distinguir instrucciones y estructuras de control.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum KeywordToken {
    Print,
    While,
    Elif,
    Else,
    If,
    In,
    Function,
    Let,
    Class,
    Return,
    Break,
    Protocol,
    Continue,
    Import,
    Type,
    Inherits,
    For,
    New,
}

impl Display for KeywordToken {
    /// Permite mostrar la palabra clave como string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeywordToken::Let => write!(f, "let"),
            KeywordToken::If => write!(f, "if"),
            KeywordToken::Else => write!(f, "else"),
            KeywordToken::While => write!(f, "while"),
            KeywordToken::Print => write!(f, "print"),
            KeywordToken::In => write!(f, "in"),    
            KeywordToken::Function => write!(f, "function"),
            KeywordToken::Class => write!(f, "class"),
            KeywordToken::Protocol => write!(f, "protocol"),
            KeywordToken::Type => write!(f, "type"),
            KeywordToken::Return => write!(f, "return"),
            KeywordToken::Inherits => write!(f, "inherits"),
            KeywordToken::Break => write!(f, "break"),
            KeywordToken::Continue => write!(f, "continue"),
            KeywordToken::Import => write!(f, "import"),
            KeywordToken::For => write!(f, "for"),
            KeywordToken::Elif => write!(f, "elif"),
            KeywordToken::New => write!(f, "new"),
        }
    }
}