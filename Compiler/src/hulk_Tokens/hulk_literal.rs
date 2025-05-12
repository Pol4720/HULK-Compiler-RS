use std::fmt::{self, Display, Formatter};

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct NumberLiteral {
    pub position: TokenPos,
    pub value: f64,
}

impl NumberLiteral {
    pub fn new(start: usize, end: usize, value: &str) -> Self {
        Self {
            position: TokenPos::new(start, end),
            value: value.parse().unwrap(),
        }
    }
}

impl Display for NumberLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BooleanLiteral {
    pub position: TokenPos,
    pub value: bool,
}

impl BooleanLiteral {
    pub fn new(start: usize, end: usize, value: &str) -> Self {
        Self {
            position: TokenPos::new(start, end),
            value: value.parse().unwrap(),
        }
    }
}

impl Display for BooleanLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral {
    pub position: TokenPos,
    pub value: String,
}

impl StringLiteral {
    pub fn new(start: usize, end: usize, value: &str) -> Self {
        Self {
            position: TokenPos::new(start, end),
            value: value.to_string(),
        }
    }
}

impl Display for StringLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
