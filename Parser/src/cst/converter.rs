use super::CstNode;
use crate::Parser::ast::{Expr, Stmt, Position};
use crate::Parser::token::TokenType;

pub struct CstToAstConverter;

impl CstToAstConverter {
    pub fn new() -> Self {
        Self
    }
    
    pub fn convert_to_ast(&self, cst: &CstNode) -> Result<Vec<Stmt>, ConversionError> {
        match cst {
            CstNode::NonTerminal { rule, children } => {
                match rule.as_str() {
                    "program" => self.convert_program(children),
                    _ => Err(ConversionError::UnsupportedRule(rule.clone())),
                }
            }
            _ => Err(ConversionError::ExpectedNonTerminal),
        }
    }
    
    fn convert_program(&self, children: &[CstNode]) -> Result<Vec<Stmt>, ConversionError> {
        // TODO: Implement program conversion
        Ok(vec![])
    }
    
    fn convert_expression(&self, node: &CstNode) -> Result<Expr, ConversionError> {
        // TODO: Implement expression conversion
        Err(ConversionError::NotImplemented)
    }
}

#[derive(Debug)]
pub enum ConversionError {
    UnsupportedRule(String),
    ExpectedNonTerminal,
    ExpectedTerminal,
    NotImplemented,
}
