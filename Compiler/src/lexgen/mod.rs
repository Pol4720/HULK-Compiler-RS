// Módulo principal para el generador de analizadores léxicos
pub mod generator;

// Aquí se pueden agregar utilidades y estructuras comunes para el generador
pub mod codegen;
pub mod dfa;
pub mod lexemes;
pub mod nfa;
pub mod simulation;
pub mod spec;

// Reexportar las funciones principales para el main
pub use simulation::simulator;
pub use simulation::visualizer;
pub use spec::token_spec::TokenSpec;
pub use spec::token_spec::read_token_spec;
