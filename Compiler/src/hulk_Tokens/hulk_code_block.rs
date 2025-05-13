pub struct CodeBlock {
    pub statements: Vec<Statement>,
}
impl CodeBlock {
    pub fn new(statements: Vec<Statement>) -> Self {
        CodeBlock { statements }
    }
}