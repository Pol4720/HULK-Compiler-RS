//! # IfExpr y ElseBranch AST Nodes
//!
//! Este módulo define los nodos de expresión condicional `IfExpr` y `ElseBranch` del AST para el compilador Hulk.
//!
//! ## Descripción
//! Permite representar expresiones condicionales tipo `if-else`, incluyendo la condición, las ramas y el tipo inferido.
//! Incluye la estructura, métodos asociados, integración con el visitor pattern y la generación de código LLVM IR.
//!
//! ## Estructuras principales
//!
//! - `IfExpr`: Representa una expresión condicional `if` en el AST de Hulk.
//!     - Campos:
//!         - `if_keyword`: Token de palabra clave `if`.
//!         - `condition`: Expresión booleana de condición.
//!         - `then_branch`: Rama a ejecutar si la condición es verdadera.
//!         - `else_branch`: Vector de ramas else/elif, cada una con una condición opcional y una expresión.
//!         - `_type`: Tipo inferido o declarado de la expresión (opcional).
//!         - `token_pos`: Posición del token en el código fuente.
//!     - Métodos:
//!         - `new`: Constructor de la expresión if.
//!         - `set_expression_type`: Establece el tipo de la expresión.
//!
//! ## Implementación de Codegen
//!
//! Implementa el trait `Codegen` para permitir la generación de código LLVM IR de la expresión condicional, generando los saltos y almacenamiento de resultados necesarios para el flujo de control.
//!
//! ## Ejemplo de uso
//!
//! ```hulk
//! if (condición) { ... } elif (otra_condición) { ... } else { ... }
//! ```
//!

use crate::hulk_tokens::hulk_keywords::KeywordToken;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::codegen::traits::Codegen;
use crate::codegen::context::CodegenContext;
use crate::hulk_tokens::TokenPos;
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
    pub _type: Option<TypeNode>,
    pub token_pos: TokenPos,
}



impl IfExpr {
    /// Crea una nueva expresión condicional `if`.
    ///
    /// # Arguments
    /// * `if_keyword` - Token de palabra clave `if`.
    /// * `condition` - Expresión de condición.
    /// * `then_branch` - Rama `then`.
    /// * `else_branch` - Rama `else` (opcional).
    pub fn new(if_keyword: KeywordToken, condition: Box<Expr>, then_branch: Box<Expr>, else_branch: Vec<(Option<Expr>, Expr)> , token_pos: TokenPos) -> Self {
        IfExpr { if_keyword, condition, then_branch, else_branch, _type: None , token_pos }
    }

    /// Establece el tipo de la expresión condicional.
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}

impl Codegen for IfExpr {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // Get the node type and convert to LLVM type
        let node_type = self._type.clone().unwrap().type_name;
        let llvm_type = CodegenContext::to_llvm_type(node_type.clone());
        
        // Create result register and exit label
        let result_reg = context.generate_temp();
        let exit_id = context.new_id();
        let exit_label = format!("if_else_exit.{}", exit_id);
        
        // Allocate space for the result - use 'ptr' instead of 'ptr*'
        context.emit(&format!("{} = alloca {}", result_reg, llvm_type));
        
        // Generate code for the condition
        let cond_reg = self.condition.codegen(context);
        let if_id = context.new_id();
        let if_true_label = format!("if_true.{}", if_id);
        let if_false_label = format!("if_false.{}", if_id);
        
        // Branch based on condition
        context.emit(&format!(
            "br i1 {}, label %{}, label %{}",
            cond_reg, if_true_label, if_false_label
        ));
        
        // THEN branch
        context.emit(&format!("{}:", if_true_label));
        let then_val = self.then_branch.codegen(context);
        context.emit(&format!(
            "store {} {}, ptr {}",  // Changed ptr* to ptr
            llvm_type, then_val, result_reg
        ));
        context.emit(&format!("br label %{}", exit_label));
        
        // ELSE branch and ELIF branches
        context.emit(&format!("{}:", if_false_label));
        
        if !self.else_branch.is_empty() {
            // Process all elif/else branches
            for (i, (cond, expr)) in self.else_branch.iter().enumerate() {
                let elif_id = context.new_id();
                let elif_true_label = format!("elif_true.{}", elif_id);
                let elif_false_label = format!("elif_false.{}", elif_id);
                
                if let Some(cond_expr) = cond {
                    // This is an ELIF with a condition
                    let elif_cond_reg = cond_expr.codegen(context);
                    context.emit(&format!(
                        "br i1 {}, label %{}, label %{}",
                        elif_cond_reg, elif_true_label, elif_false_label
                    ));
                } else {
                    // This is an ELSE (no condition)
                    context.emit(&format!("br label %{}", elif_true_label));
                }
                
                // ELIF/ELSE body
                context.emit(&format!("{}:", elif_true_label));
                let expr_val = expr.codegen(context);
                context.emit(&format!(
                    "store {} {}, ptr {}",  // Changed ptr* to ptr
                    llvm_type, expr_val, result_reg
                ));
                context.emit(&format!("br label %{}", exit_label));
                
                // If this is an ELIF (has condition), set up the next branch
                if let Some(_) = cond {
                    context.emit(&format!("{}:", elif_false_label));
                    
                    // If this is the last branch and there's no explicit else,
                    // branch to the exit
                    if i == self.else_branch.len() - 1 {
                        context.emit(&format!("br label %{}", exit_label));
                    }
                }
            }
        } else {
            // No else branch at all, just branch to exit
            context.emit(&format!("br label %{}", exit_label));
        }
        
        // EXIT - load the result
        context.emit(&format!("{}:", exit_label));
        let final_result = context.generate_temp();
        context.emit(&format!(
            "{} = load {}, ptr {}",  // Changed ptr* to ptr
            final_result, llvm_type, result_reg
        ));
        
        // Register the type of the final result
        context.temp_types.insert(final_result.clone(), node_type);
        
        final_result
    }
}
