use crate::codegen::context::CodegenContext;
use crate::codegen::traits::Codegen;
use crate::hulk_tokens::Block;
use crate::hulk_tokens::DestructiveAssignment;
use crate::hulk_tokens::FunctionCall;
use crate::hulk_tokens::hulk_assignment::Assignment;
use crate::hulk_tokens::hulk_binary_expr::*;
use crate::hulk_tokens::hulk_for_expr::ForExpr;
use crate::hulk_tokens::hulk_identifier::*;
use crate::hulk_tokens::hulk_ifExp::*;
use crate::hulk_tokens::hulk_let_in::*;
use crate::hulk_tokens::hulk_literal::*;
use crate::hulk_tokens::hulk_operators::*;
use crate::hulk_tokens::hulk_unary_expr::*;
use crate::hulk_tokens::hulk_whileloop::*;
use crate::visitor::hulk_accept::Accept;
use crate::visitor::hulk_visitor::Visitor;

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
}

impl Accept for Expr {
    fn accept<V: Visitor<T>, T>(&self, visitor: &mut V) -> T {
        self.kind.accept(visitor)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Number(NumberLiteral),
    Boolean(BooleanLiteral),
    String(StringLiteral),
    Identifier(Identifier),
    BinaryOp(BinaryExpr),
    UnaryOp(UnaryExpr),
    If(IfExpr),

    FunctionCall(FunctionCall),
    Assignment(Assignment),
    LetIn(LetIn),
    WhileLoop(WhileLoop),
    ForExp(ForExpr),
    CodeBlock(Block),
    DestructiveAssign(DestructiveAssignment),
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
}

impl Accept for ExprKind {
    fn accept<V: Visitor<T>, T>(&self, visitor: &mut V) -> T {
        match self {
            ExprKind::Number(node) => visitor.visit_number_literal(node),
            ExprKind::Boolean(node) => visitor.visit_boolean_literal(node),
            ExprKind::String(node) => visitor.visit_string_literal(node),
            ExprKind::Identifier(node) => visitor.visit_identifier(node),
            ExprKind::FunctionCall(node) => visitor.visit_function_call(node),
            ExprKind::WhileLoop(node) => visitor.visit_while_loop(node),
            ExprKind::CodeBlock(node) => visitor.visit_code_block(node),
            ExprKind::BinaryOp(node) => visitor.visit_binary_expr(node),
            ExprKind::ForExp(node) => visitor.visit_for_expr(node),
            ExprKind::UnaryOp(node) => visitor.visit_unary_expr(node),
            ExprKind::If(node) => visitor.visit_if_else(node),
            ExprKind::DestructiveAssign(node) => visitor.visit_destructive_assignment(node),
            ExprKind::LetIn(node) => visitor.visit_let_in(node),
            ExprKind::Assignment(node) => visitor.visit_assignment(node),
        }
    }
}

impl Codegen for Expr {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        self.kind.codegen(context)
    }
}

impl Codegen for ExprKind {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        match self {
            ExprKind::Number(n) => n.codegen(context),
            ExprKind::Boolean(b) => b.codegen(context),
            ExprKind::String(s) => s.codegen(context),
            ExprKind::Identifier(id) => id.codegen(context),
            ExprKind::BinaryOp(bin) => bin.codegen(context),
            ExprKind::UnaryOp(un) => un.codegen(context),
            ExprKind::If(ifexp) => ifexp.codegen(context),
            ExprKind::FunctionCall(call) => call.codegen(context),
            ExprKind::Assignment(assign) => assign.codegen(context),
            ExprKind::LetIn(letin) => letin.codegen(context),
            ExprKind::WhileLoop(whileloop) => whileloop.codegen(context),
            ExprKind::ForExp(forexp) => forexp.codegen(context),
            ExprKind::CodeBlock(block) => block.codegen(context),
            ExprKind::DestructiveAssign(destruct) => destruct.codegen(context),
        }
    }
}
