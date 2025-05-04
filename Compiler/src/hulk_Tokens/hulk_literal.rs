use std::fmt::Display;

use super::*;

pub struct NumberLiteral {
    pub position: TokenPos,
    pub value: f64,
}

impl NumberLiteral {
    pub fn new(start: usize, end: usize, value: &str) -> Self {
        NumberLiteral {
            position: TokenPos::new(start, end),
            value: value.parse::<f64>().unwrap(),
        }
    }
}

pub struct BooleanLiteral {
    pub position: TokenPos,
    pub value: bool,
}

impl BooleanLiteral {
    pub fn new(start: usize, end: usize, value: &str) -> Self {
        BooleanLiteral {
            position: TokenPos::new(start, end),
            value: value.parse::<bool>().unwrap(),
        }
    }
}

pub struct StringLiteral {
    pub position: TokenPos,
    pub value: String,
}

impl StringLiteral {
    pub fn new(start: usize, end: usize, value: &str) -> Self {
        StringLiteral {
            position: TokenPos::new(start, end),
            value: value.to_string(),
        }
    }
}