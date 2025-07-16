use crate::grammar::Production;
use crate::token::{Token};
use crate::ll1::LL1Table;
use crate::cst::DerivationNode;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    table: LL1Table,
    grammar: Vec<Production>,
    start_symbol: String,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, table: LL1Table, grammar: Vec<Production>, start: &str) -> Self {
        Self {
            tokens,
            pos: 0,
            table,
            grammar,
            start_symbol: start.to_string(),
        }
    }

    pub fn parse(&mut self) -> DerivationNode {
        let start = self.start_symbol.clone();
        self.expand(&start)
    }

    fn expand(&mut self, symbol: &str) -> DerivationNode {
        if symbol == "ε" {
            return DerivationNode::new("ε");
        }

        // Terminal
        if self.is_terminal(symbol) {
            let current = self.current_token();
            let token_type_str = format!("{:?}", current.token_type);
            if token_type_str == symbol {
                let node = DerivationNode::with_token(symbol, current.clone());
                self.pos += 1;
                return node;
            } else {
                self.report_error(symbol, current);
            }
        }

        // Non terminal
        let lookahead = format!("{:?}", self.current_token().token_type);
        if let Some(row) = self.table.get(symbol) {
            if let Some(prod) = row.get(&lookahead) {
                let mut node = DerivationNode::new(symbol);
                let production = prod.clone(); // Clone to avoid borrowing issues
                for sym in &production {
                    let child = self.expand(sym);
                    node.add_child(child);
                }
                return node;
            }
        }
        
        self.report_error(symbol, self.current_token());
    }

    fn current_token(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or_else(|| {
            eprintln!("Unexpected end of input at position {}", self.pos);
            std::process::exit(1);
        })
    }

    fn is_terminal(&self, symbol: &str) -> bool {
        !self.grammar.iter().any(|p| p.lhs == symbol)
    }

    fn report_error(&self, expected: &str, found: &Token) -> ! {
        eprintln!(
            "Syntax Error: Expected '{}', but found '{:?}' (lexeme: '{}') at line {}, column {}",
            expected,
            found.token_type,
            found.lexeme,
            found.line,
            found.column
        );
        std::process::exit(1);
    }
}
