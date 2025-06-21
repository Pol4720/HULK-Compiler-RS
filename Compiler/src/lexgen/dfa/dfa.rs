// ===============================
// dfa.rs: Conversión de NFA a DFA y estructura DFA
// ===============================

use crate::nfa::join_nfa::{AcceptInfo, JoinedNFA};
use crate::nfa::state::StateId;
use crate::regex_parser::node::regex_char::RegexChar;
use std::collections::{BTreeSet, HashMap, VecDeque};

/// Estado del DFA: representa un subconjunto de estados del NFA
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DFAState {
    /// El subconjunto de estados del NFA que representa este estado del DFA
    pub nfa_subset: BTreeSet<StateId>,
    /// Si es de aceptación, información del token y prioridad
    pub accept: Option<AcceptInfo>,
}

/// DFA resultante de la conversión desde un NFA
pub struct DFA {
    pub states: HashMap<String, DFAState>, // key: subset string
    pub transitions: HashMap<(String, RegexChar), String>, // (from, symbol) -> to
    pub start: String,
}

impl DFA {
    /// Construye un DFA a partir de un JoinedNFA usando el algoritmo de subconjuntos
    pub fn from_joined_nfa(nfa: &JoinedNFA) -> Self {
        use super::utils::{epsilon_closure, move_on_symbol, subset_to_string};
        let mut states: HashMap<String, DFAState> = HashMap::new();
        let mut transitions: HashMap<(String, RegexChar), String> = HashMap::new();
        let mut queue = VecDeque::new();
        let mut alphabet = BTreeSet::new();
        // Construir el alfabeto (todos los símbolos excepto épsilon)
        for state in nfa.states.values() {
            for (input, _) in &state.transitions {
                if let Some(sym) = input {
                    alphabet.insert(sym.clone());
                }
            }
        }
        // Estado inicial DFA = épsilon-clausura del inicial del NFA
        let mut start_set = BTreeSet::new();
        start_set.insert(nfa.start.clone());
        let start_closure = epsilon_closure(&start_set, &nfa.states);
        let start_str = subset_to_string(&start_closure);
        let start_accept = dfa_accept_info(&start_closure, &nfa.accepts);
        let start_state = DFAState {
            nfa_subset: start_closure.clone(),
            accept: start_accept,
        };
        states.insert(start_str.clone(), start_state.clone());
        queue.push_back(start_state);
        // Algoritmo principal
        while let Some(current) = queue.pop_front() {
            let current_str = subset_to_string(&current.nfa_subset);
            for symbol in &alphabet {
                let move_set = move_on_symbol(&current.nfa_subset, symbol, &nfa.states);
                if move_set.is_empty() {
                    continue;
                }
                let closure = epsilon_closure(&move_set, &nfa.states);
                let closure_str = subset_to_string(&closure);
                if !states.contains_key(&closure_str) {
                    let accept = dfa_accept_info(&closure, &nfa.accepts);
                    let new_state = DFAState {
                        nfa_subset: closure.clone(),
                        accept,
                    };
                    states.insert(closure_str.clone(), new_state.clone());
                    queue.push_back(new_state);
                }
                transitions.insert((current_str.clone(), symbol.clone()), closure_str.clone());
            }
        }
        DFA {
            states,
            transitions,
            start: start_str,
        }
    }

    /// Imprime el DFA de forma detallada (estados y transiciones).
    pub fn imprimir(&self) {
        println!("\nDFA resultante:");
        println!("Estado inicial: {}", self.start);
        println!("Estados DFA:");
        for (name, state) in &self.states {
            let accept_str = match &state.accept {
                Some(info) => format!(
                    " (aceptación: {}, prioridad: {})",
                    info.token_type, info.priority
                ),
                None => String::new(),
            };
            println!("  {}{}", name, accept_str);
        }
        println!("Transiciones DFA:");
        for ((from, symbol), to) in &self.transitions {
            println!("  {} --{:?}--> {}", from, symbol, to);
        }
    }
}

/// Determina la aceptación y prioridad para un subconjunto de estados NFA
fn dfa_accept_info(
    subset: &BTreeSet<StateId>,
    accepts: &HashMap<StateId, AcceptInfo>,
) -> Option<AcceptInfo> {
    // Si hay varios, elige el de menor prioridad
    let mut best: Option<&AcceptInfo> = None;
    for sid in subset {
        if let Some(info) = accepts.get(sid) {
            match &best {
                None => best = Some(info),
                Some(b) if info.priority < b.priority => best = Some(info),
                _ => {}
            }
        }
    }
    best.cloned()
}
