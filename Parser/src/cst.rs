use crate::token::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub struct DerivationNode {
    pub symbol: String,
    pub children: Vec<DerivationNode>,
    pub token: Option<Token>,
}

impl DerivationNode {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            children: vec![],
            token: None,
        }
    }

    pub fn with_token(symbol: &str, token: Token) -> Self {
        Self {
            symbol: symbol.to_string(),
            children: vec![],
            token: Some(token),
        }
    }

    pub fn set_token(&mut self, token: Token) {
        self.token = Some(token);
    }

    pub fn add_child(&mut self, child: DerivationNode) {
        self.children.push(child);
    }

    pub fn is_terminal(&self) -> bool {
        self.token.is_some()
    }
}

impl fmt::Display for DerivationNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn recurse(node: &DerivationNode, depth: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for _ in 0..depth {
                write!(f, "  ")?;
            }

            if let Some(tok) = &node.token {
                writeln!(f, "{} -> '{}'", node.symbol, tok.lexeme)?;
            } else {
                writeln!(f, "{}", node.symbol)?;
            }

            for child in &node.children {
                recurse(child, depth + 1, f)?;
            }
            Ok(())
        }

        recurse(self, 0, f)
    }
}
