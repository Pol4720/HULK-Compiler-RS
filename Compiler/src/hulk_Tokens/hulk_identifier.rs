use std::fmt;
use crate::visitor::hulk_accept::Accept;
use crate::visitor::hulk_visitor::Visitor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    pub id: String,
}

impl Identifier {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
        }
    }

}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}


impl Accept for Identifier {
    fn accept<V: Visitor<T>, T>(&self, visitor: &mut V) -> T {
        visitor.visit_identifier(self)
    }
}
