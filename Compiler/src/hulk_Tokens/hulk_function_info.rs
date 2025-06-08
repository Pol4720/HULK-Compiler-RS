use std::collections::HashMap;
use crate::typings::types_node::TypeNode;
use crate::codegen::traits::Codegen;
use crate::codegen::context::CodegenContext;

#[derive(Debug, Clone)]
pub struct HulkFunctionInfo {
    pub function_name: String,
    pub argument_types: Vec<TypeNode>,
    pub return_type: TypeNode,
}

impl HulkFunctionInfo {
    pub fn new(function_name: String, argument_types: Vec<TypeNode>, return_type: TypeNode) -> Self {
        HulkFunctionInfo {
            function_name,
            argument_types,
            return_type,
        }
    }
}

impl Codegen for HulkFunctionInfo {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // TODO: Implement codegen for HulkFunctionInfo
        String::new()
    }
}