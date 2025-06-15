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

// impl Codegen for BinaryExpr {
//     /// Genera el código LLVM IR para la expresión binaria.
//     ///
//     /// Convierte los operandos a `double` si es necesario, selecciona la instrucción LLVM adecuada
//     /// según el operador y el tipo, y emite la instrucción correspondiente.
//     /// Guarda el tipo del resultado en el symbol table para su uso posterior (por ejemplo, en printf).
//     fn codegen(&self, context: &mut CodegenContext) -> String {
//         fn get_llvm_type(expr: &Expr) -> &'static str {
//             match &expr.kind {
//                 crate::hulk_ast_nodes::hulk_expression::ExprKind::Number(_) => "double",
//                 crate::hulk_ast_nodes::hulk_expression::ExprKind::Boolean(_) => "i1",
//                 _ => "double",
//             }
//         }

//         // Generar código y obtener registros
//         let left_reg = self.left.codegen(context);
//         let right_reg = self.right.codegen(context);

//         // Obtener tipo declarado por los nodos
//         let left_type = get_llvm_type(&self.left);
//         let right_type = get_llvm_type(&self.right);

//         // Seguimiento del tipo real después de coerciones
//         let mut left_actual_type = left_type;
//         let mut right_actual_type = right_type;

//         // Coerción de tipo si hay mezcla de double e i32
//         if left_type == "i32" && right_type == "double" {
//             // let conv = context.generate_temp();
//             // context.emit(&format!("  {} = sitofp i32 {}, double", conv, left_reg));
//             // left_reg = conv;
//             left_actual_type = "double";
//         } else if right_type == "i32" && left_type == "double" {
//             // let conv = context.generate_temp();
//             // context.emit(&format!("  {} = sitofp i32 {}, double", conv, right_reg));
//             // right_reg = conv;
//             right_actual_type = "double";
//         }

//         let op_type = if left_actual_type == "double" || right_actual_type == "double" {
//             "double"
//         } else {
//             left_actual_type // "i32" o "i1"
//         };

//         let result_reg = context.generate_temp();

//         let op_ir = match self.operator {
//             BinaryOperatorToken::Plus => if op_type == "double" { "fadd" } else { "add" },
//             BinaryOperatorToken::Minus => if op_type == "double" { "fsub" } else { "sub" },
//             BinaryOperatorToken::Mul => if op_type == "double" { "fmul" } else { "mul" },
//             BinaryOperatorToken::Div => if op_type == "double" { "fdiv" } else { "sdiv" },
//             BinaryOperatorToken::Mod => if op_type == "double" { "frem" } else { "srem" },
//             BinaryOperatorToken::Eq | BinaryOperatorToken::EqEq => {
//                 if op_type == "double" { "fcmp oeq" } else { "icmp eq" }
//             }
//             BinaryOperatorToken::Neq => {
//                 if op_type == "double" { "fcmp one" } else { "icmp ne" }
//             }
//             BinaryOperatorToken::Lt => {
//                 if op_type == "double" { "fcmp olt" } else { "icmp slt" }
//             }
//             BinaryOperatorToken::Gt => {
//                 if op_type == "double" { "fcmp ogt" } else { "icmp sgt" }
//             }
//             BinaryOperatorToken::Lte => {
//                 if op_type == "double" { "fcmp ole" } else { "icmp sle" }
//             }
//             BinaryOperatorToken::Gte => {
//                 if op_type == "double" { "fcmp oge" } else { "icmp sge" }
//             }
//             BinaryOperatorToken::And => "and",
//             BinaryOperatorToken::Or => "or",
//             BinaryOperatorToken::Pow => "pow",
//             BinaryOperatorToken::Concat => "concat",
//             BinaryOperatorToken::DotEqual | BinaryOperatorToken::Neg | BinaryOperatorToken::Not => {
//                 panic!("Operador no soportado en Codegen: {:?}", self.operator)
//             }
//         };

//         let line = match op_ir {
//             "pow" => {
//                 if op_type == "double" {
//                     format!("  {} = call double @llvm.pow.f64(double {}, double {})", result_reg, left_reg, right_reg)
//                 } else {
//                     format!("  {} = call i32 @llvm.powi.i32(i32 {}, i32 {})", result_reg, left_reg, right_reg)
//                 }
//             }
//             "concat" => {
//                 format!("  {} = call i8* @hulk_str_concat(i8* {}, i8* {})", result_reg, left_reg, right_reg)
//             }
//             op if op.starts_with("icmp") => {
//                 format!("  {} = {} {} {}, {}", result_reg, op, op_type, left_reg, right_reg)
//             }
//             op if op.starts_with("fcmp") => {
//                 format!("  {} = {} double {}, {}", result_reg, op, left_reg, right_reg)
//             }
//             _ => {
//                 format!("  {} = {} {} {}, {}", result_reg, op_ir, op_type, left_reg, right_reg)
//             }
//         };

//         context.emit(&line);
//         context.symbol_table.insert(format!("{}__type", result_reg), op_type.to_string());
//         context.symbol_table.insert("__last_type__".to_string(), op_type.to_string());

//         result_reg
//     }
// }

impl Codegen for BinaryExpr {
    /// Genera el código LLVM IR para la expresión binaria.
    ///
    /// Convierte los operandos a `double` si es necesario, selecciona la instrucción LLVM adecuada
    /// según el operador y el tipo, y emite la instrucción correspondiente.
    /// Guarda el tipo del resultado en el symbol table para su uso posterior (por ejemplo, en printf).
    fn codegen(&self, context: &mut CodegenContext) -> String {
        fn get_llvm_type(expr: &Expr, context: &CodegenContext) -> &'static str {
            match &expr.kind {
            crate::hulk_ast_nodes::hulk_expression::ExprKind::Number(_) => "double",
            crate::hulk_ast_nodes::hulk_expression::ExprKind::Boolean(_) => "i1",
            crate::hulk_ast_nodes::hulk_expression::ExprKind::String(_) => "i8*",
            crate::hulk_ast_nodes::hulk_expression::ExprKind::Identifier( ident) => {
                // Busca el tipo de la variable en la symbol_table
                if let Some(var_type) = context.symbol_table.get(&format!("{}__type", ident.id)) {
                match var_type.as_str() {
                    "double" => "double",
                    "i1" => "i1",
                    "i8*" => "i8*",
                    _ => "double", // Por defecto
                }
                } else {
                // Si no se encuentra, asume double por defecto
                "double"
                }
            },
            _ => "double",
            }
        }
        

        // Generar los operandos
        let left_reg = self.left.codegen(context);
        let right_reg = self.right.codegen(context);

        // Obtener tipo declarado por los nodos
        let left_type = get_llvm_type(&self.left, context);
        // let right_type = get_llvm_type(&self.right, context);

        let  left = left_reg;
        let  right = right_reg;

        let final_type: &'static str = left_type;

        // // Realiza coerción de tipos si es necesario
        // match (left_type, right_type) {
        //     ("i1", "double") => {
        //         let casted = context.generate_temp();
        //         context.emit(&format!("  {} = uitofp i1 {} to double", casted, left));
        //         left = casted;
        //         final_type = "double";
        //     }
        //     ("double", "i1") => {
        //         let casted = context.generate_temp();
        //         context.emit(&format!("  {} = uitofp i1 {} to double", casted, right));
        //         right = casted;
        //         final_type = "double";
        //     }
        //     ("i1", "i32") => {
        //         let casted = context.generate_temp();
        //         context.emit(&format!("  {} = zext i1 {} to i32", casted, left));
        //         left = casted;
        //         final_type = "i32";
        //     }
        //     ("i32", "i1") => {
        //         let casted = context.generate_temp();
        //         context.emit(&format!("  {} = zext i1 {} to i32", casted, right));
        //         right = casted;
        //         final_type = "i32";
        //     }
        //     ("i1", "i1") => final_type = "i1",
        //     ("double", "double") => final_type = "double",
        //     ("i32", "i32") => final_type = "i32",
        //     ("i8*", "i8*") => final_type = "i8*",
        //     ("i8*", _) | (_, "i8*") => {
        //         if self.operator == BinaryOperatorToken::Concat {
        //             final_type = "i8*";
        //         } else {
        //             panic!("No se puede operar entre tipos incompatibles: {} y {}", left_type, right_type);
        //         }
        //     }
        //     _ => panic!("No se puede operar entre tipos incompatibles: {} y {}", left_type, right_type),
        // }

        let result = context.generate_temp();

        let ir_code = match self.operator {
            BinaryOperatorToken::Plus => format!("  {} = fadd double {}, {}", result, left, right),
            BinaryOperatorToken::Minus => format!("  {} = fsub double {}, {}", result, left, right),
            BinaryOperatorToken::Mul => format!("  {} = fmul double {}, {}", result, left, right),
            BinaryOperatorToken::Pow => format!("  {} = call double @llvm.pow.f64(double {}, double {})", result, left, right),
            BinaryOperatorToken::Or => format!("  {} = or i1 {}, {}", result, left, right),
            BinaryOperatorToken::And => format!("  {} = and i1 {}, {}", result, left, right),
            BinaryOperatorToken::Concat => {
                format!("  {} = call i8* @hulk_str_concat(i8* {}, i8* {})", result, left, right)
            }
            BinaryOperatorToken::Div => format!("  {} = fdiv double {}, {}", result, left, right),
            BinaryOperatorToken::Mod => format!("  {} = frem double {}, {}", result, left, right),
            BinaryOperatorToken::Eq | BinaryOperatorToken::EqEq => {
                match final_type {
                    "i8*" => format!("  {} = call i1 @hulk_str_eq(i8* {}, i8* {})", result, left, right),
                    "i1" => format!("  {} = icmp eq i1 {}, {}", result, left, right),
                    _ => format!("  {} = fcmp oeq double {}, {}", result, left, right),
                }
            }
            BinaryOperatorToken::Neq => match final_type {
                "i8*" => format!("  {} = call i1 @hulk_str_neq(i8* {}, i8* {})", result, left, right),
                "i1" => format!("  {} = icmp ne i1 {}, {}", result, left, right),
                _ => format!("  {} = fcmp one double {}, {}", result, left, right),
            }
            BinaryOperatorToken::Lt => match final_type {
                "i8*" => format!("  {} = call i1 @hulk_str_lt(i8* {}, i8* {})", result, left, right),
                "i1" => format!("  {} = icmp ult i1 {}, {}", result, left, right),
                _ => format!("  {} = fcmp olt double {}, {}", result, left, right),
            },
            BinaryOperatorToken::Gt => match final_type {
                "i8*" => format!("  {} = call i1 @hulk_str_gt(i8* {}, i8* {})", result, left, right),
                "i1" => format!("  {} = icmp ugt i1 {}, {}", result, left, right),
                _ => format!("  {} = fcmp ogt double {}, {}", result, left, right),
            },
            BinaryOperatorToken::Lte => match final_type {
                "i8*" => format!("  {} = call i1 @hulk_str_lte(i8* {}, i8* {})", result, left, right),
                "i1" => format!("  {} = icmp ule i1 {}, {}", result, left, right),
                _ => format!("  {} = fcmp ole double {}, {}", result, left, right),
            },
            BinaryOperatorToken::Gte => match final_type {
                "i8*" => format!("  {} = call i1 @hulk_str_gte(i8* {}, i8* {})", result, left, right),
                "i1" => format!("  {} = icmp uge i1 {}, {}", result, left, right),
                _ => format!("  {} = fcmp oge double {}, {}", result, left, right),
            },
            // Agrega otros operadores aquí si lo deseas
            _ => panic!("Operador no implementado aún: {:?}", self.operator),
        };

        context.emit(&ir_code);
        context.symbol_table.insert(format!("{}__type", result), final_type.to_string());
        context.symbol_table.insert("__last_type__".to_string(), final_type.to_string());

        result
    }
}



