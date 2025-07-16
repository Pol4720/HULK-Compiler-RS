use crate::ast::*;
use crate::cst::DerivationNode;

pub fn convert_to_ast(root: &DerivationNode) -> Result<Program, String> {
    if root.symbol != "Program" {
        return Err("CST root must be a Program node".to_string());
    }

    let stmts = if !root.children.is_empty() {
        convert_stmt_list(&root.children[0])?
    } else {
        vec![]
    };

    Ok(Program { stmts })
}

fn convert_stmt_list(node: &DerivationNode) -> Result<Vec<Stmt>, String> {
    if node.symbol != "StmtList" {
        return Err("Expected StmtList node".to_string());
    }

    if node.children.is_empty() {
        return Ok(vec![]);
    }

    let mut stmts = vec![];
    
    // TerminatedStmt
    let terminated_stmt = &node.children[0];
    if terminated_stmt.symbol != "TerminatedStmt" {
        return Err("Expected TerminatedStmt".to_string());
    }
    
    if !terminated_stmt.children.is_empty() {
        let stmt_node = &terminated_stmt.children[0];
        stmts.push(convert_stmt(stmt_node)?);
    }
    
    // Recursive StmtList
    if node.children.len() > 1 {
        let rest = convert_stmt_list(&node.children[1])?;
        stmts.extend(rest);
    }

    Ok(stmts)
}

fn convert_stmt(node: &DerivationNode) -> Result<Stmt, String> {
    if node.symbol != "Stmt" {
        return Err("Expected Stmt node".to_string());
    }

    if node.children.is_empty() {
        return Err("Stmt node has no children".to_string());
    }

    let child = &node.children[0];
    let span = get_span(node);
    
    match child.symbol.as_str() {
        "Expr" => {
            let expr = convert_expr(child)?;
            Ok(Stmt {
                kind: StmtKind::ExprStmt(expr),
                span,
            })
        }
        "FunctionDef" => {
            let func = convert_function_def(child)?;
            Ok(Stmt {
                kind: StmtKind::FunctionDecl {
                    name: func.name,
                    params: func.params,
                    body: Box::new(func.body),
                    return_type: func.return_type,
                },
                span,
            })
        }
        "TypeDef" => {
            let type_decl = convert_type_def(child)?;
            Ok(Stmt {
                kind: StmtKind::TypeDecl {
                    name: type_decl.name,
                    type_params: type_decl.type_params,
                    attributes: type_decl.attributes,
                    methods: type_decl.methods,
                    base_type: type_decl.base_type,
                    base_args: type_decl.base_args,
                },
                span,
            })
        }
        "WhileStmt" => {
            let expr = convert_while_expr(child)?;
            Ok(Stmt {
                kind: StmtKind::ExprStmt(expr),
                span,
            })
        }
        "ForStmt" => {
            let expr = convert_for_expr(child)?;
            Ok(Stmt {
                kind: StmtKind::ExprStmt(expr),
                span,
            })
        }
        "BlockStmt" => {
            let expr = convert_block_stmt(child)?;
            Ok(Stmt {
                kind: StmtKind::ExprStmt(expr),
                span,
            })
        }
        _ => Err(format!("Unsupported statement type: {}", child.symbol)),
    }
}

fn convert_expr(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "Expr" {
        return Err("Expected Expr node".to_string());
    }

    if node.children.is_empty() {
        return Err("Expr node has no children".to_string());
    }

    let child = &node.children[0];
    let _span = get_span(node);
    
    match child.symbol.as_str() {
        "OrExpr" => convert_or_expr(child),
        "IfExpr" => convert_if_expr(child),
        "LetExpr" => convert_let_expr(child),
        _ => Err(format!("Unsupported expression type: {}", child.symbol)),
    }
}

// Implementation for OrExpr
fn convert_or_expr(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "OrExpr" {
        return Err("Expected OrExpr node".to_string());
    }

    if node.children.len() < 2 {
        return Err("OrExpr requires two children".to_string());
    }

    let left = convert_and_expr(&node.children[0])?;
    convert_or_expr_prime(left, &node.children[1])
}

fn convert_or_expr_prime(left: Expr, node: &DerivationNode) -> Result<Expr, String> {
    if node.children.is_empty() {
        return Ok(left);
    }

    if node.children.len() < 3 {
        return Err("Invalid OrExpr' structure".to_string());
    }

    let right = convert_and_expr(&node.children[1])?;
    let new_left = Expr {
        kind: ExprKind::Binary {
            op: BinaryOp::Or,
            left: Box::new(left),
            right: Box::new(right),
        },
        span: None,
    };

    convert_or_expr_prime(new_left, &node.children[2])
}

// Fix convert_let_expr to match AST structure
fn convert_let_expr(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "LetExpr" {
        return Err("Expected LetExpr node".to_string());
    }
    // Assuming structure: LET IDENT ASSIGN Expr (IN Expr)?
    if node.children.len() < 4 {
        return Err("Invalid LetExpr structure".to_string());
    }
    
    let var_name = node.children[1]
        .token
        .as_ref()
        .ok_or("Missing variable name in LetExpr")?
        .lexeme
        .clone();
    
    let value = convert_expr(&node.children[3])?;
    
    // Check if there's an "IN" part (let x = expr in body)
    let body = if node.children.len() > 5 && node.children[4].symbol == "IN" {
        Some(Box::new(convert_expr(&node.children[5])?))
    } else {
        None
    };
    
    Ok(Expr {
        kind: ExprKind::Let {
            var_name,
            value: Box::new(value),
            body,
            declared_type: None, // Add type annotation handling if needed
        },
        span: get_span(node),
    })
}

// Fix convert_if_expr to use convert_expr for both branches instead of convert_block_stmt
fn convert_if_expr(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "IfExpr" {
        return Err("Expected IfExpr node".to_string());
    }
    // Assuming structure: IF Expr THEN Expr (ELSE Expr)?
    if node.children.len() < 3 {
        return Err("Invalid IfExpr structure".to_string());
    }
    
    let condition = convert_expr(&node.children[1])?;
    
    // Find the then branch node
    let then_index = node.children.iter()
        .position(|child| child.symbol == "THEN")
        .map(|pos| pos + 1)
        .unwrap_or(2);
    
    if then_index >= node.children.len() {
        return Err("Missing then branch in if expression".to_string());
    }
    
    let then_branch = convert_expr(&node.children[then_index])?;
    
    // Find the else branch if it exists
    let else_branch = if let Some(else_pos) = node.children.iter()
        .position(|child| child.symbol == "ELSE")
    {
        if else_pos + 1 < node.children.len() {
            Some(Box::new(convert_expr(&node.children[else_pos + 1])?))
        } else {
            None
        }
    } else {
        None
    };
    
    Ok(Expr {
        kind: ExprKind::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch,
        },
        span: get_span(node),
    })
}

// Create a proper block statement to expression converter
fn convert_block_stmt(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "BlockStmt" {
        return Err("Expected BlockStmt node".to_string());
    }
    
    // Assuming structure: LBRACE StmtList? RBRACE
    if node.children.len() < 2 {
        return Err("Invalid block statement structure".to_string());
    }
    
    // Find the statement list if it exists
    let stmt_list_index = node.children.iter()
        .position(|child| child.symbol == "StmtList")
        .unwrap_or(1);
    
    let stmts = if stmt_list_index < node.children.len() {
        convert_stmt_list(&node.children[stmt_list_index])?
    } else {
        vec![]
    };
    
    Ok(Expr {
        kind: ExprKind::Block(stmts),
        span: get_span(node),
    })
}

// Fix convert_for_expr to match AST structure
fn convert_for_expr(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "ForStmt" {
        return Err("Expected ForStmt node".to_string());
    }
    // Assuming structure: FOR IDENT IN Expr BlockStmt
    if node.children.len() < 5 {
        return Err("Invalid ForStmt structure".to_string());
    }
    
    let var_name = node.children[1]
        .token
        .as_ref()
        .ok_or("Missing loop variable name")?
        .lexeme
        .clone();
    
    // Find the IN token
    let in_index = node.children.iter()
        .position(|child| child.symbol == "IN")
        .ok_or("Missing IN keyword in for loop")?;
    
    if in_index + 1 >= node.children.len() {
        return Err("Missing iterable expression in for loop".to_string());
    }
    
    let iterable = convert_expr(&node.children[in_index + 1])?;
    
    // Find the body (which should be after the iterable)
    let body_index = in_index + 2;
    if body_index >= node.children.len() {
        return Err("Missing body in for loop".to_string());
    }
    
    let body = convert_expr(&node.children[body_index])?;
    
    Ok(Expr {
        kind: ExprKind::For {
            var_name,
            iterable: Box::new(iterable),
            body: Box::new(body),
        },
        span: get_span(node),
    })
}

// Fix convert_while_expr to use convert_expr for the body
fn convert_while_expr(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "WhileStmt" {
        return Err("Expected WhileStmt node".to_string());
    }
    // Assuming structure: WHILE Expr BlockStmt
    if node.children.len() < 3 {
        return Err("Invalid WhileStmt structure".to_string());
    }
    
    let condition = convert_expr(&node.children[1])?;
    let body = convert_expr(&node.children[2])?;
    
    Ok(Expr {
        kind: ExprKind::While {
            condition: Box::new(condition),
            body: Box::new(body),
        },
        span: get_span(node),
    })
}

// Replace the dummy convert_cmp_expr with a proper implementation
fn convert_cmp_expr(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "CmpExpr" {
        return Err("Expected CmpExpr node".to_string());
    }

    if node.children.len() < 2 {
        return Err("CmpExpr requires two children".to_string());
    }

    let left = convert_concat_expr(&node.children[0])?;
    convert_cmp_expr_prime(left, &node.children[1])
}

fn convert_cmp_expr_prime(left: Expr, node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "CmpExpr'" || node.children.is_empty() {
        return Ok(left);
    }

    if node.children.len() < 3 {
        return Err("Invalid CmpExpr' structure".to_string());
    }

    let op = match node.children[0].symbol.as_str() {
        "EQ" => BinaryOp::Eq,
        "NEQ" => BinaryOp::Neq,
        "LT" => BinaryOp::Lt,
        "GT" => BinaryOp::Gt,
        "LTE" => BinaryOp::Le,
        "GTE" => BinaryOp::Ge,
        _ => return Err(format!("Unknown comparison operator: {}", node.children[0].symbol)),
    };

    let right = convert_concat_expr(&node.children[1])?;
    let new_left = Expr {
        kind: ExprKind::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        },
        span: None,
    };

    convert_cmp_expr_prime(new_left, &node.children[2])
}

fn convert_concat_expr(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "ConcatExpr" {
        return Err("Expected ConcatExpr node".to_string());
    }

    if node.children.len() < 2 {
        return Err("ConcatExpr requires two children".to_string());
    }

    let left = convert_add_expr(&node.children[0])?;
    convert_concat_expr_prime(left, &node.children[1])
}

fn convert_concat_expr_prime(left: Expr, node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "ConcatExpr'" || node.children.is_empty() {
        return Ok(left);
    }

    if node.children.len() < 3 {
        return Err("Invalid ConcatExpr' structure".to_string());
    }

    let op = match node.children[0].symbol.as_str() {
        "CONCAT" => BinaryOp::Concat,
        "CONCAT_WS" => BinaryOp::ConcatWs,
        _ => return Err(format!("Unknown concatenation operator: {}", node.children[0].symbol)),
    };

    let right = convert_add_expr(&node.children[1])?;
    let new_left = Expr {
        kind: ExprKind::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        },
        span: None,
    };

    convert_concat_expr_prime(new_left, &node.children[2])
}

fn convert_add_expr(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "AddExpr" {
        return Err("Expected AddExpr node".to_string());
    }

    if node.children.len() < 2 {
        return Err("AddExpr requires two children".to_string());
    }

    let left = convert_term(&node.children[0])?;
    convert_add_expr_prime(left, &node.children[1])
}

fn convert_add_expr_prime(left: Expr, node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "AddExpr'" || node.children.is_empty() {
        return Ok(left);
    }

    if node.children.len() < 3 {
        return Err("Invalid AddExpr' structure".to_string());
    }

    let op = match node.children[0].symbol.as_str() {
        "PLUS" => BinaryOp::Add,
        "MINUS" => BinaryOp::Sub,
        _ => return Err(format!("Unknown additive operator: {}", node.children[0].symbol)),
    };

    let right = convert_term(&node.children[1])?;
    let new_left = Expr {
        kind: ExprKind::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        },
        span: None,
    };

    convert_add_expr_prime(new_left, &node.children[2])
}

fn convert_term(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "Term" {
        return Err("Expected Term node".to_string());
    }

    if node.children.len() < 2 {
        return Err("Term requires two children".to_string());
    }

    let left = convert_factor(&node.children[0])?;
    convert_term_prime(left, &node.children[1])
}

fn convert_term_prime(left: Expr, node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "Term'" || node.children.is_empty() {
        return Ok(left);
    }

    if node.children.len() < 3 {
        return Err("Invalid Term' structure".to_string());
    }

    let op = match node.children[0].symbol.as_str() {
        "TIMES" => BinaryOp::Mul,
        "DIVIDE" => BinaryOp::Div,
        "MOD" => BinaryOp::Mod,
        _ => return Err(format!("Unknown multiplicative operator: {}", node.children[0].symbol)),
    };

    let right = convert_factor(&node.children[1])?;
    let new_left = Expr {
        kind: ExprKind::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        },
        span: None,
    };

    convert_term_prime(new_left, &node.children[2])
}

fn convert_factor(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "Factor" {
        return Err("Expected Factor node".to_string());
    }

    if node.children.is_empty() {
        return Err("Factor node has no children".to_string());
    }

    let power = convert_power(&node.children[0])?;
    
    // Handle Factor' if present
    if node.children.len() > 1 {
        // Implement handling for Factor' if needed
        // For most grammars, this might be empty
        return Ok(power);
    }
    
    Ok(power)
}

fn convert_power(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "Power" {
        return Err("Expected Power node".to_string());
    }

    if node.children.is_empty() {
        return Err("Power node has no children".to_string());
    }

    // Handle base expression
    let base = convert_unary(&node.children[0])?;
    
    // Check if there's an exponent (^)
    if node.children.len() > 1 && node.children[1].symbol == "POW" {
        if node.children.len() < 3 {
            return Err("Invalid power expression".to_string());
        }
        
        let exponent = convert_power(&node.children[2])?;
        
        return Ok(Expr {
            kind: ExprKind::Binary {
                op: BinaryOp::Pow,
                left: Box::new(base),
                right: Box::new(exponent),
            },
            span: get_span(node),
        });
    }
    
    Ok(base)
}

fn convert_unary(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "Unary" {
        return Err("Expected Unary node".to_string());
    }

    if node.children.is_empty() {
        return Err("Unary node has no children".to_string());
    }

    // Check if it's a unary operator followed by expression
    if node.children[0].symbol == "MINUS" {
        if node.children.len() < 2 {
            return Err("Invalid unary minus expression".to_string());
        }
        
        let expr = convert_unary(&node.children[1])?;
        
        return Ok(Expr {
            kind: ExprKind::Unary {
                op: UnaryOp::Neg,
                expr: Box::new(expr),
            },
            span: get_span(node),
        });
    }
    
    // Otherwise it's just a primary with optional AsExpr
    let primary = convert_primary(&node.children[0])?;
    
    // Handle AsExpr if present (type casting)
    if node.children.len() > 1 && !node.children[1].children.is_empty() {
        // Process type casting logic here if needed
        // For now, we'll just return the primary
        return Ok(primary);
    }
    
    Ok(primary)
}

// Implementaciones para diferentes tipos de expresiones

fn convert_and_expr(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "AndExpr" {
        return Err("Expected AndExpr node".to_string());
    }

    if node.children.len() < 2 {
        return Err("AndExpr requires two children".to_string());
    }

    let left = convert_cmp_expr(&node.children[0])?;
    convert_and_expr_prime(left, &node.children[1])
}

fn convert_and_expr_prime(left: Expr, node: &DerivationNode) -> Result<Expr, String> {
    if node.children.is_empty() {
        return Ok(left);
    }

    if node.children.len() < 3 {
        return Err("Invalid AndExpr' structure".to_string());
    }

    let right = convert_cmp_expr(&node.children[1])?;
    let new_left = Expr {
        kind: ExprKind::Binary {
            op: BinaryOp::And,
            left: Box::new(left),
            right: Box::new(right),
        },
        span: None,
    };

    convert_and_expr_prime(new_left, &node.children[2])
}

// fn convert_cmp_expr(node: &DerivationNode) -> Result<Expr, String> {
//     // Dummy implementation, replace with actual logic as needed
//     // For now, just call convert_primary
//     convert_primary(node)
// }

// fn convert_for_expr(node: &DerivationNode) -> Result<Expr, String> {
//     if node.symbol != "ForStmt" {
//         return Err("Expected ForStmt node".to_string());
//     }
//     // Assuming structure: FOR IDENT IN Expr BlockStmt
//     if node.children.len() < 5 {
//         return Err("Invalid ForStmt structure".to_string());
//     }
    
//     let var_name = node.children[1]
//         .token
//         .as_ref()
//         .ok_or("Missing loop variable name")?
//         .lexeme
//         .clone();
    
//     // Find the IN token
//     let in_index = node.children.iter()
//         .position(|child| child.symbol == "IN")
//         .ok_or("Missing IN keyword in for loop")?;
    
//     if in_index + 1 >= node.children.len() {
//         return Err("Missing iterable expression in for loop".to_string());
//     }
    
//     let iterable = convert_expr(&node.children[in_index + 1])?;
    
//     // Find the body (which should be after the iterable)
//     let body_index = in_index + 2;
//     if body_index >= node.children.len() {
//         return Err("Missing body in for loop".to_string());
//     }
    
//     let body = convert_expr(&node.children[body_index])?;
    
//     Ok(Expr {
//         kind: ExprKind::For {
//             var_name,
//             iterable: Box::new(iterable),
//             body: Box::new(body),
//         },
//         span: get_span(node),
//     })
// }

// fn convert_while_expr(node: &DerivationNode) -> Result<Expr, String> {
//     if node.symbol != "WhileStmt" {
//         return Err("Expected WhileStmt node".to_string());
//     }
//     // Assuming structure: WHILE Expr BlockStmt
//     if node.children.len() < 3 {
//         return Err("Invalid WhileStmt structure".to_string());
//     }
    
//     let condition = convert_expr(&node.children[1])?;
//     let body = convert_expr(&node.children[2])?;
    
//     Ok(Expr {
//         kind: ExprKind::While {
//             condition: Box::new(condition),
//             body: Box::new(body),
//         },
//         span: get_span(node),
//     })
// }

// Implementaciones para FunctionDef, TypeDef, etc.
fn convert_function_def(node: &DerivationNode) -> Result<FunctionDecl, String> {
    if node.symbol != "FunctionDef" {
        return Err("Expected FunctionDef node".to_string());
    }

    if node.children.len() < 7 {
        return Err("Invalid FunctionDef structure".to_string());
    }

    let name = node.children[1]
        .token
        .as_ref()
        .ok_or("Missing function name")?
        .lexeme
        .clone();
    
    let params = convert_arg_id_list_with_types(&node.children[3])?;
    let return_type = convert_type_annotation(&node.children[5])?;
    let body = convert_function_body(&node.children[6])?;
    
    Ok(FunctionDecl {
        name,
        params,
        body,
        return_type,
    })
}

// Implementación para TypeDef
fn convert_type_def(node: &DerivationNode) -> Result<TypeDecl, String> {
    if node.symbol != "TypeDef" {
        return Err("Expected TypeDef node".to_string());
    }

    // Suponiendo una estructura básica: TYPE IDENT TypeParams? Attributes? Methods? BASE? BASE_ARGS?
    if node.children.len() < 2 {
        return Err("Invalid TypeDef structure".to_string());
    }

    let name = node.children[1]
        .token
        .as_ref()
        .ok_or("Missing type name")?
        .lexeme
        .clone();

    // Opcional: type_params, attributes, methods, base_type, base_args
    let type_params = vec![];
    let attributes = vec![];
    let methods = vec![];
    let base_type = String::new();
    let base_args = vec![];

    // Aquí podrías agregar lógica para extraer los campos opcionales si tu CST los provee

    Ok(TypeDecl {
        name,
        type_params,
        attributes,
        methods,
        base_type,
        base_args,
    })
}

// Estructuras auxiliares para la conversión
struct FunctionDecl {
    name: String,
    params: Vec<(String, Option<Type>)>,
    body: Stmt,
    return_type: Option<Type>,
}

struct TypeDecl {
    name: String,
    type_params: Vec<String>,
    attributes: Vec<AttributeDecl>,
    methods: Vec<MethodDecl>,
    base_type: String,
    base_args: Vec<Expr>,
}

// Funciones helper
fn get_span(node: &DerivationNode) -> Option<Span> {
    node.token.as_ref().map(|token| Span {
        line: token.line,
        column: token.column,
    })
}

// Replace the convert_primary function with a proper implementation
fn convert_primary(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "Primary" {
        return Err(format!("Expected Primary node, got {}", node.symbol));
    }

    if node.children.is_empty() {
        return Err("Primary node has no children".to_string());
    }

    let child = &node.children[0];
    let span = get_span(child);
    
    match child.symbol.as_str() {
        "NUMBER" => {
            let value = child.token.as_ref().unwrap().lexeme.parse::<f64>().map_err(|e| e.to_string())?;
            Ok(Expr {
                kind: ExprKind::Number(value),
                span,
            })
        }
        "STRING" => {
            let value = child.token.as_ref().unwrap().lexeme.clone();
            Ok(Expr {
                kind: ExprKind::String(value),
                span,
            })
        }
        "TRUE" => Ok(Expr {
            kind: ExprKind::Boolean(true),
            span,
        }),
        "FALSE" => Ok(Expr {
            kind: ExprKind::Boolean(false),
            span,
        }),
        "IDENT" => {
            let name = child.token.as_ref().unwrap().lexeme.clone();
            let mut expr = Expr {
                kind: ExprKind::Variable(name),
                span,
            };
            
            if node.children.len() > 1 {
                expr = convert_primary_tail(expr, &node.children[1])?;
            }
            
            Ok(expr)
        }
        "SELF" => {
            let mut expr = Expr {
                kind: ExprKind::SelfExpr,
                span,
            };
            
            if node.children.len() > 1 {
                expr = convert_primary_tail(expr, &node.children[1])?;
            }
            
            Ok(expr)
        }
        "BASE" => {
            if node.children.len() < 4 {
                return Err("Invalid BASE expression".to_string());
            }
            
            let args = convert_arg_list(&node.children[2])?;
            Ok(Expr {
                kind: ExprKind::BaseCall { args },
                span,
            })
        }
        "NEW" => {
            if node.children.len() < 5 {
                return Err("Invalid NEW expression".to_string());
            }
            
            let type_name = node.children[1]
                .token
                .as_ref()
                .ok_or("Missing type name in NEW expression")?
                .lexeme
                .clone();
            
            let args = convert_arg_list(&node.children[3])?;
            
            Ok(Expr {
                kind: ExprKind::New {
                    type_name,
                    args,
                },
                span,
            })
        }
        "LPAREN" => {
            if node.children.len() < 3 {
                return Err("Invalid parenthesized expression".to_string());
            }
            convert_expr(&node.children[1])
        }
        _ => Err(format!("Unsupported primary expression: {}", child.symbol)),
    }
}

// Also need to update convert_primary_tail
fn convert_primary_tail(base: Expr, node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "PrimaryTail" {
        return Err(format!("Expected PrimaryTail node, got {}", node.symbol));
    }

    if node.children.is_empty() {
        return Ok(base);
    }

    let first_child = &node.children[0];
    match first_child.symbol.as_str() {
        "LPAREN" => {
            if node.children.len() < 4 {
                return Err("Invalid function call".to_string());
            }
            
            let args = convert_arg_list(&node.children[1])?;
            let mut expr = match base.kind {
                ExprKind::Variable(name) => Expr {
                    kind: ExprKind::Call {
                        function: name,
                        args,
                    },
                    span: base.span,
                },
                ExprKind::GetAttr { object, attr } => Expr {
                    kind: ExprKind::MethodCall {
                        object,
                        method: attr,
                        args,
                    },
                    span: base.span,
                },
                _ => return Err("Invalid call expression".to_string()),
            };
            
            if node.children.len() > 3 {
                expr = convert_primary_tail(expr, &node.children[3])?;
            }
            
            Ok(expr)
        }
        "DOT" => {
            if node.children.len() < 3 {
                return Err("Invalid attribute access".to_string());
            }
            
            let attr = node.children[1]
                .token
                .as_ref()
                .ok_or("Missing attribute name")?
                .lexeme
                .clone();
            
            let span = base.span.clone();
            let expr = Expr {
                kind: ExprKind::GetAttr {
                    object: Box::new(base),
                    attr,
                },
                span,
            };
            
            if node.children.len() > 2 {
                convert_primary_tail(expr, &node.children[2])
            } else {
                Ok(expr)
            }
        }
        "ASSIGN" | "ASSIGN_DESTRUCT" => {
            if node.children.len() < 2 {
                return Err("Invalid assignment".to_string());
            }
            
            let value = convert_expr(&node.children[1])?;
            let expr = match base.kind {
                ExprKind::Variable(name) => Expr {
                    kind: ExprKind::Assign {
                        var_name: name,
                        value: Box::new(value),
                    },
                    span: base.span,
                },
                ExprKind::GetAttr { object, attr } => Expr {
                    kind: ExprKind::SetAttr {
                        object,
                        attr,
                        value: Box::new(value),
                    },
                    span: base.span,
                },
                _ => return Err("Invalid assignment target".to_string()),
            };
            
            Ok(expr)
        }
        _ => Ok(base),
    }
}

// Implementaciones para FunctionDef, TypeDef, etc.
fn convert_function_def(node: &DerivationNode) -> Result<FunctionDecl, String> {
    if node.symbol != "FunctionDef" {
        return Err("Expected FunctionDef node".to_string());
    }

    if node.children.len() < 7 {
        return Err("Invalid FunctionDef structure".to_string());
    }

    let name = node.children[1]
        .token
        .as_ref()
        .ok_or("Missing function name")?
        .lexeme
        .clone();
    
    let params = convert_arg_id_list_with_types(&node.children[3])?;
    let return_type = convert_type_annotation(&node.children[5])?;
    let body = convert_function_body(&node.children[6])?;
    
    Ok(FunctionDecl {
        name,
        params,
        body,
        return_type,
    })
}

// Implementación para TypeDef
fn convert_type_def(node: &DerivationNode) -> Result<TypeDecl, String> {
    if node.symbol != "TypeDef" {
        return Err("Expected TypeDef node".to_string());
    }

    // Suponiendo una estructura básica: TYPE IDENT TypeParams? Attributes? Methods? BASE? BASE_ARGS?
    if node.children.len() < 2 {
        return Err("Invalid TypeDef structure".to_string());
    }

    let name = node.children[1]
        .token
        .as_ref()
        .ok_or("Missing type name")?
        .lexeme
        .clone();

    // Opcional: type_params, attributes, methods, base_type, base_args
    let type_params = vec![];
    let attributes = vec![];
    let methods = vec![];
    let base_type = String::new();
    let base_args = vec![];

    // Aquí podrías agregar lógica para extraer los campos opcionales si tu CST los provee

    Ok(TypeDecl {
        name,
        type_params,
        attributes,
        methods,
        base_type,
        base_args,
    })
}

// Estructuras auxiliares para la conversión
struct FunctionDecl {
    name: String,
    params: Vec<(String, Option<Type>)>,
    body: Stmt,
    return_type: Option<Type>,
}

struct TypeDecl {
    name: String,
    type_params: Vec<String>,
    attributes: Vec<AttributeDecl>,
    methods: Vec<MethodDecl>,
    base_type: String,
    base_args: Vec<Expr>,
}

// Funciones helper
fn get_span(node: &DerivationNode) -> Option<Span> {
    node.token.as_ref().map(|token| Span {
        line: token.line,
        column: token.column,
    })
}

// Replace the convert_primary function with a proper implementation
fn convert_primary(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "Primary" {
        return Err(format!("Expected Primary node, got {}", node.symbol));
    }

    if node.children.is_empty() {
        return Err("Primary node has no children".to_string());
    }

    let child = &node.children[0];
    let span = get_span(child);
    
    match child.symbol.as_str() {
        "NUMBER" => {
            let value = child.token.as_ref().unwrap().lexeme.parse::<f64>().map_err(|e| e.to_string())?;
            Ok(Expr {
                kind: ExprKind::Number(value),
                span,
            })
        }
        "STRING" => {
            let value = child.token.as_ref().unwrap().lexeme.clone();
            Ok(Expr {
                kind: ExprKind::String(value),
                span,
            })
        }
        "TRUE" => Ok(Expr {
            kind: ExprKind::Boolean(true),
            span,
        }),
        "FALSE" => Ok(Expr {
            kind: ExprKind::Boolean(false),
            span,
        }),
        "IDENT" => {
            let name = child.token.as_ref().unwrap().lexeme.clone();
            let mut expr = Expr {
                kind: ExprKind::Variable(name),
                span,
            };
            
            if node.children.len() > 1 {
                expr = convert_primary_tail(expr, &node.children[1])?;
            }
            
            Ok(expr)
        }
        "SELF" => {
            let mut expr = Expr {
                kind: ExprKind::SelfExpr,
                span,
            };
            
            if node.children.len() > 1 {
                expr = convert_primary_tail(expr, &node.children[1])?;
            }
            
            Ok(expr)
        }
        "BASE" => {
            if node.children.len() < 4 {
                return Err("Invalid BASE expression".to_string());
            }
            
            let args = convert_arg_list(&node.children[2])?;
            Ok(Expr {
                kind: ExprKind::BaseCall { args },
                span,
            })
        }
        "NEW" => {
            if node.children.len() < 5 {
                return Err("Invalid NEW expression".to_string());
            }
            
            let type_name = node.children[1]
                .token
                .as_ref()
                .ok_or("Missing type name in NEW expression")?
                .lexeme
                .clone();
            
            let args = convert_arg_list(&node.children[3])?;
            
            Ok(Expr {
                kind: ExprKind::New {
                    type_name,
                    args,
                },
                span,
            })
        }
        "LPAREN" => {
            if node.children.len() < 3 {
                return Err("Invalid parenthesized expression".to_string());
            }
            convert_expr(&node.children[1])
        }
        _ => Err(format!("Unsupported primary expression: {}", child.symbol)),
    }
}

// Also need to update convert_primary_tail
fn convert_primary_tail(base: Expr, node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "PrimaryTail" {
        return Err(format!("Expected PrimaryTail node, got {}", node.symbol));
    }

    if node.children.is_empty() {
        return Ok(base);
    }

    let first_child = &node.children[0];
    match first_child.symbol.as_str() {
        "LPAREN" => {
            if node.children.len() < 4 {
                return Err("Invalid function call".to_string());
            }
            
            let args = convert_arg_list(&node.children[1])?;
            let mut expr = match base.kind {
                ExprKind::Variable(name) => Expr {
                    kind: ExprKind::Call {
                        function: name,
                        args,
                    },
                    span: base.span,
                },
                ExprKind::GetAttr { object, attr } => Expr {
                    kind: ExprKind::MethodCall {
                        object,
                        method: attr,
                        args,
                    },
                    span: base.span,
                },
                _ => return Err("Invalid call expression".to_string()),
            };
            
            if node.children.len() > 3 {
                expr = convert_primary_tail(expr, &node.children[3])?;
            }
            
            Ok(expr)
        }
        "DOT" => {
            if node.children.len() < 3 {
                return Err("Invalid attribute access".to_string());
            }
            
            let attr = node.children[1]
                .token
                .as_ref()
                .ok_or("Missing attribute name")?
                .lexeme
                .clone();
            
            let span = base.span.clone();
            let expr = Expr {
                kind: ExprKind::GetAttr {
                    object: Box::new(base),
                    attr,
                },
                span,
            };
            
            if node.children.len() > 2 {
                convert_primary_tail(expr, &node.children[2])
            } else {
                Ok(expr)
            }
        }
        "ASSIGN" | "ASSIGN_DESTRUCT" => {
            if node.children.len() < 2 {
                return Err("Invalid assignment".to_string());
            }
            
            let value = convert_expr(&node.children[1])?;
            let expr = match base.kind {
                ExprKind::Variable(name) => Expr {
                    kind: ExprKind::Assign {
                        var_name: name,
                        value: Box::new(value),
                    },
                    span: base.span,
                },
                ExprKind::GetAttr { object, attr } => Expr {
                    kind: ExprKind::SetAttr {
                        object,
                        attr,
                        value: Box::new(value),
                    },
                    span: base.span,
                },
                _ => return Err("Invalid assignment target".to_string()),
            };
            
            Ok(expr)
        }
        _ => Ok(base),
    }
}

// Implementaciones para FunctionDef, TypeDef, etc.
fn convert_function_def(node: &DerivationNode) -> Result<FunctionDecl, String> {
    if node.symbol != "FunctionDef" {
        return Err("Expected FunctionDef node".to_string());
    }

    if node.children.len() < 7 {
        return Err("Invalid FunctionDef structure".to_string());
    }

    let name = node.children[1]
        .token
        .as_ref()
        .ok_or("Missing function name")?
        .lexeme
        .clone();
    
    let params = convert_arg_id_list_with_types(&node.children[3])?;
    let return_type = convert_type_annotation(&node.children[5])?;
    let body = convert_function_body(&node.children[6])?;
    
    Ok(FunctionDecl {
        name,
        params,
        body,
        return_type,
    })
}

// Implementación para TypeDef
fn convert_type_def(node: &DerivationNode) -> Result<TypeDecl, String> {
    if node.symbol != "TypeDef" {
        return Err("Expected TypeDef node".to_string());
    }

    // Suponiendo una estructura básica: TYPE IDENT TypeParams? Attributes? Methods? BASE? BASE_ARGS?
    if node.children.len() < 2 {
        return Err("Invalid TypeDef structure".to_string());
    }

    let name = node.children[1]
        .token
        .as_ref()
        .ok_or("Missing type name")?
        .lexeme
        .clone();

    // Opcional: type_params, attributes, methods, base_type, base_args
    let type_params = vec![];
    let attributes = vec![];
    let methods = vec![];
    let base_type = String::new();
    let base_args = vec![];

    // Aquí podrías agregar lógica para extraer los campos opcionales si tu CST los provee

    Ok(TypeDecl {
        name,
        type_params,
        attributes,
        methods,
        base_type,
        base_args,
    })
}

// Estructuras auxiliares para la conversión
struct FunctionDecl {
    name: String,
    params: Vec<(String, Option<Type>)>,
    body: Stmt,
    return_type: Option<Type>,
}

struct TypeDecl {
    name: String,
    type_params: Vec<String>,
    attributes: Vec<AttributeDecl>,
    methods: Vec<MethodDecl>,
    base_type: String,
    base_args: Vec<Expr>,
}

// Funciones helper
fn get_span(node: &DerivationNode) -> Option<Span> {
    node.token.as_ref().map(|token| Span {
        line: token.line,
        column: token.column,
    })
}

// Replace the convert_primary function with a proper implementation
fn convert_primary(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "Primary" {
        return Err(format!("Expected Primary node, got {}", node.symbol));
    }

    if node.children.is_empty() {
        return Err("Primary node has no children".to_string());
    }

    let child = &node.children[0];
    let span = get_span(child);
    
    match child.symbol.as_str() {
        "NUMBER" => {
            let value = child.token.as_ref().unwrap().lexeme.parse::<f64>().map_err(|e| e.to_string())?;
            Ok(Expr {
                kind: ExprKind::Number(value),
                span,
            })
        }
        "STRING" => {
            let value = child.token.as_ref().unwrap().lexeme.clone();
            Ok(Expr {
                kind: ExprKind::String(value),
                span,
            })
        }
        "TRUE" => Ok(Expr {
            kind: ExprKind::Boolean(true),
            span,
        }),
        "FALSE" => Ok(Expr {
            kind: ExprKind::Boolean(false),
            span,
        }),
        "IDENT" => {
            let name = child.token.as_ref().unwrap().lexeme.clone();
            let mut expr = Expr {
                kind: ExprKind::Variable(name),
                span,
            };
            
            if node.children.len() > 1 {
                expr = convert_primary_tail(expr, &node.children[1])?;
            }
            
            Ok(expr)
        }
        "SELF" => {
            let mut expr = Expr {
                kind: ExprKind::SelfExpr,
                span,
            };
            
            if node.children.len() > 1 {
                expr = convert_primary_tail(expr, &node.children[1])?;
            }
            
            Ok(expr)
        }
        "BASE" => {
            if node.children.len() < 4 {
                return Err("Invalid BASE expression".to_string());
            }
            
            let args = convert_arg_list(&node.children[2])?;
            Ok(Expr {
                kind: ExprKind::BaseCall { args },
                span,
            })
        }
        "NEW" => {
            if node.children.len() < 5 {
                return Err("Invalid NEW expression".to_string());
            }
            
            let type_name = node.children[1]
                .token
                .as_ref()
                .ok_or("Missing type name in NEW expression")?
                .lexeme
                .clone();
            
            let args = convert_arg_list(&node.children[3])?;
            
            Ok(Expr {
                kind: ExprKind::New {
                    type_name,
                    args,
                },
                span,
            })
        }
        "LPAREN" => {
            if node.children.len() < 3 {
                return Err("Invalid parenthesized expression".to_string());
            }
            convert_expr(&node.children[1])
        }
        _ => Err(format!("Unsupported primary expression: {}", child.symbol)),
    }
}

// Also need to update convert_primary_tail
fn convert_primary_tail(base: Expr, node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "PrimaryTail" {
        return Err(format!("Expected PrimaryTail node, got {}", node.symbol));
    }

    if node.children.is_empty() {
        return Ok(base);
    }

    let first_child = &node.children[0];
    match first_child.symbol.as_str() {
        "LPAREN" => {
            if node.children.len() < 4 {
                return Err("Invalid function call".to_string());
            }
            
            let args = convert_arg_list(&node.children[1])?;
            let mut expr = match base.kind {
                ExprKind::Variable(name) => Expr {
                    kind: ExprKind::Call {
                        function: name,
                        args,
                    },
                    span: base.span,
                },
                ExprKind::GetAttr { object, attr } => Expr {
                    kind: ExprKind::MethodCall {
                        object,
                        method: attr,
                        args,
                    },
                    span: base.span,
                },
                _ => return Err("Invalid call expression".to_string()),
            };
            
            if node.children.len() > 3 {
                expr = convert_primary_tail(expr, &node.children[3])?;
            }
            
            Ok(expr)
        }
        "DOT" => {
            if node.children.len() < 3 {
                return Err("Invalid attribute access".to_string());
            }
            
            let attr = node.children[1]
                .token
                .as_ref()
                .ok_or("Missing attribute name")?
                .lexeme
                .clone();
            
            let span = base.span.clone();
            let expr = Expr {
                kind: ExprKind::GetAttr {
                    object: Box::new(base),
                    attr,
                },
                span,
            };
            
            if node.children.len() > 2 {
                convert_primary_tail(expr, &node.children[2])
            } else {
                Ok(expr)
            }
        }
        "ASSIGN" | "ASSIGN_DESTRUCT" => {
            if node.children.len() < 2 {
                return Err("Invalid assignment".to_string());
            }
            
            let value = convert_expr(&node.children[1])?;
            let expr = match base.kind {
                ExprKind::Variable(name) => Expr {
                    kind: ExprKind::Assign {
                        var_name: name,
                        value: Box::new(value),
                    },
                    span: base.span,
                },
                ExprKind::GetAttr { object, attr } => Expr {
                    kind: ExprKind::SetAttr {
                        object,
                        attr,
                        value: Box::new(value),
                    },
                    span: base.span,
                },
                _ => return Err("Invalid assignment target".to_string()),
            };
            
            Ok(expr)
        }
        _ => Ok(base),
    }
}

// Implementaciones para FunctionDef, TypeDef, etc.
fn convert_function_def(node: &DerivationNode) -> Result<FunctionDecl, String> {
    if node.symbol != "FunctionDef" {
        return Err("Expected FunctionDef node".to_string());
    }

    if node.children.len() < 7 {
        return Err("Invalid FunctionDef structure".to_string());
    }

    let name = node.children[1]
        .token
        .as_ref()
        .ok_or("Missing function name")?
        .lexeme
        .clone();
    
    let params = convert_arg_id_list_with_types(&node.children[3])?;
    let return_type = convert_type_annotation(&node.children[5])?;
    let body = convert_function_body(&node.children[6])?;
    
    Ok(FunctionDecl {
        name,
        params,
        body,
        return_type,
    })
}

// Implementación para TypeDef
fn convert_type_def(node: &DerivationNode) -> Result<TypeDecl, String> {
    if node.symbol != "TypeDef" {
        return Err("Expected TypeDef node".to_string());
    }

    // Suponiendo una estructura básica: TYPE IDENT TypeParams? Attributes? Methods? BASE? BASE_ARGS?
    if node.children.len() < 2 {
        return Err("Invalid TypeDef structure".to_string());
    }

    let name = node.children[1]
        .token
        .as_ref()
        .ok_or("Missing type name")?
        .lexeme
        .clone();

    // Opcional: type_params, attributes, methods, base_type, base_args
    let type_params = vec![];
    let attributes = vec![];
    let methods = vec![];
    let base_type = String::new();
    let base_args = vec![];

    // Aquí podrías agregar lógica para extraer los campos opcionales si tu CST los provee

    Ok(TypeDecl {
        name,
        type_params,
        attributes,
        methods,
        base_type,
        base_args,
    })
}

// Estructuras auxiliares para la conversión
struct FunctionDecl {
    name: String,
    params: Vec<(String, Option<Type>)>,
    body: Stmt,
    return_type: Option<Type>,
}

struct TypeDecl {
    name: String,
    type_params: Vec<String>,
    attributes: Vec<AttributeDecl>,
    methods: Vec<MethodDecl>,
    base_type: String,
    base_args: Vec<Expr>,
}

// Funciones helper
fn get_span(node: &DerivationNode) -> Option<Span> {
    node.token.as_ref().map(|token| Span {
        line: token.line,
        column: token.column,
    })
}

// Replace the convert_primary function with a proper implementation
fn convert_primary(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "Primary" {
        return Err(format!("Expected Primary node, got {}", node.symbol));
    }

    if node.children.is_empty() {
        return Err("Primary node has no children".to_string());
    }

    let child = &node.children[0];
    let span = get_span(child);
    
    match child.symbol.as_str() {
        "NUMBER" => {
            let value = child.token.as_ref().unwrap().lexeme.parse::<f64>().map_err(|e| e.to_string())?;
            Ok(Expr {
                kind: ExprKind::Number(value),
                span,
            })
        }
        "STRING" => {
            let value = child.token.as_ref().unwrap().lexeme.clone();
            Ok(Expr {
                kind: ExprKind::String(value),
                span,
            })
        }
        "TRUE" => Ok(Expr {
            kind: ExprKind::Boolean(true),
            span,
        }),
        "FALSE" => Ok(Expr {
            kind: ExprKind::Boolean(false),
            span,
        }),
        "IDENT" => {
            let name = child.token.as_ref().unwrap().lexeme.clone();
            let mut expr = Expr {
                kind: ExprKind::Variable(name),
                span,
            };
            
            if node.children.len() > 1 {
                expr = convert_primary_tail(expr, &node.children[1])?;
            }
            
            Ok(expr)
        }
        "SELF" => {
            let mut expr = Expr {
                kind: ExprKind::SelfExpr,
                span,
            };
            
            if node.children.len() > 1 {
                expr = convert_primary_tail(expr, &node.children[1])?;
            }
            
            Ok(expr)
        }
        "BASE" => {
            if node.children.len() < 4 {
                return Err("Invalid BASE expression".to_string());
            }
            
            let args = convert_arg_list(&node.children[2])?;
            Ok(Expr {
                kind: ExprKind::BaseCall { args },
                span,
            })
        }
        "NEW" => {
            if node.children.len() < 5 {
                return Err("Invalid NEW expression".to_string());
            }
            
            let type_name = node.children[1]
                .token
                .as_ref()
                .ok_or("Missing type name in NEW expression")?
                .lexeme
                .clone();
            
            let args = convert_arg_list(&node.children[3])?;
            
            Ok(Expr {
                kind: ExprKind::New {
                    type_name,
                    args,
                },
                span,
            })
        }
        "LPAREN" => {
            if node.children.len() < 3 {
                return Err("Invalid parenthesized expression".to_string());
            }
            convert_expr(&node.children[1])
        }
        _ => Err(format!("Unsupported primary expression: {}", child.symbol)),
    }
}

// Also need to update convert_primary_tail
fn convert_primary_tail(base: Expr, node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "PrimaryTail" {
        return Err(format!("Expected PrimaryTail node, got {}", node.symbol));
    }

    if node.children.is_empty() {
        return Ok(base);
    }

    let first_child = &node.children[0];
    match first_child.symbol.as_str() {
        "LPAREN" => {
            if node.children.len() < 4 {
                return Err("Invalid function call".to_string());
            }
            
            let args = convert_arg_list(&node.children[1])?;
            let mut expr = match base.kind {
                ExprKind::Variable(name) => Expr {
                    kind: ExprKind::Call {
                        function: name,
                        args,
                    },
                    span: base.span,
                },
                ExprKind::GetAttr { object, attr } => Expr {
                    kind: ExprKind::MethodCall {
                        object,
                        method: attr,
                        args,
                    },
                    span: base.span,
                },
                _ => return Err("Invalid call expression".to_string()),
            };
            
            if node.children.len() > 3 {
                expr = convert_primary_tail(expr, &node.children[3])?;
            }
            
            Ok(expr)
        }
        "DOT" => {
            if node.children.len() < 3 {
                return Err("Invalid attribute access".to_string());
            }
            
            let attr = node.children[1]
                .token
                .as_ref()
                .ok_or("Missing attribute name")?
                .lexeme
                .clone();
            
            let span = base.span.clone();
            let expr = Expr {
                kind: ExprKind::GetAttr {
                    object: Box::new(base),
                    attr,
                },
                span,
            };
            
            if node.children.len() > 2 {
                convert_primary_tail(expr, &node.children[2])
            } else {
                Ok(expr)
            }
        }
        "ASSIGN" | "ASSIGN_DESTRUCT" => {
            if node.children.len() < 2 {
                return Err("Invalid assignment".to_string());
            }
            
            let value = convert_expr(&node.children[1])?;
            let expr = match base.kind {
                ExprKind::Variable(name) => Expr {
                    kind: ExprKind::Assign {
                        var_name: name,
                        value: Box::new(value),
                    },
                    span: base.span,
                },
                ExprKind::GetAttr { object, attr } => Expr {
                    kind: ExprKind::SetAttr {
                        object,
                        attr,
                        value: Box::new(value),
                    },
                    span: base.span,
                },
                _ => return Err("Invalid assignment target".to_string()),
            };
            
            Ok(expr)
        }
        _ => Ok(base),
    }
}

// Implementaciones para FunctionDef, TypeDef, etc.
fn convert_function_def(node: &DerivationNode) -> Result<FunctionDecl, String> {
    if node.symbol != "FunctionDef" {
        return Err("Expected FunctionDef node".to_string());
    }

    if node.children.len() < 7 {
        return Err("Invalid FunctionDef structure".to_string());
    }

    let name = node.children[1]
        .token
        .as_ref()
        .ok_or("Missing function name")?
        .lexeme
        .clone();
    
    let params = convert_arg_id_list_with_types(&node.children[3])?;
    let return_type = convert_type_annotation(&node.children[5])?;
    let body = convert_function_body(&node.children[6])?;
    
    Ok(FunctionDecl {
        name,
        params,
        body,
        return_type,
    })
}

// Implementación para TypeDef
fn convert_type_def(node: &DerivationNode) -> Result<TypeDecl, String> {
    if node.symbol != "TypeDef" {
        return Err("Expected TypeDef node".to_string());
    }

    // Suponiendo una estructura básica: TYPE IDENT TypeParams? Attributes? Methods? BASE? BASE_ARGS?
    if node.children.len() < 2 {
        return Err("Invalid TypeDef structure".to_string());
    }

    let name = node.children[1]
        .token
        .as_ref()
        .ok_or("Missing type name")?
        .lexeme
        .clone();

    // Opcional: type_params, attributes, methods, base_type, base_args
    let type_params = vec![];
    let attributes = vec![];
    let methods = vec![];
    let base_type = String::new();
    let base_args = vec![];

    // Aquí podrías agregar lógica para extraer los campos opcionales si tu CST los provee

    Ok(TypeDecl {
        name,
        type_params,
        attributes,
        methods,
        base_type,
        base_args,
    })
}

// Estructuras auxiliares para la conversión
struct FunctionDecl {
    name: String,
    params: Vec<(String, Option<Type>)>,
    body: Stmt,
    return_type: Option<Type>,
}

struct TypeDecl {
    name: String,
    type_params: Vec<String>,
    attributes: Vec<AttributeDecl>,
    methods: Vec<MethodDecl>,
    base_type: String,
    base_args: Vec<Expr>,
}

// Funciones helper
fn get_span(node: &DerivationNode) -> Option<Span> {
    node.token.as_ref().map(|token| Span {
        line: token.line,
        column: token.column,
    })
}

// Replace the convert_primary function with a proper implementation
fn convert_primary(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "Primary" {
        return Err(format!("Expected Primary node, got {}", node.symbol));
    }

    if node.children.is_empty() {
        return Err("Primary node has no children".to_string());
    }

    let child = &node.children[0];
    let span = get_span(child);
    
    match child.symbol.as_str() {
        "NUMBER" => {
            let value = child.token.as_ref().unwrap().lexeme.parse::<f64>().map_err(|e| e.to_string())?;
            Ok(Expr {
                kind: ExprKind::Number(value),
                span,
            })
        }
        "STRING" => {
            let value = child.token.as_ref().unwrap().lexeme.clone();
            Ok(Expr {
                kind: ExprKind::String(value),
                span,
            })
        }
        "TRUE" => Ok(Expr {
            kind: ExprKind::Boolean(true),
            span,
        }),
        "FALSE" => Ok(Expr {
            kind: ExprKind::Boolean(false),
            span,
        }),
        "IDENT" => {
            let name = child.token.as_ref().unwrap().lexeme.clone();
            let mut expr = Expr {
                kind: ExprKind::Variable(name),
                span,
            };
            
            if node.children.len() > 1 {
                expr = convert_primary_tail(expr, &node.children[1])?;
            }
            
            Ok(expr)
        }
        "SELF" => {
            let mut expr = Expr {
                kind: ExprKind::SelfExpr,
                span,
            };
            
            if node.children.len() > 1 {
                expr = convert_primary_tail(expr, &node.children[1])?;
            }
            
            Ok(expr)
        }
        "BASE" => {
            if node.children.len() < 4 {
                return Err("Invalid BASE expression".to_string());
            }
            
            let args = convert_arg_list(&node.children[2])?;
            Ok(Expr {
                kind: ExprKind::BaseCall { args },
                span,
            })
        }
        "NEW" => {
            if node.children.len() < 5 {
                return Err("Invalid NEW expression".to_string());
            }
            
            let type_name = node.children[1]
                .token
                .as_ref()
                .ok_or("Missing type name in NEW expression")?
                .lexeme
                .clone();
            
            let args = convert_arg_list(&node.children[3])?;
            
            Ok(Expr {
                kind: ExprKind::New {
                    type_name,
                    args,
                },
                span,
            })
        }
        "LPAREN" => {
            if node.children.len() < 3 {
                return Err("Invalid parenthesized expression".to_string());
            }
            convert_expr(&node.children[1])
        }
        _ => Err(format!("Unsupported primary expression: {}", child.symbol)),
    }
}

// Also need to update convert_primary_tail
fn convert_primary_tail(base: Expr, node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "PrimaryTail" {
        return Err(format!("Expected PrimaryTail node, got {}", node.symbol));
    }

    if node.children.is_empty() {
        return Ok(base);
    }

    let first_child = &node.children[0];
    match first_child.symbol.as_str() {
        "LPAREN" => {
            if node.children.len() < 4 {
                return Err("Invalid function call".to_string());
            }
            
            let args = convert_arg_list(&node.children[1])?;
            let mut expr = match base.kind {
                ExprKind::Variable(name) => Expr {
                    kind: ExprKind::Call {
                        function: name,
                        args,
                    },
                    span: base.span,
                },
                ExprKind::GetAttr { object, attr } => Expr {
                    kind: ExprKind::MethodCall {
                        object,
                        method: attr,
                        args,
                    },
                    span: base.span,
                },
                _ => return Err("Invalid call expression".to_string()),
            };
            
            if node.children.len() > 3 {
                expr = convert_primary_tail(expr, &node.children[3])?;
            }
            
            Ok(expr)
        }
        "DOT" => {
            if node.children.len() < 3 {
                return Err("Invalid attribute access".to_string());
            }
            
            let attr = node.children[1]
                .token
                .as_ref()
                .ok_or("Missing attribute name")?
                .lexeme
                .clone();
            
            let span = base.span.clone();
            let expr = Expr {
                kind: ExprKind::GetAttr {
                    object: Box::new(base),
                    attr,
                },
                span,
            };
            
            if node.children.len() > 2 {
                convert_primary_tail(expr, &node.children[2])
            } else {
                Ok(expr)
            }
        }
        "ASSIGN" | "ASSIGN_DESTRUCT" => {
            if node.children.len() < 2 {
                return Err("Invalid assignment".to_string());
            }
            
            let value = convert_expr(&node.children[1])?;
            let expr = match base.kind {
                ExprKind::Variable(name) => Expr {
                    kind: ExprKind::Assign {
                        var_name: name,
                        value: Box::new(value),
                    },
                    span: base.span,
                },
                ExprKind::GetAttr { object, attr } => Expr {
                    kind: ExprKind::SetAttr {
                        object,
                        attr,
                        value: Box::new(value),
                    },
                    span: base.span,
                },
                _ => return Err("Invalid assignment target".to_string()),
            };
            
            Ok(expr)
        }
        _ => Ok(base),
    }
}

// Implementaciones para FunctionDef, TypeDef, etc.
fn convert_function_def(node: &DerivationNode) -> Result<FunctionDecl, String> {
    if node.symbol != "FunctionDef" {
        return Err("Expected FunctionDef node".to_string());
    }

    if node.children.len() < 7 {
        return Err("Invalid FunctionDef structure".to_string());
    }

    let name = node.children[1]
        .token
        .as_ref()
        .ok_or("Missing function name")?
        .lexeme
        .clone();
    
    let params = convert_arg_id_list_with_types(&node.children[3])?;
    let return_type = convert_type_annotation(&node.children[5])?;
    let body = convert_function_body(&node.children[6])?;
    
    Ok(FunctionDecl {
        name,
        params,
        body,
        return_type,
    })
}

// Implementación para TypeDef
fn convert_type_def(node: &DerivationNode) -> Result<TypeDecl, String> {
    if node.symbol != "TypeDef" {
        return Err("Expected TypeDef node".to_string());
    }

    // Suponiendo una estructura básica: TYPE IDENT TypeParams? Attributes? Methods? BASE? BASE_ARGS?
    if node.children.len() < 2 {
        return Err("Invalid TypeDef structure".to_string());
    }

    let name = node.children[1]
        .token
        .as_ref()
        .ok_or("Missing type name")?
        .lexeme
        .clone();

    // Opcional: type_params, attributes, methods, base_type, base_args
    let type_params = vec![];
    let attributes = vec![];
    let methods = vec![];
    let base_type = String::new();
    let base_args = vec![];

    // Aquí podrías agregar lógica para extraer los campos opcionales si tu CST los provee

    Ok(TypeDecl {
        name,
        type_params,
        attributes,
        methods,
        base_type,
        base_args,
    })
}

// Estructuras auxiliares para la conversión
struct FunctionDecl {
    name: String,
    params: Vec<(String, Option<Type>)>,
    body: Stmt,
    return_type: Option<Type>,
}

struct TypeDecl {
    name: String,
    type_params: Vec<String>,
    attributes: Vec<AttributeDecl>,
    methods: Vec<MethodDecl>,
    base_type: String,
    base_args: Vec<Expr>,
}

// Funciones helper
fn get_span(node: &DerivationNode) -> Option<Span> {
    node.token.as_ref().map(|token| Span {
        line: token.line,
        column: token.column,
    })
}

// Replace the convert_primary function with a proper implementation
fn convert_primary(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "Primary" {
        return Err(format!("Expected Primary node, got {}", node.symbol));
    }

    if node.children.is_empty() {
        return Err("Primary node has no children".to_string());
    }

    let child = &node.children[0];
    let span = get_span(child);
    
    match child.symbol.as_str() {
        "NUMBER" => {
            let value = child.token.as_ref().unwrap().lexeme.parse::<f64>().map_err(|e| e.to_string())?;
            Ok(Expr {
                kind: ExprKind::Number(value),
                span,
            })
        }
        "STRING" => {
            let value = child.token.as_ref().unwrap().lexeme.clone();
            Ok(Expr {
                kind: ExprKind::String(value),
                span,
            })
        }
        "TRUE" => Ok(Expr {
            kind: ExprKind::Boolean(true),
            span,
        }),
        "FALSE" => Ok(Expr {
            kind: ExprKind::Boolean(false),
            span,
        }),
        "IDENT" => {
            let name = child.token.as_ref().unwrap().lexeme.clone();
            let mut expr = Expr {
                kind: ExprKind::Variable(name),
                span,
            };
            
            if node.children.len() > 1 {
                expr = convert_primary_tail(expr, &node.children[1])?;
            }
            
            Ok(expr)
        }
        "SELF" => {
            let mut expr = Expr {
                kind: ExprKind::SelfExpr,
                span,
            };
            
            if node.children.len() > 1 {
                expr = convert_primary_tail(expr, &node.children[1])?;
            }
            
            Ok(expr)
        }
        "BASE" => {
            if node.children.len() < 4 {
                return Err("Invalid BASE expression".to_string());
            }
            
            let args = convert_arg_list(&node.children[2])?;
            Ok(Expr {
                kind: ExprKind::BaseCall { args },
                span,
            })
        }
        "NEW" => {
            if node.children.len() < 5 {
                return Err("Invalid NEW expression".to_string());
            }
            
            let type_name = node.children[1]
                .token
                .as_ref()
                .ok_or("Missing type name in NEW expression")?
                .lexeme
                .clone();
            
            let args = convert_arg_list(&node.children[3])?;
            
            Ok(Expr {
                kind: ExprKind::New {
                    type_name,
                    args,
                },
                span,
            })
        }
        "LPAREN" => {
            if node.children.len() < 3 {
                return Err("Invalid parenthesized expression".to_string());
            }
            convert_expr(&node.children[1])
        }
        _ => Err(format!("Unsupported primary expression: {}", child.symbol)),
    }
}

// Also need to update convert_primary_tail
fn convert_primary_tail(base: Expr, node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "PrimaryTail" {
        return Err(format!("Expected PrimaryTail node, got {}", node.symbol));
    }

    if node.children.is_empty() {
        return Ok(base);
    }

    let first_child = &node.children[0];
    match first_child.symbol.as_str() {
        "LPAREN" => {
            if node.children.len() < 4 {
                return Err("Invalid function call".to_string());
            }
            
            let args = convert_arg_list(&node.children[1])?;
            let mut expr = match base.kind {
                ExprKind::Variable(name) => Expr {
                    kind: ExprKind::Call {
                        function: name,
                        args,
                    },
                    span: base.span,
                },
                ExprKind::GetAttr { object, attr } => Expr {
                    kind: ExprKind::MethodCall {
                        object,
                        method: attr,
                        args,
                    },
                    span: base.span,
                },
                _ => return Err("Invalid call expression".to_string()),
            };
            
            if node.children.len() > 3 {
                expr = convert_primary_tail(expr, &node.children[3])?;
            }
            
            Ok(expr)
        }
        "DOT" => {
            if node.children.len() < 3 {
                return Err("Invalid attribute access".to_string());
            }
            
            let attr = node.children[1]
                .token
                .as_ref()
                .ok_or("Missing attribute name")?
                .lexeme
                .clone();
            
            let span = base.span.clone();
            let expr = Expr {
                kind: ExprKind::GetAttr {
                    object: Box::new(base),
                    attr,
                },
                span,
            };
            
            if node.children.len() > 2 {
                convert_primary_tail(expr, &node.children[2])
            } else {
                Ok(expr)
            }
        }
        "ASSIGN" | "ASSIGN_DESTRUCT" => {
            if node.children.len() < 2 {
                return Err("Invalid assignment".to_string());
            }
            
            let value = convert_expr(&node.children[1])?;
            let expr = match base.kind {
                ExprKind::Variable(name) => Expr {
                    kind: ExprKind::Assign {
                        var_name: name,
                        value: Box::new(value),
                    },
                    span: base.span,
                },
                ExprKind::GetAttr { object, attr } => Expr {
                    kind: ExprKind::SetAttr {
                        object,
                        attr,
                        value: Box::new(value),
                    },
                    span: base.span,
                },
                _ => return Err("Invalid assignment target".to_string()),
            };
            
            Ok(expr)
        }
        _ => Ok(base),
    }
}

// Implementaciones para FunctionDef, TypeDef, etc.
fn convert_function_def(node: &DerivationNode) -> Result<FunctionDecl, String> {
    if node.symbol != "FunctionDef" {
        return Err("Expected FunctionDef node".to_string());
    }

    if node.children.len() < 7 {
        return Err("Invalid FunctionDef structure".to_string());
    }

    let name = node.children[1]
        .token
        .as_ref()
        .ok_or("Missing function name")?
        .lexeme
        .clone();
    
    let params = convert_arg_id_list_with_types(&node.children[3])?;
    let return_type = convert_type_annotation(&node.children[5])?;
    let body = convert_function_body(&node.children[6])?;
    
    Ok(FunctionDecl {
        name,
        params,
        body,
        return_type,
    })
}

