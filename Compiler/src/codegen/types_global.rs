use std::collections::HashMap;

// Representa un nodo de tipo en el AST
#[derive(Debug, Clone)]
pub trait TypesGlobalHelper {
    fn add_type(&mut self, name: String, node: TypeNode) -> bool;
    fn get_type(&self, name: &str) -> Option<&TypeNode>;
    fn has_type(&self, name: &str) -> bool;
}

impl TypesGlobalHelper for TypesGlobal {
    
    }