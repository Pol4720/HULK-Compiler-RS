//! # MemberAccess AST Node
//!
//! Este módulo define el nodo `MemberAccess` del AST para el compilador Hulk.
//! Permite representar el acceso a miembros o propiedades de un objeto, como `obj.prop`.
//! Incluye la estructura, métodos asociados y el tipo inferido o declarado de la expresión.

use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::hulk_ast_nodes::hulk_identifier::Identifier;
use crate::hulk_tokens::TokenPos;
use crate::typings::types_node::TypeNode;

/// Representa el acceso a un miembro (propiedad o campo) de un objeto en el AST.
///
/// Por ejemplo: `obj.prop`
///
/// - `object`: expresión que representa el objeto.
/// - `member`: identificador del miembro al que se accede.
/// - `_type`: tipo inferido o declarado del acceso (opcional).
#[derive(Debug, Clone, PartialEq)]
pub struct MemberAccess {
    pub object: Box<Expr>,
    pub member: Identifier,
    pub _type: Option<TypeNode>, // Tipo opcional para el acceso al miembro
    pub token_pos: TokenPos,
}

impl MemberAccess {
    /// Crea un nuevo acceso a miembro.
    ///
    /// # Arguments
    /// * `object` - Expresión del objeto.
    /// * `member` - Identificador del miembro.
    pub fn new(object: Expr, member: Identifier, token_pos: TokenPos) -> Self {
        Self {
            object: Box::new(object),
            member,
            _type: None,
            token_pos,
        }
    }

    /// Establece el tipo de la expresión de acceso a miembro.
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}
impl Codegen for MemberAccess {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // Evalúa el objeto
        let object_reg = self.object.codegen(context);
        print!("{}", &object_reg);
        // Intenta deducir el tipo del objeto
        let object_type = context.get_register_hulk_type(&object_reg).cloned().unwrap_or_else(|| "candela".to_string());
        print!("{}", &object_type);
        // Obtiene el índice del miembro
        let member_index_val = {
            let key = (object_type.clone(), self.member.to_string());
            *context.type_members_ids.get(&key).expect("Member not found")
        };
        // Determina el tipo LLVM del campo
        let node_type = self._type.as_ref().map(|t| t.type_name.clone()).unwrap_or_else(|| "ptr".to_string());
        let llvm_type = CodegenContext::to_llvm_type(node_type.clone());
        // Obtiene el puntero al campo
        let ptr_temp = context.generate_temp();
        context.emit(&format!(
            "{} = getelementptr %{}_type, ptr %self.{}, i32 0 , i32 {}",
            ptr_temp, object_type, context.get_scope(), member_index_val// +2 por vtable y parent
        ));
        // Carga el valor del campo
        let result = context.generate_temp();
        context.emit(&format!(
            "{} = load {}, ptr {}",
            result, llvm_type, ptr_temp
        ));
        // Registra el tipo temporal para futuras inferencias
        context.temp_types.insert(result.clone(), node_type);
        result
    }
}
