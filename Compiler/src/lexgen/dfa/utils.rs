use crate::nfa::state::StateId;
use std::collections::{BTreeSet, HashSet};

/// Convierte un conjunto de StateId a una cadena única (útil para identificar estados DFA)
pub fn subset_to_string(subset: &BTreeSet<StateId>) -> String {
    let mut ids: Vec<_> = subset.iter().map(|id| id.0).collect();
    ids.sort();
    format!(
        "{{{}}}",
        ids.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    )
}

/// Convierte un vector de StateId a un BTreeSet<StateId>
pub fn vec_to_subset(vec: Vec<StateId>) -> BTreeSet<StateId> {
    vec.into_iter().collect()
}

/// Devuelve el subconjunto alcanzable por épsilon-clausura desde un conjunto de estados
pub fn epsilon_closure(
    states: &BTreeSet<StateId>,
    nfa_states: &std::collections::HashMap<StateId, crate::nfa::state::State>,
) -> BTreeSet<StateId> {
    let mut closure = states.clone();
    let mut stack: Vec<_> = states.iter().cloned().collect();
    while let Some(state_id) = stack.pop() {
        if let Some(state) = nfa_states.get(&state_id) {
            if let Some(targets) = state.transitions.get(&None) {
                for t in targets {
                    if closure.insert(t.clone()) {
                        stack.push(t.clone());
                    }
                }
            }
        }
    }
    closure
}

/// Devuelve el conjunto de estados alcanzables desde un conjunto dado por un símbolo
pub fn move_on_symbol(
    states: &BTreeSet<StateId>,
    symbol: &crate::regex_parser::node::regex_char::RegexChar,
    nfa_states: &std::collections::HashMap<StateId, crate::nfa::state::State>,
) -> BTreeSet<StateId> {
    let mut result = BTreeSet::new();
    for state_id in states {
        if let Some(state) = nfa_states.get(state_id) {
            if let Some(targets) = state.transitions.get(&Some(symbol.clone())) {
                for t in targets {
                    result.insert(t.clone());
                }
            }
        }
    }
    result
}
