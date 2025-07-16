pub mod expr;
pub mod stmt;

pub use expr::*;
pub use stmt::*;

/// Base trait for all AST nodes
pub trait AstNode {
    fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T;
}

/// Visitor pattern for traversing AST nodes
pub trait Visitor<T> {
    fn visit_expr(&self, expr: &Expr) -> T;
    fn visit_stmt(&self, stmt: &Stmt) -> T;
}

/// Position information for AST nodes
#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}
