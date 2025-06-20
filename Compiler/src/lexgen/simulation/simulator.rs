use crate::nfa::nfa::NFA;
use crate::nfa::state::{State, StateId};
use crate::regex_parser::node::regex_char::RegexChar;
use std::collections::{HashSet, VecDeque};

impl NFA {
    /// Verifica si la cadena de entrada es aceptada por el NFA (simulación completa con épsilon).
    pub fn accepts(&self, input: &str) -> bool {
        let chars: Vec<char> = input.chars().collect();
        let mut current_states = epsilon_closure(&[self.start.clone()], &self.states);
        for &c in &chars {
            let mut next_states = HashSet::new();
            for state_id in &current_states {
                if let Some(state) = self.states.get(state_id) {
                    if let Some(targets) = state.transitions.get(&Some(RegexChar::Literal(c))) {
                        for t in targets {
                            next_states.extend(epsilon_closure(&[t.clone()], &self.states));
                        }
                    }
                }
            }
            current_states = next_states;
            if current_states.is_empty() {
                return false;
            }
        }
        // ¿Algún estado actual es de aceptación?
        current_states.iter().any(|s| self.accepts.contains(s))
    }
}

/// Calcula la épsilon-clausura de un conjunto de estados
fn epsilon_closure(
    states: &[StateId],
    all_states: &std::collections::HashMap<StateId, State>,
) -> HashSet<StateId> {
    let mut closure: HashSet<StateId> = states.iter().cloned().collect();
    let mut stack: VecDeque<StateId> = states.iter().cloned().collect();
    while let Some(state_id) = stack.pop_front() {
        if let Some(state) = all_states.get(&state_id) {
            if let Some(targets) = state.transitions.get(&None) {
                for t in targets {
                    if closure.insert(t.clone()) {
                        stack.push_back(t.clone());
                    }
                }
            }
        }
    }
    closure
}
