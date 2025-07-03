use crate::hulk_ast_nodes::hulk_program::ProgramNode;
use crate::hulk_ast_nodes::hulk_type_def::HulkTypeNode;
use std::collections::{HashMap, HashSet};

pub struct TypesGlobal {
    pub inheritance_map: HashMap<String, Option<String>>,
    pub attributes_map: HashMap<String, Vec<String>>,
    pub methods_map: HashMap<String, Vec<String>>,
    pub attribute_indices: HashMap<String, HashMap<String, usize>>,
    pub method_indices: HashMap<String, HashMap<String, usize>>,
    // Puedes agregar más campos si es necesario
}

impl TypesGlobal {
    /// Recorre todas las definiciones de tipos y almacena la relación de herencia.
    pub fn register_inheritance(program: &ProgramNode) -> HashMap<String, Option<String>> {
        let mut inheritance_map = HashMap::new();
        for def in &program.definitions {
            if let Some(ty) = def.as_type_def() {
                inheritance_map.insert(ty.type_name.clone(), ty.parent.clone());
            }
        }
        inheritance_map
    }

    /// Registra los miembros (atributos y métodos) de cada tipo, incluyendo los heredados.
    pub fn register_members(
        program: &ProgramNode,
        inheritance_map: &HashMap<String, Option<String>>,
    ) -> (HashMap<String, Vec<String>>, HashMap<String, Vec<String>>) {
        // Primero, mapea el nombre del tipo a su HulkTypeNode
        let mut type_map = HashMap::new();
        for def in &program.definitions {
            if let Some(ty) = def.as_type_def() {
                type_map.insert(ty.type_name.clone(), ty);
            }
        }
        // Función auxiliar para obtener atributos heredados y propios
        fn collect_attributes(
            type_name: &str,
            type_map: &HashMap<String, &HulkTypeNode>,
            inheritance_map: &HashMap<String, Option<String>>,
        ) -> Vec<String> {
            let mut attrs = Vec::new();
            // Hereda primero
            if let Some(Some(parent)) = inheritance_map.get(type_name) {
                attrs.extend(collect_attributes(parent, type_map, inheritance_map));
            }
            // Luego agrega los propios
            if let Some(ty) = type_map.get(type_name) {
                for key in ty.attributes.keys() {
                    if !attrs.contains(key) {
                        attrs.push(key.clone());
                    }
                }
            }
            attrs
        }
        // Función auxiliar para obtener métodos heredados y propios (sobrescribe si redefine)
        fn collect_methods(
            type_name: &str,
            type_map: &HashMap<String, &HulkTypeNode>,
            inheritance_map: &HashMap<String, Option<String>>,
        ) -> Vec<String> {
            let mut methods = Vec::new();
            // Hereda primero
            if let Some(Some(parent)) = inheritance_map.get(type_name) {
                methods.extend(collect_methods(parent, type_map, inheritance_map));
            }
            // Luego agrega/reemplaza los propios
            if let Some(ty) = type_map.get(type_name) {
                for key in ty.methods.keys() {
                    // Si ya existe, reemplaza (sobrescribe)
                    if let Some(pos) = methods.iter().position(|m| m == key) {
                        methods[pos] = key.clone();
                    } else {
                        methods.push(key.clone());
                    }
                }
            }
            methods
        }
        let mut attributes_map = HashMap::new();
        let mut methods_map = HashMap::new();
        for type_name in type_map.keys() {
            attributes_map.insert(
                type_name.clone(),
                collect_attributes(type_name, &type_map, inheritance_map),
            );
            methods_map.insert(
                type_name.clone(),
                collect_methods(type_name, &type_map, inheritance_map),
            );
        }
        (attributes_map, methods_map)
    }

    /// Asigna índices a los atributos y métodos de cada tipo
    pub fn assign_indices(
        attributes_map: &HashMap<String, Vec<String>>,
        methods_map: &HashMap<String, Vec<String>>,
    ) -> (HashMap<String, HashMap<String, usize>>, HashMap<String, HashMap<String, usize>>) {
        let mut attribute_indices = HashMap::new();
        let mut method_indices = HashMap::new();
        for (type_name, attrs) in attributes_map {
            let mut idx_map = HashMap::new();
            for (i, attr) in attrs.iter().enumerate() {
                idx_map.insert(attr.clone(), i);
            }
            attribute_indices.insert(type_name.clone(), idx_map);
        }
        for (type_name, methods) in methods_map {
            let mut idx_map = HashMap::new();
            for (i, method) in methods.iter().enumerate() {
                idx_map.insert(method.clone(), i);
            }
            method_indices.insert(type_name.clone(), idx_map);
        }
        (attribute_indices, method_indices)
    }

    /// Crea una instancia de TypesGlobal registrando las herencias, miembros e índices
    pub fn from_program(program: &ProgramNode) -> Self {
        let inheritance_map = Self::register_inheritance(program);
        let (attributes_map, methods_map) = Self::register_members(program, &inheritance_map);
        let (attribute_indices, method_indices) = Self::assign_indices(&attributes_map, &methods_map);
        TypesGlobal {
            inheritance_map,
            attributes_map,
            methods_map,
            attribute_indices,
            method_indices,
        }
    }
}

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
        // Ordenamos el resultado por nombre para garantizar un orden consistente
        result.sort_by(|a, b| a.type_name.cmp(&b.type_name));
        result
    }
}
