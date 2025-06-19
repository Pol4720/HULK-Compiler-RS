use crate::{hulk_tokens::{BinaryOperatorToken, TokenPos, UnaryOperator}, typings::types_node::TypeNode};

#[derive(Debug, Clone, PartialEq)]
pub enum SemanticError {
    DivisionByZero(TokenPos),
    UndefinedIdentifier(String, TokenPos),
    InvalidConditionType(TypeNode, TokenPos),
    InvalidBinaryOperation(TypeNode, TypeNode, BinaryOperatorToken, TokenPos),
    InvalidUnaryOperation(TypeNode, UnaryOperator, TokenPos),
    RedefinitionOfFunction(String, TokenPos),
    UndeclaredFunction(String, TokenPos),
    UnknownError(String, TokenPos),
    InvalidArgumentsCount(usize, usize, String, TokenPos),
    InvalidTypeArgument(String, String, String, usize, String, TokenPos),
    InvalidFunctionReturn(TypeNode, TypeNode, String, TokenPos),
    RedefinitionOfVariable(String, TokenPos),
    UndefinedType(String, TokenPos),
    ParamNameAlreadyExist(String, String, String, TokenPos),
    RedefinitionOfType(String, TokenPos),
    CycleDetected(String, TokenPos),
    InvalidTypeArgumentCount(usize, usize, String, TokenPos),
    InvalidTypeFunctionAccess(String, String, TokenPos),
    InvalidTypePropertyAccess(String, String, TokenPos),
    InvalidTypeProperty(String, String, TokenPos),
    InvalidPrint(String, TokenPos),
    InvalidIterable(String, usize, TokenPos),
}

impl SemanticError {
    pub fn message(&self) -> String {
        match self {
            SemanticError::DivisionByZero(_) => "Division by zero is not allowed".to_string(),
            SemanticError::UndefinedIdentifier(id, _) => {
                format!("Undefined identifier: {id}")
            }
            SemanticError::InvalidConditionType(t, _) => {
                format!("Invalid condition type: {}", t.type_name)
            }
            SemanticError::InvalidBinaryOperation(l, r, op, _) => format!(
                "Invalid binary operation between types {} and {} with operator {}",
                l.type_name, r.type_name, op
            ),
            SemanticError::InvalidUnaryOperation(t, op, _) => format!(
                "Invalid unary operation on type {} with operator {}",
                t.type_name, op
            ),
            SemanticError::RedefinitionOfFunction(name, _) => {
                format!("Function '{name}' is already defined")
            }
            SemanticError::UndeclaredFunction(name, _) => {
                format!("Function '{name}' is not defined")
            }
            SemanticError::InvalidArgumentsCount(found, expected, fname, _) => {
                format!("Function '{fname}' expects {expected} arguments, found {found}")
            }
            SemanticError::InvalidTypeArgument(_, found, expected, pos, stmt_name, _) => {
                format!(
                    "{stmt_name}: Argument {} should be {expected}, found {found}",
                    pos + 1
                )
            }
            SemanticError::InvalidFunctionReturn(body, ret, fname, _) => format!(
                "Function '{fname}' should return {}, found {}",
                ret.type_name, body.type_name
            ),
            SemanticError::RedefinitionOfVariable(var, _) => {
                format!("Variable '{var}' is already defined")
            }
            SemanticError::UndefinedType(ty, _) => {
                format!("Type '{ty}' is not defined")
            }
            SemanticError::ParamNameAlreadyExist(param, stmt_name, kind, _) => {
                format!("Duplicate parameter '{param}' in {kind} '{stmt_name}'")
            }
            SemanticError::RedefinitionOfType(ty, _) => {
                format!("Type '{ty}' is already defined")
            }
            SemanticError::CycleDetected(node, _) => {
                format!("Type dependency cycle detected: {node}")
            }
            SemanticError::InvalidTypeArgumentCount(found, expected, ty, _) => {
                format!("Type '{ty}' expects {expected} arguments, found {found}")
            }
            SemanticError::InvalidTypeFunctionAccess(ty, fn_name, _) => {
                format!("Type '{ty}' has no method '{fn_name}'")
            }
            SemanticError::InvalidTypePropertyAccess(ty, prop, _) => {
                format!("Property '{prop}' of type '{ty}' is private")
            }
            SemanticError::InvalidTypeProperty(ty, prop, _) => {
                format!("Type '{ty}' has no property '{prop}'")
            }
            SemanticError::InvalidPrint(ty, _) => {
                format!("Cannot print values of type '{ty}'")
            }
            SemanticError::InvalidIterable(fn_name, cnt, _) => {
                format!("For loops require range() function, found '{fn_name}({cnt} arguments)'")
            }
            SemanticError::UnknownError(msg, _) => msg.clone(),
        }
    }

    fn token_pos(&self) -> &TokenPos {
        match self {
            SemanticError::DivisionByZero(sp)
            | SemanticError::UndefinedIdentifier(_, sp)
            | SemanticError::InvalidConditionType(_, sp)
            | SemanticError::InvalidBinaryOperation(_, _, _, sp)
            | SemanticError::InvalidUnaryOperation(_, _, sp)
            | SemanticError::RedefinitionOfFunction(_, sp)
            | SemanticError::UndeclaredFunction(_, sp)
            | SemanticError::UnknownError(_, sp)
            | SemanticError::InvalidArgumentsCount(_, _, _, sp)
            | SemanticError::InvalidTypeArgument(_, _, _, _, _, sp)
            | SemanticError::InvalidFunctionReturn(_, _, _, sp)
            | SemanticError::RedefinitionOfVariable(_, sp)
            | SemanticError::UndefinedType(_, sp)
            | SemanticError::ParamNameAlreadyExist(_, _, _, sp)
            | SemanticError::RedefinitionOfType(_, sp)
            | SemanticError::CycleDetected(_, sp)
            | SemanticError::InvalidTypeArgumentCount(_, _, _, sp)
            | SemanticError::InvalidTypeFunctionAccess(_, _, sp)
            | SemanticError::InvalidTypePropertyAccess(_, _, sp)
            | SemanticError::InvalidTypeProperty(_, _, sp)
            | SemanticError::InvalidPrint(_, sp)
            | SemanticError::InvalidIterable(_, _, sp) => sp,
        }
    }

    pub fn report(&self, input: &str) -> String {
        let token_pos = self.token_pos();
        let (line, col, line_str, _) = get_line_context(input, token_pos.start);
        let caret = build_caret_point(col);

        let message = self.message();
        let location = format!("(line {line}, column {col})");

        format!(
            "\x1b[31mError {location}: {message}\n  {}\n  {}\x1b[0m",
            line_str, caret
        )
    }
}

fn get_line_context(
    input: &str,
    offset: usize,
) -> (usize, usize, String, usize) {
    if input.is_empty() {
        return (1, 1, String::new(), 0);
    }
    let mut line_start = 0;
    let mut line_number = 1;
    for (idx, c) in input.char_indices() {
        if idx > offset {
            break;
        }
        if c == '\n' {
            line_number += 1;
            line_start = idx + 1;
        }
    }
    let rest = &input[line_start..];
    let line_end = rest
        .find('\n')
        .map(|p| line_start + p)
        .unwrap_or(input.len());
    let line_str = input[line_start..line_end].to_string();

    let byte_in_line = offset.saturating_sub(line_start);
    let chars_before = input[line_start..line_start + byte_in_line].chars().count();
    let column = chars_before + 1;

    (line_number, column, line_str, line_start)
}

fn build_caret_point(col: usize) -> String {
    " ".repeat(col.saturating_sub(1)) + "^"
}
