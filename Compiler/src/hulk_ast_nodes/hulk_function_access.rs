
pub struct FunctionAccess {
    pub object: Box<Expr>,
    pub op: DotOperator,
    pub member: FunctionCall,
}

impl FunctionAccess {
    pub fn new(object: Expression, op: DotOperator, member: FunctionCall) -> Self {
        Self {
            object: Box::new(object),
            op,
            member,
        }
    }
}