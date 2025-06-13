//! # ProgramNode e Instruction AST Nodes
//!
//! Este módulo define los nodos `ProgramNode` e `Instruction` del AST para el compilador Hulk.
//! `ProgramNode` representa el nodo raíz del AST, que contiene todas las instrucciones de alto nivel de un programa Hulk.
//! `Instruction` es un enum que agrupa las posibles instrucciones de nivel superior: definición de tipos, funciones y expresiones.
//! Ambos nodos soportan integración con el visitor pattern y la generación de código LLVM IR.

use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::visitor::hulk_accept::Accept;
use crate::visitor::hulk_visitor::Visitor;
use crate::hulk_ast_nodes::hulk_type_def::HulkTypeNode;

use super::FunctionDef;

/// Nodo raíz del AST que representa un programa completo.
/// 
/// Contiene una lista de instrucciones de alto nivel (definiciones de tipos, funciones y expresiones).
#[derive(Debug, Clone)]
pub struct ProgramNode {
    pub instructions: Vec<Instruction>,
}

impl ProgramNode {
    /// Crea un nuevo nodo de programa con una lista de instrucciones.
    pub fn new(instructions: Vec<Instruction>) -> Self {
        ProgramNode { instructions }
    }

    /// Crea un nodo de programa combinando instrucciones previas, una expresión y posteriores.
    pub fn with_instructions(pre: Vec<Instruction>, expr: Box<Expr>, post: Vec<Instruction>) -> Self {
        let mut instructions = pre;
        instructions.push(Instruction::Expression(expr));
        instructions.extend(post);
        ProgramNode { instructions }
    }
}

impl Accept for ProgramNode {
    /// Permite que el nodo acepte un visitor.
    fn accept<V: Visitor<T>, T>(&mut self, visitor: &mut V) -> T {
        visitor.visit_program(self)
    }
}

/// Enum que representa las instrucciones de alto nivel de un programa Hulk.
/// 
/// - `TypeDef`: definición de tipo/clase.
/// - `FunctionDef`: definición de función.
/// - `Expression`: expresión evaluable.
#[derive(Debug, Clone)]
pub enum Instruction {
    TypeDef(HulkTypeNode),
    FunctionDef(FunctionDef),
    // Protocol(ProtocolDecl), // Futuro: soporte para protocolos
    Expression(Box<Expr>)
}

impl Instruction {
    /// Evalúa la instrucción si es una expresión, retornando su valor.
    /// Para otros tipos de instrucción, retorna un error.
    pub fn eval(&self) -> Result<f64, String> {
        match self {
            Instruction::Expression(expr) => expr.eval(),
            _ => Err("Solo se pueden evaluar expresiones.".to_string()),
        }
    }
}

impl Accept for Instruction {
    /// Permite que la instrucción acepte un visitor.
    fn accept<V: Visitor<T>, T>(&mut self, visitor: &mut V) -> T {
        match self {
            Instruction::Expression(expr) => expr.accept(visitor),
            Instruction::FunctionDef(func_def) => visitor.visit_function_def(func_def),
            Instruction::TypeDef(type_node) => visitor.visit_type_def(type_node),
        }
    }
}

impl Codegen for ProgramNode {
    /// Genera el código LLVM IR para todo el programa.
    ///
    /// Recorre todas las instrucciones y genera el código correspondiente.
    fn codegen(&self, context: &mut CodegenContext) -> String {
        let mut last_reg = String::new();
        for instr in &self.instructions {
            last_reg = instr.codegen(context);
        }
        last_reg
    }
}

impl Codegen for Instruction {
    /// Genera el código LLVM IR para una instrucción.
    ///
    /// Soporta generación para funciones y expresiones. Para definiciones de tipo, no genera código por defecto.
    fn codegen(&self, context: &mut CodegenContext) -> String {
        match self {
            Instruction::FunctionDef(func_def) => func_def.codegen(context),
            Instruction::Expression(expr) => expr.codegen(context),
            Instruction::TypeDef(_type_node) => {
                // Puedes implementar codegen para TypeDef aquí si es necesario
                String::new()
            }
        }
    }
}
