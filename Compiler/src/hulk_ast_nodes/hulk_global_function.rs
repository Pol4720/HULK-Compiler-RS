use crate::{hulk_ast_nodes::{hulk_function_def::{FunctionBody, FunctionHeaderStruct, FunctionParams}, FunctionDef}, hulk_tokens::KeywordToken};

#[derive(Debug, Clone)]
pub struct GlobalFunctionDef {
    pub function_token: KeywordToken,
    pub function_def: FunctionDef,
}

impl GlobalFunctionDef {
    pub fn new(
        function_token: KeywordToken,
        identifier: String,
        parameters: Vec<FunctionParams>,
        body: FunctionBody,
    ) -> Self {
        GlobalFunctionDef {
            function_token,
            function_def: FunctionDef::new_expr(identifier, parameters, String::from(""), body),
        }
    }

    pub fn from_header_and_body(function_token: KeywordToken, header: FunctionHeaderStruct, body: FunctionBody) -> Self {
        Self {
            function_token,
            function_def: FunctionDef::from_header(header, body)
        }
    }
}