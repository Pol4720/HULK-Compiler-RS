use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::typings::types_node::TypeNode;

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub funct_name: String,             
    pub arguments: Vec<Expr>,
    pub _type: Option<TypeNode>,
}


impl FunctionCall {
    pub fn new(funct_name: String, arguments: Vec<Expr>) -> Self {
        FunctionCall { funct_name, arguments, _type: None }
    }
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}

impl Codegen for FunctionCall {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // Genera el código para cada argumento y obtiene los registros
        let arg_regs: Vec<String> = self
            .arguments
            .iter()
            .map(|arg| arg.codegen(context))
            .collect();
        // Prepara la lista de argumentos para LLVM IR (asume i32 para todos)
        let args_str = arg_regs
            .iter()
            .map(|reg| format!("i32 {}", reg))
            .collect::<Vec<_>>()
            .join(", ");
        // Obtiene un nuevo registro temporal para el resultado
        let result_reg = context.generate_temp();
        // Emite la instrucción de llamada
        context.emit(&format!(
            "  {} = call i32 @{}({})",
            result_reg, self.funct_name, args_str
        ));
        result_reg
    }
}
