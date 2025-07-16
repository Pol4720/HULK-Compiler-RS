use crate::grammar::Production;

pub fn is_terminal(symbol: &str, productions: &[Production]) -> bool {
    if symbol == "ε" { return true; }
    !productions.iter().any(|p| p.lhs == symbol)
}
