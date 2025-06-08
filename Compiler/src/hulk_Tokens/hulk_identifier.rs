use std::fmt;
use crate::codegen::traits::Codegen;
use crate::codegen::context::CodegenContext;

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

impl Codegen for Identifier {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // TODO: Implement codegen for Identifier
        String::new()
    }
}
