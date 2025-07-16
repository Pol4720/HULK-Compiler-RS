pub mod token;
pub mod ast;
pub mod cst;
pub mod grammar;
pub mod parser;
pub mod ll1_table;

pub use token::*;
pub use ast::*;
pub use cst::*;
pub use grammar::*;
pub use parser::*;
pub use ll1_table::*;

/// Main parser module for the HULK language compiler
pub struct HulkParser {
    grammar: Grammar,
    ll1_table: LL1Table,
}

impl HulkParser {
    pub fn new() -> Self {
        let grammar = Self::build_hulk_grammar();
        let ll1_table = LL1Table::new(&grammar);
        
        Self {
            grammar,
            ll1_table,
        }
    }
    
    pub fn parse(&self, tokens: Vec<Token>) -> Result<Vec<Stmt>, ParseError> {
        let mut parser = Parser::new(self.grammar.clone(), self.ll1_table.clone());
        let cst = parser.parse(tokens)?;
        
        let converter = cst::converter::CstToAstConverter::new();
        converter.convert_to_ast(&cst)
            .map_err(|_| ParseError::UnexpectedEof) // Simplified error handling
    }
    
    fn build_hulk_grammar() -> Grammar {
        let mut grammar = Grammar::new();
        
        // TODO: Add HULK grammar productions
        // Example:
        // grammar.add_production("program".to_string(), vec![
        //     Symbol::NonTerminal("stmt_list".to_string())
        // ]);
        
        grammar
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parser_creation() {
        let parser = HulkParser::new();
        // Add basic tests
    }
}
