use crate::ast::{Expr, BinaryOp};

pub fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::BinaryOp { op, left, right } => {
            let l = eval(left);
            let r = eval(right);
            match op {
                BinaryOp::Add => l + r,
                BinaryOp::Sub => l - r,
                BinaryOp::Mul => l * r,
                BinaryOp::Div => l / r,
            }
        }
    }
}