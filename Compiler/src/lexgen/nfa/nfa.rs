// ===============================
// nfa.rs: Definición y lógica principal del NFA
// ===============================

use super::builder::builder::NFABuilder;
use crate::nfa::state::{State, StateId};
use crate::regex_parser::node::ast_node_impl::AstNodeImpl;
use std::collections::HashMap;

/// Representa un autómata finito no determinista (NFA).
pub struct NFA {
    pub states: HashMap<StateId, State>,
    pub start: StateId,
    pub accepts: Vec<StateId>,
}

impl NFA {
    /// Construye un NFA a partir de un AST de expresión regular.
    pub fn from_ast(ast: &AstNodeImpl) -> Self {
        let mut builder = NFABuilder::new();
        let frag = builder.build(ast);
        let mut nfa = NFA {
            states: builder.states,
            start: frag.start.clone(),
            accepts: frag.accepts.clone(),
        };
        // Marcar estados finales
        for accept in &nfa.accepts {
            if let Some(state) = nfa.states.get_mut(accept) {
                state.is_final = true;
            }
        }
        nfa
    }

    /// Devuelve una representación en string del NFA (estados, transiciones y finales).
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("Inicio: {:?}\n", self.start));
        s.push_str(&format!("Finales: {:?}\n", self.accepts));
        for (id, state) in &self.states {
            for (input, targets) in &state.transitions {
                let label = match input {
                    Some(c) => format!("{:?}", c),
                    None => "ε".to_string(),
                };
                for t in targets {
                    s.push_str(&format!("  {:?} --{}--> {:?}\n", id, label, t));
                }
            }
        }
        s
    }
}
