use crate::Parser::token::TokenType;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Production {
    pub lhs: String,
    pub rhs: Vec<Symbol>,
}

#[derive(Debug, Clone)]
pub enum Symbol {
    Terminal(TokenType),
    NonTerminal(String),
    Epsilon,
}

pub struct Grammar {
    pub productions: Vec<Production>,
    pub start_symbol: String,
}

impl Grammar {
    pub fn new() -> Self {
        Self {
            productions: vec![],
            start_symbol: "program".to_string(),
        }
    }
    
    pub fn add_production(&mut self, lhs: String, rhs: Vec<Symbol>) {
        self.productions.push(Production { lhs, rhs });
    }
    
    pub fn get_productions_for(&self, non_terminal: &str) -> Vec<&Production> {
        self.productions
            .iter()
            .filter(|p| p.lhs == non_terminal)
            .collect()
    }
    
    pub fn get_non_terminals(&self) -> Vec<String> {
        let mut non_terminals = Vec::new();
        for production in &self.productions {
            if !non_terminals.contains(&production.lhs) {
                non_terminals.push(production.lhs.clone());
            }
        }
        non_terminals
    }
    
    pub fn get_terminals(&self) -> Vec<TokenType> {
        let mut terminals = Vec::new();
        for production in &self.productions {
            for symbol in &production.rhs {
                if let Symbol::Terminal(token_type) = symbol {
                    if !terminals.contains(token_type) {
                        terminals.push(token_type.clone());
                    }
                }
            }
        }
        terminals
    }
}
