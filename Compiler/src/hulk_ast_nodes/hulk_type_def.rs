use std::collections::HashMap;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::hulk_ast_nodes::hulk_function_def::{FunctionDef, FunctionParams};
use crate::hulk_ast_nodes::hulk_identifier::Identifier;
use crate::hulk_ast_nodes::hulk_inheritance::Inheritance;
use crate::typings::types_node::TypeNode;

#[derive(Debug, Clone)]
pub struct HulkTypeNode {
    pub type_name: String,
    pub parent: Option<String>,
    pub parent_args: Vec<Expr>,
    pub parameters: Vec<FunctionParams>, // Parámetros genéricos, si los hay
    pub inheritance_option: Option<Inheritance>, // Herencia opcional
    pub attributes: HashMap<String, AttributeDef>,   // Atributos privados
    pub methods: HashMap<String, FunctionDef>,       // Métodos públicos y virtuales
    pub _type: Option<TypeNode>
}


#[derive(Debug, Clone)]
pub struct AttributeDef {
    pub name: Identifier,
    pub init_expr: Box<Expr>, 
}

impl HulkTypeNode {
    pub fn new(type_name: String, parent: Option<String>, parent_args: Vec<Expr>, parameters: Vec<FunctionParams>) -> Self {
        HulkTypeNode {
            type_name,
            parent,
            parent_args,
            parameters,
            inheritance_option: None,
            attributes: HashMap::new(),
            methods: HashMap::new(),
            _type:None
        }
    }

    pub fn set_inheritance(&mut self, inheritance: Inheritance) {
        self.inheritance_option = Some(inheritance);
    }
}

impl HulkTypeNode {
    pub fn with_members(mut self, members: (Vec<AttributeDef>, Vec<FunctionDef>)) -> Self {
        for attr in members.0 {
            self.attributes.insert(attr.name.to_string(), attr);
        }
        for method in members.1 {
            self.methods.insert(method.name.clone(), method);
        }
        self
    }

        pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}
