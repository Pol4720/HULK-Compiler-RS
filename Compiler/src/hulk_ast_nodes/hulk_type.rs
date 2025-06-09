use std::collections::HashMap;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::hulk_ast_nodes::hulk_function_def::FunctionDef;
use crate::hulk_ast_nodes::hulk_identifier::Identifier;
use crate::hulk_ast_nodes::hulk_inheritance::Inheritance;

/// Representa un tipo en HULK
#[derive(Debug, Clone)]
pub struct HulkTypeNode {
    pub type_name: String,
    pub parameters: Vec<Identifier>, // Parámetros genéricos, si los hay
    pub inheritance_option: Option<Inheritance>, // Herencia opcional
    pub attributes: HashMap<String, AttributeDef>,   // Atributos privados
    pub methods: HashMap<String, FunctionDef>,       // Métodos públicos y virtuales
}


#[derive(Debug, Clone)]
pub struct AttributeDef {
    pub name: Identifier,
    pub init_expr: Box<Expr>, 
}

impl HulkTypeNode {
    pub fn new(
        type_name: String,
        parameters: Vec<Identifier>,
        inheritance_option: Option<Inheritance>,
    ) -> Self {
        HulkTypeNode {
            type_name,
            parameters,
            inheritance_option,
            attributes: HashMap::new(),
            methods: HashMap::new(),
        }
    }
}
