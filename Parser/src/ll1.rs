use std::collections::{HashMap, HashSet};
use crate::grammar::Production;
use crate::utils::is_terminal;

pub type FirstSets = HashMap<String, HashSet<String>>;
pub type FollowSets = HashMap<String, HashSet<String>>;
pub type LL1Table = HashMap<String, HashMap<String, Vec<String>>>;

pub fn compute_first(productions: &[Production]) -> FirstSets {
    let mut first: FirstSets = HashMap::new();

    // Initialize FIRST sets
    for prod in productions {
        first.entry(prod.lhs.clone()).or_default();
        for sym in &prod.rhs {
            first.entry(sym.clone()).or_default();
        }
    }

    let mut changed = true;
    while changed {
        changed = false;

        for prod in productions {
            let lhs = &prod.lhs;
            let rhs = &prod.rhs;

            let mut can_be_empty = true;
            let mut tokens_to_add = Vec::new();
            
            for sym in rhs {
                let empty_set = HashSet::new();
                let sym_first = first.get(sym).unwrap_or(&empty_set);

                for t in sym_first {
                    if t != "ε" && !first[lhs].contains(t) {
                        tokens_to_add.push(t.clone());
                    }
                }

                if !sym_first.contains("ε") {
                    can_be_empty = false;
                    break;
                }
            }

            // Apply collected changes
            let old_size = first[lhs].len();
            for token in tokens_to_add {
                first.get_mut(lhs).unwrap().insert(token);
            }
            
            if can_be_empty {
                first.get_mut(lhs).unwrap().insert("ε".to_string());
            }
            
            if first[lhs].len() > old_size {
                changed = true;
            }
        }
    }

    first
}

pub fn compute_follow(
    productions: &[Production],
    first: &FirstSets,
    start_symbol: &str,
) -> FollowSets {
    let mut follow: FollowSets = HashMap::new();

    // Initialize FOLLOW sets for non-terminals
    for prod in productions {
        follow.entry(prod.lhs.clone()).or_default();
    }

    // Add EOF marker to FOLLOW(start symbol)
    follow.get_mut(start_symbol).unwrap().insert("EOF".to_string());

    let mut changed = true;
    while changed {
        changed = false;

        for prod in productions {
            let lhs = &prod.lhs;
            let rhs = &prod.rhs;

            for i in 0..rhs.len() {
                let B = &rhs[i];
                if is_terminal(B, productions) {
                    continue;
                }

                // β = rest of RHS after B
                let beta = &rhs[i + 1..];

                let mut first_beta = compute_first_of_sequence(beta, first, productions);
                let before = follow[B].len();

                // Add FIRST(β) - {ε} to FOLLOW(B)
                first_beta.remove("ε");
                follow.get_mut(B).unwrap().extend(first_beta);

                // If ε ∈ FIRST(β), add FOLLOW(lhs) to FOLLOW(B)
                if compute_first_of_sequence(beta, first, productions).contains("ε") {
                    let follow_lhs = follow[lhs].clone();
                    follow.get_mut(B).unwrap().extend(follow_lhs);
                }

                if follow[B].len() > before {
                    changed = true;
                }
            }
        }
    }

    follow
}

fn compute_first_of_sequence(
    symbols: &[String],
    first: &FirstSets,
    _productions: &[Production],
) -> HashSet<String> {
    let mut result = HashSet::new();
    if symbols.is_empty() {
        result.insert("ε".to_string());
        return result;
    }

    for symbol in symbols {
        let empty_set = HashSet::new();
        let sym_first = first.get(symbol).unwrap_or(&empty_set);
        result.extend(sym_first.iter().filter(|&&ref s| s != "ε").cloned());

        if !sym_first.contains("ε") {
            return result;
        }
    }

    result.insert("ε".to_string());
    result
}

pub fn build_ll1_table(
    productions: &[Production],
    first: &FirstSets,
    follow: &FollowSets,
) -> LL1Table {
    let mut table: LL1Table = HashMap::new();

    for prod in productions {
        let lhs = &prod.lhs;
        let rhs = &prod.rhs;

        let first_set = compute_first_of_sequence(rhs, first, productions);

        for terminal in &first_set {
            if terminal == "ε" {
                // Add FOLLOW(lhs) to table[lhs]
                let empty_set = HashSet::new();
                for t in follow.get(lhs).unwrap_or(&empty_set) {
                    table
                        .entry(lhs.clone())
                        .or_default()
                        .insert(t.clone(), rhs.clone());
                }
            } else {
                table
                    .entry(lhs.clone())
                    .or_default()
                    .insert(terminal.clone(), rhs.clone());
            }
        }
    }

    table
}
