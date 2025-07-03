//! # ProgramNode e Instruction AST Nodes
//!
//! Este módulo define los nodos `ProgramNode` e `Instruction` del AST para el compilador Hulk.
//! `ProgramNode` representa el nodo raíz del AST, que contiene todas las instrucciones de alto nivel de un programa Hulk.
//! `Instruction` es un enum que agrupa las posibles instrucciones de nivel superior: definición de tipos, funciones y expresiones.
//! Ambos nodos soportan integración con el visitor pattern y la generación de código LLVM IR.

use std::collections::HashMap;

use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::codegen::types_global::TypesGlobal;
use crate::hulk_ast_nodes::GlobalFunctionDef;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::hulk_ast_nodes::hulk_type_def::HulkTypeNode;
use crate::visitor::hulk_accept::Accept;
use crate::visitor::hulk_visitor::Visitor;

/// Nodo raíz del AST que representa un programa completo.
///
/// Contiene una lista de instrucciones de alto nivel (definiciones de tipos, funciones y expresiones).
#[derive(Debug, Clone)]
pub struct ProgramNode {
    pub instructions: Vec<Expr>,
    pub definitions: Vec<Definition>,
}

impl ProgramNode {
    /// Crea un nuevo nodo de programa con una lista de instrucciones.
    pub fn new(instructions: Vec<Expr>, definitions: Vec<Definition>) -> Self {
        ProgramNode {
            instructions,
            definitions,
        }
    }

    /// Genera la función global de acceso a métodos virtuales (VTable)
    fn get_vtable_method(context: &mut CodegenContext, count_types: usize, max_functions: usize) {
        context.emit_global("define ptr @get_vtable_method(i32 %type_id, i32 %method_id) {" );
        context.emit_global(&format!(
            "  %vtable_ptr_ptr = getelementptr [{} x ptr], ptr @super_vtable, i32 0, i32 %type_id",
            count_types
        ));
        context.emit_global("  %vtable_ptr = load ptr, ptr %vtable_ptr_ptr");
        context.emit_global("  %typed_vtable = bitcast ptr %vtable_ptr to ptr");
        context.emit_global(&format!(
            "  %method_ptr = getelementptr [{} x ptr], ptr %typed_vtable, i32 0, i32 %method_id",
            max_functions
        ));
        context.emit_global("  %method = load ptr, ptr %method_ptr");
        context.emit_global("  ret ptr %method");
        context.emit_global("}");
    }

    /// Registra la información de tipos, miembros y métodos en el contexto de generación de código.
    pub fn generate_type_tables_for_node(context: &mut CodegenContext, type_node: &HulkTypeNode) {
        let type_name = type_node.type_name.clone();
        let mut member_index: i32 = 2; // 0: vtable, 1: parent, 2...: miembros
        let mut props_list = Vec::new();

        // Herencia: si tiene padre, copia miembros del padre
        if let Some(parent_name) = &type_node.parent {
            if let Some(parent_members) = context.types_members_functions.get(&(parent_name.clone(), String::new(), 0)) {
                for (index, member_name) in parent_members.iter().enumerate() {
                    // Aquí asumimos que tienes una forma de obtener el tipo del miembro del padre
                    if let Some(member_type) = context.type_members_types.get(&(parent_name.clone(), member_name.clone())) {
                        context.type_members_types.insert((type_name.clone(), member_name.clone()), member_type.clone());
                        context.type_members_ids.insert((type_name.clone(), member_name.clone()), index as i32);
                        member_index += 1;
                    }
                    props_list.push((member_name.clone(), context.type_members_types.get(&(type_name.clone(), member_name.clone())).cloned().unwrap_or_default()));
                }
            }
            if let Some(parent) = &type_node.parent {
                context.inherits.insert(type_name.clone(), parent.clone());
            }
        }

        // Miembros propios
        // Primero atributos
        for (member_name, attr_def) in &type_node.attributes {
            let member_type = attr_def.init_expr._type.as_ref().map(|t| t.type_name.clone()).unwrap_or_default();
            context.type_members_ids.insert((type_name.clone(), member_name.clone()), member_index);
            context.type_members_types.insert((type_name.clone(), member_name.clone()), member_type.clone());
            props_list.push((member_name.clone(), member_type));
            member_index += 1;
        }
        // Luego métodos
        for (method_name, method_def) in &type_node.methods {
            context.function_member_llvm_names.insert((type_name.clone(), method_name.clone()), format!("@{}_{}", type_name, method_name));
            let method_args_types: Vec<String> = method_def.params.iter().map(|p| p.param_type.clone()).collect();
            context.types_members_functions.insert((type_name.clone(), method_name.clone(), member_index), method_args_types);
            if let Some(ret_type) = &method_def._type {
                context.type_members_types.insert((type_name.clone(), method_name.clone()), ret_type.type_name.clone());
            }
        }

        // Registra lista de miembros para el tipo
        // (puedes ajustar la clave según tu diseño)
        context.types_members_functions.insert((type_name.clone(), String::new(), 0), props_list.iter().map(|(n, _)| n.clone()).collect());

        // Registra tipos de argumentos del constructor
        let params_types_list: Vec<String> = type_node.parameters.iter().map(|p| p.param_type.clone()).collect();
        context.constructor_args_types.insert(type_name.clone(), params_types_list);
        // Guarda también los nombres de los parámetros del constructor
        let params_names_list: Vec<String> = type_node.parameters.iter().map(|p| p.name.clone()).collect();
        context.constructor_args_names.insert(type_name.clone(), params_names_list);

        // Emite el tipo LLVM para el struct
        let props_types: Vec<String> = props_list.iter().map(|(_, t)| CodegenContext::to_llvm_type(t.clone())).collect();
        let list_props_str = props_types.join(", ");
        if !props_types.is_empty() {
            context.emit_global(&format!("%{}_type = type {{ i32, ptr, {} }}", type_name, list_props_str));
        } else {
            context.emit_global(&format!("%{}_type = type {{ i32, ptr }}", type_name));
        }
    }
}

impl Accept for ProgramNode {
    /// Permite que el nodo acepte un visitor.
    fn accept<V: Visitor<T>, T>(&mut self, visitor: &mut V) -> T {
        visitor.visit_program(self)
    }
}

#[derive(Debug, Clone)]
pub enum Definition {
    TypeDef(HulkTypeNode),
    FunctionDef(GlobalFunctionDef),
}

impl Definition {
    pub fn as_type_def(&self) -> Option<&HulkTypeNode> {
        if let Self::TypeDef(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_function_def(&self) -> Option<&GlobalFunctionDef> {
        if let Self::FunctionDef(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl From<GlobalFunctionDef> for Definition {
    fn from(v: GlobalFunctionDef) -> Self {
        Self::FunctionDef(v)
    }
}

impl From<HulkTypeNode> for Definition {
    fn from(v: HulkTypeNode) -> Self {
        Self::TypeDef(v)
    }
}

impl Accept for Definition {
    fn accept<V: Visitor<T>, T>(&mut self, visitor: &mut V) -> T {
        match self {
            Self::FunctionDef(func_def) => visitor.visit_function_def(&mut func_def.function_def),
            Definition::TypeDef(type_node) => visitor.visit_type_def(type_node),
        }
    }
}

impl Codegen for ProgramNode {
    /// Genera el código LLVM IR para todo el programa.
    ///
    /// Recorre todas las instrucciones y genera el código correspondiente.
    fn codegen(&self, context: &mut CodegenContext) -> String {
        let mut last_reg = String::new();

        // Registra la herencia y los miembros de los tipos antes de procesar las definiciones
        let type_defs = TypesGlobal::from_program(&self);

        // --- ACTUALIZA LOS HASHMAPS DE TIPOS, MIEMBROS Y MÉTODOS EN EL CONTEXTO ---
        // Esto asegura que context siempre tenga la información de tipos, atributos y métodos
        // y que los índices y tipos sean correctos para acceso y codegen posterior
        for def in &self.definitions {
            if let Some(type_node) = def.as_type_def() {
                let type_name = &type_node.type_name;
                // Herencia
                if let Some(parent) = type_defs.inheritance_map.get(type_name) {
                    if let Some(parent_name) = parent {
                        context.inherits.insert(type_name.clone(), parent_name.clone());
                    }
                }
                // Constructor args
                let params_types_list: Vec<String> = type_node.parameters.iter().map(|p| p.param_type.clone()).collect();
                context.constructor_args_types.insert(type_name.clone(), params_types_list);
                // Guarda también los nombres de los parámetros del constructor
                let params_names_list: Vec<String> = type_node.parameters.iter().map(|p| p.name.clone()).collect();
                context.constructor_args_names.insert(type_name.clone(), params_names_list);
                // Atributos
                if let Some(attr_names) = type_defs.attributes_map.get(type_name) {
                    for attr in attr_names {
                        // Busca el tipo concreto del atributo
                        if let Some(attr_def) = type_node.attributes.get(attr) {
                            let attr_type = attr_def.init_expr._type.as_ref().map(|t| t.type_name.clone()).unwrap_or_default();
                            context.type_members_types.insert((type_name.clone(), attr.clone()), attr_type);
                        }
                        // Índice del atributo
                        if let Some(idx) = type_defs.attribute_indices.get(type_name).and_then(|m| m.get(attr)) {
                            context.type_members_ids.insert((type_name.clone(), attr.clone()), *idx as i32 + 2); // +2 por vtable y parent
                        }
                    }
                }
                // Métodos
                if let Some(method_names) = type_defs.methods_map.get(type_name) {
                    for method in method_names {
                        // Busca el método concreto
                        if let Some(method_def) = type_node.methods.get(method) {
                            // Argumentos del método
                            let method_args_types: Vec<String> = method_def.params.iter().map(|p| p.param_type.clone()).collect();
                            // Usa el índice correcto para la clave
                            if let Some(idx) = type_defs.method_indices.get(type_name).and_then(|m| m.get(method)) {
                                context.types_members_functions.insert((type_name.clone(), method.clone(), *idx as i32), method_args_types);
                                context.type_functions_ids.insert((type_name.clone(), method.clone()), *idx as i32);
                            }
                            // Tipo de retorno del método
                            if let Some(ret_type) = &method_def._type {
                                context.type_members_types.insert((type_name.clone(), method.clone()), ret_type.type_name.clone());
                            }
                            // Nombre LLVM del método
                            context.function_member_llvm_names.insert((type_name.clone(), method.clone()), format!("@{}_{}", type_name, method));
                        }
                    }
                }
            }
        }

        // --- DEFINICIÓN DE VTABLE GLOBAL Y TIPO ---
        // Calcula el máximo de funciones (columnas de la vtable) usando type_defs
        let max_functions = type_defs.methods_map.values()
            .map(|methods| methods.len())
            .max()
            .unwrap_or(0)
            .max(1); // Al menos 1 para evitar error

        // Calcula la cantidad de tipos (filas de la vtable) usando type_defs
        let count_types = type_defs.methods_map.len().max(1);

        // Emite el tipo de la vtable
        context.emit_global(&format!("%VTableType = type [{} x ptr]", max_functions));
        context.max_function = max_functions;
        
        // Creamos un mapa que asigna cada nombre de tipo a su índice en la vtable
        let mut type_id_map = HashMap::new();
        
        // Recopilamos los nombres de los tipos y los ordenamos para garantizar consistencia
        let mut type_names: Vec<String> = type_defs.methods_map.keys().cloned().collect();
        // Ordenamos alfabéticamente para garantizar un orden determinista
        type_names.sort();
        
        // Asignamos índices secuenciales a cada tipo
        for (index, type_name) in type_names.iter().enumerate() {
            type_id_map.insert(type_name.clone(), index as i32);
            // Guardar en el contexto para que esté disponible durante la generación de código
            context.type_ids.insert(type_name.clone(), index as i32);
            
            // También registramos el ID del tipo como una función especial
            context.type_functions_ids.insert((type_name.clone(), "__typeid__".to_string()), index as i32);
        }

        // Emite la declaración global de la super vtable - esto no cambia
        let vtable_declarations: Vec<String> = type_names
            .iter()
            .map(|type_name| format!("@{}_vtable", type_name))
            .collect();
        
        // Fix para el caso de que no haya tipos definidos
        let vtable_init = if vtable_declarations.is_empty() {
            "ptr null".to_string()  // Proporciona al menos un elemento
        } else {
            vtable_declarations
                .iter()
                .map(|v| format!("ptr {}", v))
                .collect::<Vec<_>>()
                .join(", ")
        };
        
        context.emit_global(&format!(
            "@super_vtable = global [{} x ptr] [{}]",
            count_types,
            vtable_init
        ));
        // Llama a la función auxiliar para definir get_vtable_method
        ProgramNode::get_vtable_method(context, count_types, max_functions);
   

        // Procesa todas las definiciones (funciones y tipos)
        for def in self.definitions.iter() {
            match def {
                Definition::FunctionDef(func_def) => {
                    func_def.codegen(context);
                }
                Definition::TypeDef(type_def) => {
                    let type_name = &type_def.type_name;
                    let attrs = type_defs.attributes_map.get(type_name);
                    let methods = type_defs.methods_map.get(type_name);
                    let attr_indices = type_defs.attribute_indices.get(type_name);
                    let method_indices = type_defs.method_indices.get(type_name);

                    let mut type_def_mut = type_def.clone();

                    type_def_mut.codegen_with_type_info(
                        context,
                        attrs,
                        methods,
                        attr_indices,
                        method_indices,
                    );
                }
            }
        }

        // Luego genera el código de las instrucciones ejecutables (main, prints, exprs, etc)
        for instr in &self.instructions {
            last_reg = instr.codegen(context);
        }

        last_reg
    }
    

}
