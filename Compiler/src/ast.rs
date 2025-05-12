use crate::hulk_tokens::hulk_operators::*;
use crate::hulk_tokens::hulk_literal::*;
use crate::hulk_tokens::hulk_identifier::*;
use crate::hulk_tokens::hulk_ifExp::*;


#[derive(Debug, Clone)]
pub struct ProgramNode {
    pub instructions: Vec<Instruction>,
}

impl ProgramNode {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        ProgramNode { instructions: instructions }
    }
    pub fn with_instructions(pre: Vec<Instruction>, expr: Box<Expr>, post: Vec<Instruction>) -> Self {
        let mut instructions = pre;
        instructions.push(Instruction::Expression(expr));
        instructions.extend(post);
        ProgramNode { instructions }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
//    Class(ClassDecl),
   FunctionDef(FuncDef),
//    Protocol(ProtocolDecl),
   Expression(Box<Expr>)
}

#[derive(Debug, PartialEq, Clone)]
pub enum FuncDef{
    FunctionFullDef(FunctionDef),
    FunctionArrowDef(FunctionDef),
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<String>,
    pub body: Box<Expr>,
}




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
    BinaryOp(Box<Expr>, BinaryOperatorToken, Box<Expr>),
    UnaryOp(UnaryOperator, Box<Expr>),
    Print(Box<Expr>),
    If(IfExpr),

    FunctionCall(String, Vec<Box<Expr>>),
    Assignment(String, Box<Expr>),
    FunctionFullDef(FunctionDef),//cambiar
    FunctionArrowDef(FunctionDef),
    LetIn(Vec<Box<Expr>>, Box<Expr>),
    WhileLoop(Box<Expr>, Box<Expr>),
    IfElse(Box<Expr>, Box<Expr>, Box<Expr>),
    CodeBlock(Vec<Box<Expr>>),
}

impl Expr {
    pub fn new(kind: ExprKind) -> Self {
        Expr { kind }
    }

    pub fn eval(&self) -> Result<f64, String> {
        match &self.kind {
            ExprKind::Number(n) => Ok(n.value),
            ExprKind::Boolean(b) => Ok(if b.value { 1.0 } else { 0.0 }),
            ExprKind::BinaryOp(left, op, right) => {
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
            ExprKind::UnaryOp(op, expr) => {
                let val = expr.eval()?;
                match op {
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
            ExprKind::BinaryOp(left, op, right) => format!(
                "{}BinaryOp({:?})\n{}\n{}",
                padding,
                op,
                left.to_tree(indent + 1),
                right.to_tree(indent + 1)
            ),
            ExprKind::UnaryOp(op, expr) => format!(
                "{}UnaryOp({:?})\n{}",
                padding,
                op,
                expr.to_tree(indent + 1)
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
            ExprKind::FunctionCall(name, args) => {
                let args_str = args.iter()
                    .map(|arg| arg.to_tree(indent + 2))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("{}FunctionCall({})\n{}", padding, name, args_str)
            }
            ExprKind::Assignment(name, expr) => format!(
                "{}Assignment({})\n{}",
                padding,
                name,
                expr.to_tree(indent + 1)
            ),
            ExprKind::FunctionFullDef(func) => format!(
                "{}FunctionFullDef({})\n{}Params: {:?}\n{}Body:\n{}",
                padding,
                func.name,
                padding,
                func.params,
                padding,
                func.body.to_tree(indent + 1)
            ),
            ExprKind::FunctionArrowDef(func) => format!(
                "{}FunctionArrowDef({})\n{}Params: {:?}\n{}Body:\n{}",
                padding,
                func.name,
                padding,
                func.params,
                padding,
                func.body.to_tree(indent + 1)
            ),
            ExprKind::LetIn(assignments, body) => {
                let assigns = assignments.iter()
                    .map(|a| a.to_tree(indent + 2))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!(
                    "{}LetIn\n{}Assignments:\n{}\n{}Body:\n{}",
                    padding,
                    padding,
                    assigns,
                    padding,
                    body.to_tree(indent + 1)
                )
            }
            ExprKind::WhileLoop(cond, body) => format!(
                "{}WhileLoop\n{}Condition:\n{}\n{}Body:\n{}",
                padding,
                padding,
                cond.to_tree(indent + 1),
                padding,
                body.to_tree(indent + 1)
            ),
            ExprKind::IfElse(cond, then_branch, else_branch) => format!(
                "{}IfElse\n{}Condition:\n{}\n{}Then:\n{}\n{}Else:\n{}",
                padding,
                padding,
                cond.to_tree(indent + 1),
                padding,
                then_branch.to_tree(indent + 1),
                padding,
                else_branch.to_tree(indent + 1)
            ),
            ExprKind::CodeBlock(exprs) => {
                let exprs_str = exprs.iter()
                    .map(|e| e.to_tree(indent + 1))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("{}CodeBlock\n{}", padding, exprs_str)
            }
        }
    }
}

impl Instruction {
    pub fn to_tree(&self, indent: usize) -> String {
        match self {
            Instruction::Expression(expr) => expr.to_tree(indent),
            Instruction::FunctionDef(func_def) => format!(
                "{}FunctionDef:\n{}",
                "  ".repeat(indent),
                match func_def {
                    FuncDef::FunctionFullDef(f) => format!(
                        "{}FullDef {}({:?})\n{}",
                        "  ".repeat(indent + 1),
                        f.name,
                        f.params,
                        f.body.to_tree(indent + 2)
                    ),
                    FuncDef::FunctionArrowDef(f) => format!(
                        "{}ArrowDef {}({:?})\n{}",
                        "  ".repeat(indent + 1),
                        f.name,
                        f.params,
                        f.body.to_tree(indent + 2)
                    ),
                }
            ),
        }
    }

    pub fn eval(&self) -> Result<f64, String> {
        match self {
            Instruction::Expression(expr) => expr.eval(),
            _ => Err("Solo se pueden evaluar expresiones.".to_string()),
        }
    }
}
