//! # CodegenContext
//!
//! Esta estructura administra el estado y las tablas de símbolos durante la generación de código LLVM IR para el lenguaje Hulk.
//! Lleva el control de variables temporales, tipos, funciones, vtables para despacho dinámico y el manejo de ámbitos.
//!
//! ## Responsabilidades principales
//! - Registrar y buscar variables, tipos y funciones
//! - Gestionar registros temporales y etiquetas para LLVM IR
//! - Manejar herencia y tablas de métodos para características orientadas a objetos
//! - Emitir código y definiciones globales
//! - Administrar ámbitos léxicos para variables
//!
//! ## Campos
//! - `code`: Acumula el código LLVM IR principal de la función o bloque actual.
//! - `globals`: Almacena definiciones globales de LLVM IR (por ejemplo, constantes de strings).
//! - `temp_counter`: Contador para generar nombres únicos de variables temporales.
//! - `symbol_table`: Mapea nombres de variables a registros LLVM.
//! - `type_table`: Mapea nombres de tipos a sus representaciones LLVM IR.
//! - `function_table`: Mapea nombres de funciones a nombres de funciones LLVM.
//! - `vtable`: Mapea nombres de tipos a sus tablas de métodos para despacho dinámico.
//! - `id`: Generador de identificadores únicos.
//! - `constructor_args_types`: Mapea tipos a los tipos de argumentos de sus constructores.
//! - `inherits`: Mapea tipos a su tipo padre (para herencia).
//! - `types_members_functions`: Mapea (tipo, método, id) a una lista de nombres de miembros.
//! - `type_members_types`: Mapea (tipo, miembro) al tipo del miembro.
//! - `type_members_ids`: Mapea (tipo, miembro) a un id único.
//! - `type_functions_ids`: Mapea (tipo, función) a un id único.
//! - `current_self`: Lleva el seguimiento del tipo "self" actual para generación de métodos.
//! - `function_member_llvm_names`: Mapea (tipo, función) a nombres de funciones LLVM.
//! - `scopes`: Pila de tablas de símbolos para el manejo de ámbitos léxicos.
//! - `scope_id`: Id del ámbito actual.
//! - `temp_types`: Mapea nombres de variables temporales a sus tipos.
//!
//! ## Métodos
//! - `new()`: Crea un nuevo contexto vacío.
//! - `register_method()`, `get_method()`: Gestionan las tablas de métodos de los tipos.
//! - `merge_into_global()`: Fusiona los globals y tablas de otro contexto.
//! - `register_type()`, `get_type()`: Gestionan la información de tipos.
//! - `build_scope()`, `pop_scope()`, `get_scope()`: Manejan los ámbitos léxicos.
//! - `generate_temp()`, `generate_label()`, `new_id()`: Generan nombres únicos.
//! - `emit()`, `emit_global()`: Emiten código en la sección principal o global.
//! - `register_variable()`: Registra una variable en el ámbito actual.
//! - `generate_string_const_name()`: Genera nombres únicos para constantes de string.
//! - `to_llvm_type()`: Convierte tipos Hulk a tipos LLVM

use std::collections::HashMap;


pub struct CodegenContext {
    pub code: String,    // Código dentro de main
    pub globals: String, // Definiciones globales (strings, etc.)
    pub temp_counter: usize,
    pub symbol_table: HashMap<String, String>,
    pub register_hulk_type_map: HashMap<String, String>,
    pub type_table: HashMap<String, String>,
    pub function_table: HashMap<String, String>,
    pub f_table: HashMap<String, String>,
    pub vtable: HashMap<String, HashMap<String, String>>,
    pub id: usize,
    pub constructor_args_types: HashMap<String, Vec<String>>,
    pub inherits: HashMap<String, String>,
    pub types_members_functions: HashMap<(String,String,i32), Vec<String>>,
    pub type_members_types: HashMap<(String, String), String>,
    pub type_members_ids: HashMap<(String, String), i32>,
    pub type_functions_ids: HashMap<(String,String),i32>,
    pub current_self: Option<String>,
    pub function_member_llvm_names: HashMap<(String, String), String>,
    pub scopes: Vec<HashMap<String, String>>,
    scope_id: i32,
    pub temp_types: HashMap<String, String>,
    pub type_ids: HashMap<String, i32>, // Agregar un mapa para guardar los type_ids


}

impl CodegenContext {
    pub fn new() -> Self {
        Self {
            code: String::new(),
            globals: String::new(),
            temp_counter: 0,
            symbol_table: HashMap::new(),
            register_hulk_type_map: HashMap::new(),
            type_table: HashMap::new(),
            function_table: HashMap::new(),
            f_table: HashMap::new(),
            vtable: HashMap::new(),
            id: 1,
            constructor_args_types: HashMap::new(),
            inherits: HashMap::new(),
            types_members_functions: HashMap::new(),
            type_members_types: HashMap::new(),
            type_members_ids: HashMap::new(),
            type_functions_ids: HashMap::new(),
            current_self: None,
            function_member_llvm_names: HashMap::new(),
            scopes: Vec::new(),
            scope_id: 0,
            temp_types: HashMap::new(),
            type_ids: HashMap::new(),
        }
    }
    pub fn add_register_hulk_type(&mut self, reg: String, type_name: String) {
        self.register_hulk_type_map.insert(reg, type_name);
    }

    pub fn get_register_hulk_type(&self, reg: &str) -> Option<&String> {
        self.register_hulk_type_map.get(reg)
    }

     pub fn register_method(&mut self, type_name: &str, method_name: &str, llvm_function_name: &str) {
        self.vtable
            .entry(type_name.to_string())
            .or_default()
            .insert(method_name.to_string(), llvm_function_name.to_string());
    }

    pub fn get_method(&self, type_name: &str, method_name: &str) -> Option<&String> {
        self.vtable
            .get(type_name)
            .and_then(|methods| methods.get(method_name))
    }
    
    pub fn merge_into_global(&mut self, other: CodegenContext) {
        self.globals.push_str(&other.globals);
        self.globals.push_str(&other.code);
        self.temp_counter = other.temp_counter.max(self.temp_counter);
        self.function_table.extend(other.function_table);
        self.f_table.extend(other.f_table);
        self.symbol_table.extend(other.symbol_table);
        self.type_table.extend(other.type_table);
        self.vtable.extend(other.vtable);
        self.type_members_types.extend(other.type_members_types);
        self.type_members_ids.extend(other.type_members_ids);
        self.type_functions_ids.extend(other.type_functions_ids);
        self.constructor_args_types.extend(other.constructor_args_types);
        self.inherits.extend(other.inherits);
        self.types_members_functions.extend(other.types_members_functions);
        self.function_member_llvm_names.extend(other.function_member_llvm_names);
        self.temp_types.extend(other.temp_types);
        self.type_ids.extend(other.type_ids);
        // No se fusionan scopes ni current_self
    }
    pub fn register_type(&mut self, name: &str, llvm_type: String) {
        self.type_table.insert(name.to_string(), llvm_type);
    }

    pub fn get_type(&self, name: &str) -> Option<&String> {
        self.type_table.get(name)
    }

    pub fn build_scope(&mut self) {
        self.scope_id += 1;
        self.scopes.push(self.symbol_table.clone())
    }

    pub fn pop_scope(&mut self) {
        self.symbol_table = self.scopes.pop().unwrap_or_default();
    }
    pub fn get_scope(&self) -> i32 {
        self.scope_id
    }
    pub fn generate_temp(&mut self) -> String {
        let temp = format!("%t{}", self.temp_counter);
        self.temp_counter += 1;
        temp
    }

    pub fn generate_label(&mut self, base: &str) -> String {
        let label = format!("{}{}", base, self.temp_counter);
        self.temp_counter += 1;
        label
    }

    pub fn new_id(&mut self) -> usize {
        let id = self.id;
        self.id += 1;
        id
    }

    pub fn emit(&mut self, line: &str) {
        self.code.push_str(line);
        self.code.push('\n');
    }

    pub fn emit_global(&mut self, line: &str) {
        self.globals.push_str(line);
        self.globals.push('\n');
    }

    pub fn register_variable(&mut self, name: &str, reg: String) {
        self.symbol_table.insert(name.to_string(), reg);
    }
    pub fn get_variable(&self, name: &str) -> Option<&String> {
        // Busca primero por el nombre directo
        if let Some(val) = self.symbol_table.get(name) {
            return Some(val);
        }
        // Si no está, busca por el nombre con el scope actual: %name.{scope_id}
        let scoped_name = format!("{}.{}", name, self.scope_id);
        self.symbol_table.get(&scoped_name)
    }

    pub fn generate_string_const_name(&mut self) -> String {
        let name = format!(".str.{}", self.temp_counter);
        self.temp_counter += 1;
        name
    }

    pub fn to_llvm_type(type_node: String) -> String {
        match type_node.as_str() {
            "Number" => "double".to_string(),
            "Boolean" => "i1".to_string(),
            "String" => "i8*".to_string(),
            _ => "ptr".to_string(), 
        }
    }

    pub fn clone_for_type_codegen(&self) -> CodegenContext {
        let mut ctx = CodegenContext::new();
        ctx.function_table = self.function_table.clone();
        ctx.f_table = self.f_table.clone();
        ctx.type_table = self.type_table.clone();
        ctx.vtable = self.vtable.clone();
        ctx.type_members_types = self.type_members_types.clone();
        ctx.type_members_ids = self.type_members_ids.clone();
        ctx.type_functions_ids = self.type_functions_ids.clone();
        ctx.constructor_args_types = self.constructor_args_types.clone();
        ctx.inherits = self.inherits.clone();
        ctx.types_members_functions = self.types_members_functions.clone();
        ctx.function_member_llvm_names = self.function_member_llvm_names.clone();
        ctx.temp_types = self.temp_types.clone();
        ctx.type_ids = self.type_ids.clone();
        ctx.id = self.id;
        ctx.temp_counter = self.temp_counter;
        ctx
    }

}
