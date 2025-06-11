use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_tokens::hulk_expression::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct WhileLoop {
    pub condition: Box<Expr>,
    pub body: Box<Expr>,
}

impl WhileLoop {
    pub fn new(condition: Box<Expr>, body: Box<Expr>) -> Self {
        Self { condition, body }
    }
}

impl Codegen for WhileLoop {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // Genera etiquetas únicas
        let start_label = context.generate_label("while_start");
        let body_label = context.generate_label("while_body");
        let end_label = context.generate_label("while_end");

        // Salto incondicional al inicio del bucle
        context.emit(&format!("  br label %{}", start_label));

        // Etiqueta de inicio
        context.emit(&format!("{}:", start_label));
        let cond_reg = self.condition.codegen(context);
        // Salto condicional al cuerpo o al final
        context.emit(&format!(
            "  br i1 {}, label %{}, label %{}",
            cond_reg, body_label, end_label
        ));

        // Etiqueta del cuerpo
        context.emit(&format!("{}:", body_label));
        let _body_reg = self.body.codegen(context);
        // Al terminar el cuerpo, vuelve a evaluar la condición
        context.emit(&format!("  br label %{}", start_label));

        // Etiqueta de fin
        context.emit(&format!("{}:", end_label));
        // El valor de un while como expresión suele ser 0 (o unit), aquí devolvemos 0
        let result_reg = context.generate_temp();
        context.emit(&format!("  {} = add i32 0, 0", result_reg));
        result_reg
    }
}
