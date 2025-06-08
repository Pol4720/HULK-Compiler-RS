use std::fmt::{self, Display, Formatter};
use crate::visitor::hulk_accept::Accept;
use crate::visitor::hulk_visitor::Visitor;
use crate::codegen::traits::Codegen;
use crate::codegen::context::CodegenContext;

#[derive(Debug, Clone, PartialEq)]
pub struct NumberLiteral {
    pub value: f64,
}

impl NumberLiteral {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.parse().unwrap(),
        }
    }
}

impl Display for NumberLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Codegen for NumberLiteral {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // TODO: Implement codegen for NumberLiteral
        String::new()
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct BooleanLiteral {
    pub value: bool,
}

impl BooleanLiteral {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.parse().unwrap(),
        }
    }
}

impl Display for BooleanLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Codegen for BooleanLiteral {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // TODO: Implement codegen for BooleanLiteral
        String::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral {
    pub value: String,
}

impl StringLiteral {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl Display for StringLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Codegen for StringLiteral {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // TODO: Implement codegen for StringLiteral
        String::new()
    }
}

