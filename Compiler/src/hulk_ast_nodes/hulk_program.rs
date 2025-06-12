use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::visitor::hulk_accept::Accept;
use crate::visitor::hulk_visitor::Visitor;
use crate::hulk_ast_nodes::hulk_type_def::HulkTypeNode;

use super::FunctionDef;

#[derive(Debug, Clone)]
pub struct ProgramNode {
    pub instructions: Vec<Instruction>,
}

impl ProgramNode {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        ProgramNode { instructions: instructions }
    }
    pub fn with_instructions(pre: Vec<Instruction>, expr: Box<Expr>, post: Vec<Instruction>) -> Self {
        let mut instructions = pre;
        instructions.push(Instruction::Expression(expr));
        instructions.extend(post);
        ProgramNode { instructions }
    }
}

impl Accept for ProgramNode {
    fn accept<V: Visitor<T>, T>(&mut self, visitor: &mut V) -> T {
        visitor.visit_program(self)
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    TypeDef(HulkTypeNode),
    FunctionDef(FunctionDef),
//    Protocol(ProtocolDecl),
   Expression(Box<Expr>)
}

impl Instruction {
    pub fn eval(&self) -> Result<f64, String> {
        match self {
            Instruction::Expression(expr) => expr.eval(),
            _ => Err("Solo se pueden evaluar expresiones.".to_string()),
        }
    }
}

impl Accept for Instruction {
    fn accept<V: Visitor<T>, T>(&mut self, visitor: &mut V) -> T {
        match self {
            Instruction::Expression(expr) => expr.accept(visitor),
            Instruction::FunctionDef(func_def) => visitor.visit_function_def(func_def),
            Instruction::TypeDef(type_node) => visitor.visit_type_def(type_node),
        }
    }
}


impl Codegen for ProgramNode {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        let mut last_reg = String::new();
        for instr in &self.instructions {
            last_reg = instr.codegen(context);
        }
        last_reg
    }
}

impl Codegen for Instruction {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        match self {
            Instruction::FunctionDef(func_def) => func_def.codegen(context),
            Instruction::Expression(expr) => expr.codegen(context),
            Instruction::TypeDef(_type_node) => {
                // You can implement codegen for TypeDef here if needed
                String::new()
            }
        }
    }
}
