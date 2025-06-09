use crate::hulk_tokens::hulk_keywords::KeywordToken;
use crate::hulk_tokens::hulk_expression::Expr;
use crate::visitor::hulk_accept::Accept;
use crate::visitor::hulk_visitor::Visitor;
use crate::codegen::traits::Codegen;
use crate::codegen::context::CodegenContext;

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

impl Codegen for IfExpr {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        let cond_val = self.condition.codegen(context);

        let then_label = context.generate_label("then");
        let else_label = context.generate_label("else");
        let merge_label = context.generate_label("ifend");

        let result_reg = context.generate_temp(); // Para el resultado del `if` como expresión

        // Br condicional
        context.emit(&format!(
            "  br i1 {}, label %{}, label %{}",
            cond_val, then_label, else_label
        ));

        // Then block
        context.emit(&format!("{}:", then_label));
        let then_val = self.then_branch.codegen(context);
        context.emit(&format!("  br label %{}", merge_label));

        // Else block
        context.emit(&format!("{}:", else_label));
        let else_val = if let Some(else_branch) = &self.else_branch {
            else_branch.codegen(context)
        } else {
            // Por defecto, `0` si no hay rama else
            let tmp = context.generate_temp();
            context.emit(&format!("  {} = add i32 0, 0", tmp));
            tmp
        };
        context.emit(&format!("  br label %{}", merge_label));

        // Merge block
        context.emit(&format!("{}:", merge_label));
        context.emit(&format!(
            "  {} = phi i32 [ {}, %{} ], [ {}, %{} ]",
            result_reg,
            then_val,
            then_label,
            else_val,
            else_label
        ));

        result_reg
    }
}

impl Codegen for ElseBranch {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        self.body.codegen(context)
    }
}