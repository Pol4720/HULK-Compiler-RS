//! # BinaryExpr AST Node
//!
//! Este módulo define el nodo de expresión binaria (`BinaryExpr`) del AST para el compilador Hulk.
//! Incluye la estructura, métodos asociados, integración con el visitor pattern y generación de código LLVM IR.

use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::typings::types_node::TypeNode;
use crate::hulk_tokens::hulk_operators::BinaryOperatorToken;

/// Representa una expresión binaria en el AST.
/// 
/// Por ejemplo: `a + b`, `x > 5`, `foo && bar`
/// 
/// - `left`: expresión del lado izquierdo.
/// - `operator`: operador binario.
/// - `right`: expresión del lado derecho.
/// - `_type`: tipo inferido o declarado de la expresión (opcional).
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: BinaryOperatorToken,
    pub right: Box<Expr>,
    pub _type: Option<TypeNode>,
}

impl BinaryExpr {
    /// Crea una nueva expresión binaria.
    ///
    /// # Arguments
    /// * `left` - Expresión del lado izquierdo.
    /// * `operator` - Operador binario.
    /// * `right` - Expresión del lado derecho.
    pub fn new(left: Box<Expr>, operator: BinaryOperatorToken, right: Box<Expr>) -> Self {
        BinaryExpr { left, operator, right, _type: None }
    }

    /// Establece el tipo de la expresión binaria.
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}

impl Codegen for BinaryExpr {
    /// Genera el código LLVM IR para la expresión binaria.
    ///
    /// Convierte los operandos a `double` si es necesario, selecciona la instrucción LLVM adecuada
    /// según el operador y el tipo, y emite la instrucción correspondiente.
    /// Guarda el tipo del resultado en el symbol table para su uso posterior (por ejemplo, en printf).
    fn codegen(&self, context: &mut CodegenContext) -> String {
        fn get_llvm_type(expr: &Expr) -> &'static str {
            match &expr.kind {
                crate::hulk_ast_nodes::hulk_expression::ExprKind::Number(_) => "double",
                crate::hulk_ast_nodes::hulk_expression::ExprKind::Boolean(_) => "i1",
                _ => "i32",
            }
        }

        // Generar código y obtener registros
        let mut left_reg = self.left.codegen(context);
        let mut right_reg = self.right.codegen(context);

        // Obtener tipo declarado por los nodos
        let left_type = get_llvm_type(&self.left);
        let right_type = get_llvm_type(&self.right);

        // Seguimiento del tipo real después de coerciones
        let mut left_actual_type = left_type;
        let mut right_actual_type = right_type;

        // Coerción de tipo si hay mezcla de double e i32
        if left_type == "i32" && right_type == "double" {
            // let conv = context.generate_temp();
            // context.emit(&format!("  {} = sitofp i32 {}, double", conv, left_reg));
            // left_reg = conv;
            left_actual_type = "double";
        } else if right_type == "i32" && left_type == "double" {
            // let conv = context.generate_temp();
            // context.emit(&format!("  {} = sitofp i32 {}, double", conv, right_reg));
            // right_reg = conv;
            right_actual_type = "double";
        }

        let op_type = if left_actual_type == "double" || right_actual_type == "double" {
            "double"
        } else {
            left_actual_type // "i32" o "i1"
        };

        let result_reg = context.generate_temp();

        let op_ir = match self.operator {
            BinaryOperatorToken::Plus => if op_type == "double" { "fadd" } else { "add" },
            BinaryOperatorToken::Minus => if op_type == "double" { "fsub" } else { "sub" },
            BinaryOperatorToken::Mul => if op_type == "double" { "fmul" } else { "mul" },
            BinaryOperatorToken::Div => if op_type == "double" { "fdiv" } else { "sdiv" },
            BinaryOperatorToken::Mod => if op_type == "double" { "frem" } else { "srem" },
            BinaryOperatorToken::Eq | BinaryOperatorToken::EqEq => {
                if op_type == "double" { "fcmp oeq" } else { "icmp eq" }
            }
            BinaryOperatorToken::Neq => {
                if op_type == "double" { "fcmp one" } else { "icmp ne" }
            }
            BinaryOperatorToken::Lt => {
                if op_type == "double" { "fcmp olt" } else { "icmp slt" }
            }
            BinaryOperatorToken::Gt => {
                if op_type == "double" { "fcmp ogt" } else { "icmp sgt" }
            }
            BinaryOperatorToken::Lte => {
                if op_type == "double" { "fcmp ole" } else { "icmp sle" }
            }
            BinaryOperatorToken::Gte => {
                if op_type == "double" { "fcmp oge" } else { "icmp sge" }
            }
            BinaryOperatorToken::And => "and",
            BinaryOperatorToken::Or => "or",
            BinaryOperatorToken::Pow => "pow",
            BinaryOperatorToken::Concat => "concat",
            BinaryOperatorToken::DotEqual | BinaryOperatorToken::Neg | BinaryOperatorToken::Not => {
                panic!("Operador no soportado en Codegen: {:?}", self.operator)
            }
        };

        let line = match op_ir {
            "pow" => {
                if op_type == "double" {
                    format!("  {} = call double @llvm.pow.f64(double {}, double {})", result_reg, left_reg, right_reg)
                } else {
                    format!("  {} = call i32 @llvm.powi.i32(i32 {}, i32 {})", result_reg, left_reg, right_reg)
                }
            }
            "concat" => {
                format!("  {} = call i8* @hulk_str_concat(i8* {}, i8* {})", result_reg, left_reg, right_reg)
            }
            op if op.starts_with("icmp") => {
                format!("  {} = {} {} {}, {}", result_reg, op, op_type, left_reg, right_reg)
            }
            op if op.starts_with("fcmp") => {
                format!("  {} = {} double {}, {}", result_reg, op, left_reg, right_reg)
            }
            _ => {
                format!("  {} = {} {} {}, {}", result_reg, op_ir, op_type, left_reg, right_reg)
            }
        };

        context.emit(&line);
        context.symbol_table.insert(format!("{}__type", result_reg), op_type.to_string());
        context.symbol_table.insert("__last_type__".to_string(), op_type.to_string());

        result_reg
    }
}


