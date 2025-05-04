use crate::hulk_tokens::*;

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Boolean(bool),
    Str(String),
    Identifier(String),
    BinaryOp(Box<Expr>, OperatorToken, Box<Expr>),
    UnaryOp(OperatorToken, Box<Expr>),
    Print(Box<Expr>),
}

impl Expr {
    pub fn eval(&self) -> Result<i64, String> {
        match self {
            Expr::Number(n) => Ok((*n).into()),
            Expr::BinaryOp(left, op, right) => {
                let left_val = left.eval()?;
                let right_val = right.eval()?;
                match op {
                    OperatorToken::Plus => Ok(left_val + right_val),
                    OperatorToken::Minus => Ok(left_val - right_val),
                    OperatorToken::Mul => Ok(left_val * right_val),
                    OperatorToken::Div => {
                        if right_val == 0 {
                            Err("Error: División por cero".to_string())
                        } else {
                            Ok(left_val / right_val)
                        }
                    }
                    OperatorToken::Mod => Ok(left_val % right_val),
                    OperatorToken::Pow => Ok(left_val.pow(right_val as u32)),
                    OperatorToken::Eq => Ok((left_val == right_val) as i64),
                    OperatorToken::Neq => Ok((left_val != right_val) as i64),
                    OperatorToken::Gt => Ok((left_val > right_val) as i64),
                    OperatorToken::Gte => Ok((left_val >= right_val) as i64),
                    OperatorToken::Lt => Ok((left_val < right_val) as i64),
                    OperatorToken::Lte => Ok((left_val <= right_val) as i64),
                    _ => Err("Operador no soportado".to_string()),
                }
            }
            Expr::UnaryOp(op, expr) => {
                let val = expr.eval()?;
                match op {
                    OperatorToken::Neg => Ok(-val),
                    OperatorToken::Not => Ok((val == 0) as i64),
                    _ => Err("Operador unario no soportado".to_string()),
                }
            }
            _ => Err("Expresión no soportada".to_string()),
        }
    }

    pub fn to_tree(&self, indent: usize) -> String {
        let padding = "  ".repeat(indent); // Sangría para cada nivel
        match self {
            Expr::Number(n) => format!("{}Number({})", padding, n),
            Expr::Boolean(b) => format!("{}Boolean({})", padding, b),
            Expr::Str(s) => format!("{}Str(\"{}\")", padding, s),
            Expr::Identifier(id) => format!("{}Identifier({})", padding, id),
            Expr::BinaryOp(left, op, right) => {
                let op_str = format!("{:?}", op);
                format!(
                    "{}BinaryOp({})\n{}\n{}",
                    padding,
                    op_str,
                    left.to_tree(indent + 1),
                    right.to_tree(indent + 1)
                )
            }
            Expr::UnaryOp(op, expr) => {
                let op_str = format!("{:?}", op);
                format!(
                    "{}UnaryOp({})\n{}",
                    padding,
                    op_str,
                    expr.to_tree(indent + 1)
                )
            }
            Expr::Print(expr) => format!(
                "{}Print\n{}",
                padding,
                expr.to_tree(indent + 1)
            ),
        }
    }
}