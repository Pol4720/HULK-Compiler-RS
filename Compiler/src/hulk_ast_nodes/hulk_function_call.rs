//! # FunctionCall AST Node
//!
//! Este módulo define el nodo de llamada a función (`FunctionCall`) del AST para el compilador Hulk.
//! Permite representar y generar código para llamadas a funciones, incluyendo el nombre de la función, los argumentos y el tipo de retorno inferido o declarado.

use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::hulk_tokens::{token_pos, TokenPos};
use crate::typings::types_node::TypeNode;

/// Representa una llamada a función en el AST.
/// 
/// Por ejemplo: `foo(1, 2)`
/// 
/// - `funct_name`: nombre de la función a llamar.
/// - `arguments`: lista de expresiones que representan los argumentos.
/// - `_type`: tipo inferido o declarado del resultado de la llamada (opcional).
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub funct_name: String,             
    pub arguments: Vec<Expr>,
    pub _type: Option<TypeNode>,
    pub token_pos: TokenPos
}

impl FunctionCall {
    /// Crea una nueva llamada a función.
    ///
    /// # Arguments
    /// * `funct_name` - Nombre de la función.
    /// * `arguments` - Vector de expresiones como argumentos.
    pub fn new(funct_name: String, arguments: Vec<Expr>, token_pos:TokenPos) -> Self {
        FunctionCall { funct_name, arguments, _type: None , token_pos }
    }

    /// Establece el tipo de la expresión de la llamada a función.
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}

impl Codegen for FunctionCall {
    /// Genera el código LLVM IR para la llamada a función.
    ///
    /// Genera el código para cada argumento, prepara la lista de argumentos para LLVM IR (asumiendo `i32` para todos),
    /// obtiene un nuevo registro temporal para el resultado y emite la instrucción de llamada.
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // 1. Genera el código de los argumentos y guarda los registros y tipos
        let mut llvm_args = Vec::new();

        for arg in &self.arguments {
            let reg = arg.codegen(context);
            // let ty_str = arg.kind
            let llvm_type = context
                .symbol_table
                .get("__last_type__")
                .cloned()
                .expect("Tipo no encontrado");

            // let llvm_ty = CodegenContext::to_llvm_type(ty_str);
            llvm_args.push(format!("{} {}", llvm_type, reg));
        }

        let args_str = llvm_args.join(", ");
        
        // 2. Determina el tipo de retorno de la función llamada
        let return_type_str = context
            .function_table
            .get(&self.funct_name)
            .expect(&format!("Tipo de retorno de la función no encontrado, function name: {}", self.funct_name))
            .clone();

        let llvm_ret_type = return_type_str.to_string();

        // 3. Emitimos la llamada
        let result_reg = context.generate_temp();
        context.emit(&format!(
            "  {} = call {} @{}({})",
            result_reg, llvm_ret_type, self.funct_name, args_str
        ));

        //Actualizar contexto
        let type_function = context.f_table.get(&self.funct_name);
        if let Some(type_name) = type_function {
            context.add_register_hulk_type(result_reg.clone(), type_name.clone());
        }
        context
            .symbol_table
            .insert("__last_type__".to_string(), llvm_ret_type.clone());

        result_reg
    }
}

