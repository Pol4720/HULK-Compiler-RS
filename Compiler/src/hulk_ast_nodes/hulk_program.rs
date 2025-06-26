//! # ProgramNode e Instruction AST Nodes
//!
//! Este módulo define los nodos `ProgramNode` e `Instruction` del AST para el compilador Hulk.
//! `ProgramNode` representa el nodo raíz del AST, que contiene todas las instrucciones de alto nivel de un programa Hulk.
//! `Instruction` es un enum que agrupa las posibles instrucciones de nivel superior: definición de tipos, funciones y expresiones.
//! Ambos nodos soportan integración con el visitor pattern y la generación de código LLVM IR.

use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::codegen::types_global::TypesGlobal;
use crate::hulk_ast_nodes::GlobalFunctionDef;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::hulk_ast_nodes::hulk_type_def::HulkTypeNode;
use crate::visitor::hulk_accept::Accept;
use crate::visitor::hulk_visitor::Visitor;

/// Nodo raíz del AST que representa un programa completo.
///
/// Contiene una lista de instrucciones de alto nivel (definiciones de tipos, funciones y expresiones).
#[derive(Debug, Clone)]
pub struct ProgramNode {
    pub instructions: Vec<Expr>,
    pub definitions: Vec<Definition>,
}

impl ProgramNode {
    /// Crea un nuevo nodo de programa con una lista de instrucciones.
    pub fn new(instructions: Vec<Expr>, definitions: Vec<Definition>) -> Self {
        ProgramNode {
            instructions,
            definitions,
        }
    }
}

impl Accept for ProgramNode {
    /// Permite que el nodo acepte un visitor.
    fn accept<V: Visitor<T>, T>(&mut self, visitor: &mut V) -> T {
        visitor.visit_program(self)
    }
}

#[derive(Debug, Clone)]
pub enum Definition {
    TypeDef(HulkTypeNode),
    FunctionDef(GlobalFunctionDef),
}

impl Definition {
    pub fn as_type_def(&self) -> Option<&HulkTypeNode> {
        if let Self::TypeDef(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_function_def(&self) -> Option<&GlobalFunctionDef> {
        if let Self::FunctionDef(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl From<GlobalFunctionDef> for Definition {
    fn from(v: GlobalFunctionDef) -> Self {
        Self::FunctionDef(v)
    }
}

impl From<HulkTypeNode> for Definition {
    fn from(v: HulkTypeNode) -> Self {
        Self::TypeDef(v)
    }
}

impl Accept for Definition {
    fn accept<V: Visitor<T>, T>(&mut self, visitor: &mut V) -> T {
        match self {
            Self::FunctionDef(func_def) => visitor.visit_function_def(&mut func_def.function_def),
            Definition::TypeDef(type_node) => visitor.visit_type_def(type_node),
        }
    }
}

impl Codegen for ProgramNode {
    /// Genera el código LLVM IR para todo el programa.
    ///
    /// Recorre todas las instrucciones y genera el código correspondiente.
    fn codegen(&self, context: &mut CodegenContext) -> String {
        let mut last_reg = String::new();

        // Registra la herencia y los miembros de los tipos antes de procesar las definiciones
        let type_defs = TypesGlobal::from_program(&self);

        // Procesa todas las definiciones (funciones y tipos)
        for def in self.definitions.iter() {
            match def {
                Definition::FunctionDef(func_def) => {
                    func_def.codegen(context); // Esto define una función global
                }
                Definition::TypeDef(type_def) => {
                    // Llama a codegen_with_type_info de TypeNode pasando context y las tablas asociadas
                    let type_name = &type_def.type_name;
                    let attrs = type_defs.attributes_map.get(type_name);
                    let methods = type_defs.methods_map.get(type_name);
                    let attr_indices = type_defs.attribute_indices.get(type_name);
                    let method_indices = type_defs.method_indices.get(type_name);

                    // Crea un nuevo contexto para el tipo, copiando la function_table del contexto original
                    let mut type_context = CodegenContext::new();
                    type_context.function_table = context.function_table.clone();

                    // Crea una copia mutable del tipo para poder llamar a codegen_with_type_info
                    let mut type_def_mut = type_def.clone();
                    type_def_mut.codegen_with_type_info(
                        &mut type_context,
                        attrs,
                        methods,
                        attr_indices,
                        method_indices,
                    );
                }
                
            }
        }
        // Luego genera el código de las instrucciones ejecutables (main, prints, exprs, etc)
        for instr in &self.instructions {
            last_reg = instr.codegen(context);
        }

        last_reg
    }
}
