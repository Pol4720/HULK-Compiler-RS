//! # FunctionAccess AST Node
//!
//! Este módulo define el nodo de acceso a función (`FunctionAccess`) del AST para el compilador Hulk.
//! Permite representar llamadas a métodos sobre objetos, como `obj.metodo()`.
//! Incluye la estructura, métodos asociados y el tipo inferido o declarado de la expresión.

use crate::{codegen::{context::CodegenContext, traits::Codegen}, hulk_ast_nodes::{Expr, FunctionCall}, hulk_tokens::{token_pos, TokenPos}, typings::types_node::TypeNode};


/// Representa el acceso a una función (método) de un objeto en el AST.
/// 
/// Por ejemplo: `obj.metodo()`
/// 
/// - `object`: expresión que representa el objeto sobre el que se accede al método.
/// - `member`: llamada a función que representa el método.
/// - `_type`: tipo inferido o declarado de la expresión (opcional).
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionAccess {
    pub object: Box<Expr>,
    pub member: Box<FunctionCall>,
    pub _type: Option<TypeNode>, 
    pub token_pos: TokenPos
}

impl FunctionAccess {
    /// Crea un nuevo acceso a función.
    ///
    /// # Arguments
    /// * `object` - Expresión del objeto.
    /// * `member` - Llamada a función (método) sobre el objeto.
    pub fn new(object: Expr, member: FunctionCall, token_pos: TokenPos) -> Self {
        Self {
            object: Box::new(object),
            member: Box::new(member),
            _type: None,
            token_pos
        }
    }

    /// Establece el tipo de la expresión de acceso a función.
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}
impl Codegen for FunctionAccess {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // Evalúa el objeto original - guardamos esta referencia para usarla después
        let original_object_reg = self.object.codegen(context);
        let original_object_type = context.get_register_hulk_type(&original_object_reg).cloned()
            .unwrap_or_else(|| panic!("Could not determine object type for method call"));
        
        // Copia del objeto para buscar el método en la jerarquía
        let mut curr_type_reg_ptr = original_object_reg.clone();
        let mut curr_object_type = original_object_type.clone();
        
        let function_name = self.member.funct_name.clone();

        // Busca el método en la jerarquía de herencia (esto no cambia)
        while !context.type_functions_ids.contains_key(&(curr_object_type.clone(), function_name.clone())) {
            let parent_opt = context.inherits.get(&curr_object_type).cloned();
            if let Some(parent) = parent_opt {
                let parent_ptr_ptr = context.generate_temp();
                context.emit(&format!("{} = getelementptr %{}_type, ptr {}, i32 0, i32 1", 
                    parent_ptr_ptr, curr_object_type, curr_type_reg_ptr));
                let parent_ptr = context.generate_temp();
                context.emit(&format!("{} = load ptr, ptr {}", parent_ptr, parent_ptr_ptr));
                curr_object_type = parent;
                curr_type_reg_ptr = parent_ptr;
            } else {
                panic!("Method '{}' not found in type hierarchy.", function_name);
            }
        }

        // Obtiene el índice del método en la jerarquía
        let function_index = *context.type_functions_ids.get(&(curr_object_type.clone(), function_name.clone())).unwrap();
        
        // IMPORTANTE: Obtenemos el ID del tipo original del objeto
        // Este debería ser un entero literal en tiempo de compilación, no un valor en tiempo de ejecución
        let type_id = match context.type_ids.get(&original_object_type) {
            Some(id) => *id,
            None => panic!("Type ID not found for type: {}", original_object_type)
        };
        
        // Usa el type_id del tipo concreto y el índice del método
        let func_ptr = context.generate_temp();
        context.emit(&format!(
            "{} = call ptr @get_vtable_method(i32 {}, i32 {})", 
            func_ptr, 
            type_id, 
            function_index
        ));
        
        // Prepara los argumentos
        let mut llvm_args: Vec<String> = Vec::new();
        llvm_args.push(format!("ptr {}", original_object_reg)); // Usa el objeto original
        for arg in self.member.arguments.iter() {
            let arg_reg = arg.codegen(context);
            let arg_type = context.temp_types.get(&arg_reg).cloned().unwrap_or_else(|| "ptr".to_string());
            llvm_args.push(format!("{} {}", CodegenContext::to_llvm_type(arg_type), arg_reg));
        }
        let args_str = llvm_args.join(", ");
        // Determina el tipo de retorno
        let return_type = self._type.as_ref().map(|t| t.type_name.clone()).unwrap_or_else(|| "ptr".to_string());
        let return_llvm = CodegenContext::to_llvm_type(return_type.clone());
        let temp = context.generate_temp();
        context.emit(&format!(
            "{} = call {} {}({})",
            temp, return_llvm, func_ptr, args_str
        ));
        context
            .symbol_table
            .insert("__last_type__".to_string(), return_llvm.clone());
        temp
    }
}