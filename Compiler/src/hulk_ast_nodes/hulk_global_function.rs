use crate::{codegen::{context::CodegenContext, traits::Codegen}, hulk_ast_nodes::{hulk_function_def::{FunctionBody, FunctionHeaderStruct, FunctionParams}, Expr, FunctionDef}, hulk_tokens::{token_pos, KeywordToken, TokenPos}};

#[derive(Debug, Clone)]
pub struct GlobalFunctionDef {
    pub function_token: KeywordToken,
    pub token_pos: TokenPos,
    pub function_def: FunctionDef,
}

impl GlobalFunctionDef {
    pub fn new(
        function_token: KeywordToken,
        identifier: String,
        parameters: Vec<FunctionParams>,
        body: Box<Expr>,
        token_pos: TokenPos,
    ) -> Self {
        GlobalFunctionDef {
            function_token,
            token_pos,
            function_def: FunctionDef::new_expr(identifier, parameters, String::from(""), body, token_pos),
        }
    }

    pub fn from_header_and_body(function_token: KeywordToken, header: FunctionHeaderStruct, body: FunctionBody, token_pos: TokenPos) -> Self {
        Self {
            function_token,
            token_pos,
            function_def: FunctionDef::from_header(header, body, token_pos),
        }
    }
}

impl Codegen for GlobalFunctionDef {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        self.function_def.codegen(context)
    }
}