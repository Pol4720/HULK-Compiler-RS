//! # SemanticError Enum
//!
//! Este módulo define el enum `SemanticError` para el compilador Hulk.
//! Representa los posibles errores semánticos que pueden ocurrir durante el análisis semántico del AST.
//! Incluye errores como división por cero, identificadores indefinidos, operaciones inválidas, redefiniciones, errores de tipos y más.
//! Provee un método para obtener un mensaje de error legible para cada caso.

use crate::hulk_tokens::hulk_operators::{BinaryOperatorToken, UnaryOperator};
use crate::typings::types_node::TypeNode;

/// Enum que representa los errores semánticos posibles durante el análisis del AST.
/// 
/// - `DivisionByZero`: intento de dividir por cero.
/// - `UndefinedIdentifier`: uso de un identificador no declarado.
/// - `InvalidConditionType`: tipo inválido en una condición.
/// - `InvalidBinaryOperation`: operación binaria inválida entre tipos.
/// - `InvalidUnaryOperation`: operación unaria inválida.
/// - `RedefinitionOfFunction`: redefinición de una función.
/// - `UndeclaredFunction`: uso de una función no declarada.
/// - `UnknownError`: error semántico genérico.
/// - `InvalidArgumentsCount`: número incorrecto de argumentos en llamada a función.
/// - `InvalidTypeArgument`: tipo de argumento incorrecto.
/// - `InvalidFunctionReturn`: tipo de retorno incorrecto en función.
/// - `RedefinitionOfVariable`: redefinición de variable.
/// - `UndefinedType`: uso de un tipo no declarado.
/// - `ParamNameAlreadyExist`: nombre de parámetro repetido.
/// - `RedefinitionOfType`: redefinición de tipo.
/// - `CicleDetected`: ciclo de herencia o dependencia.
/// - `InvalidTypeArgumentCount`: número incorrecto de argumentos de tipo.
/// - `InvalidTypeFunctionAccess`: acceso a función inexistente en un tipo.
/// - `InvalidTypePropertyAccess`: acceso a propiedad privada.
/// - `InvalidTypeProperty`: acceso a propiedad inexistente.
#[derive(Debug, Clone, PartialEq)]
pub enum SemanticError {
    DivisionByZero,
    UndefinedIdentifier(String),
    InvalidConditionType(TypeNode),
    InvalidBinaryOperation(TypeNode, TypeNode, BinaryOperatorToken),
    InvalidUnaryOperation(TypeNode, UnaryOperator),
    RedefinitionOfFunction(String),
    UndeclaredFunction(String),
    UnknownError(String),
    InvalidArgumentsCount(usize,usize,String),
    InvalidTypeArgument(String,String,String,usize,String),
    InvalidFunctionReturn(TypeNode,TypeNode,String),
    RedefinitionOfVariable(String),
    UndefinedType(String),
    ParamNameAlreadyExist(String,String,String),
    RedefinitionOfType(String),
    CicleDetected(String),
    InvalidTypeArgumentCount(usize,usize,String),
    InvalidTypeFunctionAccess(String,String),
    InvalidTypePropertyAccess(String,String),
    InvalidTypeProperty(String, String),
}

impl SemanticError {
    /// Devuelve un mensaje de error legible para el usuario según el tipo de error semántico.
    pub fn message(&self) -> String {
        match self {
            SemanticError::DivisionByZero => "Error: Division by zero is not allowed".to_string(),
            SemanticError::UndefinedIdentifier(identifier) => {
                format!("Error: Undefined identifier: {}.", identifier)
            }
            SemanticError::InvalidConditionType(return_type) => {
                format!("Error: Invalid condition type: {:?}.", return_type.type_name)
            }
            SemanticError::InvalidBinaryOperation(left, right, op) => {
                format!(
                    "Error: Invalid binary operation between types {:?} and {:?} with operator {:?}.",
                    left.type_name, right.type_name, op
                )
            }
            SemanticError::InvalidUnaryOperation(return_type, op) => {
                format!(
                    "Error: Invalid unary operation on type {:?} with operator {:?}.",
                    return_type.type_name, op
                )
            }
            SemanticError::UnknownError(message) => {
                format!("Error: {}", message)
            }
            SemanticError::RedefinitionOfFunction(function_name) => {
                format!("Error: function with name {} already exist.", function_name)
            }
            SemanticError::UndeclaredFunction(function_name) => {
                format!("Error: function {} is not declared in this context.", function_name)
            }
            SemanticError::InvalidArgumentsCount(curr_arg_count,func_arg_count,func_name) => {
                format!("Error: function call to {}, expected {} arguments, found {}.",func_name,func_arg_count,curr_arg_count)
            }
            SemanticError::InvalidTypeArgument(stmt,curr_type,arg_type,arg_pos ,stmt_name ) => {
                format!("Error: {} {} receives {} on argument {} but {} was found.",stmt,stmt_name,arg_type,arg_pos + 1,curr_type)
            }
            SemanticError::InvalidFunctionReturn(body_type,func_return ,func_name ) => {
                format!("Error: function {} returns {} but function's body returns {}",func_name,func_return.type_name,body_type.type_name)
            }
            SemanticError::RedefinitionOfVariable(var_name) => {
                format!("Error: variable {} already defined in this context.", var_name)
            }
            SemanticError::UndefinedType(type_name) => {
                format!("Error: type {} is not defined.", type_name)
            }
            SemanticError::ParamNameAlreadyExist(param_name, stmt_name, stmt) => {
                format!("Error: parameter name {} already exists in the context of {} {}.", param_name, stmt, stmt_name )
            }
            SemanticError::RedefinitionOfType(type_name) => {
                format!("Error: type {} already defined in this context.", type_name)
            }
            SemanticError::CicleDetected(cycle_node) => {
                format!("Error: Cicle detected on type {}", cycle_node)
            }
            SemanticError::InvalidTypeArgumentCount(curr_arg_count, expected_arg_count, type_name) => {
                format!("Error: type {} expected {} arguments, found {}.", type_name, expected_arg_count, curr_arg_count)
            }
            SemanticError::InvalidTypeFunctionAccess(type_name, function_name) => {
                format!("Error: type {} does not have a function named {}.", type_name, function_name)
            }
            SemanticError::InvalidTypePropertyAccess(type_name, property_name) => {
                format!("Error: can not access property {} of type {} because properties are private.", property_name, type_name)
            }
            SemanticError::InvalidTypeProperty(type_name, property_name) => {
                format!("Error: type {} does not have a property named {}.", type_name, property_name)
            }
        }
    }
}