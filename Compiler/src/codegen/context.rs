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

    pub fn emit(&mut self, line: &str) {
        self.code.push_str(line);
        self.code.push('\n');
    }
}
