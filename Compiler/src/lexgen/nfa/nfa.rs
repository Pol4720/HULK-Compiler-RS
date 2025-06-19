// Construcción de NFA a partir de expresiones regulares

use crate::nfa::state::{NFAFragment, State, StateId};
use crate::regex_parser::node::ast_node_impl::{AstNodeImpl, AstNodeKind};
use crate::regex_parser::node::bin_op::RegexBinOp;
use crate::regex_parser::node::regex_char::RegexChar;
use crate::regex_parser::node::regex_class::RegexClass;
use crate::regex_parser::node::un_op::RegexUnOp;
use std::collections::HashMap;

pub struct NFA {
    pub states: HashMap<StateId, State>,
    pub start: StateId,
    pub accepts: Vec<StateId>,
}

impl NFA {
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
}

struct NFABuilder {
    states: HashMap<StateId, State>,
    next_id: usize,
}

impl NFABuilder {
    fn new() -> Self {
        NFABuilder {
            states: HashMap::new(),
            next_id: 0,
        }
    }

    fn new_state(&mut self) -> StateId {
        let id = self.next_id;
        self.next_id += 1;
        let state = State::new(id);
        let sid = state.id.clone();
        self.states.insert(sid.clone(), state);
        sid
    }

    fn build(&mut self, node: &AstNodeImpl) -> NFAFragment {
        match &node.kind {
            AstNodeKind::RegexChar(c) => {
                // Un solo carácter (incluye epsilon, start, end, literal)
                let start = self.new_state();
                let accept = self.new_state();
                self.states
                    .get_mut(&start)
                    .unwrap()
                    .add_transition(Some(c.clone()), accept.clone());
                NFAFragment {
                    start,
                    accepts: vec![accept],
                }
            }
            AstNodeKind::BinOp { op, left, right } => match op {
                RegexBinOp::Concat => {
                    // Concatenación: conectar aceptaciones de left con inicio de right (epsilon)
                    let left_frag = self.build(left);
                    let right_frag = self.build(right);
                    for accept in &left_frag.accepts {
                        self.states
                            .get_mut(accept)
                            .unwrap()
                            .add_transition(None, right_frag.start.clone());
                    }
                    NFAFragment {
                        start: left_frag.start,
                        accepts: right_frag.accepts,
                    }
                }
                RegexBinOp::Or => {
                    // Alternancia: nuevo estado inicial, epsilon a ambos, unir aceptaciones
                    let left_frag = self.build(left);
                    let right_frag = self.build(right);
                    let start = self.new_state();
                    self.states
                        .get_mut(&start)
                        .unwrap()
                        .add_transition(None, left_frag.start.clone());
                    self.states
                        .get_mut(&start)
                        .unwrap()
                        .add_transition(None, right_frag.start.clone());
                    let mut accepts = left_frag.accepts;
                    accepts.extend(right_frag.accepts);
                    NFAFragment { start, accepts }
                }
            },
            AstNodeKind::UnOp { op, expr } => match op {
                RegexUnOp::Star => {
                    // Cero o más: nuevo inicio, epsilon a expr y a aceptaciones, aceptaciones epsilon a expr y a nuevo inicio
                    let frag = self.build(expr);
                    let start = self.new_state();
                    for accept in &frag.accepts {
                        self.states
                            .get_mut(accept)
                            .unwrap()
                            .add_transition(None, frag.start.clone());
                        self.states
                            .get_mut(accept)
                            .unwrap()
                            .add_transition(None, start.clone());
                    }
                    self.states
                        .get_mut(&start)
                        .unwrap()
                        .add_transition(None, frag.start.clone());
                    NFAFragment {
                        start: start.clone(),
                        accepts: vec![start],
                    }
                }
                RegexUnOp::Plus => {
                    // Una o más: como star pero sin epsilon directo al inicio
                    let frag = self.build(expr);
                    for accept in &frag.accepts {
                        self.states
                            .get_mut(accept)
                            .unwrap()
                            .add_transition(None, frag.start.clone());
                    }
                    NFAFragment {
                        start: frag.start.clone(),
                        accepts: frag.accepts,
                    }
                }
                RegexUnOp::Optional => {
                    // Cero o una: nuevo inicio, epsilon a expr y a aceptaciones
                    let frag = self.build(expr);
                    let start = self.new_state();
                    self.states
                        .get_mut(&start)
                        .unwrap()
                        .add_transition(None, frag.start.clone());
                    let mut accepts = frag.accepts.clone();
                    accepts.push(start.clone());
                    NFAFragment { start, accepts }
                }
            },
            AstNodeKind::Group(group) => self.build(&group.expr),
            AstNodeKind::Class(class) => {
                // Clase de caracteres: transición para cada carácter
                let start = self.new_state();
                let accept = self.new_state();
                match class {
                    RegexClass::Set(chars) => {
                        for c in chars {
                            self.states
                                .get_mut(&start)
                                .unwrap()
                                .add_transition(Some(c.clone()), accept.clone());
                        }
                    }
                    RegexClass::Range(a, b) => {
                        for ch in (*a as u8)..=(*b as u8) {
                            self.states.get_mut(&start).unwrap().add_transition(
                                Some(RegexChar::Literal(ch as char)),
                                accept.clone(),
                            );
                        }
                    }
                    RegexClass::Negated(_) => {
                        // No implementado: se puede extender
                    }
                    RegexClass::Dot => {
                        // Cualquier carácter excepto salto de línea
                        for ch in 0u8..=255u8 {
                            let c = ch as char;
                            if c != '\n' {
                                self.states
                                    .get_mut(&start)
                                    .unwrap()
                                    .add_transition(Some(RegexChar::Literal(c)), accept.clone());
                            }
                        }
                    }
                }
                NFAFragment {
                    start,
                    accepts: vec![accept],
                }
            }
        }
    }
}
