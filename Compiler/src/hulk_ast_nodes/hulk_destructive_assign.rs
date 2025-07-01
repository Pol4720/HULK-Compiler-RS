//! # DestructiveAssignment AST Node
//!
//! Este módulo define el nodo de asignación destructiva (`DestructiveAssignment`) del AST para el compilador Hulk.
//! Una asignación destructiva permite modificar el valor de una variable o propiedad existente, por ejemplo: `x := 5`.
//! Incluye la estructura, métodos asociados y la generación de código LLVM IR.

use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::hulk_ast_nodes::hulk_expression::ExprKind;
use crate::hulk_tokens::TokenPos;
use crate::typings::types_node::TypeNode;

/// Representa una asignación destructiva en el AST.
/// 
/// Por ejemplo: `x := 5`
/// 
/// - `identifier`: expresión que representa el identificador o propiedad a modificar.
/// - `expression`: expresión cuyo valor se asigna.
/// - `_type`: tipo inferido o declarado de la asignación (opcional).
#[derive(Debug, PartialEq, Clone)]
pub struct DestructiveAssignment {
    pub identifier: Box<Expr>,
    pub expression: Box<Expr>,
    pub _type: Option<TypeNode>,
    pub token_pos: TokenPos,
}

impl DestructiveAssignment {
    /// Crea una nueva asignación destructiva.
    ///
    /// # Arguments
    /// * `identifier` - Expresión que representa el identificador o propiedad.
    /// * `expression` - Expresión a asignar.
    pub fn new(identifier: Box<Expr>, expression: Expr, token_pos: TokenPos) -> Self {
        Self {
            identifier,
            expression: Box::new(expression),
            _type: None,
            token_pos,
        }
    }

    /// Establece el tipo de la expresión asignada.
    pub fn set_expression_type(&mut self, _type: TypeNode){
        self._type = Some(_type)
    }
}
impl Codegen for DestructiveAssignment {
    /// Genera el código LLVM IR para la asignación destructiva.
    ///
    /// Busca el puntero de la variable en el contexto y almacena el valor generado por la expresión.
    /// Si la variable no existe en el contexto, lanza un panic.
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // Generar el valor de la expresión
        let value_reg = self.expression.codegen(context);
        
        // Obtener el tipo inferido
        let hulk_type = self._type.clone().expect("DestructiveAssignment debe tener tipo inferido");
        let llvm_type = CodegenContext::to_llvm_type(hulk_type.type_name);

        // Manejar diferentes tipos de identificadores en el lado izquierdo
        match &self.identifier.kind {
            // Caso 1: Identificador simple (variable)
            ExprKind::Identifier(name) => {
                let ptr = context.symbol_table.get(&name.id).cloned().unwrap_or_else(|| {
                    panic!("Variable '{}' no definida en el contexto para asignación destructiva", name.id)
                });
                
                context.emit(&format!("  store {} {}, {}* {}", llvm_type, value_reg, llvm_type, ptr));
            },
            
            // Caso 2: Acceso a miembro de un objeto
            ExprKind::MemberAccess(member_access) => {
                let obj_type = context.current_self.clone().expect("No current self for member access");
                let prop_reg = context.generate_temp();
                
                let prop_index = context.type_members_ids
                    .get(&(obj_type.clone(), member_access.member.id.clone()))
                    .expect("Miembro no encontrado en el tipo");
                
                // Generar código para acceder a la propiedad
                context.emit(&format!(
                    "{} = getelementptr %{}_type, ptr %self.{}, i32 0, i32 {}",
                    prop_reg, 
                    obj_type,
                    context.get_scope(),
                    prop_index
                ));
                
                // Almacenar el valor en la propiedad
                context.emit(&format!(
                    "  store {} {}, ptr {}", 
                    llvm_type, 
                    value_reg, 
                    prop_reg
                ));
            },
            
            _ => panic!("Tipo de expresión no soportado en el lado izquierdo de asignación destructiva"),
        }
        
        value_reg
    }
}
