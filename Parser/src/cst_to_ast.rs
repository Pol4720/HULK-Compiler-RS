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

    if node.children[0].symbol == "ε" {
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

    if node.children[0].symbol == "ε" {
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
            let expr = convert_block_expr(child)?;
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

    if node.children[0].symbol == "ε" {
        return Err("Expr node has no children".to_string());
    }

    let child = &node.children[0];
    let span = get_span(node);
    
    match child.symbol.as_str() {
        "OrExpr" => convert_or_expr(child),
        "IfExpr" => convert_if_expr(child),
        "LetExpr" => convert_let_expr(child),
        _ => Err(format!("Unsupported expression type: {}", child.symbol)),
    }
}

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
    if node.children[0].symbol == "ε" {
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
    if node.children[0].symbol == "ε" {
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
    if node.children[0].symbol == "ε" {
        return Ok(left);
    }

    if node.children.len() < 3 {
        return Err("Invalid CmpExpr' structure".to_string());
    }

    if node.children[0].symbol == "IS" {
        let type_name = get_token_value(&node.children[1])?;
        let expr = Box::new(Expr {
            kind: ExprKind::Is {
                expr: Box::new(left),
                type_name,
            },
            span: None,
        });
        convert_cmp_expr_prime(*expr, &node.children[2])
    } else {
        let op = convert_binary_op(&node.children[0].symbol)?;
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
    if node.children[0].symbol == "ε" {
        return Ok(left);
    }

    if node.children.len() < 3 {
        return Err("Invalid ConcatExpr' structure".to_string());
    }

    let op = convert_binary_op(&node.children[0].symbol)?;
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
    
    if node.children[0].symbol == "ε" {
        
        return Ok(left);
    }

    if node.children.len() < 3 {
        return Err("Invalid AddExpr' structure".to_string());
    }

    let op = convert_binary_op(&node.children[0].symbol)?;

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
    if node.children[0].symbol == "ε" {
        return Ok(left);
    }

    if node.children.len() < 3 {
        return Err("Invalid Term' structure".to_string());
    }

    let op = convert_binary_op(&node.children[0].symbol)?;
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

    if node.children.len() < 2 {
        return Err("Factor requires two children".to_string());
    }

    let left = convert_power(&node.children[0])?;
    convert_factor_prime(left, &node.children[1])
}

fn convert_factor_prime(left: Expr, node: &DerivationNode) -> Result<Expr, String> {
    if node.children[0].symbol == "ε" {
        return Ok(left);
    }

    if node.children.len() < 3 {
        return Err("Invalid Factor' structure".to_string());
    }

    let right = convert_power(&node.children[1])?;
    let new_left = Expr {
        kind: ExprKind::Binary {
            op: BinaryOp::Pow,
            left: Box::new(left),
            right: Box::new(right),
        },
        span: None,
    };

    convert_factor_prime(new_left, &node.children[2])
}

fn convert_power(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "Power" {
        return Err("Expected Power node".to_string());
    }

    if node.children[0].symbol == "ε" {
        return Err("Power node has no children".to_string());
    }

    convert_unary(&node.children[0])
}

fn convert_unary(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "Unary" {
        return Err("Expected Unary node".to_string());
    }

    if node.children[0].symbol == "ε" {
        return Err("Unary node has no children".to_string());
    }

    let child = &node.children[0];
    let span = get_span(child);

    // Unary → MINUS Unary | Primary AsExpr
    if node.children.len() == 2 && node.children[0].symbol == "MINUS" {
        let expr = convert_unary(&node.children[1])?;
        return Ok(Expr {
            kind: ExprKind::Unary {
                op: UnaryOp::Neg,
                expr: Box::new(expr),
            },
            span,
        });
    }


    // Primary AsExpr
    let primary = convert_primary(&node.children[0])?;
    if node.children.len() > 1 {
        convert_as_expr(primary, &node.children[1])
    } else {
        Ok(primary)
    }
}

fn convert_primary(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "Primary" {
        return Err("Expected Primary node".to_string());
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

fn convert_primary_tail(base: Expr, node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "PrimaryTail" {
        return Err("Expected PrimaryTail node".to_string());
    }

    if node.children[0].symbol == "ε" {
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

fn convert_as_expr(base: Expr, node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "AsExpr" {
        return Err("Expected AsExpr node".to_string());
    }

    if node.children[0].symbol == "ε" {
        return Ok(base);
    }

    if node.children.len() < 3 {
        return Err("Invalid AsExpr structure".to_string());
    }

    let type_name = get_token_value(&node.children[1])?;
    let expr = Expr {
        kind: ExprKind::As {
            expr: Box::new(base),
            type_name,
        },
        span: None,
    };

    if node.children.len() > 2 {
        convert_as_expr(expr, &node.children[2])
    } else {
        Ok(expr)
    }
}

fn convert_if_expr(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "IfExpr" {
        return Err("Expected IfExpr node".to_string());
    }

    // IfExpr → IF LPAREN Expr RPAREN IfBody ElifList ELSE IfBody
    if node.children.len() < 8 {
        return Err("Invalid IfExpr structure".to_string());
    }

    let condition = convert_expr(&node.children[2])?;
    let then_branch = convert_if_body(&node.children[4])?;
    let elif_list = convert_elif_list(&node.children[5])?;
    let else_branch = convert_if_body(&node.children[7])?;

    // Construir la cadena de if-elif-else
    let mut current_else = else_branch;
    
    for elif in elif_list.into_iter().rev() {
        current_else = Expr {
            kind: ExprKind::If {
                condition: Box::new(elif.condition),
                then_branch: Box::new(elif.then_branch),
                else_branch: Some(Box::new(current_else)),
            },
            span: None,
        };
    }

    Ok(Expr {
        kind: ExprKind::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: Some(Box::new(current_else)),
        },
        span: get_span(node),
    })
}

fn convert_let_expr(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "LetExpr" {
        return Err("Expected LetExpr node".to_string());
    }

    // LetExpr → LET VarBindingList IN LetBody
    if node.children.len() < 4 {
        return Err("Invalid LetExpr structure".to_string());
    }

    let bindings = convert_var_binding_list(&node.children[1])?;
    let body = convert_let_body(&node.children[3])?;

    // Construir expresiones let anidadas
    let mut current_body = body;
    
    for binding in bindings.into_iter().rev() {
        current_body = Expr {
            kind: ExprKind::Let {
                name: binding.name,
                value: Box::new(binding.value),
                body: Box::new(current_body),
                declared_type: binding.declared_type,
            },
            span: None,
        };
    }

    Ok(current_body)
}

fn convert_while_expr(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "WhileStmt" {
        return Err("Expected WhileStmt node".to_string());
    }

    // WhileStmt → WHILE Expr WhileBody
    if node.children.len() < 3 {
        return Err("Invalid WhileStmt structure".to_string());
    }

    let condition = convert_expr(&node.children[1])?;
    let body = convert_while_body(&node.children[2])?;

    Ok(Expr {
        kind: ExprKind::While {
            condition: Box::new(condition),
            body: Box::new(body),
        },
        span: get_span(node),
    })
}

fn convert_for_expr(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "ForStmt" {
        return Err("Expected ForStmt node".to_string());
    }

    // ForStmt → FOR LPAREN IDENT IN Expr RPAREN ForBody
    if node.children.len() < 7 {
        return Err("Invalid ForStmt structure".to_string());
    }

    let iterator = get_token_value(&node.children[2])?;
    let collection = convert_expr(&node.children[4])?;
    let body = convert_for_body(&node.children[6])?;

    Ok(Expr {
        kind: ExprKind::For {
            iterator,
            collection: Box::new(collection),
            body: Box::new(body),
        },
        span: get_span(node),
    })
}

fn convert_block_expr(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "BlockStmt" {
        return Err("Expected BlockStmt node".to_string());
    }

    // BlockStmt → LBRACE StmtList RBRACE
    if node.children.len() < 3 {
        return Err("Invalid BlockStmt structure".to_string());
    }

    let stmts = convert_stmt_list(&node.children[1])?;
    
    Ok(Expr {
        kind: ExprKind::Block { stmts },
        span: get_span(node),
    })
}

// Funciones para declaraciones
fn convert_function_def(node: &DerivationNode) -> Result<FunctionDecl, String> {
    if node.symbol != "FunctionDef" {
        return Err("Expected FunctionDef node".to_string());
    }

    // FunctionDef → FUNCTION IDENT LPAREN ArgIdList RPAREN TypeAnnotation FunctionBody
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

fn convert_type_def(node: &DerivationNode) -> Result<TypeDecl, String> {
    if node.symbol != "TypeDef" {
        return Err("Expected TypeDef node".to_string());
    }

    // TypeDef → TYPE IDENT TypeParams TypeInheritance LBRACE TypeBody RBRACE
    if node.children.len() < 7 {
        return Err("Invalid TypeDef structure".to_string());
    }

    let name = node.children[1]
        .token
        .as_ref()
        .ok_or("Missing type name")?
        .lexeme
        .clone();
    
    let type_params = convert_type_params(&node.children[2])?;
    let (base_type, base_args) = convert_type_inheritance(&node.children[3])?;
    let (attributes, methods) = convert_type_body(&node.children[5])?;
    
    Ok(TypeDecl {
        name,
        type_params,
        attributes,
        methods,
        base_type,
        base_args,
    })
}

// Funciones auxiliares
fn get_span(node: &DerivationNode) -> Option<Span> {
    node.token.as_ref().map(|token| Span {
        line: token.line,
        column: token.column,
    })
}

fn get_token_value(node: &DerivationNode) -> Result<String, String> {
    node.token
        .as_ref()
        .map(|token| token.lexeme.clone())
        .ok_or("Node has no token".to_string())
}

fn convert_binary_op(op: &str) -> Result<BinaryOp, String> {
    match op {
        "PLUS" => Ok(BinaryOp::Add),
        "MINUS" => Ok(BinaryOp::Sub),
        "MULT" => Ok(BinaryOp::Mul),
        "DIV" => Ok(BinaryOp::Div),
        "MOD" => Ok(BinaryOp::Mod),
        "POW" => Ok(BinaryOp::Pow),
        "EQ" => Ok(BinaryOp::Eq),
        "NEQ" => Ok(BinaryOp::Neq),
        "LT" => Ok(BinaryOp::Lt),
        "GT" => Ok(BinaryOp::Gt),
        "LE" => Ok(BinaryOp::Le),
        "GE" => Ok(BinaryOp::Ge),
        "AND" => Ok(BinaryOp::And),
        "OR" => Ok(BinaryOp::Or),
        "CONCAT" => Ok(BinaryOp::Concat),
        "CONCAT_WS" => Ok(BinaryOp::ConcatWs),
        _ => Err(format!("Unknown binary operator: {}", op)),
    }
}

fn convert_arg_list(node: &DerivationNode) -> Result<Vec<Expr>, String> {
    if node.symbol != "ArgList" {
        return Err("Expected ArgList node".to_string());
    }

    let mut args = vec![];
    
    // ArgList → Expr ArgListTail | ε
    if node.children[0].children.is_empty() {
        return Ok(args);
    }
    
    // Primer argumento
    if !node.children[0].children.is_empty() {
        args.push(convert_expr(&node.children[0].children[0])?);
    }
    
    // Argumentos adicionales
    if node.children.len() > 1 {
        let rest = convert_arg_list_tail(&node.children[1])?;
        args.extend(rest);
    }
    
    Ok(args)
}

fn convert_arg_list_tail(node: &DerivationNode) -> Result<Vec<Expr>, String> {
    if node.symbol != "ArgListTail" {
        return Err("Expected ArgListTail node".to_string());
    }

    let mut args = vec![];
    
    // ArgListTail → COMMA Expr ArgListTail | ε
    if node.children[0].children.is_empty() {
        return Ok(args);
    }
    
    for i in (1..node.children.len()).step_by(2) {
        if i < node.children.len() {
            args.push(convert_expr(&node.children[i])?);
        }
    }
    
    Ok(args)
}

fn convert_type_annotation(node: &DerivationNode) -> Result<Option<Type>, String> {
    if node.symbol != "TypeAnnotation" || node.children[0].children.is_empty() {
        return Ok(None);
    }
    
    if node.children.len() < 2 {
        return Err("Invalid type annotation".to_string());
    }
    
    let type_name = node.children[1]
        .token
        .as_ref()
        .ok_or("Missing type name")?
        .lexeme
        .clone();
    
    let ty = match type_name.as_str() {
        "Number" => Type::Number,
        "String" => Type::String,
        "Boolean" => Type::Boolean,
        _ => Type::Object(type_name),
    };
    
    Ok(Some(ty))
}

fn convert_arg_id_list_with_types(node: &DerivationNode) -> Result<Vec<(String, Option<Type>)>, String> {
    if node.symbol != "ArgIdList" {
        return Err("Expected ArgIdList node".to_string());
    }

    let mut params = vec![];
    
    for child in &node.children {
        if child.symbol == "ArgId" && child.children.len() >= 2 {
            let name = child.children[0]
                .token
                .as_ref()
                .ok_or("Missing parameter name")?
                .lexeme
                .clone();
            
            let ty = if child.children.len() > 1 {
                convert_type_annotation(&child.children[1])?
            } else {
                None
            };
            
            params.push((name, ty));
        }
    }
    
    Ok(params)
}

fn convert_function_body(node: &DerivationNode) -> Result<Stmt, String> {
    if node.children[0].children.is_empty() {
        return Err("Empty function body".to_string());
    }
    
    let child = &node.children[0];
    match child.symbol.as_str() {
        "ARROW" => {
            if node.children.len() < 2 {
                return Err("Invalid arrow function".to_string());
            }
            let expr = convert_expr(&node.children[1])?;
            Ok(Stmt {
                kind: StmtKind::ExprStmt(expr),
                span: get_span(child),
            })
        }
        "BlockStmt" => convert_stmt(child),
        _ => Err("Unsupported function body".to_string()),
    }
}

fn convert_type_params(node: &DerivationNode) -> Result<Vec<String>, String> {
    if node.symbol != "TypeParams" {
        return Err("Expected TypeParams node".to_string());
    }

    let mut params = vec![];
    
    for child in &node.children {
        if child.symbol == "IDENT" {
            if let Some(token) = &child.token {
                params.push(token.lexeme.clone());
            }
        }
    }
    
    Ok(params)
}

fn convert_type_inheritance(node: &DerivationNode) -> Result<(String, Vec<Expr>), String> {
    if node.symbol != "TypeInheritance" || node.children[0].children.is_empty() {
        return Ok(("Object".to_string(), vec![]));
    }

    if node.children.len() < 3 {
        return Err("Invalid TypeInheritance structure".to_string());
    }

    let base_type = get_token_value(&node.children[1])?;
    let base_args = convert_type_base_args(&node.children[2])?;
    
    Ok((base_type, base_args))
}

fn convert_type_base_args(node: &DerivationNode) -> Result<Vec<Expr>, String> {
    if node.symbol != "TypeBaseArgs" || node.children[0].children.is_empty() {
        return Ok(vec![]);
    }

    if node.children.len() < 3 {
        return Err("Invalid TypeBaseArgs structure".to_string());
    }

    convert_arg_list(&node.children[1])
}

fn convert_type_body(node: &DerivationNode) -> Result<(Vec<AttributeDecl>, Vec<MethodDecl>), String> {
    if node.symbol != "TypeBody" {
        return Err("Expected TypeBody node".to_string());
    }

    let mut attributes = vec![];
    let mut methods = vec![];
    
    for child in &node.children {
        if child.symbol == "TypeMember" {
            let (attr, method) = convert_type_member(child)?;
            if let Some(attr) = attr {
                attributes.push(attr);
            }
            if let Some(method) = method {
                methods.push(method);
            }
        }
    }
    
    Ok((attributes, methods))
}

fn convert_type_member(node: &DerivationNode) -> Result<(Option<AttributeDecl>, Option<MethodDecl>), String> {
    if node.symbol != "TypeMember" || node.children.len() < 2 {
        return Ok((None, None));
    }

    let ident = &node.children[0];
    let name = get_token_value(ident)?;
    let tail = &node.children[1];
    
    convert_type_member_tail(name, tail)
}

fn convert_type_member_tail(name: String, node: &DerivationNode) -> Result<(Option<AttributeDecl>, Option<MethodDecl>), String> {
    if node.symbol != "TypeMemberTail" || node.children[0].children.is_empty() {
        return Ok((None, None));
    }

    let first = &node.children[0];
    match first.symbol.as_str() {
        "TypeAnnotation" => {
            // Atributo
            let declared_type = convert_type_annotation(first)?;
            let initializer = if node.children.len() > 1 {
                if let Some(assign) = node.children[1].children.get(1) {
                    Some(convert_expr(assign)?)
                } else {
                    None
                }
            } else {
                None
            };
            
            Ok((Some(AttributeDecl {
                name,
                initializer,
                declared_type,
            }), None))
        }
        "LPAREN" => {
            // Método
            if node.children.len() < 6 {
                return Err("Invalid method declaration".to_string());
            }
            
            let params = convert_arg_id_list_with_types(&node.children[1])?;
            let return_type = convert_type_annotation(&node.children[3])?;
            let body = convert_function_body(&node.children[4])?;
            
            Ok((None, Some(MethodDecl {
                name,
                params,
                body: Box::new(body),
                return_type,
            })))
        }
        _ => Ok((None, None)),
    }
}

fn convert_if_body(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "IfBody" || node.children[0].children.is_empty() {
        return Err("Invalid IfBody".to_string());
    }

    let child = &node.children[0];
    match child.symbol.as_str() {
        "BlockStmt" => convert_block_expr(child),
        "Expr" => convert_expr(child),
        _ => Err("Unsupported IfBody".to_string()),
    }
}

fn convert_elif_list(node: &DerivationNode) -> Result<Vec<ElifBranch>, String> {
    if node.symbol != "ElifList" {
        return Err("Expected ElifList node".to_string());
    }

    let mut branches = vec![];
    
    for child in &node.children {
        if child.symbol == "ElifBranch" {
            branches.push(convert_elif_branch(child)?);
        }
    }
    
    Ok(branches)
}

fn convert_elif_branch(node: &DerivationNode) -> Result<ElifBranch, String> {
    if node.symbol != "ElifBranch" || node.children.len() < 5 {
        return Err("Invalid ElifBranch".to_string());
    }

    let condition = convert_expr(&node.children[2])?;
    let then_branch = convert_if_body(&node.children[4])?;
    
    Ok(ElifBranch {
        condition,
        then_branch,
    })
}

fn convert_var_binding_list(node: &DerivationNode) -> Result<Vec<VarBinding>, String> {
    if node.symbol != "VarBindingList" {
        return Err("Expected VarBindingList node".to_string());
    }

    let mut bindings = vec![];
    
    for child in &node.children {
        if child.symbol == "VarBinding" {
            bindings.push(convert_var_binding(child)?);
        }
    }
    
    Ok(bindings)
}

fn convert_var_binding(node: &DerivationNode) -> Result<VarBinding, String> {
    if node.symbol != "VarBinding" || node.children.len() < 4 {
        return Err("Invalid VarBinding".to_string());
    }

    let name = get_token_value(&node.children[0])?;
    let declared_type = convert_type_annotation(&node.children[1])?;
    let value = convert_expr(&node.children[3])?;
    
    Ok(VarBinding {
        name,
        value,
        declared_type,
    })
}

fn convert_let_body(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "LetBody" || node.children[0].children.is_empty() {
        return Err("Invalid LetBody".to_string());
    }

    let child = &node.children[0];
    match child.symbol.as_str() {
        "BlockStmt" => convert_block_expr(child),
        "Expr" => convert_expr(child),
        "WhileStmt" => convert_while_expr(child),
        "ForStmt" => convert_for_expr(child),
        _ => Err("Unsupported LetBody".to_string()),
    }
}

fn convert_while_body(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "WhileBody" || node.children[0].children.is_empty() {
        return Err("Invalid WhileBody".to_string());
    }

    let child = &node.children[0];
    match child.symbol.as_str() {
        "BlockStmt" => convert_block_expr(child),
        "Expr" => convert_expr(child),
        _ => Err("Unsupported WhileBody".to_string()),
    }
}

fn convert_for_body(node: &DerivationNode) -> Result<Expr, String> {
    if node.symbol != "ForBody" || node.children[0].children.is_empty() {
        return Err("Invalid ForBody".to_string());
    }

    let child = &node.children[0];
    match child.symbol.as_str() {
        "BlockStmt" => convert_block_expr(child),
        "Expr" => convert_expr(child),
        _ => Err("Unsupported ForBody".to_string()),
    }
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

struct ElifBranch {
    condition: Expr,
    then_branch: Expr,
}

struct VarBinding {
    name: String,
    value: Expr,
    declared_type: Option<Type>,}


pub fn print_ast(program: &Program) {
    fn print_stmt(stmt: &Stmt, indent: usize) {
        let pad = "  ".repeat(indent);
        match &stmt.kind {
            StmtKind::ExprStmt(expr) => {
                println!("{}ExprStmt:", pad);
                print_expr(expr, indent + 1);
            }
            StmtKind::FunctionDecl { name, params, body, return_type } => {
                println!("{}FunctionDecl: {}", pad, name);
                println!("{}  Params:", pad);
                for (pname, ptype) in params {
                    println!("{}    {}: {:?}", pad, pname, ptype);
                }
                println!("{}  ReturnType: {:?}", pad, return_type);
                println!("{}  Body:", pad);
                print_stmt(body, indent + 2);
            }
            StmtKind::TypeDecl { name, type_params, attributes, methods, base_type, base_args } => {
                println!("{}TypeDecl: {}", pad, name);
                println!("{}  TypeParams: {:?}", pad, type_params);
                println!("{}  BaseType: {}", pad, base_type);
                println!("{}  BaseArgs:", pad);
                for arg in base_args {
                    print_expr(arg, indent + 2);
                }
                println!("{}  Attributes:", pad);
                for attr in attributes {
                    println!("{}    {}: {:?}", pad, attr.name, attr.declared_type);
                }
                println!("{}  Methods:", pad);
                for method in methods {
                    println!("{}    Method: {}", pad, method.name);
                }
            }
        }
    }

    fn print_expr(expr: &Expr, indent: usize) {
        let pad = "  ".repeat(indent);
        match &expr.kind {
            ExprKind::Number(n) => println!("{}Number: {}", pad, n),
            ExprKind::String(s) => println!("{}String: {:?}", pad, s),
            ExprKind::Boolean(b) => println!("{}Boolean: {}", pad, b),
            ExprKind::Variable(name) => println!("{}Variable: {}", pad, name),
            ExprKind::SelfExpr => println!("{}Self", pad),
            ExprKind::BaseCall { args } => {
                println!("{}BaseCall:", pad);
                for arg in args {
                    print_expr(arg, indent + 1);
                }
            }
            ExprKind::New { type_name, args } => {
                println!("{}New: {}", pad, type_name);
                for arg in args {
                    print_expr(arg, indent + 1);
                }
            }
            ExprKind::Call { function, args } => {
                println!("{}Call: {}", pad, function);
                for arg in args {
                    print_expr(arg, indent + 1);
                }
            }
            ExprKind::MethodCall { object, method, args } => {
                println!("{}MethodCall: {}", pad, method);
                print_expr(object, indent + 1);
                for arg in args {
                    print_expr(arg, indent + 1);
                }
            }
            ExprKind::GetAttr { object, attr } => {
                println!("{}GetAttr: {}", pad, attr);
                print_expr(object, indent + 1);
            }
            ExprKind::Assign { var_name, value } => {
                println!("{}Assign: {}", pad, var_name);
                print_expr(value, indent + 1);
            }
            ExprKind::SetAttr { object, attr, value } => {
                println!("{}SetAttr: {}", pad, attr);
                print_expr(object, indent + 1);
                print_expr(value, indent + 1);
            }
            ExprKind::Binary { op, left, right } => {
                println!("{}BinaryOp: {:?}", pad, op);
                print_expr(left, indent + 1);
                print_expr(right, indent + 1);
            }
            ExprKind::Unary { op, expr: subexpr } => {
                println!("{}UnaryOp: {:?}", pad, op);
                print_expr(subexpr, indent + 1);
            }
            ExprKind::As { expr: subexpr, type_name } => {
                println!("{}As: {}", pad, type_name);
                print_expr(subexpr, indent + 1);
            }
            ExprKind::Is { expr: subexpr, type_name } => {
                println!("{}Is: {}", pad, type_name);
                print_expr(subexpr, indent + 1);
            }
            ExprKind::If { condition, then_branch, else_branch } => {
                println!("{}If:", pad);
                println!("{}  Condition:", pad);
                print_expr(condition, indent + 2);
                println!("{}  Then:", pad);
                print_expr(then_branch, indent + 2);
                if let Some(else_branch) = else_branch {
                    println!("{}  Else:", pad);
                    print_expr(else_branch, indent + 2);
                }
            }
            ExprKind::Let { name, value, body, declared_type } => {
                println!("{}Let: {} {:?}", pad, name, declared_type);
                println!("{}  Value:", pad);
                print_expr(value, indent + 2);
                println!("{}  Body:", pad);
                print_expr(body, indent + 2);
            }
            ExprKind::While { condition, body } => {
                println!("{}While:", pad);
                println!("{}  Condition:", pad);
                print_expr(condition, indent + 2);
                println!("{}  Body:", pad);
                print_expr(body, indent + 2);
            }
            ExprKind::For { iterator, collection, body } => {
                println!("{}For: {}", pad, iterator);
                println!("{}  Collection:", pad);
                print_expr(collection, indent + 2);
                println!("{}  Body:", pad);
                print_expr(body, indent + 2);
            }
            ExprKind::Block { stmts } => {
                println!("{}Block:", pad);
                for stmt in stmts {
                    print_stmt(stmt, indent + 1);
                }
            }
        }
    }

    println!("Program:");
    for stmt in &program.stmts {
        print_stmt(stmt, 1);
    }
}

