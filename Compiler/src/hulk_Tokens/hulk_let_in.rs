use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_tokens::hulk_assignment::Assignment;
use crate::hulk_tokens::hulk_expression::Expr;
use crate::hulk_tokens::hulk_keywords::KeywordToken;

#[derive(Debug, Clone, PartialEq)]
pub struct LetIn {
    pub let_token: KeywordToken,
    pub assignment: Vec<Assignment>,
    pub in_keyword: KeywordToken,
    pub body: Box<Expr>,
}

impl LetIn {
    pub fn new(
        let_token: KeywordToken,
        assignment: Vec<Assignment>,
        in_keyword: KeywordToken,
        body: Box<Expr>,
    ) -> Self {
        LetIn {
            let_token,
            assignment,
            in_keyword,
            body,
        }
    }
}

impl Codegen for LetIn {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // Guarda el estado original del símbolo (para restaurar al final)
        let mut previous_bindings: Vec<(String, Option<String>)> = vec![];

        for assignment in &self.assignment {
            let name = assignment.identifier.id.clone();
            let value_expr = &assignment.expression;

            let value_reg = value_expr.codegen(context);

            let alloca_reg = context.generate_temp();
            context.emit(&format!("  {} = alloca i32", alloca_reg));
            context.emit(&format!("  store i32 {}, i32* {}", value_reg, alloca_reg));

            // Guarda cualquier binding anterior para restaurar luego (sombra)
            let previous = context.symbol_table.get(&name).cloned();
            previous_bindings.push((name.clone(), previous));

            context.register_variable(&name, alloca_reg);
        }

        // Genera el cuerpo de la expresión `in`
        let body_value = self.body.codegen(context);

        // Restaura bindings anteriores (shadowing reversible)
        for (name, prev) in previous_bindings {
            match prev {
                Some(ptr) => context.register_variable(&name, ptr),
                None => {
                    context.symbol_table.remove(&name);
                }
            }
        }

        body_value
    }
}
