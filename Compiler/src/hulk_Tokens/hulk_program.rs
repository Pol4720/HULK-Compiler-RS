use crate::hulk_tokens::hulk_expression::Expr;
use crate::visitor::hulk_accept::Accept;
use crate::visitor::hulk_visitor::Visitor;

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

impl ProgramNode {
    pub fn to_tree(&self, indent: usize) -> String {
        let padding = "  ".repeat(indent);
        let instrs = self.instructions
            .iter()
            .map(|i| i.to_tree(indent + 1))
            .collect::<Vec<_>>()
            .join("\n");
        format!("{}Program\n{}", padding, instrs)
    }
}

impl Accept for ProgramNode {
    fn accept<V: Visitor<T>, T>(&self, visitor: &mut V) -> T {
        visitor.visit_program(self)
    }
}


#[derive(Debug, Clone)]
pub enum Instruction {
//    Class(ClassDecl),
   FunctionDef(FunctionDef),
//    Protocol(ProtocolDecl),
   Expression(Box<Expr>)
}


impl Instruction {
    pub fn to_tree(&self, indent: usize) -> String {
        match self {
            Instruction::Expression(expr) => expr.to_tree(indent),
            Instruction::FunctionDef(func_def) => format!(
                "{}FunctionDef:\n{}FullDef {}({:?})\n{}",
                "  ".repeat(indent),
                "  ".repeat(indent + 1),
                func_def.name,
                func_def.params,
                func_def.body.to_tree(indent + 2)
            ),
        }
    }

    pub fn eval(&self) -> Result<f64, String> {
        match self {
            Instruction::Expression(expr) => expr.eval(),
            _ => Err("Solo se pueden evaluar expresiones.".to_string()),
        }
    }
}
impl Accept for Instruction {
    fn accept<V: Visitor<T>, T>(&self, visitor: &mut V) -> T {
        match self {
            Instruction::Expression(expr) => expr.accept(visitor),
            Instruction::FunctionDef(func_def) => visitor.visit_function_def(func_def)
        }
    }
}
