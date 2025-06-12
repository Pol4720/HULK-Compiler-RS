use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::hulk_ast_nodes::hulk_expression::ExprKind;
use crate::typings::types_node::TypeNode;

#[derive(Debug, PartialEq, Clone)]
pub struct DestructiveAssignment {
    pub identifier: Box<Expr>,
    pub expression: Box<Expr>,
    pub _type: Option<TypeNode>,
}

impl DestructiveAssignment {
    pub fn new(identifier: Box<Expr>, expression: Expr) -> Self {
        Self {
            identifier,
            expression: Box::new(expression),
            _type: None,
        }
    }
    pub fn set_expression_type(&mut self, _type: TypeNode){
        self._type = Some(_type)
    }
}

impl Codegen for DestructiveAssignment {
    fn codegen(&self, context: &mut CodegenContext) -> String {

        let var_name = match *self.identifier {
            Expr {
                kind: ExprKind::Identifier(ref name),
                ..
            } => name,
            _ => panic!("Expected identifier on left side of destructive assignment"),
        };
        let ptr = context.symbol_table.get(&var_name.to_string()).cloned();
        if let Some(ptr) = ptr {
            let value_reg = self.expression.codegen(context);
            context.emit(&format!("  store i32 {}, i32* {}", value_reg, ptr));
            value_reg
        } else {
            panic!(
                "Variable '{}' no definida en el contexto para asignación destructiva",
                var_name
            );
        }
    }
}
