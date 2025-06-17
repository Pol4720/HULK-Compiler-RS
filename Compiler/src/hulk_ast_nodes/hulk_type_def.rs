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
    pub _type: Option<TypeNode>
}

/// Representa un atributo (propiedad) de un tipo en el AST.
/// 
/// - `name`: identificador del atributo.
/// - `init_expr`: expresión de inicialización del atributo.
#[derive(Debug, Clone)]
pub struct AttributeDef {
    pub name: Identifier,
    pub init_expr: Box<Expr>, 
}

impl HulkTypeNode {
    /// Crea una nueva definición de tipo.
    ///
    /// # Arguments
    /// * `type_name` - Nombre del tipo.
    /// * `parent` - Nombre del tipo padre (opcional).
    /// * `parent_args` - Argumentos para el constructor del padre.
    /// * `parameters` - Parámetros del tipo.
    pub fn new(type_name: String, parent: Option<String>, parent_args: Vec<Expr>, parameters: Vec<FunctionParams>) -> Self {
        HulkTypeNode {
            type_name,
            parent,
            parent_args,
            parameters,
            inheritance_option: None,
            attributes: HashMap::new(),
            methods: HashMap::new(),
            _type: None
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

// impl Codegen for HulkTypeNode {
//     fn codegen(&self, context: &mut CodegenContext) -> String {
//         let mut method_map: HashMap<String, String> = HashMap::new();

//         for (method_name, method_def) in &self.methods {
//             // Genera un nombre único para evitar conflictos
//             let unique_name = format!("{}_{}", self.type_name, method_name);

//             // Clona y ajusta la función (si es necesario agregar el parámetro `self`)
//             let mut method_clone = method_def.clone();

//             // Registro en la VTable
//             context.register_method(&self.type_name, method_name, &unique_name);

//             // Emitir función global
//             let llvm_code = method_clone.codegen_with_name_override(context, &unique_name);

//             // Se almacena como código global
//             context.emit_global(&llvm_code);

//             method_map.insert(method_name.clone(), unique_name);
//         }

//         // Herencia: copiar métodos del padre si no están sobrescritos
//         if let Some(ref parent) = self.parent {
//             if let Some(parent_methods) = context.vtable.get(parent) {
//                 for (meth, func) in parent_methods {
//                     method_map.entry(meth.clone()).or_insert(func.clone());
//                 }
//             }
//         }

//         // Finalmente, actualiza la tabla virtual
//         context.vtable.insert(self.type_name.clone(), method_map);

//         String::new() // Nada se devuelve directamente
//     }
// }

impl Codegen for HulkTypeNode {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        let mut method_map: HashMap<String, String> = HashMap::new();
        let mut struct_fields: Vec<(String, String)> = vec![]; // (field_name, llvm_type)

        // === 1. Heredar atributos del padre ===
        if let Some(ref parent) = self.parent {
            if let Some(parent_fields) = context.struct_layouts.get(parent) {
                struct_fields.extend(parent_fields.clone());
            }
        }

        // === 2. Procesar atributos actuales ===
        for (name, attr) in &self.attributes {
            let attr_type = attr.name // Necesita implementar .get_type() o similar
            let llvm_type = CodegenContext::to_llvm_type(attr_type.to_string());
            struct_fields.push((name.clone(), llvm_type));
        }

        // === 3. Registrar layout en contexto ===
        context.register_type_struct_layout(&self.type_name, struct_fields.clone());

        // === 4. Definir struct LLVM ===
        let struct_llvm_def = context.allocate_struct_type(&self.type_name, struct_fields.clone());
        context.emit_global(&struct_llvm_def);

        // === 5. Emitir constructor "__init__" ===
        let ctor_name = format!("{}_init", self.type_name);
        let mut ctor_body = vec![];
        let mut param_strs = vec![];

        // Agregar parámetros desde los definidos en el type
        for param in &self.parameters {
            let llvm_type = CodegenContext::to_llvm_type(param.param_type.to_string());
            param_strs.push(format!("{} %{}", llvm_type, param.name));
        }

        // Primer parámetro es un puntero a la instancia: %self
        let self_type = format!("%{}*", self.type_name);
        ctor_body.push(format!("define void @{}({} %self, {}) {{", ctor_name, self_type, param_strs.join(", ")));

        // Seteo de campos
        let mut field_index = if self.parent.is_some() {
            context.struct_layouts.get(self.parent.as_ref().unwrap()).unwrap().len()
        } else {
            0
        };

        for (name, attr) in &self.attributes {
            let value_reg = attr.init_expr.codegen(context);
            let llvm_type = CodegenContext::to_llvm_type(attr.init_expr.get_type().to_string());
            ctor_body.push(format!(
                "  %ptr_{} = getelementptr inbounds %{}, %{}* %self, i32 0, i32 {}",
                name, self.type_name, self.type_name, field_index
            ));
            ctor_body.push(format!(
                "  store {} {}, {}* %ptr_{}",
                llvm_type, value_reg, llvm_type, name
            ));
            field_index += 1;
        }

        ctor_body.push("  ret void".to_string());
        ctor_body.push("}".to_string());

        context.emit_global(&ctor_body.join("\n"));
        context.register_constructor(&self.type_name, &ctor_name);

        // === 6. Procesar métodos ===
        for (method_name, method_def) in &self.methods {
            let unique_name = format!("{}_{}", self.type_name, method_name);
            let mut method_clone = method_def.clone();
            context.register_method(&self.type_name, method_name, &unique_name);

            let llvm_code = method_clone.codegen_with_name_override(context, &unique_name);
            context.emit_global(&llvm_code);

            method_map.insert(method_name.clone(), unique_name);
        }

        // === 7. Herencia de métodos ===
        if let Some(ref parent) = self.parent {
            if let Some(parent_methods) = context.vtable.get(parent) {
                for (meth, func) in parent_methods {
                    method_map.entry(meth.clone()).or_insert(func.clone());
                }
            }
        }

        // === 8. Registrar VTable final ===
        context.vtable.insert(self.type_name.clone(), method_map);

        String::new()
    }
}


