use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_tokens::hulk_expression::Expr;

#[derive(Debug, PartialEq, Clone)]
pub struct DestructiveAssignment {
    pub identifier: String,
    pub expression: Box<Expr>,
}

impl DestructiveAssignment {
    pub fn new(identifier: String, expression: Expr) -> Self {
        Self {
            identifier,
            expression: Box::new(expression),
        }
    }
}

impl Codegen for DestructiveAssignment {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // Busca el puntero de la variable en la tabla de símbolos
        let var_name = &self.identifier;
        if let Some(ptr) = context.symbol_table.get(var_name) {
            // Genera el valor de la expresión
            let value_reg = self.expression.codegen(context);
            // Emite la instrucción de almacenamiento
            context.emit(&format!("  store i32 {}, i32* {}", value_reg, ptr));
            value_reg
        } else {
            panic!(
                "Variable '{}' no definida en el contexto para asignación destructiva",
                var_name
            );
        }
    }
}
