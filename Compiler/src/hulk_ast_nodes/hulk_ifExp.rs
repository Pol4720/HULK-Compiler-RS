use crate::hulk_tokens::hulk_keywords::KeywordToken;
use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::visitor::hulk_accept::Accept;
use crate::visitor::hulk_visitor::Visitor;

#[derive(Debug, Clone, PartialEq)]
pub struct IfExpr {
    pub if_keyword: KeywordToken,
    pub condition: Box<Expr>,
    pub then_branch: Box<Expr>,
    pub else_branch: Option<ElseBranch>,
}

impl IfExpr {
    pub fn new(if_keyword: KeywordToken, condition: Box<Expr>, then_branch: Box<Expr>, else_branch: Option<ElseBranch>) -> Self {
        IfExpr { if_keyword, condition, then_branch, else_branch }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct ElseBranch {
    pub else_keyword: KeywordToken,
    pub body: Box<Expr>,
}

impl ElseBranch {
    pub fn new(else_keyword: KeywordToken, body: Box<Expr>) -> Self {
        ElseBranch { else_keyword, body }
    }
}

impl Accept for ElseBranch {
    fn accept<V: Visitor<T>, T>(&self, visitor: &mut V) -> T {
        visitor.visit_else_branch(self)
    }
}