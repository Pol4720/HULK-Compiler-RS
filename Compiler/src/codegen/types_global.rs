use std::collections::{HashMap, HashSet};
use crate::hulk_ast_nodes::hulk_program::{ProgramNode, Definition};
use crate::hulk_ast_nodes::hulk_type_def::HulkTypeNode;


pub trait TypesGlobalHelper {
    fn find_all_type_defs(program: &ProgramNode) -> Vec<&HulkTypeNode>;
}

impl TypesGlobalHelper for TypesGlobal {
    fn find_all_type_defs(program: &ProgramNode) -> Vec<&HulkTypeNode> {
        let mut seen = HashSet::new();
        let mut result = Vec::new();
        for def in &program.definitions {
            if let Some(ty) = def.as_type_def() {
                if seen.insert(&ty.type_name) {
                    result.push(ty);
                }
            }
        }
        result
    }
}

pub struct TypesGlobal {
    // Puedes agregar campos si es necesario
}