use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::types_node::TypeNode;

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
    pub root: Rc<RefCell<TypeNode>>,
    pub nodes: HashMap<String, Rc<RefCell<TypeNode>>>,
}

impl TypeAST {
    pub fn new() -> Self {
       let mut tree = TypeAST {
            root: TypeNode::new("Object".to_string(), 0, None),
            nodes: HashMap::new(),
        };

        tree.nodes.insert("Object".to_string(), tree.root.clone());
        tree.add_type("String".to_string(),  Some("Object".to_string()));
        tree.add_type("Number".to_string(),  Some("Object".to_string()));
        tree.add_type("Boolean".to_string(),  Some("Object".to_string()));
        tree.add_type("Unknown".to_string(),  Some("Object".to_string()));
        tree

    }
    pub fn add_type(&mut self, type_name: String, parent: Option<String>) -> Rc<RefCell<TypeNode>> {
        let parent_node = if let Some(parent_name) = parent {
            self.nodes.get(&parent_name).cloned()
        } else {
            None
        };

        let new_node = TypeNode::new(type_name.clone(), 0, parent_node.as_ref().map(|n| Rc::downgrade(n)));
        let new_node_rc = new_node.clone();

        if let Some(parent_node) = parent_node {
            TypeNode::add_child(&parent_node, new_node_rc.clone());
        }

        self.nodes.insert(type_name, new_node_rc.clone());
        new_node_rc
    }
    pub fn get_type(&self, type_name: &str) -> Option<Rc<RefCell<TypeNode>>> {
        self.nodes.get(type_name).cloned()
    }
    pub fn get_root(&self) -> Rc<RefCell<TypeNode>> {
        self.root.clone()
    }

    pub fn find_lca(&self, type1: &str, type2: &str) -> Option<Rc<RefCell<TypeNode>>> {
        let mut ancestors1 = Vec::new();
        let mut current = self.get_type(type1);
        while let Some(node_rc) = current {
            let node = node_rc.borrow();
            ancestors1.push(node.type_name.clone());
            current = node.parent.as_ref().and_then(|p| p.upgrade());
        }

        let mut current = self.get_type(type2);
        while let Some(node_rc) = current {
            let node = node_rc.borrow();
            if ancestors1.contains(&node.type_name) {
                return self.get_type(&node.type_name);
            }
            current = node.parent.as_ref().and_then(|p| p.upgrade());
        }
        None
    }

    pub fn is_ancestor(&self, ancestor: &str, descendant: &str) -> bool {
        let mut current = self.get_type(descendant);
        while let Some(node_rc) = current {
            let node = node_rc.borrow();
            if node.type_name == ancestor {
                return true;
            }
            current = node.parent.as_ref().and_then(|p| p.upgrade());
        }
        false
    }
    

}
