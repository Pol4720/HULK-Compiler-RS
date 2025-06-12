use super::hulk_identifier::Identifier;
use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::typings::types_node::TypeNode;
use crate::visitor::hulk_accept::Accept;
use crate::visitor::hulk_visitor::Visitor;

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub identifier: Identifier,
    pub expression: Box<Expr>,
    pub _type: Option<TypeNode>,
}

impl Assignment {
    pub fn new(identifier: Identifier, expression: Box<Expr>) -> Self {
        Assignment { identifier, expression, _type: None }
    }

    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}



impl Accept for Assignment {
    fn accept<V: Visitor<T>, T>(&mut self, visitor: &mut V) -> T {
        visitor.visit_assignment(self)
    }
}

impl Codegen for Assignment {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        let var_name = &self.identifier.id;
        let ptr = context.symbol_table.get(var_name).cloned();
        if let Some(ptr) = ptr {
            let value_reg = self.expression.codegen(context);
            context.emit(&format!("  store i32 {}, i32* {}", value_reg, ptr));
            value_reg
        } else {
            panic!(
                "Variable '{}' no definida en el contexto para asignación",
                var_name
            );
        }
    }
}
