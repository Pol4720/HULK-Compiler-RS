use std::collections::HashMap;

pub struct CodegenContext {
    pub code: String,
    pub temp_counter: usize,
    pub symbol_table: HashMap<String, String>, // nombre -> registro
}

impl CodegenContext {
    pub fn new() -> Self {
        Self {
            code: String::new(),
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
        self.code.push_str(line);
        self.code.push('\n');
    }

    pub fn register_variable(&mut self, name: &str, reg: String) {
        self.symbol_table.insert(name.to_string(), reg);
    }

    // Si necesitas manejar strings constantes en LLVM IR, implementa este método
    pub fn generate_string_const_name(&mut self) -> String {
        let name = format!(".str.{}", self.temp_counter);
        self.temp_counter += 1;
        name
    }
}
