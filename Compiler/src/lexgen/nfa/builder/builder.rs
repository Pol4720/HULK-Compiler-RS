// ===============================
// builder.rs: Lógica interna de construcción de NFA
// ===============================

use crate::nfa::state::{NFAFragment, State, StateId};
use crate::regex_parser::node::alphabet::ALPHABET;
use crate::regex_parser::node::ast_node_impl::{AstNodeImpl, AstNodeKind};
use crate::regex_parser::node::bin_op::RegexBinOp;
use crate::regex_parser::node::regex_char::RegexChar;
use crate::regex_parser::node::regex_class::RegexClass;
use crate::regex_parser::node::un_op::RegexUnOp;
use std::collections::HashMap;

/// Builder para construir un NFA a partir de un AST de regex.
pub struct NFABuilder {
    pub states: HashMap<StateId, State>,
    next_id: usize,
}

impl NFABuilder {
    pub fn new() -> Self {
        NFABuilder {
            states: HashMap::new(),
            next_id: 0,
        }
    }

    /// Crea un nuevo estado y lo agrega al builder.
    pub fn new_state(&mut self) -> StateId {
        let id = self.next_id;
        self.next_id += 1;
        let state = State::new(id);
        let sid = state.id.clone();
        self.states.insert(sid.clone(), state);
        sid
    }

    /// Construye recursivamente el fragmento NFA para un nodo AST.
    pub fn build(&mut self, node: &AstNodeImpl) -> NFAFragment {
        match &node.kind {
            AstNodeKind::RegexChar(c) => {
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
                    // Caso ^...$
                    if let AstNodeKind::RegexChar(RegexChar::Start) = left.kind {
                        if let AstNodeKind::BinOp {
                            op: RegexBinOp::Concat,
                            left: inner_left,
                            right: inner_right,
                        } = &right.kind
                        {
                            if let AstNodeKind::RegexChar(RegexChar::End) = inner_right.kind {
                                // ^...$
                                let inner_frag = self.build(inner_left);
                                let start = self.new_state();
                                let end = self.new_state();
                                self.states.get_mut(&start).unwrap().add_transition(
                                    Some(RegexChar::Start),
                                    inner_frag.start.clone(),
                                );
                                for accept in &inner_frag.accepts {
                                    self.states
                                        .get_mut(accept)
                                        .unwrap()
                                        .add_transition(Some(RegexChar::End), end.clone());
                                }
                                return NFAFragment {
                                    start,
                                    accepts: vec![end],
                                };
                            }
                        }
                        // Solo ^...
                        let right_frag = self.build(right);
                        let dot_start = self.new_state();
                        let dot_accept = self.new_state();
                        // Nodo Dot después de la expresión
                        for &c in ALPHABET {
                            self.states
                                .get_mut(&dot_start)
                                .unwrap()
                                .add_transition(Some(RegexChar::Literal(c)), dot_accept.clone());
                        }
                        // Bucle en dot_accept para todos los caracteres del alfabeto
                        for &c in ALPHABET {
                            self.states
                                .get_mut(&dot_accept)
                                .unwrap()
                                .add_transition(Some(RegexChar::Literal(c)), dot_accept.clone());
                        }
                        // Transición de Start a expresión
                        let start = self.new_state();
                        self.states
                            .get_mut(&start)
                            .unwrap()
                            .add_transition(Some(RegexChar::Start), right_frag.start.clone());
                        // Transición de la expresión al nodo Dot
                        for accept in &right_frag.accepts {
                            self.states
                                .get_mut(accept)
                                .unwrap()
                                .add_transition(None, dot_start.clone());
                        }
                        // Acepta tanto si termina en la expresión como si pasa por el dot
                        let mut accepts = right_frag.accepts.clone();
                        accepts.push(dot_accept);
                        return NFAFragment { start, accepts };
                    } else if let AstNodeKind::RegexChar(RegexChar::End) = right.kind {
                        // ...$
                        let left_frag = self.build(left);
                        let dot_start = self.new_state();
                        let dot_accept = self.new_state();
                        // Nodo Dot antes de la expresión
                        for &c in ALPHABET {
                            self.states
                                .get_mut(&dot_start)
                                .unwrap()
                                .add_transition(Some(RegexChar::Literal(c)), dot_accept.clone());
                        }
                        // Transición de Dot a expresión
                        self.states
                            .get_mut(&dot_accept)
                            .unwrap()
                            .add_transition(None, left_frag.start.clone());
                        // Transición de la expresión a End
                        let end = self.new_state();
                        for accept in &left_frag.accepts {
                            self.states
                                .get_mut(accept)
                                .unwrap()
                                .add_transition(Some(RegexChar::End), end.clone());
                            // Desde cualquier nodo de la expresión, si no coincide, volver al nodo Dot
                            self.states
                                .get_mut(accept)
                                .unwrap()
                                .add_transition(None, dot_start.clone());
                        }
                        return NFAFragment {
                            start: dot_start,
                            accepts: vec![end],
                        };
                    } else {
                        // Lógica normal de concatenación
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
                }
                RegexBinOp::Or => {
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
                        // No implementado
                    }
                    RegexClass::Dot => {
                        for &c in ALPHABET {
                            self.states
                                .get_mut(&start)
                                .unwrap()
                                .add_transition(Some(RegexChar::Literal(c)), accept.clone());
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
