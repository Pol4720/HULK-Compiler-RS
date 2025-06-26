//! Combina múltiples NFAs en un solo NFA con etiquetado de aceptación y prioridad.

use super::nfa::NFA;
use super::state::{State, StateId};
use std::collections::HashMap;

/// Información de aceptación para un estado final
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AcceptInfo {
    pub token_type: String,
    pub priority: usize,
}

/// NFA combinado con información de aceptación
pub struct JoinedNFA {
    pub states: HashMap<StateId, State>,
    pub start: StateId,
    /// Mapea los estados de aceptación a su información de token y prioridad
    pub accepts: HashMap<StateId, AcceptInfo>,
}

impl JoinedNFA {
    /// Combina varios NFAs en uno solo.
    /// - `nfas`: tuplas de (NFA, tipo_token, prioridad)
    pub fn join(nfas: Vec<(NFA, String, usize)>) -> Self {
        let mut states = HashMap::new();
        let mut accepts = HashMap::new();
        let mut id_offset = 1; // Reservamos 0 para el nuevo estado inicial
        let mut start_targets = Vec::new();

        // Mapeo de ids antiguos a nuevos para evitar colisiones
        for (nfa, token_type, priority) in nfas {
            let mut id_map = HashMap::new();
            // Reasignar ids de estados
            for old_id in nfa.states.keys() {
                let new_id = StateId(id_offset);
                id_map.insert(old_id.clone(), new_id.clone());
                id_offset += 1;
            }
            // Copiar estados con nuevos ids
            for (old_id, state) in nfa.states {
                let mut new_state = State {
                    id: id_map[&old_id].clone(),
                    transitions: HashMap::new(),
                    is_final: state.is_final,
                };
                // Remapear transiciones
                for (input, targets) in state.transitions {
                    let new_targets = targets.into_iter().map(|t| id_map[&t].clone()).collect();
                    new_state.transitions.insert(input, new_targets);
                }
                // Si es estado inicial, lo conectamos desde el nuevo start
                if old_id == nfa.start {
                    start_targets.push(new_state.id.clone());
                }
                // Si es de aceptación, lo etiquetamos
                if nfa.accepts.contains(&old_id) {
                    accepts.insert(
                        new_state.id.clone(),
                        AcceptInfo {
                            token_type: token_type.clone(),
                            priority,
                        },
                    );
                }
                states.insert(new_state.id.clone(), new_state);
            }
        }
        // Crear el nuevo estado inicial (id 0)
        let mut start_state = State::new(0);
        for target in start_targets {
            start_state.add_transition(None, target); // None = épsilon
        }
        states.insert(start_state.id.clone(), start_state);
        JoinedNFA {
            states,
            start: StateId(0),
            accepts,
        }
    }
    /// Devuelve una representación en string del NFA combinado (estados, transiciones y aceptaciones)
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("Inicio: {:?}\n", self.start));
        s.push_str("Estados de aceptación y etiquetas:\n");
        for (id, info) in &self.accepts {
            s.push_str(&format!(
                "  Estado {:?}: token='{}', prioridad={}\n",
                id, info.token_type, info.priority
            ));
        }
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

// ---
// Ejemplo de uso:
// let joined = JoinedNFA::join(vec![ (nfa1, "ID".to_string(), 1), (nfa2, "NUM".to_string(), 2) ]);
// let descripcion = joined.to_string();
// println!("{}", descripcion);
// ---
