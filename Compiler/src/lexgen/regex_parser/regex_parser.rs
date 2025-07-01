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

    // Alternancia (|) de nivel superior - tiene menor precedencia
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

    // Concatenación implícita - procesar factor por factor
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

    // Procesamiento de un factor individual (con posibles operadores unarios)
    // Un factor puede ser: clase [abc], grupo (abc), escape \n, o literal a
    let (factor, rest) = if input.starts_with('[') {
        // Buscar el cierre correspondiente del corchete
        let mut depth = 0;
        let mut end_idx = None;
        for (i, c) in input.chars().enumerate() {
            if c == '[' {
                depth += 1;
            } else if c == ']' {
                depth -= 1;
                if depth == 0 {
                    end_idx = Some(i);
                    break;
                }
            }
        }
        if let Some(idx) = end_idx {
            let inner = &input[1..idx];
            let class = helpers::parse_class(inner)?;
            let class_node = AstNodeImpl {
                kind: AstNodeKind::Class(class),
            };
            (class_node, &input[idx + 1..])
        } else {
            // Corchetes desbalanceados
            return None;
        }
    } else if input.starts_with('(') {
        // Buscar el cierre correspondiente del paréntesis
        let mut depth = 0;
        let mut end_idx = None;
        for (i, c) in input.chars().enumerate() {
            if c == '(' {
                depth += 1;
            } else if c == ')' {
                depth -= 1;
                if depth == 0 {
                    end_idx = Some(i);
                    break;
                }
            }
        }
        if let Some(idx) = end_idx {
            let inner = &input[1..idx];
            let expr = parse_regex(inner)?;
            let group_node = AstNodeImpl {
                kind: AstNodeKind::Group(RegexGroup::new(Box::new(expr))),
            };
            (group_node, &input[idx + 1..])
        } else {
            // Paréntesis desbalanceados
            return None;
        }
    } else if input.starts_with("\\") && input.len() >= 2 {
        if let Some(esc) = RegexEscape::from_char(input.chars().nth(1).unwrap()) {
            (
                AstNodeImpl {
                    kind: AstNodeKind::RegexChar(RegexChar::Escape(esc)),
                },
                &input[2..],
            )
        } else {
            // Si no es un escape reconocido, tratar el carácter después del backslash como literal
            let c = input.chars().nth(1).unwrap();
            (LiteralNode::new(c).to_ast(), &input[2..])
        }
    } else if input.len() >= 1 {
        let c = input.chars().next().unwrap();
        (LiteralNode::new(c).to_ast(), &input[1..])
    } else {
        return None;
    };

    // Verificar si hay operador unario después del factor
    let rest = rest.trim_start();
    if let Some(first_char) = rest.chars().next() {
        if first_char == '*' || first_char == '+' || first_char == '?' {
            let op = match first_char {
                '*' => RegexUnOp::Star,
                '+' => RegexUnOp::Plus,
                '?' => RegexUnOp::Optional,
                _ => unreachable!(),
            };
            let unary_node = AstNodeImpl {
                kind: AstNodeKind::UnOp {
                    op,
                    expr: Box::new(factor),
                },
            };
            let after_op = &rest[1..].trim_start();
            if !after_op.is_empty() {
                // Hay más contenido después del operador unario, concatenar
                let right = parse_regex(after_op)?;
                return Some(AstNodeImpl {
                    kind: AstNodeKind::BinOp {
                        op: RegexBinOp::Concat,
                        left: Box::new(unary_node),
                        right: Box::new(right),
                    },
                });
            } else {
                return Some(unary_node);
            }
        }
    }

    // Si hay contenido restante sin operador unario, concatenar
    if !rest.is_empty() {
        let right = parse_regex(rest)?;
        return Some(AstNodeImpl {
            kind: AstNodeKind::BinOp {
                op: RegexBinOp::Concat,
                left: Box::new(factor),
                right: Box::new(right),
            },
        });
    }

    Some(factor)
}

// ===============================
// helpers: Submódulo privado para utilidades de parsing
// ===============================
mod helpers {
    use super::*;

    /// Busca el índice de un operador binario de nivel superior (no anidado en paréntesis).
    pub fn find_top_level(input: &str, op: char) -> Option<usize> {
        let mut depth = 0;
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            match chars[i] {
                '\\' => {
                    // Saltar el carácter escapado
                    i += 2; // Saltar '\' y el carácter siguiente
                    continue;
                }
                '(' => {
                    depth += 1;
                    i += 1;
                }
                ')' => {
                    depth -= 1;
                    i += 1;
                }
                c if c == op && depth == 0 => return Some(i),
                _ => {
                    i += 1;
                }
            }
        }
        None
    }

    /// Busca el punto de concatenación implícita de nivel superior.
    pub fn find_concat_point(input: &str) -> Option<usize> {
        let mut paren_depth = 0;
        let mut bracket_depth = 0;
        let chars: Vec<char> = input.chars().collect();

        let mut i = 1;
        while i < chars.len() {
            // Saltar escapes
            if i > 0 && chars[i - 1] == '\\' {
                i += 1;
                continue;
            }

            match chars[i - 1] {
                '(' => paren_depth += 1,
                ')' => paren_depth -= 1,
                '[' => bracket_depth += 1,
                ']' => bracket_depth -= 1,
                _ => {}
            }

            // Solo permitir concatenación fuera de paréntesis y corchetes
            if paren_depth == 0 && bracket_depth == 0 {
                // Saltar operadores unarios - no son puntos de concatenación
                if chars[i] == '*' || chars[i] == '+' || chars[i] == '?' {
                    i += 1;
                    continue;
                }

                // No concatenar en bordes de paréntesis o corchetes
                if chars[i - 1] != '('
                    && chars[i] != ')'
                    && chars[i - 1] != '['
                    && chars[i] != ']'
                    && chars[i - 1] != '*'
                    && chars[i - 1] != '+'
                    && chars[i - 1] != '?'
                    && chars[i - 1] != '\\'
                // No concatenar después de escape
                {
                    return Some(i);
                }
            }
            i += 1;
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
            // Manejar escapes dentro de clases de caracteres
            if chars[i] == '\\' && i + 1 < chars.len() {
                let escaped_char = match chars[i + 1] {
                    'r' => '\r',
                    'n' => '\n',
                    't' => '\t',
                    '\\' => '\\',
                    '[' => '[',
                    ']' => ']',
                    '^' => '^',
                    '-' => '-',
                    c => c, // Para otros caracteres, usar el literal
                };
                singles.push(RegexChar::Literal(escaped_char));
                i += 2;
            } else if i + 2 < chars.len() && chars[i + 1] == '-' && chars[i + 2] != ']' {
                // Rango como a-z (pero no al final donde - es literal)
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
