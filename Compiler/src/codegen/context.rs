use std::collections::HashMap;

pub struct CodegenContext {
    pub code: String,    // Código dentro de main
    pub globals: String, // Definiciones globales (strings, etc.)
    pub temp_counter: usize,
    pub symbol_table: HashMap<String, String>,
    pub type_table: HashMap<String, String>,
    pub function_table: HashMap<String, String>,
    pub vtable: HashMap<String, HashMap<String, String>>,
}

impl CodegenContext {
    pub fn new() -> Self {
        Self {
            code: String::new(),
            globals: String::new(),
            temp_counter: 0,
            symbol_table: HashMap::new(),
            type_table: HashMap::new(),
            function_table: HashMap::new(),
            vtable: HashMap::new(), 
        }
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
        self.symbol_table.extend(other.symbol_table);
    }
    pub fn register_type(&mut self, name: &str, llvm_type: String) {
        self.type_table.insert(name.to_string(), llvm_type);
    }

    pub fn get_type(&self, name: &str) -> Option<&String> {
        self.type_table.get(name)
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
            _ => "double".to_string(), // Default to pointer type for unknown types
        }
    }
}
