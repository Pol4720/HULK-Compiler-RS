//! # FunctionDef y FunctionParams AST Nodes
//!
//! Este módulo define los nodos de definición de función (`FunctionDef`) y de parámetros de función (`FunctionParams`) del AST para el compilador Hulk.
//! Permite representar funciones, sus parámetros, el tipo de retorno, el cuerpo y la integración con el visitor pattern y la generación de código LLVM IR.

use std::fmt;

use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_ast_nodes::hulk_expression::{Expr, ExprKind};
use crate::hulk_ast_nodes::Block;
use crate::typings::types_node::TypeNode;
use crate::visitor::hulk_accept::Accept;
use crate::visitor::hulk_visitor::Visitor;


#[derive(Debug, Clone, PartialEq)]
pub enum FunctionBody {
    Block(Block),
    ArrowExpression(ArrowExpression),
}

impl FunctionBody {
    pub fn as_arrow_expression(&self) -> Option<&ArrowExpression> {
        if let Self::ArrowExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_block(&self) -> Option<&Block> {
        if let Self::Block(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl From<Box<Expr>> for FunctionBody {
    fn from(expr: Box<Expr>) -> Self {
        match expr.kind {
            ExprKind::CodeBlock(block) => FunctionBody::from(block),
            _ => {
                let arrow = ArrowExpression::new(expr);
                FunctionBody::from(arrow)
            }
        }
    }
}


impl From<Block> for FunctionBody {
    fn from(v: Block) -> Self {
        Self::Block(v)
    }
}

impl From<ArrowExpression> for FunctionBody {
    fn from(v: ArrowExpression) -> Self {
        Self::ArrowExpression(v)
    }
}

impl Codegen for FunctionBody {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        match self {
            FunctionBody::Block(b) => b.codegen(context),
            FunctionBody::ArrowExpression(a) => a.codegen(context),
        }
    }
}

impl Accept for FunctionBody {
    fn accept<V: Visitor<T>, T>(&mut self, visitor: &mut V) -> T{
        match self {
            FunctionBody::Block(b) => visitor.visit_code_block(b),
            FunctionBody::ArrowExpression(a) => a.expression.accept(visitor),
        }
    }
}

/// Representa un parámetro de función en el AST.
/// 
/// - `name`: nombre del parámetro.
/// - `param_type`: tipo del parámetro.
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionParams {
    pub name: String,
    pub param_type: String,
}

impl FunctionParams {
    /// Crea un nuevo parámetro de función.
    ///
    /// # Arguments
    /// * `name` - Nombre del parámetro.
    /// * `param_type` - Tipo del parámetro.
    pub fn new(name: String, param_type: String) -> Self {
        FunctionParams { name, param_type }
    }
}

impl fmt::Display for FunctionParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionHeaderStruct {
    pub name: String,
    pub params: Vec<FunctionParams>,
    pub signature: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrowExpression {
    pub expression: Box<Expr>,
}

impl ArrowExpression {
    pub fn new(expression: Box<Expr>) -> Self {
        Self {
            expression,
        }
    }
}

impl Codegen for ArrowExpression {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        self.expression.codegen(context)
    }
}

/// Representa la definición de una función en el AST.
/// 
/// - `name`: nombre de la función.
/// - `params`: lista de parámetros.
/// - `return_type`: tipo de retorno de la función.
/// - `body`: cuerpo de la función (expresión).
/// - `_type`: tipo inferido o declarado de la función (opcional).
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<FunctionParams>,
    pub return_type: String,
    pub body: FunctionBody,
    pub _type: Option<TypeNode>,
}

impl FunctionDef {
    /// Crea una nueva definición de función a partir de una expresión.
    ///
    /// # Arguments
    /// * `name` - Nombre de la función.
    /// * `params` - Vector de parámetros.
    /// * `return_type` - Tipo de retorno.
    /// * `expr` - Cuerpo de la función.
    pub fn new_expr(name: String, params: Vec<FunctionParams>, return_type: String, body: Box<Expr>) -> Self {
        FunctionDef {
            name,
            params,
            return_type,
            body: FunctionBody::from(body),
            _type: None,
        }
    }

    pub fn from_header(header: FunctionHeaderStruct, body: FunctionBody) -> Self {
        FunctionDef {
            name: header.name,
            params: header.params,
            return_type: String::from("i32"),
            body,
            _type: None,
        }
    }

    /// Establece el tipo de la función.
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}

impl Codegen for FunctionParams {
    /// Genera el código LLVM IR para un parámetro de función.
    ///
    /// Reserva espacio local para el argumento, almacena el valor recibido y lo registra en la tabla de símbolos.
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // Genera un nombre de argumento LLVM (por ejemplo, %x)
        let arg_name = format!("%{}", self.name);
        // Reserva espacio local para el argumento
        let alloca_reg = context.generate_temp();
        context.emit(&format!("  {} = alloca i32", alloca_reg));
        // Almacena el argumento en el espacio local
        context.emit(&format!("  store i32 {}, i32* {}", arg_name, alloca_reg));
        // Registra el parámetro en la tabla de símbolos
        context.register_variable(&self.name, alloca_reg.clone());
        alloca_reg
    }
}

impl Codegen for FunctionDef {
    /// Genera el código LLVM IR para la definición de la función.
    ///
    /// Emite la cabecera de la función, reserva espacio para los parámetros, genera el cuerpo y emite la instrucción de retorno.
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // Prepara la lista de parámetros para LLVM IR
        let params_ir: Vec<String> = self
            .params
            .iter()
            .map(|p| format!("i32 %{}", p.name))
            .collect();
        let params_str = params_ir.join(", ");
        // Cabecera de la función
        context.emit(&format!("define i32 @{}({}) {{", self.name, params_str));
        // Prologo: asigna espacio y almacena los argumentos
        for param in &self.params {
            param.codegen(context);
        }
        // Genera el cuerpo de la función
        let ret_val = self.body.codegen(context);
        // Retorno
        context.emit(&format!("  ret i32 {}", ret_val));
        // Cierre de la función
        context.emit("}");
        String::new() // No se usa el valor de retorno
    }
}
