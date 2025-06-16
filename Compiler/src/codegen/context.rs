use std::collections::HashMap;

pub struct CodegenContext {
    pub code: String,                     // Código dentro de main
    pub globals: String,                 // Definiciones globales (strings, etc.)
    pub temp_counter: usize,
    pub symbol_table: HashMap<String, String>,
}

impl CodegenContext {
    pub fn new() -> Self {
        Self {
            code: String::new(),
            globals: String::new(),
            temp_counter: 0,
            symbol_table: HashMap::new(),
        }
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
        _ => "i8*".to_string(), // Default to pointer type for unknown types
    }
}
}

