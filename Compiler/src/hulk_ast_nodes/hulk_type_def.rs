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

        for (_name, method) in self.methods.iter_mut() {
            // Renombrar el método con el prefijo del tipo
            method.name = format!("{}_{}", type_name, method.name.clone());
            method.codegen(&mut type_context);
        }

        type_context.current_self = None;

        // Unificar el contexto temporal con el global
        context.merge_into_global(type_context);

        format!("%{}_type", type_name)
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

        // 1. Build params list
        let mut params_list = Vec::new();
        context.build_scope();
        for param in self.parameters.iter() {
            let param_name = format!("%{}.{}", param.name.clone(), context.get_scope());
            params_list.push(format!("ptr {}", param_name.clone()));
            context.register_variable(&param_name, param.param_type.clone()); // Usa param_type en lugar de signature
        }
        let params_str = params_list.join(", ");

        // Determina el número máximo de métodos (max_functions) considerando la herencia
        let max_functions = 5;

        let mut method_list = vec!["ptr null".to_string(); max_functions];

        // Llena la lista con el nombre de la función en el índice correspondiente usando los hashmaps proporcionados
        if let (Some(methods), Some(method_indices)) = (methods, method_indices) {
            for method_name in methods {
                if let Some(&index) = method_indices.get(method_name) {
                    if index < method_list.len() {
                        let llvm_name = format!("{}_{}", type_name, method_name);
                        method_list[index] = format!("ptr {}", llvm_name);
                    }
                }
            }
        }
        // 2.  Preparar vtable  usando context.vtable
        let type_table_instance = format!("@{}_vtable", type_name);
        context.emit(&format!(
            "{} = constant %VTableType [ {} ]",
            type_table_instance,
            method_list.join(", ")
        ));

        // 3. Build constructor
        context.emit(&format!(
            "define ptr @{}_new( {} ) {{",
            type_name.clone(),
            params_str.clone()
        ));

        // 4. Reserva memoria para la instancia
        let size_temp = context.generate_temp();
        context.emit(&format!(
            "{} = ptrtoint ptr getelementptr({}, ptr null, i32 1) to i64",
            size_temp, type_reg
        ));
        let mem_temp = context.generate_temp();
        context.emit(&format!(
            "{} = call ptr @malloc(i64 {})",
            mem_temp, size_temp
        ));

        // 5. (Opcional) Set type index o vtable si tu diseño lo requiere
        context.emit(&format!(
            "%index_ptr = getelementptr {}, ptr {}, i32 0, i32 0",
            type_reg, mem_temp
        ));

        // Guarda el índice de tipo (id) en la instancia
        // Usar type_functions_ids o agregar un campo type_ids para id global de tipo
        let type_id = context
            .type_functions_ids
            .get(&(type_name.clone(), "__typeid__".to_string()))
            .cloned()
            .unwrap_or(0);
        context.emit(&format!("store i32 {}, ptr %index_ptr", type_id));

        // Inicializa los atributos del padre
        if let Some(parent_name) = self.parent.clone() {
            let mut parent_args_values = Vec::new();
            for arg in self.parent_args.iter() {
                let arg_result = arg.codegen(context);
                let arg_reg = context.generate_temp();
                // Determina el tipo LLVM usando el último tipo inferido en el symbol_table
                let llvm_type = context
                    .symbol_table
                    .get("__last_type__")
                    .cloned()
                    .expect("Tipo no encontrado para argumento de herencia");

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

            // Copia los miembros del padre usando los parámetros attrs y attr_indices
            if let (Some(parent_attrs), Some(attr_indices)) = (attrs, attr_indices) {
                for attr_name in parent_attrs {
                    if let Some(&index) = attr_indices.get(attr_name) {
                        // Determina el tipo LLVM del atributo usando el contexto
                        if let Some(attr_type) = context
                            .type_members_types
                            .get(&(parent_name.clone(), attr_name.clone()))
                        {
                            let llvm_type = CodegenContext::to_llvm_type(attr_type.clone());
                            let parent_type = format!("%{}_type", parent_name);

                            // Carga el valor del atributo desde el padre
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

                            // Guarda el valor en el atributo correspondiente de la instancia actual
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

        // 6. Inicializa los atributos propios
        if let (Some(attrs), Some(attr_indices)) = (attrs, attr_indices) {
            for attr_name in attrs {
                if let Some(attr_def) = self.attributes.get(attr_name) {
                    // Determina el tipo LLVM correcto del atributo usando el contexto
                    let llvm_type = context
                        .type_members_types
                        .get(&(type_name.clone(), attr_name.clone()))
                        .map(|t| CodegenContext::to_llvm_type(t.clone()))
                        .unwrap_or_else(|| "i8*".to_string());
                    // Registra la variable en el contexto antes de generar el código
                    context.register_variable(attr_name, llvm_type.clone());
                    // Genera código para la expresión de inicialización del atributo
                    let prop_reg = attr_def.init_expr.codegen(context);
                    let result_reg = context.generate_temp();
                    let member_index = attr_indices.get(attr_name).expect("No index for attr");
                    context.emit(&format!(
                        "{} = getelementptr {}, ptr {}, i32 0, i32 {}",
                        result_reg, type_reg, mem_temp, member_index
                    ));
                    context.emit(&format!(
                        "store {} {}, ptr {}",
                        llvm_type, prop_reg, result_reg
                    ));
                }
            }
        }

        context.emit(&format!("ret ptr {}", mem_temp));
        context.emit("}");
    }

    // --- Aux functions ---

    // Simulate codegen for an expression node
    // Returns a String register for now
    // You should adapt this to your actual Expr codegen
    // For now, it's a stub
    #[allow(dead_code)]
    fn codegen_expr(expr: &Expr, context: &mut CodegenContext) -> String {
        expr.codegen(context)
    }

    // Simulate codegen for an assignment node
    // Returns a String register for now
    // You should adapt this to your actual Assignment codegen
    // For now, it's a stub
    #[allow(dead_code)]
    fn codegen_assignment(assign: &Assignment, context: &mut CodegenContext) -> String {
        assign.codegen(context)
    }
}

impl Codegen for HulkTypeNode {
    fn codegen(&self, _context: &mut CodegenContext) -> String {
        // Implementación básica del trait Codegen
        String::new()
    }
}
