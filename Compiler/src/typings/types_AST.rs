use std::collections::HashMap;

use crate::{hulk_ast_nodes::{hulk_function_def::FunctionParams, FunctionDef}, typings::types_node::TypeNode};


#[derive(Debug, Clone)]
pub enum HulkTypesInfo {
    Object,
    String,
    Number,
    Boolean,
    Unknown,
}

impl HulkTypesInfo {
    pub fn as_str(&self) -> &str {
        match self {
            HulkTypesInfo::Object => "Object",
            HulkTypesInfo::String => "String",
            HulkTypesInfo::Number => "Number",
            HulkTypesInfo::Boolean => "Boolean",
            HulkTypesInfo::Unknown => "Unknown",
        }
    }
}

pub struct TypeAST {
    pub root: TypeNode,
    pub nodes: HashMap<String, TypeNode>,
}

impl TypeAST {
    pub fn new() -> Self {
       let mut tree = TypeAST {
            root: TypeNode::new("Object".to_string(), vec![], 0, None, Vec::new(), HashMap::new(), HashMap::new()),
            nodes: HashMap::new(),
        };

        tree.nodes.insert("Object".to_string(), tree.root.clone());
        tree.add_type(
            "String".to_string(),
            Vec::new(),
            Some("Object".to_string()),
            HashMap::new(),
            HashMap::new(),
        );
        tree.add_type(
            "Number".to_string(),
            Vec::new(),
            Some("Object".to_string()),
            HashMap::new(),
            HashMap::new(),
        );
        tree.add_type(
            "Boolean".to_string(),
            Vec::new(),
            Some("Object".to_string()),
            HashMap::new(),
            HashMap::new(),
        );
        tree.add_type(
            "Unknown".to_string(),
            Vec::new(),
            Some("Object".to_string()),
            HashMap::new(),
            HashMap::new(),
        );
        tree

    }
    pub fn add_type(&mut self, type_name: String, params: Vec<FunctionParams>, parent_name: Option<String>, variables: HashMap<String, Box<String>>, methods: HashMap<String, Box<FunctionDef>>) {
        let depth = match &parent_name {
            Some(name) => {
                if let Some(parent) = self.nodes.get(name) {
                    parent.depth + 1
                } else {
                    0
                }
            }
            None => 0,
        };

        let new_node = TypeNode::new(
            type_name.clone(),
            params,
            depth,
            parent_name.clone(),
            Vec::new(),
            variables,
            methods,
        );

        if let Some(name) = &parent_name {
            if let Some(parent) = self.nodes.get_mut(name) {
                parent.add_child(type_name.clone());
            }
        } else {
            self.root.add_child(type_name.clone());
        }

        self.nodes.insert(type_name, new_node);

    }
    pub fn get_type(&self, type_name: &str) -> Option<TypeNode> {
        self.nodes.get(type_name).cloned()
    }

    pub fn find_lca(&self, type1: &TypeNode, type2: &TypeNode) -> TypeNode {
        let mut node1 = type1;
        let mut node2 = type2;

        // Bring both nodes to the same depth
        while node1.depth > node2.depth {
            if let Some(parent_name) = &node1.parent {
                if let Some(parent_node) = self.nodes.get(parent_name) {
                    node1 = parent_node;
                } else {
                    return self.root.clone();
                }
            } else {
                return self.root.clone();
            }
        }
        while node2.depth > node1.depth {
            if let Some(parent_name) = &node2.parent {
                if let Some(parent_node) = self.nodes.get(parent_name) {
                    node2 = parent_node;
                } else {
                    return self.root.clone();
                }
            } else {
                return self.root.clone();
            }
        }

        while node1.type_name != node2.type_name {
            let parent1 = node1.parent.as_ref().and_then(|p| self.nodes.get(p));
            let parent2 = node2.parent.as_ref().and_then(|p| self.nodes.get(p));
            match (parent1, parent2) {
                (Some(p1), Some(p2)) => {
                    node1 = p1;
                    node2 = p2;
                }
                _ => return self.root.clone(),
            }
        }
        node1.clone()
    }

    pub fn is_ancestor(&self, ancestor: &TypeNode, descendant: &TypeNode) -> bool {
        let mut current = Some(descendant);
        while let Some(node) = current {
            if node.type_name == ancestor.type_name {
                return true;
            }
            current = node.parent.as_ref().and_then(|parent_name| self.nodes.get(parent_name));
        }
        false
    }

      pub fn inheritance_cicle(&mut self) -> Option<String> {
        let mut visited = HashMap::new();
        for (type_name,_) in self.nodes.clone() {
            if ! visited.contains_key(&type_name) {
                if let Some(cycle_node) = self.helper(type_name, &mut visited) {
                    return Some(cycle_node)
                }
            }
        }
        return None;
    }

    fn helper(&mut self, node_name: String, visited: &mut HashMap<String, bool>) -> Option<String> {
        if visited.get(&node_name).copied().unwrap_or(false) {
            return Some(node_name);
        }
        if let Some(node) = self.nodes.get_mut(&node_name) {
            visited.insert(node_name.clone(), true);
            let parent_depth = node.depth;
            let children = node.children.clone();
            for child in children {
                if let Some(child_node) = self.nodes.get_mut(&child) {
                    child_node.depth = parent_depth + 1;
                }
                if let Some(cycle_node) = self.helper(child, visited) {
                    return Some(cycle_node);
                }
            }
            visited.remove(&node_name);
        }
        None
    }

    pub fn find_method(&mut self, node_name: String, method_name: String) -> Option<Box<FunctionDef>> {
        if let Some(type_node) = self.nodes.get_mut(&node_name) {
            if let Some(method) = type_node.get_method(&method_name) {
                return Some(method);
            } else {
                if let Some(parent) = type_node.parent.clone() {
                    return self.find_method(parent, method_name);
                } else {
                    return None;
                }  
            }
        }
        return None;
    }
    

}
