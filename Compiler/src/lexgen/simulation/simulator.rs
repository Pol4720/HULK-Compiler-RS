// ===============================
// simulator.rs: Simulación de NFA (con épsilon)
// ===============================

use crate::nfa::nfa::NFA;
use crate::nfa::state::{State, StateId};
use crate::regex_parser::node::regex_char::RegexChar;
use std::collections::{HashSet, VecDeque};

impl NFA {
    /// Verifica si la cadena de entrada es aceptada por el NFA (simulación completa con épsilon).
    pub fn accepts(&self, input: &str) -> bool {
        let chars: Vec<char> = input.chars().collect();
        // 1. Epsilon-clausura desde el estado inicial
        let mut current_states = epsilon_closure(&[self.start.clone()], &self.states);
        // --- Manejo de RegexChar::Start ---
        let mut after_start = HashSet::new();
        for state_id in &current_states {
            if let Some(state) = self.states.get(state_id) {
                if let Some(targets) = state.transitions.get(&Some(RegexChar::Start)) {
                    after_start.extend(targets.iter().cloned());
                }
            }
        }
        if !after_start.is_empty() {
            let mut closure = epsilon_closure(
                &after_start.iter().cloned().collect::<Vec<_>>(),
                &self.states,
            );
            current_states.extend(closure.drain());
        }
        // --- Fin manejo Start ---
        for &c in &chars {
            let mut next_states = HashSet::new();
            for state_id in &current_states {
                if let Some(state) = self.states.get(state_id) {
                    // Transiciones literales específicas
                    if let Some(targets) = state.transitions.get(&Some(RegexChar::Literal(c))) {
                        next_states.extend(targets.iter().cloned());
                    }
                    // Transiciones "cualquier carácter" (.)
                    if let Some(targets) = state.transitions.get(&Some(RegexChar::Any)) {
                        next_states.extend(targets.iter().cloned());
                    }
                    // Transiciones de escape que podrían coincidir
                    for (transition_char, targets) in &state.transitions {
                        if let Some(RegexChar::Escape(escape)) = transition_char {
                            if let Some(literal_char) = escape.as_char() {
                                if literal_char == c {
                                    next_states.extend(targets.iter().cloned());
                                }
                            }
                        }
                    }
                }
            }
            current_states = epsilon_closure(
                &next_states.iter().cloned().collect::<Vec<_>>(),
                &self.states,
            );
            if current_states.is_empty() {
                return false;
            }
        }
        // 3. épsilon-clausura final (opcional, pero seguro)
        current_states = epsilon_closure(
            &current_states.iter().cloned().collect::<Vec<_>>(),
            &self.states,
        );
        // --- Manejo de RegexChar::End ---
        let mut end_states = current_states.clone();
        for state_id in &current_states {
            if let Some(state) = self.states.get(state_id) {
                if let Some(targets) = state.transitions.get(&Some(RegexChar::End)) {
                    end_states.extend(targets.iter().cloned());
                }
            }
        }
        current_states = epsilon_closure(
            &end_states.iter().cloned().collect::<Vec<_>>(),
            &self.states,
        );
        // --- Fin manejo End ---
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
