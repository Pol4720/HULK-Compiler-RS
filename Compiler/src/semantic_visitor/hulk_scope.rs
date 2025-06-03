use std::collections::HashMap;

use crate::typings::types_node::TypeNode;
// Update the path below to the correct module path for HulkFunctionInfo
use crate::hulk_tokens::hulk_function_info::HulkFunctionInfo;

#[derive(Debug, Clone)]
pub struct Scope{
    pub variables: HashMap<String, TypeNode>, // Variable name and type
    pub declared_functions: HashMap<String, HulkFunctionInfo>
}