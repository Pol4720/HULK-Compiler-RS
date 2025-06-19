use crate::regex_parser::node::regex_char::RegexChar;
use std::collections::HashMap;

// Definición de estados y transiciones del NFA

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StateId(pub usize);

#[derive(Debug, Clone)]
pub struct State {
    pub id: StateId,
    pub transitions: HashMap<Option<RegexChar>, Vec<StateId>>, // None = epsilon
    pub is_final: bool,
}

impl State {
    pub fn new(id: usize) -> Self {
        State {
            id: StateId(id),
            transitions: HashMap::new(),
            is_final: false,
        }
    }
    pub fn add_transition(&mut self, input: Option<RegexChar>, target: StateId) {
        self.transitions.entry(input).or_default().push(target);
    }
}

#[derive(Debug, Clone)]
pub struct NFAFragment {
    pub start: StateId,
    pub accepts: Vec<StateId>,
}
