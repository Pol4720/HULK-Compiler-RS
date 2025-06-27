//! # NewTypeInstance AST Node
//!
//! Este módulo define el nodo `NewTypeInstance` del AST para el compilador Hulk.
//! Permite representar la creación de nuevas instancias de tipos (clases/objetos) en el lenguaje Hulk,
//! incluyendo el nombre del tipo y los argumentos para el constructor.
//! Incluye la estructura, métodos asociados y el tipo inferido o declarado de la instancia.

use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_ast_nodes::hulk_identifier::Identifier;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::hulk_tokens::TokenPos;
use crate::typings::types_node::TypeNode;

/// Representa la creación de una nueva instancia de tipo (objeto) en el AST.
/// 
/// Por ejemplo: `Point(1, 2)`
/// 
/// - `type_name`: identificador del tipo a instanciar.
/// - `arguments`: lista de expresiones que representan los argumentos del constructor.
/// - `_type`: tipo inferido o declarado de la instancia (opcional).
#[derive(Debug, PartialEq, Clone)]
pub struct NewTypeInstance {
    pub type_name: Identifier,             
    pub arguments: Vec<Expr>,
    pub _type: Option<TypeNode>,
    pub token_pos: TokenPos,
}

impl NewTypeInstance {
    /// Crea una nueva instancia de tipo.
    ///
    /// # Arguments
    /// * `type_name` - Identificador del tipo a instanciar.
    /// * `arguments` - Vector de expresiones como argumentos del constructor.
    pub fn new(type_name: Identifier, arguments: Vec<Expr>, token_pos: TokenPos) -> Self {
        NewTypeInstance { type_name, arguments, _type: None, token_pos  }
    }

    /// Establece el tipo de la instancia creada.
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}
impl Codegen for NewTypeInstance {
     fn codegen(&self, context: &mut CodegenContext) -> String {
        let type_constructor = format!("@{}_new", self.type_name);
        // Evalúa cada argumento y obtiene el registro LLVM
        let llvm_args: Vec<String> = self.arguments.iter().map(|arg| {
            let arg_reg = arg.codegen(context);
            // Busca el tipo LLVM del argumento (si está en el contexto, si no, usa ptr)
            let arg_type = context.temp_types.get(&arg_reg).cloned().unwrap_or_else(|| "ptr".to_string());
            format!("{} {}", CodegenContext::to_llvm_type(arg_type), arg_reg)
        }).collect();
        let args_str = llvm_args.join(", ");
        let result = context.generate_temp();
        context.emit(&format!(
            "{} = call ptr {}({})",
            result, type_constructor, args_str
        ));
        // Guarda el tipo de la instancia creada en la tabla de tipos temporales y symbol_table
        let final_type = "ptr".to_string();
        context.temp_types.insert(result.clone(), final_type.clone());
        context.symbol_table.insert(format!("{}__type", result), final_type.clone());
        context.symbol_table.insert("__last_type__".to_string(), final_type);
        result
    }
}