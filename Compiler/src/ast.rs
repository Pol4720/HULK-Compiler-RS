use crate::hulk_tokens::hulk_operators::*;
use crate::hulk_tokens::hulk_assignment::Assignment;
use crate::hulk_tokens::hulk_literal::*;
use crate::hulk_tokens::hulk_identifier::*;
use crate::hulk_tokens::hulk_ifExp::*;
use crate::hulk_tokens::hulk_binary_expr::*;
use crate::hulk_tokens::hulk_unary_expr::*;
use crate::hulk_tokens::hulk_let_in::*;
use crate::hulk_tokens::hulk_whileloop::*;
use crate::hulk_tokens::Block;
use crate::hulk_tokens::FunctionCall;


#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Number(NumberLiteral),
    Boolean(BooleanLiteral),
    String(StringLiteral),
    Identifier(Identifier),
    BinaryOp(BinaryExpr),
    UnaryOp(UnaryExpr),
    Print(Box<Expr>),
    If(IfExpr),

    FunctionCall(FunctionCall),
    Assignment(Assignment),
    LetIn(LetIn),
    WhileLoop(WhileLoop),
    CodeBlock(Block),
}

impl Expr {
    pub fn new(kind: ExprKind) -> Self {
        Expr { kind }
    }

    pub fn eval(&self) -> Result<f64, String> {
        match &self.kind {
            ExprKind::Number(n) => Ok(n.value),
            ExprKind::Boolean(b) => Ok(if b.value { 1.0 } else { 0.0 }),
            ExprKind::BinaryOp(binary_expr) => {
                let left_val = binary_expr.left.eval()?;
                let right_val = binary_expr.right.eval()?;
                match &binary_expr.operator {
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
            ExprKind::UnaryOp(unary_expr) => {
                let val = unary_expr.operand.eval()?;
                match &unary_expr.operator {
                    UnaryOperator::Plus => Ok(val),
                    UnaryOperator::Minus => Ok(-val),
                    UnaryOperator::LogicalNot => Ok((val == 0.0) as i64 as f64),
                    _ => Err("Operador unario no soportado".to_string()),
                }
            }
            _ => Err("Expresión no soportada".to_string()),
        }
    }

    pub fn to_tree(&self, indent: usize) -> String {
        let padding = "  ".repeat(indent);
        match &self.kind {
            ExprKind::Number(n) => format!("{}NumberLiteral({})", padding, n),
            ExprKind::Boolean(b) => format!("{}BooleanLiteral({})", padding, b),
            ExprKind::String(s) => format!("{}StringLiteral(\"{}\")", padding, s),
            ExprKind::Identifier(id) => format!("{}Identifier({})", padding, id),
            ExprKind::BinaryOp(binary_expr) => format!(
                "{}BinaryOp({:?})\n{}\n{}",
                padding,
                binary_expr.operator,
                binary_expr.left.to_tree(indent + 1),
                binary_expr.right.to_tree(indent + 1)
            ),
            ExprKind::UnaryOp(unary_expr) => format!(
                "{}UnaryOp({:?})\n{}",
                padding,
                unary_expr.operator,
                unary_expr.operand.to_tree(indent + 1)
            ),
            ExprKind::Print(expr) => format!(
                "{}Print\n{}",
                padding,
                expr.to_tree(indent + 1)
            ),
            ExprKind::If(if_expr) => {
                let else_branch = if let Some(else_branch) = &if_expr.else_branch {
                    format!("\n{}Else\n{}", padding, else_branch.body.to_tree(indent + 1))
                } else {
                    String::new()
                };
                format!(
                    "{}If\n{}Condition\n{}\n{}Then\n{}{}",
                    padding,
                    padding,
                    if_expr.condition.to_tree(indent + 1),
                    padding,
                    if_expr.then_branch.to_tree(indent + 1),
                    else_branch
                )
            }
            ExprKind::FunctionCall(func_call) => {
                let args_str = func_call.arguments.iter()
                    .map(|arg| arg.to_tree(indent + 2))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("{}FunctionCall({})\n{}", padding, func_call.funct_name, args_str)
            }
            ExprKind::LetIn(let_in) => {
                let assigns = let_in.assignment.iter()
                    .map(|a| a.to_tree(indent + 2))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!(
                    "{}LetIn\n{}Assignments:\n{}\n{}Body:\n{}",
                    padding,
                    padding,
                    assigns,
                    padding,
                    let_in.body.to_tree(indent + 1)
                )
            }
            ExprKind::Assignment(assign) => format!(
                "{}Assignment({})\n{}",
                padding,
                assign.identifier,
                assign.expression.to_tree(indent + 1)
            ),

            ExprKind::WhileLoop(while_loop) => format!(
                "{}WhileLoop\n{}Condition:\n{}\n{}Body:\n{}",
                padding,
                padding,
                while_loop.condition.to_tree(indent + 1),
                padding,
                while_loop.body.to_tree(indent + 1)
            ),
            ExprKind::CodeBlock(exprs) => {
                let exprs_str = exprs.expression_list.expressions.iter()
                    .map(|e| e.to_tree(indent + 1))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("{}CodeBlock\n{}", padding, exprs_str)
            }
        }
    }
}