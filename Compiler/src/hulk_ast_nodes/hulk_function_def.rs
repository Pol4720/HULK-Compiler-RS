//! # FunctionDef y FunctionParams AST Nodes
//!
//! Este módulo define los nodos de definición de función (`FunctionDef`) y de parámetros de función (`FunctionParams`) del AST para el compilador Hulk.
//! Permite representar funciones, sus parámetros, el tipo de retorno, el cuerpo y la integración con el visitor pattern y la generación de código LLVM IR.

use std::fmt;

use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_ast_nodes::hulk_expression::{Expr, ExprKind};
use crate::hulk_ast_nodes::{Block};
use crate::hulk_tokens::TokenPos;
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
    pub token_pos: TokenPos,
}

impl FunctionParams {
    /// Crea un nuevo parámetro de función.
    ///
    /// # Arguments
    /// * `name` - Nombre del parámetro.
    /// * `param_type` - Tipo del parámetro.
    pub fn new(name: String, param_type: String, token_pos: TokenPos) -> Self {
        FunctionParams { name, param_type, token_pos  }
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
    pub token_pos: TokenPos,
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
    pub token_pos: TokenPos,
}

impl FunctionDef {
    /// Crea una nueva definición de función a partir de una expresión.
    ///
    /// # Arguments
    /// * `name` - Nombre de la función.
    /// * `params` - Vector de parámetros.
    /// * `return_type` - Tipo de retorno.
    /// * `expr` - Cuerpo de la función.
    pub fn new_expr(name: String, params: Vec<FunctionParams>, return_type: String, body: Box<Expr>, token_pos:TokenPos) -> Self {
        FunctionDef {
            name,
            params,
            return_type,
            body: FunctionBody::from(body),
            _type: None,
            token_pos,
        }
    }

    pub fn from_header(header: FunctionHeaderStruct, body: FunctionBody, token_pos: TokenPos) -> Self {
        FunctionDef {
            name: header.name,
            params: header.params,
            return_type: header.signature,
            body,
            _type: None,
            token_pos,
        }
    }

    /// Establece el tipo de la función.
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
     pub fn codegen_with_name_override(&self, context: &mut CodegenContext, new_name: &str) -> String {
        let backup_code = std::mem::take(&mut context.code); // 🔒 Backup del main
        let backup_symbols = std::mem::take(&mut context.symbol_table);

        let params_ir: Vec<String> = self
            .params
            .iter()
            .map(|p| format!("{} %{}", CodegenContext::to_llvm_type(p.param_type.clone()), p.name))
            .collect();
        let params_str = params_ir.join(", ");

        context.emit(&format!("define {} @{}({}) {{",
            CodegenContext::to_llvm_type(self.return_type.clone()), 
            new_name, 
            params_str));

        for param in &self.params {
            param.codegen(context);
        }

        let ret_val = self.body.codegen(context);
        context.emit(&format!("  ret {} {}", CodegenContext::to_llvm_type(self.return_type.clone()), ret_val));
        context.emit("}");

        let result = std::mem::take(&mut context.code); // Función generada
        context.code = backup_code;
        context.symbol_table = backup_symbols;
        result
    }
}


impl Codegen for FunctionParams {
    /// Genera el código LLVM IR para un parámetro de función.
    ///
    /// Reserva espacio local para el argumento, almacena el valor recibido y lo registra en la tabla de símbolos.
    fn codegen(&self, context: &mut CodegenContext) -> String {
        let llvm_type = CodegenContext::to_llvm_type(self.param_type.clone());
        let arg_name = format!("%{}", self.name);

        let alloca_reg = context.generate_temp();
        context.emit(&format!("  {} = alloca {}", alloca_reg, llvm_type));
        context.emit(&format!("  store {} {}, {}* {}", llvm_type, arg_name, llvm_type, alloca_reg));

        context.register_variable(&self.name, alloca_reg.clone());
        context.register_type(&self.name, llvm_type);

        alloca_reg
    }
}


impl Codegen for FunctionDef {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // Creamos un subcontexto aislado copiando los datos relevantes del contexto global
        let mut fn_context = context.clone_for_type_codegen();

        // Traduce tipo de retorno
        let llvm_return_type = CodegenContext::to_llvm_type(self.return_type.clone());

        // Construye lista de parámetros para LLVM
        let mut params_ir: Vec<String> = self.params.iter()
            .map(|p| {
                let llvm_ty = CodegenContext::to_llvm_type(p.param_type.clone());
                format!("{} %{}", llvm_ty, p.name)
            })
            .collect();

        // Si es método de tipo, agrega self al contexto y como primer argumento
        if let Some(type_name) = context.current_self.clone() {
            // let self_var = format!("%self.{}", context.get_scope());
            // fn_context.register_variable(&self_var, type_name.clone());
            // Si necesitas modificar la lista de argumentos LLVM, deberías hacerlo antes de generar la cabecera
            // Aquí solo se registra la variable en el contexto
            params_ir.insert(0, format!("ptr %self.{}", fn_context.get_scope()));
        }
        let params_str = params_ir.join(", ");

        // Emite la cabecera de la función en el contexto de función
        fn_context.emit(&format!("define {} @{}({}) {{", llvm_return_type, self.name, params_str));
        
        context.function_table.insert(self.name.clone(), llvm_return_type.clone());

        // 🧾 Registra nombre de la función en sí misma (permite recursividad)
        fn_context.function_table.insert(self.name.clone(), llvm_return_type.clone());

        //📦 Reserva espacio para parámetros y almacena
        for param in &self.params {
            param.codegen(&mut fn_context);
            println!("Generando código para parámetro: {} de tipo {}", param.name, param.param_type);
        }
        if let Some(type_name) = context.current_self.clone() {
            let llvm_type = "ptr".to_string();
            let arg_name = format!("%self.{}", fn_context.get_scope());

            let alloca_reg = fn_context.generate_temp();
            fn_context.emit(&format!("  {} = alloca {}", alloca_reg, llvm_type));
            fn_context.emit(&format!("  store {} {}, {} {}", llvm_type, arg_name, llvm_type, alloca_reg));

            fn_context.register_variable(&format!("self.{}", fn_context.get_scope()), alloca_reg.clone());
            fn_context.register_type(&format!("self.{}", fn_context.get_scope()), llvm_type);
        }

        

        // Genera el cuerpo
        let result_reg = self.body.codegen(&mut fn_context);

        // Emitir retorno
        fn_context.emit(&format!("  ret {} {}", llvm_return_type, result_reg));
        fn_context.emit("}");

        // Fusiona el código generado al global
        context.merge_into_global(fn_context);

        // No devuelve valor porque no aplica aquí
        String::new()
    }
}



