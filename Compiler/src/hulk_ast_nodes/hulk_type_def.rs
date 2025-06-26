//! # HulkTypeNode y AttributeDef AST Nodes
//!
//! Este módulo define los nodos `HulkTypeNode` y `AttributeDef` del AST para el compilador Hulk.
//! Permite representar la definición de tipos (clases) en el lenguaje Hulk, incluyendo herencia, parámetros, atributos y métodos.
//! Incluye métodos para construir tipos, agregar herencia, atributos y métodos, y establecer el tipo inferido o declarado.

use std::collections::HashMap;
use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::hulk_ast_nodes::hulk_function_def::{FunctionDef, FunctionParams};
use crate::hulk_ast_nodes::hulk_identifier::Identifier;
use crate::hulk_ast_nodes::hulk_inheritance::Inheritance;
use crate::hulk_ast_nodes::Assignment;
use crate::hulk_tokens::TokenPos;
use crate::typings::types_node::TypeNode;

/// Representa la definición de un tipo (clase) en el AST.
/// 
/// - `type_name`: nombre del tipo.
/// - `parent`: nombre del tipo padre (si hay herencia).
/// - `parent_args`: argumentos para el constructor del padre.
/// - `parameters`: parámetros del tipo (por ejemplo, genéricos o del constructor).
/// - `inheritance_option`: información detallada de herencia (opcional).
/// - `attributes`: atributos (propiedades) del tipo.
/// - `methods`: métodos definidos en el tipo.
/// - `_type`: tipo inferido o declarado del tipo (opcional).
#[derive(Debug, Clone)]
pub struct HulkTypeNode {
    pub type_name: String,
    pub parent: Option<String>,
    pub parent_args: Vec<Expr>,
    pub parameters: Vec<FunctionParams>, 
    pub inheritance_option: Option<Inheritance>, 
    pub attributes: HashMap<String, AttributeDef>,   
    pub methods: HashMap<String, FunctionDef>,       
    pub _type: Option<TypeNode>,
    pub token_pos: TokenPos,
}

/// Representa un atributo (propiedad) de un tipo en el AST.
/// 
/// - `name`: identificador del atributo.
/// - `init_expr`: expresión de inicialización del atributo.
#[derive(Debug, Clone)]
pub struct AttributeDef {
    pub name: Identifier,
    pub init_expr:Assignment, 
}

impl HulkTypeNode {
    /// Crea una nueva definición de tipo.
    ///
    /// # Arguments
    /// * `type_name` - Nombre del tipo.
    /// * `parent` - Nombre del tipo padre (opcional).
    /// * `parent_args` - Argumentos para el constructor del padre.
    /// * `parameters` - Parámetros del tipo.
    pub fn new(type_name: String, parent: Option<String>, parent_args: Vec<Expr>, parameters: Vec<FunctionParams>, token_pos: TokenPos) -> Self {
        HulkTypeNode {
            type_name,
            parent,
            parent_args,
            parameters,
            inheritance_option: None,
            attributes: HashMap::new(),
            methods: HashMap::new(),
            _type: None,
            token_pos,
        }
    }

    /// Establece la información de herencia para el tipo.
    pub fn set_inheritance(&mut self, inheritance: Inheritance) {
        self.inheritance_option = Some(inheritance);
    }

    /// Agrega atributos y métodos al tipo.
    ///
    /// # Arguments
    /// * `members` - Tupla con un vector de atributos y un vector de métodos.
    pub fn with_members(mut self, members: (Vec<AttributeDef>, Vec<FunctionDef>)) -> Self {
        for attr in members.0 {
            self.attributes.insert(attr.name.to_string(), attr);
        }
        for method in members.1 {
            self.methods.insert(method.name.clone(), method);
        }
        self
    }

    /// Establece el tipo inferido o declarado del tipo.
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}

impl Codegen for HulkTypeNode {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        String::new() 
    }
}

