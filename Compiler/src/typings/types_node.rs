use std::collections::HashMap;

use crate::hulk_ast_nodes::hulk_function_def::FunctionParams;
use crate::hulk_ast_nodes::FunctionDef;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeNode {
    pub type_name: String,
    pub depth: i32,
    pub params: Vec<FunctionParams>,
    pub parent: Option<String>,
    pub children: Vec<String>,
    pub variables: HashMap<String, Box<String>>, 
    pub methods: HashMap<String, Box<FunctionDef>>, 
}


impl TypeNode {
    pub fn new(type_name: String, params: Vec<FunctionParams>, depth: i32, parent: Option<String> , children: Vec<String>, variables: HashMap<String, Box<String>>, methods: HashMap<String, Box<FunctionDef>>) -> Self {
        TypeNode {
            type_name,
            params,
            depth,
            parent,
            children,
            variables,
            methods,
        }
    }

    pub fn add_child(&mut self, child_name: String) {
        self.children.push(child_name);
    }

    pub fn set_parent(&mut self, parent_name: String) {
        self.parent = Some(parent_name);
    }

    pub fn add_variable(&mut self, name: String, variable: Box<String>) {
        self.variables.insert(name, variable);
    }

    pub fn add_method(&mut self, name: String, method: Box<FunctionDef>) {
        self.methods.insert(name, method);
    }

    pub fn get_method(&mut self, method_name: &String) -> Option<Box<FunctionDef>> {
        if let Some(method) = self.methods.get(method_name) {
            Some(method.clone())
        } else {
           None
        }
    }
}