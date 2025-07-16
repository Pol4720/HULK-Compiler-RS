use crate::Parser::token::{Token, TokenType};
use crate::Parser::grammar::{Grammar, Symbol};
use crate::Parser::cst::CstNode;
use crate::Parser::ll1_table::LL1Table;
use std::collections::VecDeque;

pub struct Parser {
    grammar: Grammar,
    ll1_table: LL1Table,
    tokens: VecDeque<Token>,
    current_token: usize,
}

impl Parser {
    pub fn new(grammar: Grammar, ll1_table: LL1Table) -> Self {
        Self {
            grammar,
            ll1_table,
            tokens: VecDeque::new(),
            current_token: 0,
        }
    }
    
    pub fn parse(&mut self, tokens: Vec<Token>) -> Result<CstNode, ParseError> {
        self.tokens = VecDeque::from(tokens);
        self.current_token = 0;
        
        let mut stack = Vec::new();
        stack.push(Symbol::NonTerminal(self.grammar.start_symbol.clone()));
        
        let mut cst_stack = Vec::new();
        
        while !stack.is_empty() {
            let top = stack.pop().unwrap();
            
            match top {
                Symbol::Terminal(expected_token_type) => {
                    if let Some(current_token) = self.peek_token() {
                        if current_token.token_type == expected_token_type {
                            let token = self.advance();
                            cst_stack.push(CstNode::new_terminal(token));
                        } else {
                            return Err(ParseError::UnexpectedToken {
                                expected: expected_token_type,
                                found: current_token.token_type.clone(),
                            });
                        }
                    } else {
                        return Err(ParseError::UnexpectedEof);
                    }
                }
                Symbol::NonTerminal(non_terminal) => {
                    if let Some(current_token) = self.peek_token() {
                        if let Some(production) = self.ll1_table.get_production(&non_terminal, &current_token.token_type) {
                            // Push production symbols in reverse order
                            for symbol in production.rhs.iter().rev() {
                                if !matches!(symbol, Symbol::Epsilon) {
                                    stack.push(symbol.clone());
                                }
                            }
                        } else {
                            return Err(ParseError::NoProductionFound {
                                non_terminal,
                                token: current_token.token_type.clone(),
                            });
                        }
                    } else {
                        return Err(ParseError::UnexpectedEof);
                    }
                }
                Symbol::Epsilon => {
                    // Do nothing for epsilon
                }
            }
        }
        
        // Build CST from stack (simplified)
        Ok(CstNode::new_non_terminal("program".to_string(), cst_stack))
    }
    
    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.current_token)
    }
    
    fn advance(&mut self) -> Token {
        let token = self.tokens[self.current_token].clone();
        self.current_token += 1;
        token
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken {
        expected: TokenType,
        found: TokenType,
    },
    UnexpectedEof,
    NoProductionFound {
        non_terminal: String,
        token: TokenType,
    },
}
