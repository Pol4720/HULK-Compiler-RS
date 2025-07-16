use crate::Parser::token::TokenType;
use crate::Parser::grammar::{Grammar, Production, Symbol};
use std::collections::{HashMap, HashSet};

pub struct LL1Table {
    table: HashMap<(String, TokenType), Production>,
    first_sets: HashMap<String, HashSet<TokenType>>,
    follow_sets: HashMap<String, HashSet<TokenType>>,
}

impl LL1Table {
    pub fn new(grammar: &Grammar) -> Self {
        let mut table = Self {
            table: HashMap::new(),
            first_sets: HashMap::new(),
            follow_sets: HashMap::new(),
        };
        
        table.compute_first_sets(grammar);
        table.compute_follow_sets(grammar);
        table.build_table(grammar);
        
        table
    }
    
    pub fn get_production(&self, non_terminal: &str, token: &TokenType) -> Option<&Production> {
        self.table.get(&(non_terminal.to_string(), token.clone()))
    }
    
    fn compute_first_sets(&mut self, grammar: &Grammar) {
        let non_terminals = grammar.get_non_terminals();
        
        // Initialize FIRST sets
        for nt in &non_terminals {
            self.first_sets.insert(nt.clone(), HashSet::new());
        }
        
        let mut changed = true;
        while changed {
            changed = false;
            
            for production in &grammar.productions {
                let old_size = self.first_sets[&production.lhs].len();
                let first_of_rhs = self.first_of_symbols(&production.rhs, grammar);
                
                self.first_sets.get_mut(&production.lhs).unwrap().extend(first_of_rhs);
                
                if self.first_sets[&production.lhs].len() > old_size {
                    changed = true;
                }
            }
        }
    }
    
    fn compute_follow_sets(&mut self, grammar: &Grammar) {
        let non_terminals = grammar.get_non_terminals();
        
        // Initialize FOLLOW sets
        for nt in &non_terminals {
            self.follow_sets.insert(nt.clone(), HashSet::new());
        }
        
        // Add EOF to FOLLOW of start symbol
        self.follow_sets.get_mut(&grammar.start_symbol).unwrap().insert(TokenType::Eof);
        
        let mut changed = true;
        while changed {
            changed = false;
            
            for production in &grammar.productions {
                for (i, symbol) in production.rhs.iter().enumerate() {
                    if let Symbol::NonTerminal(nt) = symbol {
                        let old_size = self.follow_sets[nt].len();
                        
                        // FIRST of beta
                        let beta = &production.rhs[i + 1..];
                        let first_of_beta = self.first_of_symbols(beta, grammar);
                        
                        self.follow_sets.get_mut(nt).unwrap().extend(
                            first_of_beta.iter().filter(|&&token| token != TokenType::Eof)
                        );
                        
                        // If epsilon in FIRST(beta), add FOLLOW(A)
                        if beta.is_empty() || self.contains_epsilon(beta, grammar) {
                            let follow_a = self.follow_sets[&production.lhs].clone();
                            self.follow_sets.get_mut(nt).unwrap().extend(follow_a);
                        }
                        
                        if self.follow_sets[nt].len() > old_size {
                            changed = true;
                        }
                    }
                }
            }
        }
    }
    
    fn build_table(&mut self, grammar: &Grammar) {
        for production in &grammar.productions {
            let first_of_rhs = self.first_of_symbols(&production.rhs, grammar);
            
            for token in first_of_rhs {
                if token != TokenType::Eof {
                    self.table.insert(
                        (production.lhs.clone(), token),
                        production.clone(),
                    );
                }
            }
            
            // If epsilon in FIRST(alpha), add production for FOLLOW(A)
            if self.contains_epsilon(&production.rhs, grammar) {
                let follow_a = self.follow_sets[&production.lhs].clone();
                for token in follow_a {
                    self.table.insert(
                        (production.lhs.clone(), token),
                        production.clone(),
                    );
                }
            }
        }
    }
    
    fn first_of_symbols(&self, symbols: &[Symbol], grammar: &Grammar) -> HashSet<TokenType> {
        let mut result = HashSet::new();
        
        if symbols.is_empty() {
            result.insert(TokenType::Eof);
            return result;
        }
        
        for symbol in symbols {
            match symbol {
                Symbol::Terminal(token_type) => {
                    result.insert(token_type.clone());
                    break;
                }
                Symbol::NonTerminal(nt) => {
                    if let Some(first_set) = self.first_sets.get(nt) {
                        result.extend(first_set.iter().filter(|&&token| token != TokenType::Eof));
                        
                        if !first_set.contains(&TokenType::Eof) {
                            break;
                        }
                    }
                }
                Symbol::Epsilon => {
                    result.insert(TokenType::Eof);
                    break;
                }
            }
        }
        
        result
    }
    
    fn contains_epsilon(&self, symbols: &[Symbol], grammar: &Grammar) -> bool {
        symbols.is_empty() || symbols.iter().all(|symbol| {
            match symbol {
                Symbol::Epsilon => true,
                Symbol::NonTerminal(nt) => {
                    self.first_sets.get(nt)
                        .map(|set| set.contains(&TokenType::Eof))
                        .unwrap_or(false)
                }
                _ => false,
            }
        })
    }
}
