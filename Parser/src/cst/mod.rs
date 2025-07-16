pub mod converter;

use crate::Parser::token::Token;

/// Concrete Syntax Tree node
#[derive(Debug, Clone)]
pub enum CstNode {
    Terminal(Token),
    NonTerminal {
        rule: String,
        children: Vec<CstNode>,
    },
}

impl CstNode {
    pub fn new_terminal(token: Token) -> Self {
        CstNode::Terminal(token)
    }
    
    pub fn new_non_terminal(rule: String, children: Vec<CstNode>) -> Self {
        CstNode::NonTerminal { rule, children }
    }
    
    pub fn is_terminal(&self) -> bool {
        matches!(self, CstNode::Terminal(_))
    }
    
    pub fn get_token(&self) -> Option<&Token> {
        match self {
            CstNode::Terminal(token) => Some(token),
            _ => None,
        }
    }
}
