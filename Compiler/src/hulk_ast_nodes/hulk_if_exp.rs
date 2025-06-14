//! # IfExpr y ElseBranch AST Nodes
//!
//! Este módulo define los nodos de expresión condicional `IfExpr` y `ElseBranch` del AST para el compilador Hulk.
//! Permite representar expresiones condicionales tipo `if-else`, incluyendo la condición, las ramas y el tipo inferido.
//! Incluye la estructura, métodos asociados, integración con el visitor pattern y la generación de código LLVM IR.

use crate::hulk_tokens::hulk_keywords::KeywordToken;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::visitor::hulk_accept::Accept;
use crate::visitor::hulk_visitor::Visitor;
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

#[derive(Debug,Clone,PartialEq)]
pub enum ElseOrElif {
    Else(ElseBranch),
    Elif(ElifBranch),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElifBranch {
    pub elif_keyword: KeywordToken,
    pub condition: Box<Expr>,
    pub body: Box<Expr>,
    pub next: Option<Box<ElseOrElif>>, // Puede encadenar otro `elif` o `else`
    pub _type: Option<TypeNode>,
}

impl ElifBranch {
    pub fn new(
        elif_keyword: KeywordToken,
        condition: Box<Expr>,
        body: Box<Expr>,
        next: Option<Box<ElseOrElif>>,
    ) -> Self {
        ElifBranch {
            elif_keyword,
            condition,
            body,
            next,
            _type: None,
        }
    }

    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}

impl Accept for ElifBranch {
    fn accept<V: Visitor<T>, T>(&mut self, visitor: &mut V) -> T {
        visitor.visit_elif_branch(self)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct IfExpr {
    pub if_keyword: KeywordToken,
    pub condition: Box<Expr>,
    pub then_branch: Box<Expr>,
    pub else_branch: Option<ElseOrElif>,
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
    pub fn new(if_keyword: KeywordToken, condition: Box<Expr>, then_branch: Box<Expr>, else_branch: Option<ElseOrElif>) -> Self {
        IfExpr { if_keyword, condition, then_branch, else_branch, _type: None }
    }

    /// Establece el tipo de la expresión condicional.
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}

/// Representa la rama `else` de una expresión condicional en el AST.
/// 
/// - `else_keyword`: token de palabra clave `else`.
/// - `body`: expresión a ejecutar si la condición es falsa.
/// - `_type`: tipo inferido o declarado de la rama (opcional).
#[derive(Debug, Clone, PartialEq)]
pub struct ElseBranch {
    pub else_keyword: KeywordToken,
    pub body: Box<Expr>,
    pub _type: Option<TypeNode>
}

impl ElseBranch {
    /// Crea una nueva rama `else`.
    ///
    /// # Arguments
    /// * `else_keyword` - Token de palabra clave `else`.
    /// * `body` - Expresión de la rama `else`.
    pub fn new(else_keyword: KeywordToken, body: Box<Expr>) -> Self {
        ElseBranch { else_keyword, body, _type: None }
    }

    /// Establece el tipo de la rama `else`.
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}

impl Accept for ElseBranch {
    /// Permite que el nodo acepte un visitor.
    fn accept<V: Visitor<T>, T>(&mut self, visitor: &mut V) -> T {
        visitor.visit_else_branch(self)
    }
}

impl Codegen for IfExpr {
    /// Genera el código LLVM IR para la expresión condicional `if`.
    ///
    /// Crea las etiquetas y el flujo de control necesarios para implementar la condición,
    /// ejecuta las ramas y utiliza una instrucción `phi` para seleccionar el resultado.
    fn codegen(&self, context: &mut CodegenContext) -> String {
        let cond_val = self.condition.codegen(context);


        let then_label = context.generate_label("then");
        let else_label = context.generate_label("else");
        let merge_label = context.generate_label("ifend");

        let result_reg = context.generate_temp();

        // Br condicional
        context.emit(&format!(
            "  br i1 {}, label %{}, label %{}",
            cond_val, then_label, else_label
        ));

        // Then block
        context.emit(&format!("{}:", then_label));
        let then_val = self.then_branch.codegen(context);
        context.emit(&format!("  br label %{}", merge_label));

        // Else block
        context.emit(&format!("{}:", else_label));
        let else_val = match &self.else_branch {
            Some(ElseOrElif::Else(else_branch)) => {
            else_branch.codegen(context)
            }
            Some(ElseOrElif::Elif(elif_branch)) => {
            // Construye un nuevo IfExpr a partir del ElifBranch y llama a su codegen
            let new_if = IfExpr {
                if_keyword: elif_branch.elif_keyword.clone(),
                condition: elif_branch.condition.clone(),
                then_branch: elif_branch.body.clone(),
                else_branch: elif_branch.next.as_ref().map(|b| (**b).clone()),
                _type: elif_branch._type.clone(),
            };
            new_if.codegen(context)
            }
            None => {
            // Por defecto, `0` si no hay rama else
            let tmp = context.generate_temp();
            context.emit(&format!("  {} = add i32 0, 0", tmp));
            tmp
            }
        };
        context.emit(&format!("  br label %{}", merge_label));

        // Merge block
        context.emit(&format!("{}:", merge_label));
        context.emit(&format!(
            "  {} = phi i32 [ {}, %{} ], [ {}, %{} ]",
            result_reg,
            then_val,
            then_label,
            else_val,
            else_label
        ));

        result_reg
    }
}

impl Codegen for ElseBranch {
    /// Genera el código LLVM IR para la rama `else`.
    fn codegen(&self, context: &mut CodegenContext) -> String {
        self.body.codegen(context)
    }
}

//En caso de q no pinche codegen de if con elif.

// impl Codegen for ElifBranch {
//     fn codegen(&self, context: &mut CodegenContext) -> String {
//         let cond_val = self.condition.codegen(context);

//         let then_label = context.generate_label("elif_then");
//         let else_label = context.generate_label("elif_else");
//         let merge_label = context.generate_label("elif_end");

//         let result_reg = context.generate_temp();

//         // Br condicional
//         context.emit(&format!(
//             "  br i1 {}, label %{}, label %{}",
//             cond_val, then_label, else_label
//         ));

//         // Then block
//         context.emit(&format!("{}:", then_label));
//         let then_val = self.body.codegen(context);
//         context.emit(&format!("  br label %{}", merge_label));

//         // Else block (puede ser otro elif o else)
//         context.emit(&format!("{}:", else_label));
//         let else_val = match &self.next {
//             Some(next) => match &**next {
//                 ElseOrElif::Else(else_branch) => else_branch.codegen(context),
//                 ElseOrElif::Elif(elif_branch) => elif_branch.codegen(context),
//             },
//             None => {
//                 let tmp = context.generate_temp();
//                 context.emit(&format!("  {} = add i32 0, 0", tmp));
//                 tmp
//             }
//         };
//         context.emit(&format!("  br label %{}", merge_label));

//         // Merge block
//         context.emit(&format!("{}:", merge_label));
//         context.emit(&format!(
//             "  {} = phi i32 [ {}, %{} ], [ {}, %{} ]",
//             result_reg,
//             then_val,
//             then_label,
//             else_val,
//             else_label
//         ));

//         result_reg
//     }
// }