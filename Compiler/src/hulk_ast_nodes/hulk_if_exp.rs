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

// impl Codegen for IfExpr {
//     /// Genera el código LLVM IR para la expresión condicional `if`.
//     ///
//     /// Crea las etiquetas y el flujo de control necesarios para implementar la condición,
//     /// ejecuta las ramas y utiliza una instrucción `phi` para seleccionar el resultado.
//     fn codegen(&self, context: &mut CodegenContext) -> String {
//         let cond_val = self.condition.codegen(context);

//         let then_label = context.generate_label("then");
//         let else_label = context.generate_label("else");
//         let merge_label = context.generate_label("ifend");

//         let result_reg = context.generate_temp();

//         // Br condicional
//         context.emit(&format!(
//             "  br i1 {}, label %{}, label %{}",
//             cond_val, then_label, else_label
//         ));

//         // Then block
//         context.emit(&format!("{}:", then_label));
//         let then_val = self.then_branch.codegen(context);
//         context.emit(&format!("  br label %{}", merge_label));

//         // Else block
//         context.emit(&format!("{}:", else_label));
//         let else_val = match &self.else_branch {
//             Some(ElseOrElif::Else(else_branch)) => {
//             else_branch.codegen(context)
//             }
//             Some(ElseOrElif::Elif(elif_branch)) => {
//             // Construye un nuevo IfExpr a partir del ElifBranch y llama a su codegen
//             let new_if = IfExpr {
//                 if_keyword: elif_branch.elif_keyword.clone(),
//                 condition: elif_branch.condition.clone(),
//                 then_branch: elif_branch.body.clone(),
//                 else_branch: elif_branch.next.as_ref().map(|b| (**b).clone()),
//                 _type: elif_branch._type.clone(),
//             };
//             new_if.codegen(context)
//             }
//             None => {
//             // Por defecto, `0` si no hay rama else
//             let tmp = context.generate_temp();
//             context.emit(&format!("  {} = fadd double 0.0, 0.0", tmp));
//             tmp
//             }
//         };
//         context.emit(&format!("  br label %{}", merge_label));

//         // Merge block
//         context.emit(&format!("{}:", merge_label));
//         context.emit(&format!(
//             "  {} = phi double [ {}, %{} ], [ {}, %{} ]",
//             result_reg,
//             then_val,
//             then_label,
//             else_val,
//             else_label
//         ));

//         result_reg
//     }
// }

impl Codegen for IfExpr {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        let end_label = context.generate_label("endif");
        let result = context.generate_temp();
        let mut phi_entries = vec![];

        // Evaluar condición principal
        let cond_val = self.condition.codegen(context);
        let cond_bool = context.generate_temp();
        context.emit(&format!("  {} = icmp ne i1 {}, 0", cond_bool, cond_val));

        let then_label = context.generate_label("then");
        let else_label = context.generate_label("else");

        context.emit(&format!(
            "  br i1 {}, label %{}, label %{}",
            cond_bool, then_label, else_label
        ));

        // THEN
        context.emit(&format!("{}:", then_label));
        let then_val = self.then_branch.codegen(context);
        let then_type = context
            .symbol_table
            .get("__last_type__")
            .cloned()
            .unwrap_or("i32".to_string());
        // Si la rama no retorna valor, asigna un valor por defecto
        let then_val = if then_val.is_empty() || then_val == "undef" {
            let tmp = context.generate_temp();
            context.emit(&format!("  {} = add i32 0, 0", tmp));
            tmp
        } else {
            then_val
        };
        context.emit(&format!("  br label %{}", end_label));
        phi_entries.push((then_val.clone(), then_label.clone()));

        // ELSE o ELIFs
        let mut current_label = else_label.clone();
        if self.else_branch.is_empty() {
            // Si no hay else, simplemente terminamos aquí
            context.emit(&format!("{}:", else_label));
            context.emit(&format!("  br label %{}", end_label));
        } else {
            let mut else_blocks = self.else_branch.iter().peekable();

            while let Some((maybe_cond, expr)) = else_blocks.next() {
                context.emit(&format!("{}:", current_label));

                if let Some(cond) = maybe_cond {
                    let cond_val = cond.codegen(context);
                    let cond_bool = context.generate_temp();
                    let true_label = context.generate_label("elif_then");
                    let next_label = if else_blocks.peek().is_some() {
                        context.generate_label("else")
                    } else {
                        context.generate_label("final_else")
                    };

                    context.emit(&format!(
                        "  {} = icmp ne i1 {}, 0",
                        cond_bool, cond_val
                    ));
                    context.emit(&format!(
                        "  br i1 {}, label %{}, label %{}",
                        cond_bool, true_label, next_label
                    ));

                    // ELIF THEN
                    context.emit(&format!("{}:", true_label));
                    let elif_val = expr.codegen(context);
                    let elif_type = context
                        .symbol_table
                        .get("__last_type__")
                        .cloned()
                        .unwrap_or("i32".to_string());

                    // Si la rama no retorna valor, asigna un valor por defecto
                    let elif_val = if elif_val.is_empty() || elif_val == "undef" {
                        let tmp = context.generate_temp();
                        context.emit(&format!("  {} = add i32 0, 0", tmp));
                        tmp
                    } else {
                        elif_val
                    };

                    if elif_type != then_type {
                        panic!(
                            "Tipos incompatibles en ramas if/elif: {} vs {}",
                            then_type, elif_type
                        );
                    }

                    context.emit(&format!("  br label %{}", end_label));
                    phi_entries.push((elif_val.clone(), true_label.clone()));

                    current_label = next_label;
                } else {
                    // ELSE FINAL
                    let else_val = expr.codegen(context);
                    let else_type = context
                        .symbol_table
                        .get("__last_type__")
                        .cloned()
                        .unwrap_or("i32".to_string());

                    // Si la rama no retorna valor, asigna un valor por defecto
                    let else_val = if else_val.is_empty() || else_val == "undef" {
                        let tmp = context.generate_temp();
                        context.emit(&format!("  {} = add i32 0, 0", tmp));
                        tmp
                    } else {
                        else_val
                    };

                    if else_type != then_type {
                        panic!(
                            "Tipos incompatibles en ramas if/else: {} vs {}",
                            then_type, else_type
                        );
                    }

                    context.emit(&format!("  br label %{}", end_label));
                    phi_entries.push((else_val.clone(), current_label.clone()));
                    break;
                }
            }
        }

        // ENDIF y PHI
        context.emit(&format!("{}:", end_label));

        if !phi_entries.is_empty() {
            let mut phi_type = then_type.clone();
            if phi_type == "ptr" {
                phi_type = "i8*".to_string();
            }

            let phi_str = phi_entries
                .into_iter()
                .map(|(val, label)| format!("[ {}, %{} ]", val, label))
                .collect::<Vec<_>>()
                .join(", ");

            context.emit(&format!("  {} = phi {} {}", result, phi_type, phi_str));
            context.symbol_table.insert("__last_type__".to_string(), then_type);
            result
        } else {
            // No hay valor a retornar (solo if sin else)
            "undef".to_string()
        }
    }
}
