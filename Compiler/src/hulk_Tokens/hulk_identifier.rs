use super::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    pub position: TokenPos,
    pub id: String,
}

impl Identifier {
    pub fn new(start: usize, end: usize, id: &str) -> Self {
        Self {
            position: TokenPos::new(start, end),
            id: id.to_string(),
        }
    }

    pub fn get_position(&self) -> TokenPos {
        self.position
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
