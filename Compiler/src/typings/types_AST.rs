//! # TypeAST
//!
//! Este módulo define la estructura `TypeAST` para el compilador Hulk.
//! `TypeAST` representa el árbol de tipos del lenguaje Hulk, permitiendo registrar, consultar y analizar la jerarquía de tipos (clases), sus métodos y relaciones de herencia.
//! Proporciona utilidades para búsqueda de tipos, métodos, detección de ciclos de herencia y operaciones sobre la jerarquía de tipos.

use std::collections::HashMap;

use crate::{
    hulk_ast_nodes::{FunctionDef, hulk_function_def::FunctionParams},
    typings::types_node::TypeNode,
};

/// Estructura que representa el árbol de tipos del lenguaje Hulk.
/// 
/// - `root`: nodo raíz del árbol de tipos (por lo general "Object").
/// - `nodes`: mapa de nombre de tipo a su nodo correspondiente.
pub struct TypeAST {
    pub root: TypeNode,
    pub nodes: HashMap<String, TypeNode>,
}

impl TypeAST {
    /// Crea un nuevo árbol de tipos con los tipos básicos (`Object`, `String`, `Number`, `Boolean`, `Unknown`).
    pub fn new() -> Self {
        let mut tree = TypeAST {
            root: TypeNode::new(
                "Object".to_string(),
                vec![],
                0,
                None,
                Vec::new(),
                HashMap::new(),
                HashMap::new(),
            ),
            nodes: HashMap::new(),
        };

        tree.nodes.insert("Object".to_string(), tree.root.clone());
        tree.add_type("String".to_string(), vec![], Some("Object".to_string()), HashMap::new(), HashMap::new());
        tree.add_type("Number".to_string(), vec![], Some("Object".to_string()), HashMap::new(), HashMap::new());
        tree.add_type("Boolean".to_string(), vec![], Some("Object".to_string()), HashMap::new(), HashMap::new());
        tree.add_type("Unknown".to_string(), vec![], Some("Object".to_string()), HashMap::new(), HashMap::new());
        tree
    }
    /// Agrega un nuevo tipo al árbol.
    ///
    /// # Arguments
    /// * `type_name` - Nombre del tipo.
    /// * `params` - Parámetros del tipo (por ejemplo, genéricos o del constructor).
    /// * `parent_name` - Nombre del tipo padre (opcional).
    /// * `variables` - Variables/atributos del tipo.
    /// * `methods` - Métodos del tipo.
    pub fn add_type(
        &mut self,
        type_name: String,
        params: Vec<FunctionParams>,
        parent_name: Option<String>,
        variables: HashMap<String, Box<String>>,
        methods: HashMap<String, Box<FunctionDef>>,
    ) {
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

    /// Obtiene el nodo de tipo correspondiente a un nombre de tipo.
    pub fn get_type(&self, type_name: &str) -> Option<TypeNode> {
        self.nodes.get(type_name).cloned()
    }

    /// Busca el ancestro común más cercano (LCA) entre dos tipos.
    ///
    /// # Arguments
    /// * `type1` - Primer tipo.
    /// * `type2` - Segundo tipo.
    /// 
    /// # Returns
    /// El nodo de tipo que es ancestro común más cercano.
    pub fn find_lca(&self, type1: &TypeNode, type2: &TypeNode) -> TypeNode {
        let mut node1 = type1;
        let mut node2 = type2;

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

    /// Verifica si un tipo es ancestro de otro en la jerarquía de tipos.
    ///
    /// # Arguments
    /// * `ancestor` - Nodo de tipo ancestro.
    /// * `descendant` - Nodo de tipo descendiente.
    /// 
    /// # Returns
    /// `true` si `ancestor` es ancestro de `descendant`, `false` en caso contrario.
    pub fn is_ancestor(&self, ancestor: &TypeNode, descendant: &TypeNode) -> bool {
        let mut current = Some(descendant);
        while let Some(node) = current {
            if node.type_name == ancestor.type_name {
                return true;
            }
            current = node
                .parent
                .as_ref()
                .and_then(|parent_name| self.nodes.get(parent_name));
        }
        false
    }

    /// Detecta ciclos de herencia en el árbol de tipos.
    ///
    /// # Returns
    /// El nombre del tipo donde se detectó el ciclo, o `None` si no hay ciclos.
    pub fn inheritance_cicle(&mut self) -> Option<String> {
        let mut visited = HashMap::new();
        for (type_name, _) in self.nodes.clone() {
            if !visited.contains_key(&type_name) {
                if let Some(cycle_node) = self.helper(type_name, &mut visited) {
                    return Some(cycle_node);
                }
            }
        }
        return None;
    }

    /// Función auxiliar para la detección de ciclos de herencia y actualización de profundidad.
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

    /// Busca un método en un tipo y, si no existe, recorre la jerarquía de herencia hacia arriba.
    ///
    /// # Arguments
    /// * `node_name` - Nombre del tipo donde iniciar la búsqueda.
    /// * `method_name` - Nombre del método a buscar.
    /// 
    /// # Returns
    /// El método encontrado como `Box<FunctionDef>`, o `None` si no existe en la jerarquía.
    pub fn find_method(
        &mut self,
        node_name: String,
        method_name: String,
    ) -> Option<Box<FunctionDef>> {
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
