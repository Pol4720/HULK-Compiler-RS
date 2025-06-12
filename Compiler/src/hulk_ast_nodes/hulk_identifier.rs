use std::fmt;

use crate::typings::types_node::TypeNode;

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub id: String,
    pub _type: Option<TypeNode>
}

impl Identifier {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            _type: None,
        }
    }

    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }

}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
