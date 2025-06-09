use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    pub id: String,
}

impl Identifier {
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Codegen for Identifier {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // Busca el puntero de la variable en la tabla de símbolos
        if let Some(ptr) = context.symbol_table.get(&self.id) {
            let result_reg = context.generate_temp();
            // Asume tipo i32 (ajustar si soportas otros tipos)
            let line = format!("  {} = load i32, i32* {}", result_reg, ptr);
            context.emit(&line);
            result_reg
        } else {
            panic!("Variable '{}' no definida en el contexto", self.id);
        }
    }
}
