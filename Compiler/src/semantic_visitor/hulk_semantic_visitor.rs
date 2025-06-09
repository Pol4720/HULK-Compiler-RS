use std::collections::HashMap;

use crate::{hulk_ast_nodes::{BinaryExpr, Block, DestructiveAssignment,BooleanLiteral, ForExpr, FunctionCall, FunctionDef, HulkFunctionInfo, Identifier, IfExpr, LetIn, NumberLiteral, ProgramNode, StringLiteral, UnaryExpr, WhileLoop}, typings::{types_AST::{HulkTypesInfo, TypeAST}, types_node::TypeNode}, visitor::{hulk_accept::Accept, hulk_visitor::Visitor}};
use crate::hulk_tokens::hulk_operators::BinaryOperatorToken;
use crate::hulk_tokens::hulk_operators::UnaryOperator;
use super::{hulk_scope::Scope, hulk_semantic_error::SemanticError};



pub struct SemanticVisitor{
    pub current_scope: Scope,
    pub scopes: Vec<Scope>,
    pub errors: Vec<SemanticError>,
    pub type_ast: TypeAST,
}
impl SemanticVisitor {
    pub fn new() -> Self {
        Self {
            current_scope: Scope {
                variables: HashMap::new(),
                declared_functions: HashMap::new(),
            },
            scopes: Vec::new(),
            errors: Vec::new(),
            type_ast: TypeAST::new(),
        }
    }

    fn build_scope(&mut self) {
        self.scopes.push(self.current_scope.clone());
    }

    fn exit_scope(&mut self) {
        self.current_scope = self.scopes.pop().unwrap();
    }

    fn new_error(&mut self, error: SemanticError) {
        self.errors.push(error);
    }

    pub fn analyze(&mut self, node: &ProgramNode) -> Result<(), Vec<SemanticError>> {
        for instruction in &node.instructions {
            instruction.accept(self);
        }
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    pub fn get_type(&self , built_in: &HulkTypesInfo) -> TypeNode {
        self.type_ast.get_type(built_in.as_str()).unwrap().borrow().clone()
    }

}

impl Visitor<TypeNode> for SemanticVisitor {

    fn visit_for_expr(&mut self, node: &ForExpr) -> TypeNode {
        self.build_scope();
        self.current_scope.variables.insert(node.variable.clone(), self.get_type(&HulkTypesInfo::Number));
        let return_type = node.body.accept(self);
        self.exit_scope();
        return_type
    }

    fn visit_destructive_assignment(&mut self, node: &DestructiveAssignment) -> TypeNode {
        if self.current_scope.variables.contains_key(&node.identifier) {
            let new_type = node.expression.accept(self);
            self.current_scope.variables.insert(node.identifier.clone(), new_type.clone());
            new_type
        }
        else {
            self.new_error(SemanticError::UndefinedIdentifier(node.identifier.clone()));
            self.get_type(&HulkTypesInfo::Unknown)
        }
    }

    fn visit_function_def(&mut self, node: &FunctionDef) -> TypeNode {
        self.build_scope();
        let func_return= node.return_type.clone();
        let mut arg_types: Vec<TypeNode> = vec![];
        for param in &node.params { 
            if let Some(param_type) = self.type_ast.get_type(&param.signature) {
                self.current_scope.variables.insert(param.name.clone(), param_type.borrow().clone());
                arg_types.push(param_type.borrow().clone());
            }
            else {
                self.new_error(SemanticError::UndefinedType(param.signature.clone()));
                self.current_scope.variables.insert(param.name.clone(), self.get_type(&HulkTypesInfo::Unknown));
                arg_types.push(self.get_type(&HulkTypesInfo::Unknown));
            }
        }
        if self.current_scope.declared_functions.contains_key(&node.name) {
            self.new_error(SemanticError::RedefinitionOfFunction(node.name.clone()));
        } else {
            self.current_scope.declared_functions.insert(
                node.name.clone(),
                HulkFunctionInfo::new(
                    node.name.clone(),
                    arg_types.clone(),
                    self.type_ast.get_type(&node.return_type.clone()).unwrap().borrow().clone()
                )
            );
        }
        let body_type = node.body.accept(self);
        let mut return_type_node = self.get_type(&HulkTypesInfo::Unknown);
        if let Some(func_type_rc) = self.type_ast.get_type(&func_return.clone()) {
            let func_type_borrow = func_type_rc.borrow();
            let func_type_name = func_type_borrow.type_name.as_str();
            let body_type_name = body_type.type_name.as_str();
            if !self.type_ast.is_ancestor(func_type_name, body_type_name) {
                self.new_error(SemanticError::InvalidFunctionReturn(body_type, func_type_rc.borrow().clone(), node.name.clone()));
            }
            return_type_node = func_type_rc.borrow().clone();
        } else {
            self.new_error(SemanticError::UndefinedType(func_return.clone()));
        }
        self.exit_scope();
        self.current_scope.declared_functions.insert(node.name.clone(), HulkFunctionInfo::new(node.name.clone(), arg_types, return_type_node.clone()));
        return_type_node
    }

    fn visit_number_literal(&mut self, _node: &NumberLiteral) -> TypeNode {
        
        self.get_type(&HulkTypesInfo::Number)
    }

    fn visit_boolean_literal(&mut self, _node: &BooleanLiteral) -> TypeNode {
        self.get_type(&HulkTypesInfo::Boolean)
    }

    fn visit_string_literal(&mut self, _node: &StringLiteral) -> TypeNode {
        self.get_type(&HulkTypesInfo::String)
    }

    fn visit_identifier(&mut self, node: &Identifier) -> TypeNode {
        if let Some(return_type) = self.current_scope.variables.get(&node.id) {
            return_type.clone()
        } else {
            self.new_error(SemanticError::UndefinedIdentifier(node.id.clone()));
            self.get_type(&HulkTypesInfo::Unknown)
        }
    }

    fn visit_function_call(&mut self, node: &FunctionCall) -> TypeNode {
        if let Some(func_info) = self.current_scope.declared_functions.get(&node.funct_name) {
            let arguments_types = func_info.argument_types.clone();
            let func_name = func_info.function_name.clone();
            let func_type = func_info.return_type.clone();
            if node.arguments.len() != arguments_types.len() {
                self.new_error(SemanticError::InvalidArgumentsCount(node.arguments.len(), arguments_types.len(), node.funct_name.clone()));
            }
            else {
                for (index, arg) in node.arguments.iter().enumerate() {
                    let arg_type = arg.accept(self);
                    if arg_type != arguments_types[index] {
                        self.new_error(SemanticError::InvalidTypeArgument(arg_type, arguments_types[index].clone(), index, func_name.clone()));
                    }
                }
            }
            func_type
        } else {
            self.new_error(SemanticError::UndeclaredFunction(node.funct_name.clone()));
            self.get_type(&HulkTypesInfo::Unknown)
        }
    }

    fn visit_while_loop(&mut self, node: &WhileLoop) -> TypeNode {
        let condition_type = node.condition.accept(self);
        if condition_type != self.get_type(&HulkTypesInfo::Boolean) {
            self.new_error(SemanticError::InvalidConditionType(condition_type));
        }
        let body_type = node.body.accept(self);
        return body_type;
    }

    fn visit_code_block(&mut self, node: &Block) -> TypeNode {
        self.build_scope();
        let mut last_type = self.get_type(&HulkTypesInfo::Unknown);
        for expr in node.expression_list.expressions.iter() {
            last_type = expr.accept(self);
        }
        self.exit_scope();
        last_type
    }

    fn visit_binary_expr(&mut self, node: &BinaryExpr) -> TypeNode {
        let left_type = node.left.accept(self);
        let right_type = node.right.accept(self);
        
        match node.operator {
            BinaryOperatorToken::Plus | 
            BinaryOperatorToken::Minus |
            BinaryOperatorToken::Mul |
            BinaryOperatorToken::Div |
            BinaryOperatorToken::Mod |
            BinaryOperatorToken::Pow => {
                if left_type == self.get_type(&HulkTypesInfo::Number) && right_type == self.get_type(&HulkTypesInfo::Number) {
                    self.get_type(&HulkTypesInfo::Number)
                } else {
                    self.new_error(SemanticError::InvalidBinaryOperation(left_type, right_type,node.operator.clone()));
                    self.get_type(&HulkTypesInfo::Unknown)
                }
            },
            BinaryOperatorToken::Gt |
            BinaryOperatorToken::Gte |
            BinaryOperatorToken::Lt |
            BinaryOperatorToken::Lte |
            BinaryOperatorToken::Eq |
            BinaryOperatorToken::Neq |
            BinaryOperatorToken::Neg => {
                if left_type == self.get_type(&HulkTypesInfo::Number) && right_type == self.get_type(&HulkTypesInfo::Number) {
                    self.get_type(&HulkTypesInfo::Boolean)
                } else {
                    self.new_error(SemanticError::InvalidBinaryOperation(left_type, right_type,node.operator.clone()));
                    self.get_type(&HulkTypesInfo::Unknown)
                }
            }
            BinaryOperatorToken::Concat => {
                if left_type == self.get_type(&HulkTypesInfo::String) || left_type == self.get_type(&HulkTypesInfo::Boolean) || left_type == self.get_type(&HulkTypesInfo::Number) && right_type == self.get_type(&HulkTypesInfo::String) || right_type == self.get_type(&HulkTypesInfo::Boolean) || right_type == self.get_type(&HulkTypesInfo::Number) {
                    self.get_type(&HulkTypesInfo::String)
                } else {
                    self.new_error(SemanticError::InvalidBinaryOperation(left_type, right_type,node.operator.clone()));
                    self.get_type(&HulkTypesInfo::Unknown)
                }

            },
            BinaryOperatorToken::And |
            BinaryOperatorToken::Or => {
                if left_type == self.get_type(&HulkTypesInfo::Boolean) && right_type == self.get_type(&HulkTypesInfo::Boolean) {
                    self.get_type(&HulkTypesInfo::Boolean)
                } else {
                    self.new_error(SemanticError::InvalidBinaryOperation(left_type, right_type,node.operator.clone()));
                    self.get_type(&HulkTypesInfo::Unknown)
                }
            },
            _ => {
                self.new_error(SemanticError::UnknownError(format!("Operator ( {:?} ) not supported in binary operation",node.operator)));
                self.get_type(&HulkTypesInfo::Unknown)
            }
        }
    }

    fn visit_unary_expr(&mut self, node: &UnaryExpr) -> TypeNode {
        let operand_type = node.operand.accept(self);
        
        match node.operator {
            UnaryOperator::Minus => {
                if operand_type == self.get_type(&HulkTypesInfo::Number) {
                    self.get_type(&HulkTypesInfo::Number)
                } else {
                    self.new_error(SemanticError::InvalidUnaryOperation(operand_type, node.operator.clone()));
                    self.get_type(&HulkTypesInfo::Unknown)
                }
            },
            UnaryOperator::LogicalNot => {
                if operand_type == self.get_type(&HulkTypesInfo::Boolean) {
                    self.get_type(&HulkTypesInfo::Boolean)
                } else {
                    self.new_error(SemanticError::InvalidUnaryOperation(operand_type, node.operator.clone()));
                    self.get_type(&HulkTypesInfo::Unknown)
                }
            },
            _ => {
                self.new_error(SemanticError::UnknownError(format!("Operator ( {:?} ) not supported in unary operation",node.operator.clone())));
                self.get_type(&HulkTypesInfo::Unknown)
            }
        }
    }

    fn visit_if_else(&mut self, node: &IfExpr) -> TypeNode {
        let condition_type = node.condition.accept(self);
        if condition_type != self.get_type(&HulkTypesInfo::Boolean) {
            self.new_error(SemanticError::InvalidConditionType(condition_type));
        }
        let then_type = node.then_branch.accept(self);
        let else_type = if let Some(else_branch) = &node.else_branch {
            else_branch.accept(self)
        } else {
            self.get_type(&HulkTypesInfo::Unknown)
        };
        
        if then_type != else_type {
            let lca_option = self.type_ast.find_lca(then_type.type_name.as_str(), else_type.type_name.as_str());
            if let Some(lca_rc) = lca_option {
                let lca_ref = lca_rc.borrow();
                if lca_ref.type_name == "Unknown" {
                    self.new_error(SemanticError::UnknownError("Incompatible types in if-else branches".to_string()));
                }
                lca_ref.clone()
            } else {
                self.new_error(SemanticError::UnknownError("Incompatible types in if-else branches".to_string()));
                self.get_type(&HulkTypesInfo::Unknown)
            }
        } else {
            then_type
        }
    }
    
    fn visit_let_in(&mut self, node: &LetIn) -> TypeNode {
        self.build_scope();
        for assig in node.assignment.iter() {
            let expr_type = assig.expression.accept(self);
            if let Some(_) = self.current_scope.variables.get(&assig.identifier.id) {
                self.new_error(SemanticError::RedefinitionOfVariable(assig.identifier.id.clone()));
            } else {
                self.current_scope.variables.insert(assig.identifier.id.clone(), expr_type);
            }
        }
        let return_type = node.body.accept(self);
        self.exit_scope();
        return_type
    }
    
    fn visit_program(&mut self, node: &ProgramNode) -> TypeNode {
        let mut last_type = self.get_type(&HulkTypesInfo::Unknown);
        for instruction in &node.instructions {
            last_type = instruction.accept(self);
        }
        last_type
    }
    
    fn visit_expression_list(&mut self, node: &crate::hulk_ast_nodes::ExpressionList) -> TypeNode {
        let mut last_type = self.get_type(&HulkTypesInfo::Unknown);
        for expr in &**node.expressions {
            last_type = expr.accept(self);
        }
        last_type
    }
    
    fn visit_assignment(&mut self, node: &crate::hulk_ast_nodes::Assignment) -> TypeNode {
        let expr_type = node.expression.accept(self);
        if self.current_scope.variables.contains_key(&node.identifier.id) {
            self.new_error(SemanticError::RedefinitionOfVariable(node.identifier.id.clone()));
        } else {
            self.current_scope.variables.insert(node.identifier.id.clone(), expr_type.clone());
        }
        expr_type
    }
    
    fn visit_else_branch(&mut self, node: &crate::hulk_ast_nodes::ElseBranch) -> TypeNode {
        node.body.accept(self)
    }
}