//! # TypeNode
//!
//! Este módulo define la estructura `TypeNode` para el compilador Hulk.
//! `TypeNode` representa un nodo en el árbol de tipos del lenguaje Hulk, modelando la información de una clase o tipo definido por el usuario.
//! Incluye nombre, jerarquía de herencia, parámetros, atributos (variables) y métodos asociados.
//! Proporciona métodos para manipular la jerarquía y los miembros del tipo.

use std::collections::HashMap;

use crate::hulk_ast_nodes::hulk_function_def::FunctionParams;
use crate::hulk_ast_nodes::FunctionDef;

/// Representa un tipo (clase) en el árbol de tipos del compilador Hulk.
/// 
/// - `type_name`: nombre del tipo.
/// - `depth`: profundidad en la jerarquía de herencia.
/// - `params`: parámetros del tipo (por ejemplo, genéricos o del constructor).
/// - `parent`: nombre del tipo padre (si existe).
/// - `children`: nombres de los tipos hijos.
/// - `variables`: atributos del tipo (nombre → tipo como string).
/// - `methods`: métodos definidos en el tipo (nombre → definición de función).
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
    /// Crea un nuevo nodo de tipo.
    ///
    /// # Arguments
    /// * `type_name` - Nombre del tipo.
    /// * `params` - Parámetros del tipo.
    /// * `depth` - Profundidad en la jerarquía.
    /// * `parent` - Nombre del tipo padre (opcional).
    /// * `children` - Vector de nombres de tipos hijos.
    /// * `variables` - HashMap de atributos.
    /// * `methods` - HashMap de métodos.
    pub fn new(
        type_name: String,
        params: Vec<FunctionParams>,
        depth: i32,
        parent: Option<String>,
        children: Vec<String>,
        variables: HashMap<String, Box<String>>,
        methods: HashMap<String, Box<FunctionDef>>,
    ) -> Self {
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

    /// Agrega un hijo a la lista de hijos del tipo.
    pub fn add_child(&mut self, child_name: String) {
        self.children.push(child_name);
    }

    /// Establece el nombre del tipo padre.
    pub fn set_parent(&mut self, parent_name: String) {
        self.parent = Some(parent_name);
    }

    /// Agrega una variable (atributo) al tipo.
    pub fn add_variable(&mut self, name: String, variable: Box<String>) {
        self.variables.insert(name, variable);
    }

    /// Agrega un método al tipo.
    pub fn add_method(&mut self, name: String, method: Box<FunctionDef>) {
        self.methods.insert(name, method);
    }

    /// Obtiene un método por nombre, si existe.
    pub fn get_method(&mut self, method_name: &String) -> Option<Box<FunctionDef>> {
        if let Some(method) = self.methods.get(method_name) {
            Some(method.clone())
        } else {
           None
        }
    }
}