use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::typings::types_node::TypeNode;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub id: String,
    pub _type: Option<TypeNode>
}

impl Identifier {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            _type: None,
        }
    }

    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
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
        let ptr = context.symbol_table.get(&self.id).cloned();
        if let Some(ptr) = ptr {
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
