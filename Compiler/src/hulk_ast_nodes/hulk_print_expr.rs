use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_tokens::TokenPos;
use crate::{hulk_ast_nodes::Expr, typings::types_node::TypeNode};

#[derive(Debug, PartialEq, Clone)]

pub struct PrintExpr {
    pub expr: Box<Expr>,
    pub _type: Option<TypeNode>,
    pub token_pos: TokenPos,
}

impl PrintExpr {
    pub fn new(expr: Box<Expr>, _type: Option<TypeNode>, token_pos: TokenPos) -> Self {
        PrintExpr { expr, _type , token_pos }
    }

    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}

impl Codegen for PrintExpr {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // Genera el valor de la expresión a imprimir
        let value_reg = self.expr.codegen(context);

        // Detecta el tipo inferido
        let hulk_type = self
            ._type
            .clone()
            .expect("PrintExpr debe tener un tipo inferido");
        let llvm_type = CodegenContext::to_llvm_type(hulk_type.type_name);

        match llvm_type.as_str() {
            "double" => {
                context.emit(&format!(
            "  call i32 (i8*, ...) @printf(i8* getelementptr ([4 x i8], [4 x i8]* @format_double, i32 0, i32 0), double {})",
            value_reg
        ));
            }
            "i32" => {
                context.emit(&format!(
            "  call i32 (i8*, ...) @printf(i8* getelementptr ([4 x i8], [4 x i8]* @format_int, i32 0, i32 0), i32 {})",
            value_reg
        ));
            }
            "i1" => {
                // Amplía i1 a i32 antes de imprimir
                let extended = context.generate_temp();
                context.emit(&format!("  {} = zext i1 {} to i32", extended, value_reg));
                context.emit(&format!(
            "  call i32 (i8*, ...) @printf(i8* getelementptr ([4 x i8], [4 x i8]* @format_int, i32 0, i32 0), i32 {})",
            extended
        ));
            }
            "i8*" => {
                context.emit(&format!(
            "  call i32 (i8*, ...) @printf(i8* getelementptr ([3 x i8], [3 x i8]* @format_str, i32 0, i32 0), i8* {})",
            value_reg
        ));
            }
            _ => panic!("Tipo no soportado para impresión: {}", llvm_type),
        }

        // Devuelve el valor del argumento del print
        value_reg
    }
}


