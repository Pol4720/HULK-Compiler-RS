//! # HulkTypeNode y AttributeDef AST Nodes
//!
//! Este módulo define los nodos `HulkTypeNode` y `AttributeDef` del AST para el compilador Hulk.
//! Permite representar la definición de tipos (clases) en el lenguaje Hulk, incluyendo herencia, parámetros, atributos y métodos.
//! Incluye métodos para construir tipos, agregar herencia, atributos y métodos, y establecer el tipo inferido o declarado.

use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_ast_nodes::Assignment;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::hulk_ast_nodes::hulk_function_def::{FunctionDef, FunctionParams};
use crate::hulk_ast_nodes::hulk_identifier::Identifier;
use crate::hulk_ast_nodes::hulk_inheritance::Inheritance;
use crate::hulk_tokens::TokenPos;
use crate::typings::types_node::TypeNode;
use std::cmp::max;
use std::collections::{HashMap, HashSet};


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
    pub init_expr: Assignment,
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
        token_pos: TokenPos,
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

impl HulkTypeNode {
    /// Genera código LLVM para el tipo con información de herencia y miembros,
    /// adaptado a la lógica de visit_type_def.
    pub fn codegen_with_type_info(
        &mut self,
        context: &mut CodegenContext,
        attrs: Option<&Vec<String>>,
        methods: Option<&Vec<String>>,
        attr_indices: Option<&HashMap<String, usize>>,
        method_indices: Option<&HashMap<String, usize>>,
    ) -> String {
        let type_name = self.type_name.clone();

        // Crear un nuevo contexto temporal para la generación del tipo
        let mut type_context = context.clone_for_type_codegen();
        type_context.current_self = Some(type_name.clone());

        self.generate_type_constructor(&mut type_context, attrs, methods, attr_indices, method_indices);

        // Primero generamos los métodos propios definidos en este tipo
        for (_name, method) in self.methods.iter_mut() {
            // Renombrar el método con el prefijo del tipo
            method.name = format!("{}_{}", type_name, method.name.clone());
            method.codegen(&mut type_context);
        }

        // Ahora generamos delegadores para los métodos heredados que no están sobrescritos
        if let (Some(parent_name), Some(all_methods)) = (&self.parent, methods) {
            // Obtenemos los nombres de los métodos definidos en este tipo
            let own_method_names: HashSet<String> = self.methods.keys().cloned().collect();
            
            // Para cada método en la lista completa (incluyendo heredados)
            for method_name in all_methods {
                // Si este método no está definido en este tipo, es heredado y necesitamos un delegador
                if !own_method_names.contains(method_name) {
                    // Generamos un delegador que llame al método del padre
                    self.generate_method_delegator(
                        &mut type_context, 
                        method_name, 
                        parent_name, 
                        method_indices.and_then(|m| m.get(method_name).cloned())
                    );
                }
            }
        }

        type_context.current_self = None;

        // Unificar el contexto temporal con el global
        context.merge_into_global(type_context);

        format!("%{}_type", type_name)
    }

    // Método auxiliar para generar delegadores para métodos heredados
    fn generate_method_delegator(
        &self,
        context: &mut CodegenContext,
        method_name: &str,
        parent_name: &str,
        method_index: Option<usize>
    ) {
        let type_name = self.type_name.clone();
        let child_method_name = format!("{}_{}", type_name, method_name);
        let parent_method_name = format!("@{}_{}", parent_name, method_name);
        
        // Obtenemos información del tipo de retorno y parámetros del método
        let return_type = context
            .type_members_types
            .get(&(parent_name.to_string(), method_name.to_string()))
            .cloned()
            .unwrap_or_else(|| "Number".to_string()); // Por defecto asumimos Number si no hay tipo
        
        let llvm_return_type = CodegenContext::to_llvm_type(return_type.clone());
        
        // Obtenemos los parámetros del método
        let param_types = if let Some(method_idx) = method_index {
            context
                .types_members_functions
                .get(&(parent_name.to_string(), method_name.to_string(), method_idx as i32))
                .cloned()
                .unwrap_or_default()
        } else {
            Vec::new()
        };
        
        // Construimos la lista de parámetros para la declaración del método
        let mut param_list = vec![format!("ptr %self")]; // El primer parámetro siempre es 'self'
        let mut arg_list = Vec::new(); // Argumentos para llamar al método padre (sin self)
        
        for (i, param_type) in param_types.iter().enumerate() {
            let llvm_type = CodegenContext::to_llvm_type(param_type.clone());
            let param_name = format!("%arg{}", i);
            param_list.push(format!("{} {}", llvm_type, param_name));
            arg_list.push(param_name);
        }
        
        // Generamos la definición del delegador
        context.emit(&format!(
            "define {} @{}({}) {{",
            llvm_return_type,
            child_method_name,
            param_list.join(", ")
        ));
        
        // Generamos el código para cargar el puntero al padre
        context.emit(&format!("  %parent_ptr_ptr = getelementptr %{}_type, ptr %self, i32 0, i32 1", type_name));
        context.emit("  %parent_ptr = load ptr, ptr %parent_ptr_ptr");
        
        // Llamamos al método del padre
        let result_reg = if llvm_return_type == "void" {
            // Para métodos void, no necesitamos un registro de resultado
            if arg_list.is_empty() {
                context.emit(&format!("  call void {}(ptr %parent_ptr)", parent_method_name));
            } else {
                context.emit(&format!(
                    "  call void {}(ptr %parent_ptr, {})",
                    parent_method_name,
                    arg_list.join(", ")
                ));
            }
            "".to_string()
        } else {
            // Para métodos con valor de retorno
            let result = context.generate_temp();
            if arg_list.is_empty() {
                context.emit(&format!(
                    "  {} = call {} {}(ptr %parent_ptr)",
                    result,
                    llvm_return_type,
                    parent_method_name
                ));
            } else {
                context.emit(&format!(
                    "  {} = call {} {}(ptr %parent_ptr, {})",
                    result,
                    llvm_return_type,
                    parent_method_name,
                    arg_list.join(", ")
                ));
            }
            result
        };
        
        // Retornamos el resultado
        if llvm_return_type != "void" {
            context.emit(&format!("  ret {} {}", llvm_return_type, result_reg));
        } else {
            context.emit("  ret void");
        }
        
        context.emit("}");
    }

    /// Genera el constructor del tipo (estructura y lógica de inicialización)
    fn generate_type_constructor(
        &mut self,
        context: &mut CodegenContext,
        attrs: Option<&Vec<String>>,
        methods: Option<&Vec<String>>,
        attr_indices: Option<&HashMap<String, usize>>,
        method_indices: Option<&HashMap<String, usize>>,
    ) {
        let type_name = self.type_name.clone();
        let type_reg = format!("%{}_type", type_name);

        // --- Garantiza la definición del tipo antes del constructor ---
        // Recolecta los tipos LLVM de los atributos en orden
        let mut props_types = Vec::new();
        if let Some(attrs_vec) = attrs {
            for attr_name in attrs_vec {
                let attr_type = context
                    .type_members_types
                    .get(&(type_name.clone(), attr_name.clone()))
                    .cloned()
                    .unwrap_or_else(|| "i8*".to_string());
                props_types.push(CodegenContext::to_llvm_type(attr_type));
            }
        }
        let props_str = if !props_types.is_empty() {
            format!(", {}", props_types.join(", "))
        } else {
            String::new()
        };
        // Define el tipo LLVM para el struct (i32, ptr, ...atributos...)
        context.emit_global(&format!("%{}_type = type {{ i32, ptr{} }}", type_name, props_str));

        // 1. Build params list: usa el tipo real de cada parámetro
        let mut params_list = Vec::new();
        context.build_scope();

        // Agrega los parámetros propios y los registra en el contexto ANTES de usarlos
        for param in self.parameters.iter() {
            let llvm_type = CodegenContext::to_llvm_type(param.param_type.clone());
            let param_name = format!("%{}", param.name.clone());
            params_list.push(format!("{} {}", llvm_type, param_name));
            context.register_variable(&param_name, llvm_type.clone());
        }
        let params_str = params_list.join(", ");

        // 2. Inicializar la lista de métodos con punteros nulos
        // Aseguramos que la lista tenga exactamente context.max_function elementos
        let mut method_list = vec!["ptr null".to_string(); context.max_function];
        
        // Reemplazamos solo las entradas correspondientes a los métodos definidos
        if let (Some(methods), Some(method_indices)) = (methods, method_indices) {
            for method_name in methods {
                if let Some(&index) = method_indices.get(method_name) {
                    if index < context.max_function {
                        let llvm_name = format!("{}_{}", type_name, method_name);
                        method_list[index] = format!("ptr @{}", llvm_name);
                    }
                }
            }
        }
        
        // 3. Preparar vtable - garantizando que tenga exactamente context.max_function elementos
        let type_table_instance = format!("@{}_vtable", type_name);
        context.emit(&format!(
            "{} = constant %VTableType [ {} ]",
            type_table_instance,
            method_list.join(", ")
        ));

        // 4. Build constructor
        context.emit(&format!(
            "define ptr @{}_new( {} ) {{",
            type_name.clone(),
            params_str.clone()
        ));

        // 5. Reserva memoria para la instancia
        let size_temp = context.generate_temp();
        context.emit(&format!(
            "{} = getelementptr {}, ptr null, i32 1",
            size_temp, type_reg
        ));
        let size_int = context.generate_temp();
        context.emit(&format!(
            "{} = ptrtoint ptr {} to i64",
            size_int, size_temp
        ));
        let mem_temp = context.generate_temp();
        context.emit(&format!(
            "{} = call ptr @malloc(i64 {})",
            mem_temp, size_int
        ));

        // 6. Set type index
        context.emit(&format!(
            "%index_ptr = getelementptr {}, ptr {}, i32 0, i32 0",
            type_reg, mem_temp
        ));
        let type_id = context
            .type_functions_ids
            .get(&(type_name.clone(), "__typeid__".to_string()))
            .cloned()
            .unwrap_or(0);
        context.emit(&format!("store i32 {}, ptr %index_ptr", type_id));

        // 7. Inicializa los atributos del padre (igual que antes, si aplica)
        if let Some(parent_name) = self.parent.clone() {
            let mut parent_args_values = Vec::new();
            for arg in self.parent_args.iter() {
                let arg_result = arg.codegen(context);
                let arg_reg = context.generate_temp();
                let llvm_type = context
                    .symbol_table
                    .get("__last_type__")
                    .cloned()
                    .unwrap_or_else(|| "ptr".to_string());
                    context.emit(&format!(
                        "{} = alloca {}",
                    arg_reg.clone(),
                    llvm_type.clone()
                    ));
                    context.emit(&format!(
                        "store {} {}, ptr {}",
                    llvm_type,
                    arg_result,
                    arg_reg.clone()
                    ));
                parent_args_values.push(format!("ptr {}", arg_reg.clone()));
            }
            let args_regs_str = parent_args_values.join(", ");
            let parent_ptr = context.generate_temp();
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
            // Copia los miembros del padre usando attrs y attr_indices
            if let (Some(parent_attrs), Some(attr_indices)) = (attrs, attr_indices) {
                for attr_name in parent_attrs {
                    if let Some(&index) = attr_indices.get(attr_name) {
                        if let Some(attr_type) = context
                            .type_members_types
                            .get(&(parent_name.clone(), attr_name.clone()))
                        {
                            let llvm_type = CodegenContext::to_llvm_type(attr_type.clone());
                            let parent_type = format!("%{}_type", parent_name);
                            let src_ptr = context.generate_temp();
                            context.emit(&format!(
                                "{} = getelementptr {}, ptr {}, i32 0, i32 {}",
                                src_ptr,
                                parent_type,
                                parent_ptr,
                                index + 2
                            ));
                            let val = context.generate_temp();
                            context.emit(&format!("{} = load {}, ptr {}", val, llvm_type, src_ptr));
                            let dst_ptr = context.generate_temp();
                            context.emit(&format!(
                                "{} = getelementptr {}, ptr {}, i32 0, i32 {}",
                                dst_ptr,
                                type_reg,
                                mem_temp,
                                index + 2
                            ));
                            context.emit(&format!("store {} {}, ptr {}", llvm_type, val, dst_ptr));
                        }
                    }
                }
            }
        }

        // 8. Inicializa los atributos propios usando los parámetros
        if let (Some(attrs), Some(attr_indices)) = (attrs, attr_indices) {
            for attr_name in attrs {
                if let Some(attr_def) = self.attributes.get(attr_name) {
                    let llvm_type = context
                        .type_members_types
                        .get(&(type_name.clone(), attr_name.clone()))
                        .map(|t| CodegenContext::to_llvm_type(t.clone()))
                        .unwrap_or_else(|| "i8*".to_string());
                    // Busca si hay un parámetro con el mismo nombre que el atributo
                    let param_opt = self.parameters.iter().find(|p| p.name == *attr_name);
                    let member_index = attr_indices.get(attr_name).expect("No index for attr");
                    let result_reg = context.generate_temp();
                    context.emit(&format!(
                        "{} = getelementptr {}, ptr {}, i32 0, i32 {}",
                        result_reg, type_reg, mem_temp, member_index + 2
                    ));
                    if let Some(param) = param_opt {
                        // Asigna el parámetro directamente al campo con el tipo correcto
                        let param_name = format!("%{}", param.name);
                        let param_type = CodegenContext::to_llvm_type(param.param_type.clone());
                        context.emit(&format!(
                            "store {} {}, ptr {}",
                            param_type, param_name, result_reg
                        ));
                    } else {
                        // Si no hay parámetro, usa la inicialización normal
                        context.register_variable(attr_name, llvm_type.clone());
                        let prop_reg = attr_def.init_expr.codegen(context);
                        context.register_variable(attr_name, prop_reg.clone());
                        context.emit(&format!(
                            "store {} {}, ptr {}",
                            llvm_type, prop_reg, result_reg
                        ));
                    }
                }
            }
        }

        context.emit(&format!("ret ptr {}", mem_temp));
        context.emit("}");
    }

}

impl Codegen for HulkTypeNode {
    fn codegen(&self, _context: &mut CodegenContext) -> String {
        // Implementación básica del trait Codegen
        String::new()
    }
}

