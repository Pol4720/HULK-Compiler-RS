// ===============================
// regex_parser.rs: Parser de Expresiones Regulares a AST
// ===============================
// Convierte una cadena regex en un AST modularizado y documentado.

use crate::regex_parser::node::ast_node_impl::{AstNode, AstNodeImpl, AstNodeKind};
use crate::regex_parser::node::bin_op::RegexBinOp;
use crate::regex_parser::node::group::RegexGroup;
use crate::regex_parser::node::regex_char::{EndNode, LiteralNode, RegexChar, StarNode};
use crate::regex_parser::node::regex_class::RegexClass;
use crate::regex_parser::node::regex_escape::RegexEscape;
use crate::regex_parser::node::un_op::RegexUnOp;

/// Parsea una expresión regular y retorna el AST correspondiente.
/// Soporta alternancia, concatenación, agrupación, clases, escapes y operadores unarios.
pub fn parse_regex(input: &str) -> Option<AstNodeImpl> {
    let input = input.trim();
    if input.is_empty() {
        return None;
    }
    // Inicio de línea
    if input == "^" {
        return Some(StarNode::new().to_ast());
    }
    // Fin de línea
    if input == "$" {
        return Some(EndNode::new().to_ast());
    }
    // Operadores unarios: *, +, ?
    if let Some((body, op)) = helpers::parse_unary_suffix(input) {
        let expr = parse_regex(body)?;
        let op = match op {
            '*' => RegexUnOp::Star,
            '+' => RegexUnOp::Plus,
            '?' => RegexUnOp::Optional,
            _ => return None,
        };
        return Some(AstNodeImpl {
            kind: AstNodeKind::UnOp {
                op,
                expr: Box::new(expr),
            },
        });
    }

    // Clase de caracteres simple: [abc], [a-z]
    if input.starts_with('[') && input.ends_with(']') {
        let inner = &input[1..input.len() - 1];

        let class = helpers::parse_class(inner)?;
        return Some(AstNodeImpl {
            kind: AstNodeKind::Class(class),
        });
    }
    // Alternancia (|) de nivel superior
    if let Some(idx) = helpers::find_top_level(input, '|') {
        let left = &input[..idx];
        let right = &input[idx + 1..];
        let left_ast = parse_regex(left)?;
        let right_ast = parse_regex(right)?;
        return Some(AstNodeImpl {
            kind: AstNodeKind::BinOp {
                op: RegexBinOp::Or,
                left: Box::new(left_ast),
                right: Box::new(right_ast),
            },
        });
    }
    // Concatenación implícita de nivel superior
    if let Some(idx) = helpers::find_concat_point(input) {
        let left = &input[..idx];
        let right = &input[idx..];
        let left_ast = parse_regex(left)?;
        let right_ast = parse_regex(right)?;
        return Some(AstNodeImpl {
            kind: AstNodeKind::BinOp {
                op: RegexBinOp::Concat,
                left: Box::new(left_ast),
                right: Box::new(right_ast),
            },
        });
    }
    // Agrupación con paréntesis
    if input.starts_with('(') && input.ends_with(')') && helpers::is_balanced_parens(input) {
        let inner = &input[1..input.len() - 1];
        let expr = parse_regex(inner)?;
        return Some(AstNodeImpl {
            kind: AstNodeKind::Group(RegexGroup::new(Box::new(expr))),
        });
    }
    // Escape simple: \n, \t, etc.
    if input.starts_with("\\") && input.len() == 2 {
        if let Some(esc) = RegexEscape::from_char(input.chars().nth(1).unwrap()) {
            return Some(AstNodeImpl {
                kind: AstNodeKind::RegexChar(RegexChar::Escape(esc)),
            });
        }
    }
    // Un solo carácter literal
    if input.len() == 1 {
        let c = input.chars().next().unwrap();
        return Some(LiteralNode::new(c).to_ast());
    }
    None
}

// ===============================
// helpers: Submódulo privado para utilidades de parsing
// ===============================
mod helpers {
    use super::*;

    /// Busca el índice de un operador binario de nivel superior (no anidado en paréntesis).
    pub fn find_top_level(input: &str, op: char) -> Option<usize> {
        let mut depth = 0;
        for (i, c) in input.chars().enumerate() {
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                _ if c == op && depth == 0 => return Some(i),
                _ => {}
            }
        }
        None
    }

    /// Busca el punto de concatenación implícita de nivel superior.
    pub fn find_concat_point(input: &str) -> Option<usize> {
        let mut depth = 0;
        let chars: Vec<char> = input.chars().collect();
        for i in 1..chars.len() {
            match chars[i] {
                '(' => depth += 1,
                ')' => depth -= 1,
                _ => {}
            }
            if depth == 0 {
                // No separar en medio de un grupo
                return Some(i);
            }
        }
        None
    }

    /// Verifica si los paréntesis están balanceados.
    pub fn is_balanced_parens(input: &str) -> bool {
        let mut depth = 0;
        for (i, c) in input.chars().enumerate() {
            match c {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 && i != input.len() - 1 {
                        return false;
                    }
                }
                _ => {}
            }
        }
        depth == 0
    }

    /// Parsea un operador unario al final de la expresión.
    pub fn parse_unary_suffix(input: &str) -> Option<(&str, char)> {
        if input.len() < 2 {
            return None;
        }
        let last = input.chars().last().unwrap();
        if last == '*' || last == '+' || last == '?' {
            let body = &input[..input.len() - 1];
            if !body.is_empty() {
                return Some((body, last));
            }
        }
        None
    }

    /// Parsea una clase de caracteres con múltiples rangos y literales, incluyendo negación ([^...]).
    pub fn parse_class(input: &str) -> Option<RegexClass> {
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;
        let mut negated = false;
        // Detecta negación al inicio
        if !chars.is_empty() && chars[0] == '^' {
            negated = true;
            i = 1;
        }
        let mut ranges = Vec::new();
        let mut singles = Vec::new();
        while i < chars.len() {
            if i + 2 < chars.len() && chars[i + 1] == '-' {
                ranges.push((chars[i], chars[i + 2]));
                i += 3;
            } else {
                singles.push(RegexChar::Literal(chars[i]));
                i += 1;
            }
        }
        let result = if !ranges.is_empty() && singles.is_empty() {
            RegexClass::Ranges(ranges)
        } else if !ranges.is_empty() && !singles.is_empty() {
            // Mezcla: crea un set con los literales y expande los rangos
            let mut set = singles;
            for (a, b) in &ranges {
                for ch in *a as u8..=*b as u8 {
                    set.push(RegexChar::Literal(ch as char));
                }
            }
            RegexClass::Set(set)
        } else {
            RegexClass::Set(singles)
        };
        if negated {
            Some(RegexClass::Negated(Box::new(result)))
        } else {
            Some(result)
        }
    }
}
