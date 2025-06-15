//! # LetIn AST Node
//!
//! Este módulo define el nodo `LetIn` del AST para el compilador Hulk.
//! Permite representar expresiones de tipo `let-in`, donde se pueden declarar y asignar variables locales
//! que solo existen dentro del cuerpo de la expresión `in`.
//! Incluye la estructura, métodos asociados y la generación de código LLVM IR.

use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_ast_nodes::hulk_assignment::Assignment;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::hulk_tokens::hulk_keywords::KeywordToken;
use crate::typings::types_node::TypeNode;

/// Representa una expresión `let-in` en el AST.
/// 
/// Por ejemplo: `let x = 5, y = 10 in x + y`
/// 
/// - `let_token`: token de palabra clave `let`.
/// - `assignment`: lista de asignaciones locales.
/// - `in_keyword`: token de palabra clave `in`.
/// - `body`: cuerpo de la expresión donde existen las variables locales.
/// - `_type`: tipo inferido o declarado de la expresión (opcional).
#[derive(Debug, Clone, PartialEq)]
pub struct LetIn {
    pub let_token: KeywordToken,
    pub assignment: Vec<Assignment>,
    pub in_keyword: KeywordToken,
    pub body: Box<Expr>,
    pub _type: Option<TypeNode>, 
}

impl LetIn {
    /// Crea una nueva expresión `let-in`.
    ///
    /// # Arguments
    /// * `let_token` - Token de palabra clave `let`.
    /// * `assignment` - Vector de asignaciones locales.
    /// * `in_keyword` - Token de palabra clave `in`.
    /// * `body` - Cuerpo de la expresión.
    pub fn new(
        let_token: KeywordToken,
        assignment: Vec<Assignment>,
        in_keyword: KeywordToken, 
        body: Box<Expr>
    ) -> Self {
        LetIn { let_token, assignment, in_keyword, body, _type: None }
    }

    /// Establece el tipo de la expresión `let-in`.
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}

impl Codegen for LetIn {
    /// Genera el código LLVM IR para la expresión `let-in`.
    ///
    /// Reserva espacio para cada variable local, almacena su valor y gestiona el shadowing de variables.
    /// Al finalizar el cuerpo, restaura los bindings anteriores para mantener el alcance correcto.
    fn codegen(&self, context: &mut CodegenContext) -> String {
        let mut previous_bindings: Vec<(String, Option<String>)> = vec![];

        for assignment in &self.assignment {
            let name = assignment.identifier.id.clone();
            let value_expr = &assignment.expression;

            // Genera el valor (registro LLVM) de la expresión
            let value_reg = value_expr.codegen(context);

            // Obtiene el tipo LLVM desde el symbol table
            let llvm_type = context
                .symbol_table
                .get("__last_type__")
                .cloned()
                .expect("Tipo no encontrado para asignación let");

            // Genera almacenamiento y guarda el valor
            let alloca_reg = context.generate_temp();
            context.emit(&format!("  {} = alloca {}", alloca_reg, llvm_type));
            if llvm_type == "ptr" {
                // Si el tipo es un puntero, almacenamos el valor como un puntero genérico (i8*)
                context.emit(&format!("  store ptr {}, ptr {}", value_reg, alloca_reg));
            } else {
                context.emit(&format!("  store {} {}, {}* {}", llvm_type, value_reg, llvm_type, alloca_reg));
            }

            // Guarda cualquier binding anterior (shadowing reversible)
            let previous = context.symbol_table.get(&name).cloned();
            previous_bindings.push((name.clone(), previous));

            // Registra la variable nueva
            context.register_variable(&name, alloca_reg);
        }

        // Genera el cuerpo de la expresión `in`
        let body_reg = self.body.codegen(context);

        // Restaura bindings anteriores
        for (name, prev) in previous_bindings {
            match prev {
                Some(ptr) => context.register_variable(&name, ptr),
                None => {
                    context.symbol_table.remove(&name);
                }
            }
        }

        body_reg
    }
}