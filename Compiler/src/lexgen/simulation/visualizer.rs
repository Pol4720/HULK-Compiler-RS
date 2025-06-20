use crate::nfa::nfa::NFA;
use crate::nfa::state::{State, StateId};
use crate::regex_parser::node::regex_char::RegexChar;

impl NFA {
    /// Imprime la estructura del NFA: estados, transiciones, inicial y finales.
    pub fn visualize(&self) {
        println!("NFA Visualization:");
        println!("  Estado inicial: {:?}", self.start);
        println!("  Estados de aceptación: {:?}", self.accepts);
        println!("  Transiciones:");
        for (id, state) in &self.states {
            for (input, targets) in &state.transitions {
                let symbol = match input {
                    Some(sym) => format!("{:?}", sym),
                    None => "ε".to_string(),
                };
                println!("    {:?} --{}--> {:?}", id, symbol, targets);
            }
        }
    }
}
