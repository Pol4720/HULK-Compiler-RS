use std::collections::HashMap;

use crate::hulk_ast_nodes::HulkTypeNode;
use crate::typings::types_node::TypeNode;
// Update the path below to the correct module path for HulkFunctionInfo
use crate::hulk_ast_nodes::hulk_function_info::HulkFunctionInfo;

#[derive(Debug, Clone)]
pub struct Scope{
    pub variables: HashMap<String, TypeNode>, // Variable name and type
    pub declared_functions: HashMap<String, HulkFunctionInfo>,
    pub declared_types: HashMap<String, HulkTypeNode>,
    pub current_type: Option<String>,
    pub current_function: Option<String>
}