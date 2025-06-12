use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::hulk_ast_nodes::hulk_assignment::Assignment;
use crate::hulk_tokens::hulk_keywords::KeywordToken;
use crate::typings::types_node::TypeNode;

#[derive(Debug, Clone, PartialEq)]
pub struct LetIn {
    pub let_token: KeywordToken,
    pub assignment: Vec<Assignment>,
    pub in_keyword: KeywordToken,
    pub body: Box<Expr>,
    pub _type: Option<TypeNode>, 
}

impl LetIn {
    pub fn new(
        let_token: KeywordToken,
        assignment: Vec<Assignment>,
        in_keyword: KeywordToken, 
        body: Box<Expr>
    ) -> Self {
        LetIn { let_token, assignment, in_keyword, body, _type: None }
    }

    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}