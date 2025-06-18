//! # HulkTypeNode y AttributeDef AST Nodes
//!
//! Este módulo define los nodos `HulkTypeNode` y `AttributeDef` del AST para el compilador Hulk.
//! Permite representar la definición de tipos (clases) en el lenguaje Hulk, incluyendo herencia, parámetros, atributos y métodos.
//! Incluye métodos para construir tipos, agregar herencia, atributos y métodos, y establecer el tipo inferido o declarado.

use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::hulk_ast_nodes::hulk_function_def::{FunctionDef, FunctionParams};
use crate::hulk_ast_nodes::hulk_identifier::Identifier;
use crate::hulk_ast_nodes::hulk_inheritance::Inheritance;
use crate::typings::types_node::TypeNode;
use std::collections::HashMap;

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
    pub fn new(
        type_name: String,
        parent: Option<String>,
        parent_args: Vec<Expr>,
        parameters: Vec<FunctionParams>,
    ) -> Self {
        HulkTypeNode {
            type_name,
            parent,
            parent_args,
            parameters,
            inheritance_option: None,
            attributes: HashMap::new(),
            methods: HashMap::new(),
            _type: None,
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

    pub fn generate_type_table(&self, context: &mut CodegenContext) {
        let type_name = self.type_name.clone();
        let mut methods_list = Vec::new();
        let mut methods_index = 0;
        for method in self.methods {
            if let Some(llvm_name) = context
                .function_member_llvm_names
                .get_mut(&(type_name.clone(), method.0.clone()))
            {
                methods_list.push(llvm_name.clone());
                context
                    .type_functions_ids
                    .insert((type_name.clone(), method.0.clone()), methods_index);
                methods_index += 1;
            }
        }

        let table = format!("%{}_vtable", type_name);
        let ptr_types = std::iter::repeat("ptr")
            .take(methods_index as usize)
            .collect::<Vec<_>>()
            .join(", ");
        context.emit(&format!("{} = type {{ {} }}", table, ptr_types));
    }

    pub fn generate_type_constructor(&self, context: &mut CodegenContext) {
        let type_name = self.type_name.clone();
        let type_reg = format!("%{}_type", type_name);
        let mut params_list = Vec::new();
        context.enter_scope();
        for param in self.parameters {
            let param_name = format!("%{}.{}", param.name.clone(), context.get_scope());
            params_list.push(format!("ptr {}", param_name.clone()));
            context.add_variable(
                param_name.clone(),
                CodegenContext::to_llvm_type(param.param_type.clone()),
            );
        }
        let params_str = params_list.join(", ");

        let mut methods_list = Vec::new();
        for method in self.methods {
            if let Some(llvm_name) = context
                .function_member_llvm_names
                .get_mut(&(type_name.clone(), method.0.clone()))
            {
                methods_list.push(llvm_name.clone());
            }
        }

        let table = format!("%{}_vtable", type_name);

        let table_id = context.new_id();
        let type_table_instance = format!("@{}_vtable{}", type_name, table_id);

        let method_ptrs = methods_list
            .iter()
            .map(|llvm_name| format!("ptr {}", llvm_name))
            .collect::<Vec<_>>()
            .join(", ");
        context.emit(&format!(
            "{} = global {} {{ {} }}",
            type_table_instance, table, method_ptrs
        )); //Esto va en el constructor 

        context.emit(&format!(
            "define ptr @{}_new( {} ) {{",
            type_name.clone(),
            params_str.clone()
        ));

        let size_temp = context.new_temp("Number".to_string());
        context.emit(&format!(
            "{} = ptrtoint ptr getelementptr({}, ptr null, i32 1) to i64",
            size_temp, type_reg
        ));
        let mem_temp = context.new_temp(type_name.clone());
        context.emit(&format!(
            "{} = call ptr @malloc(i64 {})",
            mem_temp, size_temp
        ));

        context.emit(&format!(
            "%vtable_ptr = getelementptr {}, ptr {}, i32 0, i32 0",
            type_reg, mem_temp
        ));
        context.emit(&format!(
            "store ptr {}, ptr %vtable_ptr",
            type_table_instance
        ));

        if let Some(parent_name) = self.parent.clone() {
            let mut parent_args_values = Vec::new();
            for arg in self.parent_args.iter_mut() {
                let arg_result = arg.codegen(context);
                let arg_reg = context.new_temp(arg_result);
                context.emit(&format!(
                    "{} = alloca {}",
                    arg_reg.clone(),
                    arg_result.llvm_type.clone()
                ));
                context.emit(&format!(
                    "store {} {}, ptr {}",
                    arg_result.llvm_type,
                    arg_result.register,
                    arg_reg.clone()
                ));
                parent_args_values.push(format!("ptr {}", arg_reg.clone()));
            }
            let args_regs_str = parent_args_values.join(", ");
            let parent_ptr = context.new_temp(parent_name.clone());
            let parent_constructor_name = format!("@{}_new", parent_name.clone());
            context.emit(&format!(
                "{} = call ptr {}({})",
                parent_ptr.clone(),
                parent_constructor_name,
                args_regs_str
            ));
            context.emit(&format!(
                "%parent_ptr = getelementptr {}, ptr {}, i32 0, i32 1",
                type_reg, mem_temp
            ));
            context.emit(&format!(
                "store ptr {}, ptr %parent_ptr",
                parent_ptr.clone()
            ));
        }

        for attribute in self.attributes {
            let prop_reg = attribute.1.init_expr.codegen(context);
            let result_reg = context.new_temp(prop_reg.clone());
            let member_key = (type_name.clone(), attribute.1.name.clone());
            let member_index = context
                .type_members_ids
                .get(&member_key)
                .expect("Member index not found for type and param name");
            context.emit(&format!(
                "{} = getelementptr {}, ptr {}, i32 0, i32 {}",
                result_reg, type_reg, mem_temp, member_index
            ));
            context.emit(&format!(
                "store {} {}, ptr {}",
                prop_reg.llvm_type, prop_reg.register, result_reg
            ));
        }

        context.emit(&format!("ret ptr {}", mem_temp));
        context.emit(&"}".to_string());
    }
}

impl Codegen for HulkTypeNode {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        let type_name = self.type_name.clone();
        let mut props_types = Vec::new();
        for attr in self.attributes {
            props_types.push(CodegenContext::to_llvm_type(
                attr.1.name._type.clone().expect("Tipo de atributo no determinado").to_string(),
            ));
        }
        let list_props_str = props_types
            .iter()
            .map(|llvm_name| format!("{}", llvm_name))
            .collect::<Vec<_>>()
            .join(", ");
        // type (vtable , parent , props...)
        if props_types.len() > 0 {
            context.emit(&format!(
                "%{}_type = type {{ ptr, ptr, {} }}",
                type_name.clone(),
                list_props_str
            ));
        } else {
            context
                .emit(&format!("%{}_type = type {{ ptr, ptr }}", type_name.clone()));
        }
        self.generate_type_table(context);
        self.generate_type_constructor(context);
        context.current_self = Some(self.type_name.clone());
        for method in self.methods {
           
                 
                     let method.1 = format!("{}_{}", type_name.clone(), method.name.clone());
                    visit_function_def(method);
                }
    
        context.current_self = None;
        String::new();
    }
}
