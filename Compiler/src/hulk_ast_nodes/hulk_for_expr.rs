//! # ForExpr AST Node
//!
//! Este módulo define el nodo de expresión de bucle `for` (`ForExpr`) del AST para el compilador Hulk.
//! Permite representar y generar código para bucles tipo `for`, donde una variable toma valores en un rango.
//! Incluye la estructura, métodos asociados y la generación de código LLVM IR.

use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::codegen::traits::Codegen;
use crate::codegen::context::CodegenContext;
use crate::typings::types_node::TypeNode;

/// Representa una expresión de bucle `for` en el AST.
/// 
/// Por ejemplo: `for i = 1 to 10 { ... }`
/// 
/// - `variable`: nombre de la variable de control del bucle.
/// - `start`: expresión que representa el valor inicial.
/// - `end`: expresión que representa el valor final.
/// - `body`: cuerpo del bucle (expresión a ejecutar en cada iteración).
/// - `_type`: tipo inferido o declarado del bucle (opcional).
#[derive(Debug, PartialEq, Clone)]
pub struct ForExpr {
    pub variable: String,
    pub start: Box<Expr>,
    pub end: Box<Expr>,
    pub body: Box<Expr>,
    pub _type: Option<TypeNode>,
}

impl ForExpr {
    /// Crea una nueva expresión de bucle `for`.
    ///
    /// # Arguments
    /// * `variable` - Nombre de la variable de control.
    /// * `start` - Expresión de valor inicial.
    /// * `end` - Expresión de valor final.
    /// * `body` - Cuerpo del bucle.
    pub fn new(variable: String, start: Expr, end: Expr, body: Expr) -> Self {
        ForExpr {
            variable,
            start: Box::new(start),
            end: Box::new(end),
            body: Box::new(body),
            _type: None,
        }
    }

    /// Establece el tipo de la expresión del bucle.
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}
impl Codegen for ForExpr {
     /// Genera el código LLVM IR para la expresión de bucle `for`.
    ///
    /// Crea las etiquetas y el flujo de control necesarios para implementar el bucle,
    /// inicializa la variable, evalúa la condición, ejecuta el cuerpo y realiza la actualización.
    /// El bucle no produce un valor, por lo que retorna `"void"`.
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // Obtener tipo del bucle (debe estar definido)
        let hulk_type = self._type.clone().expect("ForExpr debe tener tipo inferido");
        let llvm_type = CodegenContext::to_llvm_type(hulk_type.type_name);

        // Generar valores de inicio y fin
        let start_val = self.start.codegen(context);
        let end_val = self.end.codegen(context);

        // Aloca espacio para la variable del bucle y almacena el valor inicial
        let loop_var_alloc = context.generate_temp();
        context.emit(&format!("  {} = alloca {}", loop_var_alloc, llvm_type));
        context.emit(&format!("  store {} {}, {}* {}", llvm_type, start_val, llvm_type, loop_var_alloc));

        // Registrar variable del iterador en el symbol_table
        context.symbol_table.insert(self.variable.clone(), loop_var_alloc.clone());

        // Etiquetas
        let loop_cond_label = context.generate_label("loop_cond");
        let loop_body_label = context.generate_label("loop_body");
        let loop_inc_label = context.generate_label("loop_inc");
        let loop_end_label = context.generate_label("loop_end");

        // Salto inicial
        context.emit(&format!("  br label %{}", loop_cond_label));

        // loop_cond:
        context.emit(&format!("{}:", loop_cond_label));
        let loop_var = context.generate_temp();
        context.emit(&format!("  {} = load {}, {}* {}", loop_var, llvm_type, llvm_type, loop_var_alloc));

        let cond_temp = context.generate_temp();
        context.emit(&format!(
            "  {} = fcmp ole double {}, {}",
            cond_temp, loop_var, end_val
        ));
        context.emit(&format!(
            "  br i1 {}, label %{}, label %{}",
            cond_temp, loop_body_label, loop_end_label
        ));

        // loop_body:
        context.emit(&format!("{}:", loop_body_label));
        let _ = self.body.codegen(context); // ejecuta cuerpo del bucle
        context.emit(&format!("  br label %{}", loop_inc_label));

        // loop_inc:
        context.emit(&format!("{}:", loop_inc_label));
        let next_val = context.generate_temp();
        context.emit(&format!(
            "  {} = fadd {} {}, 1.0", // Considerar suma adecuada para tipos no enteros si se soportan
            next_val, llvm_type, loop_var
        ));
        context.emit(&format!("  store {} {}, {}* {}", llvm_type, next_val, llvm_type, loop_var_alloc));
        context.emit(&format!("  br label %{}", loop_cond_label));

        // loop_end:
        context.emit(&format!("{}:", loop_end_label));

        // Limpiar la variable del iterador del contexto si lo deseas
        context.symbol_table.remove(&self.variable);

        String::from("void")
    }
}
