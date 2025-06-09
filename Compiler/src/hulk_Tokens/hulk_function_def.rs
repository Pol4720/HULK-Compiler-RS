use std::fmt;

use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_tokens::hulk_expression::Expr;

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionParams {
    pub name: String,
    pub signature: String,
}

impl FunctionParams {
    pub fn new(name: String, signature: String) -> Self {
        FunctionParams { name, signature }
    }
}

impl fmt::Display for FunctionParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Adjust this to print your parameters as needed
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<FunctionParams>,
    pub return_type: String,
    pub body: Box<Expr>,
}

impl FunctionDef {
    pub fn new_expr(
        name: String,
        params: Vec<FunctionParams>,
        return_type: String,
        expr: Box<Expr>,
    ) -> Self {
        FunctionDef {
            name,
            params,
            return_type,
            body: expr,
        }
    }
}

impl Codegen for FunctionParams {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // Genera un nombre de argumento LLVM (por ejemplo, %x)
        let arg_name = format!("%{}", self.name);
        // Reserva espacio local para el argumento
        let alloca_reg = context.generate_temp();
        context.emit(&format!("  {} = alloca i32", alloca_reg));
        // Almacena el argumento en el espacio local
        context.emit(&format!("  store i32 {}, i32* {}", arg_name, alloca_reg));
        // Registra el parámetro en la tabla de símbolos
        context.register_variable(&self.name, alloca_reg.clone());
        alloca_reg
    }
}

impl Codegen for FunctionDef {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // Prepara la lista de parámetros para LLVM IR
        let params_ir: Vec<String> = self
            .params
            .iter()
            .map(|p| format!("i32 %{}", p.name))
            .collect();
        let params_str = params_ir.join(", ");
        // Cabecera de la función
        context.emit(&format!("define i32 @{}({}) {{", self.name, params_str));
        // Prologo: asigna espacio y almacena los argumentos
        for param in &self.params {
            param.codegen(context);
        }
        // Genera el cuerpo de la función
        let ret_val = self.body.codegen(context);
        // Retorno
        context.emit(&format!("  ret i32 {}", ret_val));
        // Cierre de la función
        context.emit("}");
        String::new() // No se usa el valor de retorno
    }
}
