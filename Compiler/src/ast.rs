use crate::hulk_Tokens::hulk_operators::*;

#[derive(Debug)]
pub enum Expr {
    NumberLiteral(f64),
    BooleanLiteral(bool),
    StringLiteral(String),
    Identifier(String),
    BinaryOp(Box<Expr>, BinaryOperatorToken, Box<Expr>),
    UnaryOp(UnaryOperator, Box<Expr>),
    Print(Box<Expr>),
}

impl Expr {
    pub fn eval(&self) -> Result<f64, String> { // Cambiado a f64
        match self {
            Expr::NumberLiteral(n) => Ok(*n),
            Expr::BinaryOp(left, op, right) => {
                let left_val = left.eval()?;
                let right_val = right.eval()?;
                match op {
                    BinaryOperatorToken::Plus => Ok(left_val + right_val),
                    BinaryOperatorToken::Minus => Ok(left_val - right_val),
                    BinaryOperatorToken::Mul => Ok(left_val * right_val),
                    BinaryOperatorToken::Div => {
                        if right_val == 0.0 {
                            Err("Error: División por cero".to_string())
                        } else {
                            Ok(left_val / right_val)
                        }
                    }
                    BinaryOperatorToken::Mod => Ok(left_val % right_val),
                    BinaryOperatorToken::Pow => Ok(left_val.powf(right_val)),
                    BinaryOperatorToken::Eq => Ok((left_val == right_val) as i64 as f64),
                    BinaryOperatorToken::Neq => Ok((left_val != right_val) as i64 as f64),
                    BinaryOperatorToken::Gt => Ok((left_val > right_val) as i64 as f64),
                    BinaryOperatorToken::Gte => Ok((left_val >= right_val) as i64 as f64),
                    BinaryOperatorToken::Lt => Ok((left_val < right_val) as i64 as f64),
                    BinaryOperatorToken::Lte => Ok((left_val <= right_val) as i64 as f64),
                    _ => Err("Operador no soportado".to_string()),
                }
            }
            Expr::UnaryOp(op, expr) => {
                let val = expr.eval()?;
                match op {
                    UnaryOperator::Minus => Ok(-val),
                    UnaryOperator::LogicalNot => Ok((val == 0.0) as i64 as f64),
                    _ => Err("Operador unario no soportado".to_string()),
                }
            }
            _ => Err("Expresión no soportada".to_string()),
        }
    }

    pub fn to_tree(&self, indent: usize) -> String {
        let padding = "  ".repeat(indent); // Sangría para cada nivel
        match self {
            Expr::NumberLiteral(n) => format!("{}NumberLiteral({})", padding, n),
            Expr::BooleanLiteral(b) => format!("{}BooleanLiteral({})", padding, b),
            Expr::StringLiteral(s) => format!("{}StringLiteral(\"{}\")", padding, s),
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