// Módulo principal para el generador de analizadores léxicos
pub mod generator;

// Aquí se pueden agregar utilidades y estructuras comunes para el generador
pub mod codegen;
pub mod dfa;
pub mod nfa;
pub mod spec;

// Reexportar las funciones principales para el main
pub use spec::token_spec::TokenSpec;
pub use spec::token_spec::read_token_spec;
