//! # IfExpr y ElseBranch AST Nodes
//!
//! Este módulo define los nodos de expresión condicional `IfExpr` y `ElseBranch` del AST para el compilador Hulk.
//! Permite representar expresiones condicionales tipo `if-else`, incluyendo la condición, las ramas y el tipo inferido.
//! Incluye la estructura, métodos asociados, integración con el visitor pattern y la generación de código LLVM IR.

use crate::hulk_tokens::hulk_keywords::KeywordToken;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::codegen::traits::Codegen;
use crate::codegen::context::CodegenContext;
use crate::typings::types_node::TypeNode;

/// Representa una expresión condicional `if` en el AST.
/// 
/// Por ejemplo:
/// ```hulk
/// if (condición) { ... } else { ... }
/// ```
/// 
/// - `if_keyword`: token de palabra clave `if`.
/// - `condition`: expresión booleana de condición.
/// - `then_branch`: rama a ejecutar si la condición es verdadera.
/// - `else_branch`: rama a ejecutar si la condición es falsa (opcional).
/// - `_type`: tipo inferido o declarado de la expresión (opcional).

#[derive(Debug, Clone, PartialEq)]
pub struct IfExpr {
    pub if_keyword: KeywordToken,
    pub condition: Box<Expr>,
    pub then_branch: Box<Expr>,
    pub else_branch: Vec<(Option<Expr>,Expr)>,
    pub _type: Option<TypeNode>
}



impl IfExpr {
    /// Crea una nueva expresión condicional `if`.
    ///
    /// # Arguments
    /// * `if_keyword` - Token de palabra clave `if`.
    /// * `condition` - Expresión de condición.
    /// * `then_branch` - Rama `then`.
    /// * `else_branch` - Rama `else` (opcional).
    pub fn new(if_keyword: KeywordToken, condition: Box<Expr>, then_branch: Box<Expr>, else_branch: Vec<(Option<Expr>, Expr)>) -> Self {
        IfExpr { if_keyword, condition, then_branch, else_branch, _type: None }
    }

    /// Establece el tipo de la expresión condicional.
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}

impl Codegen for IfExpr {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        let node_type = self._type.clone().unwrap().type_name;
        let llvm_type = CodegenContext::to_llvm_type(node_type.clone());
        let result_reg = context.generate_temp();
        let exit_id = context.new_id();
        let exit_label = format!("if_exit_{}", exit_id);

        // Reserva espacio para el resultado
        context.emit(&format!("{} = alloca {}", result_reg, llvm_type));

        // Condición principal
        let cond_reg = self.condition.codegen(context);
        let if_id = context.new_id();
        let then_label = format!("if_then_{}", if_id);
        let else_label = format!("if_else_{}", if_id);

        context.emit(&format!(
            "br i1 {}, label %{}, label %{}",
            cond_reg, then_label, else_label
        ));

        // THEN branch
        context.emit(&format!("{}:", then_label));
        let then_val = self.then_branch.codegen(context);
        context.emit(&format!(
            "store {} {}, {}* {}",
            llvm_type, then_val, llvm_type, result_reg
        ));
        context.emit(&format!("br label %{}", exit_label));

        // ELSE/ELIF branches
        context.emit(&format!("{}:", else_label));
        let mut next_label = None;
        for (i, (cond, expr)) in self.else_branch.iter().enumerate() {
            let elif_id = context.new_id();
            let elif_label = format!("elif_then_{}", elif_id);
            let elif_else_label = format!("elif_else_{}", elif_id);

            let cond_reg = if let Some(cond_expr) = cond {
                let reg = cond_expr.codegen(context);
                context.emit(&format!(
                    "br i1 {}, label %{}, label %{}",
                    reg, elif_label, elif_else_label
                ));
                Some((elif_label.clone(), elif_else_label.clone()))
            } else {
                // ELSE final, no condición
                context.emit(&format!("br label %{}", elif_label));
                Some((elif_label.clone(), String::new()))
            };

            // ELIF/ELSE body
            context.emit(&format!("{}:", cond_reg.as_ref().unwrap().0));
            let val = expr.codegen(context);
            context.emit(&format!(
                "store {} {}, {}* {}",
                llvm_type, val, llvm_type, result_reg
            ));
            context.emit(&format!("br label %{}", exit_label));

            // Si hay siguiente elif/else, prepara el siguiente label
            if let Some((_, next)) = cond_reg {
                if !next.is_empty() {
                    context.emit(&format!("{}:", next));
                    next_label = Some(next);
                }
            }
        }
        // Si no hay else_branch, solo salta al exit
        if self.else_branch.is_empty() {
            context.emit(&format!("br label %{}", exit_label));
        }

        // EXIT
        context.emit(&format!("{}:", exit_label));
        let final_result = context.generate_temp();
        context.emit(&format!(
            "{} = load {}, {}* {}",
            final_result, llvm_type, llvm_type, result_reg
        ));
        final_result
    }
}
