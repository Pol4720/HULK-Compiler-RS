use crate::grammar::Production;
use crate::token::{Token, TokenType};
use crate::ll1::LL1Table;
use crate::cst::DerivationNode;
use crate::utils::is_terminal;
use std::collections::HashSet;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    table: LL1Table,
    grammar: Vec<Production>,
    start_symbol: String,
    error_recovery: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, table: LL1Table, grammar: Vec<Production>, start: &str) -> Self {
        Self {
            tokens,
            pos: 0,
            table,
            grammar,
            start_symbol: start.to_string(),
            error_recovery: false,
        }
    }

    pub fn with_error_recovery(mut self, enable: bool) -> Self {
        self.error_recovery = enable;
        self
    }

    pub fn parse(&mut self) -> Result<DerivationNode, String> {
        let start = self.start_symbol.clone();
        self.expand(&start)
    }

    fn expand(&mut self, symbol: &str) -> Result<DerivationNode, String> {
        if symbol == "ε" {
            return Ok(DerivationNode::new("ε"));
        }

        // Terminal
        if is_terminal(symbol, &self.grammar) {
            let current = self.current_token()?;
            let token_type_str = format!("{:?}", current.token_type);
            
            if token_type_str == symbol {
                let node = DerivationNode::with_token(symbol, current.clone());
                self.pos += 1;
                return Ok(node);
            } else {
                return Err(self.format_error(symbol, current));
            }
        }

        // Non terminal
        let current = self.current_token()?;
        let lookahead = format!("{:?}", current.token_type);
        
        if let Some(row) = self.table.get(symbol) {
            if let Some(prod) = row.get(&lookahead) {
                let mut node = DerivationNode::new(symbol);
                let production = prod.clone(); 
                
                for sym in &production {
                    match self.expand(sym) {
                        Ok(child) => node.add_child(child),
                        Err(e) => {
                            if self.error_recovery {
                                // Try synchronization - skip tokens until we find a sync point
                                self.synchronize(&[]);
                                node.add_child(DerivationNode::new("ERROR"));
                                // Continue parsing
                            } else {
                                return Err(e);
                            }
                        }
                    }
                }
                return Ok(node);
            } else {
                // No production found for this lookahead
                let expected = self.get_expected_tokens(symbol);
                return Err(format!(
                    "Syntax Error at line {}, column {}: Unexpected token '{:?}'. Expected one of: {:?}",
                    current.line, current.column, current.token_type, expected
                ));
            }
        }
        
        Err(format!("No production found for non-terminal: {}", symbol))
    }

    fn current_token(&self) -> Result<&Token, String> {
        self.tokens.get(self.pos).ok_or_else(|| 
            format!("Unexpected end of input at position {}", self.pos)
        )
    }

    fn get_expected_tokens(&self, non_terminal: &str) -> Vec<String> {
        if let Some(row) = self.table.get(non_terminal) {
            row.keys().cloned().collect()
        } else {
            vec![]
        }
    }

    fn format_error(&self, expected: &str, found: &Token) -> String {
        format!(
            "Syntax Error: Expected '{}', but found '{:?}' (lexeme: '{}') at line {}, column {}",
            expected,
            found.token_type,
            found.lexeme,
            found.line,
            found.column
        )
    }

    fn synchronize(&mut self, sync_tokens: &[TokenType]) {
        // Skip tokens until we find a synchronization point
        while self.pos < self.tokens.len() {
            let current = &self.tokens[self.pos];
            
            // Stop at statement boundaries or specified sync tokens
            if current.token_type == TokenType::SEMICOLON || 
               sync_tokens.contains(&current.token_type) {
                self.pos += 1;
                break;
            }
            
            self.pos += 1;
        }
    }
}
