use crate::grammar::Production;
use crate::token::{Token, TokenType};
use crate::ll1::LL1Table;
use crate::cst::DerivationNode;
use crate::utils::is_terminal;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    table: LL1Table,
    grammar: Vec<Production>,
    start_symbol: String,
    error_recovery: bool,
    debug_output: bool,
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
            debug_output: false,
        }
    }

    pub fn with_error_recovery(mut self, enable: bool) -> Self {
        self.error_recovery = enable;
        self
    }

    pub fn with_debug_output(mut self, enable: bool) -> Self {
        self.debug_output = enable;
        self
    }

    pub fn parse(&mut self) -> Result<DerivationNode, String> {
        if self.debug_output {
            println!("Building derivation tree (CST)...");
        }

        if self.tokens.is_empty() {
            return Err("No tokens to process".to_string());
        }

        let mut token_index = 0;
        let result = self.parse_symbol(&self.start_symbol.clone(), &mut token_index)?;
        
        // Check if all tokens were processed
        if token_index < self.tokens.len() - 1 || 
           (token_index < self.tokens.len() && self.tokens[token_index].token_type != TokenType::EOF) {
            let remaining_token = &self.tokens[token_index];
            return Err(format!(
                "Syntax Error at line {}, column {}: Unexpected token '{:?}' after complete parse",
                remaining_token.line, remaining_token.column, remaining_token.token_type
            ));
        }
        
        if self.debug_output {
            println!("✓ LL(1) parsing completed successfully");
        }
        
        Ok(result)
    }

    fn parse_symbol(&mut self, symbol: &str, token_index: &mut usize) -> Result<DerivationNode, String> {
        let mut node = DerivationNode::new(symbol);
        
        if self.debug_output {
            let current_token = if *token_index < self.tokens.len() {
                format!("{:?}", self.tokens[*token_index].token_type)
            } else {
                "EOF".to_string()
            };
            println!("Processing: {}, Token: {}", symbol, current_token);
        }

        if is_terminal(symbol, &self.grammar) {
            // Terminal
            if symbol == "ε" {
                // Epsilon production - do nothing
                if self.debug_output {
                    println!("  ✓ Applied ε production");
                }
            } else {
                // Get current token (or EOF if at end)
                if *token_index >= self.tokens.len() {
                    let line = if !self.tokens.is_empty() { self.tokens.last().unwrap().line } else { 1 };
                    return Err(format!("Unexpected end of input at line {}", line));
                }
                
                let current_token = &self.tokens[*token_index];
                let current_terminal = format!("{:?}", current_token.token_type);
                
                if current_terminal == symbol {
                    // Successful match - assign token and advance
                    node.token = Some(current_token.clone());
                    *token_index += 1;
                    if self.debug_output {
                        println!("  ✓ Successful match: {}", symbol);
                    }
                } else {
                    // Syntax error
                    let error_msg = format!(
                        "Syntax Error at line {}, column {}: Expected '{}', but found '{:?}' (lexeme: '{}')",
                        current_token.line, current_token.column, symbol, current_token.token_type, current_token.lexeme
                    );
                    
                    if self.error_recovery {
                        // Try to recover
                        self.synchronize(&[]);
                        node.add_child(DerivationNode::new("ERROR"));
                    } else {
                        return Err(error_msg);
                    }
                }
            }
        } else {
            // Non-terminal - look up in LL(1) table
            if *token_index >= self.tokens.len() {
                let line = if !self.tokens.is_empty() { self.tokens.last().unwrap().line } else { 1 };
                return Err(format!("Unexpected end of input at line {}", line));
            }
            
            let current_token = &self.tokens[*token_index];
            let current_terminal = format!("{:?}", current_token.token_type);
            
            // Special case: Handle EOF token when parsing StmtList
            if symbol == "StmtList" && current_token.token_type == TokenType::EOF {
                // Treat EOF as empty statement list
                let epsilon_node = DerivationNode::new("ε");
                node.add_child(epsilon_node);
                return Ok(node);
            }
            
            if let Some(row) = self.table.get(symbol) {
                if let Some(production) = row.get(&current_terminal) {
                    if self.debug_output {
                        print!("  Applying production: {} → ", symbol);
                        if production.is_empty() {
                            print!("ε");
                        } else {
                            for prod_symbol in production {
                                print!("{} ", prod_symbol);
                            }
                        }
                        println!();
                    }
                    
                    // Parse each symbol in the production
                    if production.is_empty() {
                        // Epsilon production
                        let epsilon_node = DerivationNode::new("ε");
                        node.add_child(epsilon_node);
                    } else {
                        // Clone the production to avoid borrowing issues
                        let production_clone = production.clone();
                        for prod_symbol in &production_clone {
                            let child_node = self.parse_symbol(prod_symbol, token_index)?;
                            node.add_child(child_node);
                        }
                    }
                } else {
                    // Error: no production for this pair (non_terminal, terminal)
                    let expected = self.get_expected_tokens(symbol);
                    let error_msg = format!(
                        "Syntax Error at line {}, column {}: Unexpected token '{:?}'. Expected one of: {:?}",
                        current_token.line, current_token.column, current_token.token_type, expected
                    );
                    
                    if self.error_recovery {
                        // Try to recover
                        self.synchronize(&[]);
                        node.add_child(DerivationNode::new("ERROR"));
                    } else {
                        return Err(error_msg);
                    }
                }
            } else {
                // Error: non-terminal not found in table
                return Err(format!("No production found for non-terminal: {}", symbol));
            }
        }
        
        Ok(node)
    }

    fn get_expected_tokens(&self, non_terminal: &str) -> Vec<String> {
        if let Some(row) = self.table.get(non_terminal) {
            row.keys().cloned().collect()
        } else {
            vec![]
        }
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
