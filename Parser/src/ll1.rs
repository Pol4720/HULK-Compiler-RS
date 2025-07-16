use std::collections::{HashMap, HashSet};
use crate::grammar::Production;
use crate::utils::is_terminal;

pub type FirstSets = HashMap<String, HashSet<String>>;
pub type FollowSets = HashMap<String, HashSet<String>>;
pub type LL1Table = HashMap<String, HashMap<String, Vec<String>>>;

pub fn compute_first(productions: &[Production]) -> FirstSets {
    let mut first: FirstSets = HashMap::new();

    // Collect all symbols (terminals and non-terminals)
    let mut all_symbols = HashSet::new();
    for prod in productions {
        all_symbols.insert(prod.lhs.clone());
        for sym in &prod.rhs {
            all_symbols.insert(sym.clone());
        }
    }

    // Initialize FIRST sets: For terminals, FIRST(terminal) = {terminal}
    for symbol in &all_symbols {
        if is_terminal(symbol, productions) {
            let mut terminal_set = HashSet::new();
            terminal_set.insert(symbol.clone());
            first.insert(symbol.clone(), terminal_set);
        } else {
            first.insert(symbol.clone(), HashSet::new());
        }
    }

    let mut changed = true;
    while changed {
        changed = false;

        for prod in productions {
            let lhs = &prod.lhs;
            let rhs = &prod.rhs;

            // If A -> ε, add ε to FIRST(A)
            if rhs.is_empty() {
                if !first[lhs].contains("ε") {
                    first.get_mut(lhs).unwrap().insert("ε".to_string());
                    changed = true;
                }
                continue;
            }

            // For A -> X1 X2 ... Xn
            let mut all_have_epsilon = true;
            for i in 0..rhs.len() {
                let x = &rhs[i];
                
                // Collect symbols to add first to avoid borrowing conflict
                let mut symbols_to_add = Vec::new();
                for symbol in &first[x] {
                    if symbol != "ε" {
                        symbols_to_add.push(symbol.clone());
                    }
                }
                
                // Add FIRST(X) - {ε} to FIRST(A)
                let old_size = first[lhs].len();
                for symbol in symbols_to_add {
                    first.get_mut(lhs).unwrap().insert(symbol);
                }
                if first[lhs].len() > old_size {
                    changed = true;
                }

                // If X doesn't have ε, stop
                if !first[x].contains("ε") {
                    all_have_epsilon = false;
                    break;
                }
            }

            // If all Xi have ε, add ε to FIRST(A)
            if all_have_epsilon && !first[lhs].contains("ε") {
                first.get_mut(lhs).unwrap().insert("ε".to_string());
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

    // Initialize FOLLOW sets for all non-terminals
    let mut non_terminals = HashSet::new();
    for prod in productions {
        non_terminals.insert(prod.lhs.clone());
        for symbol in &prod.rhs {
            if !is_terminal(symbol, productions) {
                non_terminals.insert(symbol.clone());
            }
        }
    }

    for nt in &non_terminals {
        follow.insert(nt.clone(), HashSet::new());
    }

    // Add $ (EOF) to FOLLOW(start_symbol)
    follow.get_mut(start_symbol).unwrap().insert("$".to_string());

    let mut changed = true;
    while changed {
        changed = false;

        for prod in productions {
            let lhs = &prod.lhs;
            let rhs = &prod.rhs;

            for i in 0..rhs.len() {
                let b = &rhs[i];

                // Only process non-terminals
                if is_terminal(b, productions) {
                    continue;
                }

                // For A -> αBβ
                let beta = &rhs[i + 1..];

                if beta.is_empty() {
                    // B is at the end: FOLLOW(A) ⊆ FOLLOW(B)
                    let old_size = follow[b].len();
                    let follow_lhs = follow[lhs].clone();
                    follow.get_mut(b).unwrap().extend(follow_lhs);
                    if follow[b].len() > old_size {
                        changed = true;
                    }
                } else {
                    // Calculate FIRST(β)
                    let first_beta = compute_first_of_sequence(beta, first, productions);

                    // FIRST(β) - {ε} ⊆ FOLLOW(B)
                    let old_size = follow[b].len();
                    for symbol in &first_beta {
                        if symbol != "ε" {
                            follow.get_mut(b).unwrap().insert(symbol.clone());
                        }
                    }
                    if follow[b].len() > old_size {
                        changed = true;
                    }

                    // If ε ∈ FIRST(β), then FOLLOW(A) ⊆ FOLLOW(B)
                    if first_beta.contains("ε") {
                        let old_size = follow[b].len();
                        let follow_lhs = follow[lhs].clone();
                        follow.get_mut(b).unwrap().extend(follow_lhs);
                        if follow[b].len() > old_size {
                            changed = true;
                        }
                    }
                }
            }
        }
    }

    // Special correction for recursive lists like in the C++ implementation
    if follow.contains_key("StmtList") && first.contains_key("TerminatedStmt") {
        let old_size = follow["StmtList"].len();
        for symbol in &first["TerminatedStmt"] {
            if symbol != "ε" {
                follow.get_mut("StmtList").unwrap().insert(symbol.clone());
            }
        }
        if follow["StmtList"].len() > old_size {
            println!("Debug: Added {} symbols to FOLLOW(StmtList)",
                     follow["StmtList"].len() - old_size);
        }
    }

    follow
}

pub fn build_ll1_table(
    productions: &[Production],
    first: &FirstSets,
    follow: &FollowSets,
) -> LL1Table {
    let mut table: LL1Table = HashMap::new();

    // Step 1: Process all non-epsilon productions first
    for prod in productions {
        let lhs = &prod.lhs;
        let rhs = &prod.rhs;

        // Skip epsilon productions in this step
        if rhs.is_empty() {
            continue;
        }

        // Calculate FIRST(α)
        let first_alpha = compute_first_of_sequence(rhs, first, productions);

        // Rule 1: For each terminal a in FIRST(α), add A → α to M[A, a]
        for a in &first_alpha {
            if a != "ε" {
                let terminal = if a == "EOF" { "$".to_string() } else { a.clone() };

                // Check for conflicts
                if table.contains_key(lhs) && table[lhs].contains_key(&terminal) {
                    eprintln!("LL(1) conflict: M[{}, {}] already has an entry", lhs, terminal);
                } else {
                    table.entry(lhs.clone())
                         .or_default()
                         .insert(terminal, rhs.clone());
                }
            }
        }
    }

    // Step 2: Process epsilon productions only where there's no conflict
    for prod in productions {
        let lhs = &prod.lhs;
        let rhs = &prod.rhs;

        // Calculate FIRST(α)
        let first_alpha = compute_first_of_sequence(rhs, first, productions);

        // Rule 2: If ε ∈ FIRST(α), then for each b ∈ FOLLOW(A), add A → α to M[A, b]
        if first_alpha.contains("ε") {
            if follow.contains_key(lhs) {
                for b in &follow[lhs] {
                    let terminal = if b == "EOF" { "$".to_string() } else { b.clone() };

                    // Only add epsilon if there's no non-epsilon production
                    if !table.contains_key(lhs) || !table[lhs].contains_key(&terminal) {
                        table.entry(lhs.clone())
                             .or_default()
                             .insert(terminal, rhs.clone());
                    }
                    // If an entry already exists, don't overwrite (priority to non-epsilon)
                }
            }
        }
    }

    table
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
