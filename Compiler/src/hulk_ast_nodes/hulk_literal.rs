use std::fmt::{self, Display, Formatter};

use crate::typings::types_node::TypeNode;

#[derive(Debug, Clone, PartialEq)]
pub struct NumberLiteral {
    pub value: f64,
    pub _type: Option<TypeNode>,
}

impl NumberLiteral {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.parse().unwrap(),
            _type: None, 
        }
    }
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}

impl Display for NumberLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct BooleanLiteral {
    pub value: bool,
    pub _type: Option<TypeNode>,
}

impl BooleanLiteral {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.parse().unwrap(),
            _type: None,
        }
    }
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}

impl Display for BooleanLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral {
    pub value: String,
    pub _type: Option<TypeNode>,
}

impl StringLiteral {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
            _type: None,
        }

    }
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type)
    }
}

impl Display for StringLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

