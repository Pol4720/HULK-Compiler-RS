use crate::nfa::nfa::NFA;
use crate::nfa::state::{State, StateId};

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

    /// Imprime la tabla de transición del NFA en formato tabular.
    pub fn print_transition_table(&self) {
        use std::collections::BTreeSet;

        println!("  Estado inicial: {:?}", self.start);
        println!("  Estados de aceptación: {:?}", self.accepts);

        // Recolectar todos los símbolos usados (excepto epsilon)
        let mut symbols = BTreeSet::new();
        for state in self.states.values() {
            for (input, _) in &state.transitions {
                if let Some(sym) = input {
                    symbols.insert(sym.clone());
                }
            }
        }
        // Encabezado
        print!("{:>8}", "  Estado  ");
        for sym in &symbols {
            print!(" | {:>12}", format!("{:?}", sym));
        }
        print!(" | {:>12}", "ε"); // Columna para epsilon
        println!();
        // Filas
        for (id, state) in &self.states {
            print!("{:>8}", format!("{:?}", id));
            for sym in &symbols {
                let targets = state.transitions.get(&Some(sym.clone()));
                if let Some(tgts) = targets {
                    let ids: Vec<String> = tgts.iter().map(|t| format!("{:?}", t)).collect();
                    print!(" | {:>12}", ids.join(","));
                } else {
                    print!(" | {:>12}", "-");
                }
            }
            // Mostrar transiciones epsilon
            let epsilon_targets = state.transitions.get(&None);
            if let Some(tgts) = epsilon_targets {
                let ids: Vec<String> = tgts.iter().map(|t| format!("{:?}", t)).collect();
                print!(" | {:>12}", ids.join(","));
            } else {
                print!(" | {:>12}", "-");
            }
            println!();
        }
    }
}
