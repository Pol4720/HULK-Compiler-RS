use crate::hulk_tokens::hulk_expression::Expr;
use crate::hulk_tokens::hulk_assignment::Assignment;
use crate::hulk_tokens::hulk_keywords::KeywordToken;
use crate::visitor::hulk_accept::Accept;
use crate::visitor::hulk_visitor::Visitor;
use crate::codegen::traits::Codegen;
use crate::codegen::context::CodegenContext;

#[derive(Debug, Clone, PartialEq)]
pub struct LetIn {
    pub let_token: KeywordToken,
    pub assignment: Vec<Assignment>,
    pub in_keyword: KeywordToken,
    pub body: Box<Expr>,
}

impl LetIn {
    pub fn new(
        let_token: KeywordToken,
        assignment: Vec<Assignment>,
        in_keyword: KeywordToken, 
        body: Box<Expr>
    ) -> Self {
        LetIn { let_token, assignment, in_keyword, body }
    }
}

impl Codegen for LetIn {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // TODO: Implement codegen for LetIn
        String::new()
    }
}