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

        // Create the root node of the tree
        let mut root = DerivationNode::new(&self.start_symbol);
        
        // Stack for predictive parsing (stores mutable pointers to nodes)
        let mut parse_stack: Vec<*mut DerivationNode> = Vec::new();
        parse_stack.push(&mut root as *mut DerivationNode);
        
        self.pos = 0; // Reset position counter
        
        while !parse_stack.is_empty() {
            let current_node = unsafe { &mut *parse_stack.pop().unwrap() };
            let symbol = current_node.symbol.clone();
            
            // Get the current token (or EOF if we've reached the end)
            let current_token = if self.pos < self.tokens.len() {
                &self.tokens[self.pos]
            } else {
                // Create artificial EOF token
                &Token {
                    token_type: TokenType::EOF,
                    lexeme: "$".to_string(),
                    line: if !self.tokens.is_empty() { 
                        self.tokens.last().unwrap().line 
                    } else { 
                        1 
                    },
                    column: 0,
                }
            };
            
            let mut current_terminal = format!("{:?}", current_token.token_type);
            if current_terminal == "EOF" {
                current_terminal = "$".to_string();
            }
            
            if self.debug_output {
                println!("Processing: {}, Token: {}", symbol, current_terminal);
            }
            
            if is_terminal(&symbol, &self.grammar) {
                // Terminal
                if symbol == "ε" {
                    // Epsilon production - do nothing
                    if self.debug_output {
                        println!("  ✓ Applied ε production");
                    }
                } else if symbol == "$" {
                    // End of input symbol
                    if current_token.token_type == TokenType::EOF {
                        current_node.set_token(current_token.clone());
                        if self.debug_output {
                            println!("  ✓ Successful match with EOF");
                        }
                    } else {
                        let error_msg = format!(
                            "Syntax Error at line {}, column {}: Expected EOF, but found '{:?}'",
                            current_token.line, current_token.column, current_token.token_type
                        );
                        if self.error_recovery {
                            println!("{}", error_msg);
                            self.synchronize(&[TokenType::EOF]);
                        } else {
                            return Err(error_msg);
                        }
                    }
                } else if symbol == current_terminal {
                    // Successful match - assign token and advance
                    current_node.set_token(current_token.clone());
                    if current_token.token_type != TokenType::EOF {
                        self.pos += 1;
                    }
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
                        println!("{}", error_msg);
                        self.synchronize(&[TokenType::SEMICOLON]);
                    } else {
                        return Err(error_msg);
                    }
                }
            } else {
                // Non-terminal - look up in LL(1) table
                if let Some(row) = self.table.get(&symbol) {
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
                        
                        // Create child nodes for each symbol in the production
                        if production.is_empty() {
                            // Epsilon production
                            current_node.children.push(DerivationNode::new("ε"));
                        } else {
                            // Create child nodes and add them to the tree
                            let mut children = Vec::with_capacity(production.len());
                            
                            for prod_symbol in production {
                                children.push(DerivationNode::new(prod_symbol));
                            }
                            
                            // Add children to current node
                            current_node.children = children;
                            
                            // Push child node pointers to stack in reverse order
                            for child in current_node.children.iter_mut().rev() {
                                parse_stack.push(child as *mut DerivationNode);
                            }
                        }
                    } else {
                        // Error: no production for this pair (non_terminal, terminal)
                        let expected = self.get_expected_tokens(&symbol);
                        let error_msg = format!(
                            "Syntax Error at line {}, column {}: No production for {} with terminal '{:?}'. Expected one of: {:?}",
                            current_token.line, current_token.column, symbol, current_token.token_type, expected
                        );
                        if self.error_recovery {
                            println!("{}", error_msg);
                            self.synchronize(&[TokenType::SEMICOLON]);
                        } else {
                            return Err(error_msg);
                        }
                    }
                } else {
                    // Error: non-terminal not found in table
                    let error_msg = format!("No production found for non-terminal: {}", symbol);
                    if self.error_recovery {
                        println!("{}", error_msg);
                        self.synchronize(&[TokenType::SEMICOLON]);
                    } else {
                        return Err(error_msg);
                    }
                }
            }
        }
        
        // Check if all tokens were processed (only EOF can remain unconsumed)
        if self.pos < self.tokens.len() {
            if self.pos == self.tokens.len() - 1 && self.tokens[self.pos].token_type == TokenType::EOF {
                // OK: only EOF left
            } else {
                let remaining_token = &self.tokens[self.pos];
                let error_msg = format!(
                    "Syntax Error at line {}, column {}: Unexpected token '{:?}' after complete parse",
                    remaining_token.line, remaining_token.column, remaining_token.token_type
                );
                if self.error_recovery {
                    println!("{}", error_msg);
                } else {
                    return Err(error_msg);
                }
            }
        }
        
        if self.debug_output {
            println!("✓ LL(1) parsing completed successfully");
        }
        
        Ok(root)
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