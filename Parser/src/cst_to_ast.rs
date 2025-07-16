use crate::cst::DerivationNode;
use crate::ast::{Stmt, Expr, BinaryOp};

pub fn convert_to_ast(root: &DerivationNode) -> Vec<Stmt> {
    if root.symbol != "Program" {
        panic!("Expected Program as CST root");
    }

    convert_stmt_list(&root.children[0])
}

fn convert_stmt_list(node: &DerivationNode) -> Vec<Stmt> {
    let mut stmts = vec![];

    if node.symbol != "StmtList" {
        panic!("Expected StmtList");
    }

    if node.children.is_empty() {
        return stmts;
    }

    let stmt = convert_stmt(&node.children[0].children[0]); // TerminatedStmt → Stmt ; → node[0][0]
    stmts.push(stmt);

    let rest = convert_stmt_list(&node.children[1]);
    stmts.extend(rest);
    stmts
}

fn convert_stmt(node: &DerivationNode) -> Stmt {
    if node.symbol != "Stmt" {
        panic!("Expected Stmt");
    }

    let child = &node.children[0];

    match child.symbol.as_str() {
        "Expr" => {
            let expr = convert_expr(child);
            Stmt::ExprStmt(expr)
        }
        _ => unimplemented!("Unhandled stmt type: {}", child.symbol),
    }
}

fn convert_expr(node: &DerivationNode) -> Expr {
    if node.symbol != "Expr" {
        panic!("Expected Expr");
    }

    let child = &node.children[0];
    match child.symbol.as_str() {
        "NUMBER" => {
            let tok = child.token.as_ref().unwrap();
            Expr::Number(tok.lexeme.parse().unwrap())
        }
        "IDENT" => {
            let tok = child.token.as_ref().unwrap();
            Expr::Variable(tok.lexeme.clone())
        }
        _ => unimplemented!("Unhandled Expr type: {}", child.symbol),
    }
}
