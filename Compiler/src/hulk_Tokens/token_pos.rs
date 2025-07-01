//! # TokenPos
//!
//! Estructura que representa la posición de un token en el código fuente.
//! Guarda el índice de inicio y fin del token, permitiendo localizarlo en el texto original.
//!
//! ## Campos
//! - `start`: Índice de inicio del token (inclusive).
//! - `end`: Índice de fin del token (exclusive).
//!
//! ## Métodos
//! - `new(start: usize, end: usize) -> Self`  
//!   Crea una nueva posición de token a partir de los índices de inicio y fin.
//!
//! ## Derivados
//! - `Debug`, `Copy`, `Clone`, `PartialEq`, `Eq`
//!
//! ## Uso típico
//! Se utiliza para asociar información de localización a los nodos del AST, errores de parsing y mensajes de diagnóstico.
#[derive(Debug,Copy, Clone, PartialEq, Eq)]
pub struct TokenPos {
    pub start: usize,
    pub end: usize,
}

impl TokenPos {
    pub fn new(start: usize, end: usize) -> Self {
        TokenPos { start, end }
    }
}
