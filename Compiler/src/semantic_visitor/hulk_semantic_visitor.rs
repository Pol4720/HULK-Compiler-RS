use std::collections::HashMap;

use crate::{hulk_ast_nodes::{hulk_expression::ExprKind, BinaryExpr, Block, BooleanLiteral, DestructiveAssignment, Expr, ForExpr, FunctionAccess, FunctionCall, FunctionDef, HulkFunctionInfo, HulkTypeNode, Identifier, IfExpr, Instruction, LetIn, MemberAccess, NewTypeInstance, NumberLiteral, ProgramNode, StringLiteral, UnaryExpr, WhileLoop}, typings::{types_AST::{HulkTypesInfo, TypeAST}, types_node::TypeNode}, visitor::{hulk_accept::Accept, hulk_visitor::Visitor}};
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
                declared_types: HashMap::new(),
                current_type: None,
                current_function: None,
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

    pub fn analyze(&mut self, node: &mut ProgramNode) -> Result<(), Vec<SemanticError>> {
        self.get_types_definitions(node);
        self.build_types();
        for instruction in node.instructions.iter_mut() {
            instruction.accept(self);
        }
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    pub fn get_type(&self , built_in: &HulkTypesInfo) -> TypeNode {
        self.type_ast.get_type(built_in.as_str()).unwrap()
    }
    pub fn get_types_definitions(&mut self, node: &ProgramNode) {
        for instruction in &node.instructions {
            if let Instruction::TypeDef(type_def) = instruction {
                if self.type_ast.get_type(&type_def.type_name).is_some() || self.current_scope.declared_types.contains_key(&type_def.type_name) {
                    self.new_error(SemanticError::RedefinitionOfType(type_def.type_name.clone()));
                } else {
                    if let Some(parent_type) = &type_def.parent {
                        if type_def.type_name == *parent_type {
                            self.new_error(SemanticError::UnknownError("Type cannot inherit from itself".to_string()));
                        }
                    }
                    self.current_scope.declared_types.insert(type_def.type_name.clone(), type_def.clone());
                }
            }
        }
    }

    pub fn build_types(&mut self){
                for (type_name, type_def) in self.current_scope.declared_types.clone() {
                    let mut methods = HashMap::new();
                    for (method_name, method_def) in &type_def.methods {
                        methods.insert(method_name.clone(), Box::new(method_def.clone()));
                    }
                    self.type_ast.add_type(
                        type_name.clone(),
                        type_def.parameters.clone(),
                        None,
                        HashMap::new(),
                        methods,
                    );
                }
        for (tye_name, type_def) in self.current_scope.declared_types.clone() {
            if let Some(parent_type) = type_def.parent {
                let parent_type_name = parent_type.clone();
                let child_type_name = tye_name.clone();
                if !self.type_ast.nodes.contains_key(&parent_type_name) {
                    self.new_error(SemanticError::UndefinedType(parent_type_name));
                } else {
                    let parent_params;
                    let parent_node = self.type_ast.nodes.get_mut(&parent_type_name).unwrap();
                    parent_node.add_child(child_type_name.clone());
                    parent_params = parent_node.params.clone();
                    let type_node = self.type_ast.nodes.get_mut(&child_type_name).unwrap();
                    type_node.set_parent(parent_type_name.clone()); 
                    if type_node.params.len() == 0 {
                        type_node.params = parent_params;
                    } else if type_def.parent_args.len() != parent_params.len(){
                        self.new_error(SemanticError::UnknownError(format!("Error: On definition of type {} parameters, type {} must receive {} arguments , but {} were provided", child_type_name, parent_type_name, parent_params.len(), type_def.parent_args.len())));
                    }
                }
            }
        }
        if let Some(cycle_node) = self.type_ast.inheritance_cicle() {
            self.new_error(SemanticError::CicleDetected(cycle_node)); //TODO Get node already visited
        }
    }
    
    fn base_funct_treatment(&mut self, node: &mut FunctionCall) -> Option<TypeNode> {
        if let Some(current_type) = self.current_scope.current_type.clone() {
            if let Some(type_node) = self.type_ast.get_type(&current_type){
                if let Some(parent) = type_node.parent {
                    if let Some(current_function) = self.current_scope.current_function.clone() {
                        println!("camde;a");
                        if let Some (func) = self.type_ast.find_method(parent, current_function.clone()){
                            if node.arguments.len() != func.params.len() {
                                self.new_error(SemanticError::InvalidArgumentsCount(node.arguments.len(), func.params.len(), current_function.clone()));
                            } else {
                                for (index, arg) in node.arguments.iter_mut().enumerate() {
                                    let arg_type = arg.accept(self);
                                    if arg_type.type_name != func.params[index].signature {
                                        self.new_error(SemanticError::InvalidTypeArgument("function".to_string() ,arg_type.type_name, func.params[index].signature.clone(), index, func.name.clone()));
                                    }
                                }
                            }
                            if let Some(func_type_node) = self.type_ast.get_type(&func.return_type){
                                node.set_expression_type(func_type_node.clone());
                                return Some(func_type_node)
                            } else {
                                self.new_error(SemanticError::UndefinedType(func.return_type.clone()));
                                return Some(self.get_type(&HulkTypesInfo::Unknown))
                            }
                        }
                    }
                }
            }
        }
        None
    }

}

impl Visitor<TypeNode> for SemanticVisitor {
    fn visit_for_expr(&mut self, node: &mut ForExpr) -> TypeNode {
        self.build_scope();
        self.current_scope
            .variables
            .insert(node.variable.clone(), self.get_type(&HulkTypesInfo::Number));
        let return_type = node.body.accept(self);
        self.exit_scope();
        node.set_expression_type(return_type.clone());
        return_type
    }

    fn visit_destructive_assignment(&mut self, node: &mut DestructiveAssignment) -> TypeNode {
        match  *node.identifier.clone()  {
            Expr {
                kind: ExprKind::Identifier(ref id),
                ..
            } => {
                if self.current_scope.variables.contains_key(&id.id) {
                    let new_type = node.expression.accept(self);
                    self.current_scope.variables.insert(id.id.clone(), new_type.clone());
                    node.set_expression_type(new_type.clone());
                    new_type
                }
                else {
                    self.new_error(SemanticError::UndefinedIdentifier(id.id.clone()));
                    self.get_type(&HulkTypesInfo::Unknown)
                }
            },

            Expr { 
                kind: ExprKind:: MemberAccess(ref mut access_node),
                ..
            } => {
                let mut object_type = access_node.object.accept(self);
                if let Some(_property_type) = object_type.variables.get_mut(&access_node.member.id) {
                    let new_type = node.expression.accept(self);
                    object_type.variables.insert(access_node.member.id.clone(), Box::new(new_type.type_name.clone()));
                    node.set_expression_type(new_type.clone());
                    new_type
                } else {
                    self.new_error(SemanticError::InvalidTypePropertyAccess(object_type.type_name.clone(), access_node.member.id.clone()));
                    self.get_type(&HulkTypesInfo::Unknown)
                }
            }
            _ => {
                self.new_error(SemanticError::UnknownError("Destructive assignment can only be done to an identifier or type property access".to_string()));
                self.get_type(&HulkTypesInfo::Unknown)
            }
        }
    }

    //revisar metodo

    fn visit_function_def(&mut self, node: &mut FunctionDef) -> TypeNode {
        self.build_scope();
        self.current_scope.current_function = Some(node.name.clone());
        if let Some(function) = self.current_scope.declared_functions.get(&node.name){
            for param in &function.argument_types {
                self.current_scope.variables.insert(param.type_name.clone(), param.clone());
            }
        } else if let Some(current_type) = self.current_scope.current_type.clone() {
            if let Some(type_node) = self.type_ast.get_type(&current_type){
                if let Some(function) = type_node.methods.get(&node.name) {
                    for param in &function.params.clone() {
                        if let Some(type_node) = self.type_ast.get_type(&param.signature) {
                            self.current_scope.variables.insert(param.name.clone(), type_node.clone());
                        } else {
                            self.new_error(SemanticError::UndefinedType(param.signature.clone()));
                            self.current_scope.variables.insert(param.name.clone(), self.get_type(&HulkTypesInfo::Unknown));
                        }
                    }
                } else {
                    self.new_error(SemanticError::UndeclaredFunction(node.name.clone()));
                }
            } else {
                self.new_error(SemanticError::UndefinedType(current_type));
            }

        } else {
            self.new_error(SemanticError::UndeclaredFunction(node.name.clone()));
        }
        let body_type = node.body.accept(self);
        let mut return_type_node = self.get_type(&HulkTypesInfo::Unknown);
        if let Some(func_type) = self.type_ast.get_type(&node.return_type.clone()) {
            if ! self.type_ast.is_ancestor(&func_type, &body_type) {
                self.new_error(SemanticError::InvalidFunctionReturn(body_type, func_type.clone(), node.name.clone()));
            }
            return_type_node = func_type;
        } else {
            self.new_error(SemanticError::UndefinedType(node.return_type.clone()));
        }
        self.exit_scope();
        node.set_expression_type(return_type_node.clone());
        return_type_node
    }

    fn visit_number_literal(&mut self, node: &mut NumberLiteral) -> TypeNode {
        node.set_expression_type(self.get_type(&HulkTypesInfo::Number));
        self.get_type(&HulkTypesInfo::Number)
    }

    fn visit_boolean_literal(&mut self, node: &mut BooleanLiteral) -> TypeNode {
        node.set_expression_type(self.get_type(&HulkTypesInfo::Boolean));
        self.get_type(&HulkTypesInfo::Boolean)
    }

    fn visit_string_literal(&mut self, node: &mut StringLiteral) -> TypeNode {
        node.set_expression_type(self.get_type(&HulkTypesInfo::String));
        self.get_type(&HulkTypesInfo::String)
    }

    fn visit_identifier(&mut self, node: &mut Identifier) -> TypeNode {
       if let Some(return_type) = self.current_scope.variables.get(&node.id) {
            if let Some (node_type) = self.type_ast.get_type(&return_type.type_name) {
                node.set_expression_type(node_type.clone());
                node_type.clone()
            }
            else {
                self.new_error(SemanticError::UndefinedType(return_type.type_name.clone()));
                self.get_type(&HulkTypesInfo::Unknown)
            }
        } else if node.id == "self" {
            if let Some(current_type) = &self.current_scope.current_type {
                if let Some(type_node) = self.type_ast.get_type(current_type) {
                    node.set_expression_type(type_node.clone());
                    type_node.clone()
                } else {
                    self.new_error(SemanticError::UndefinedType(current_type.clone()));
                    self.get_type(&HulkTypesInfo::Unknown)
                }
            } else {
                self.new_error(SemanticError::UndefinedIdentifier(node.id.clone()));
                self.get_type(&HulkTypesInfo::Unknown)
            }
        } else {
            self.new_error(SemanticError::UndefinedIdentifier(node.id.clone()));
            self.get_type(&HulkTypesInfo::Unknown)
        }
    }

    fn visit_function_call(&mut self, node: &mut FunctionCall) -> TypeNode {
        if self.current_scope.current_type.is_some() && node.funct_name == "base" {
            if let Some(value) = self.base_funct_treatment(node) {
                return value;
            }
        } 
        if let Some(func_info) = self.current_scope.declared_functions.get(&node.funct_name) {
            let arguments_types = func_info.argument_types.clone();
            let func_name = func_info.function_name.clone();
            let func_type = func_info.return_type.clone();
            if node.arguments.len() != arguments_types.len() {
                self.new_error(SemanticError::InvalidArgumentsCount(node.arguments.len(), arguments_types.len(), node.funct_name.clone()));
            }
            else {
                for (index, arg) in node.arguments.iter_mut().enumerate() {
                    let arg_type = arg.accept(self);
                    if arg_type.type_name != arguments_types[index].type_name {
                        self.new_error(SemanticError::InvalidTypeArgument("function".to_string() ,arg_type.type_name, arguments_types[index].type_name.clone(), index, func_name.clone()));
                    }
                }
            }
            if let Some(func_type_node) = self.type_ast.get_type(&func_type.type_name) {
                node.set_expression_type(func_type_node.clone());
                func_type_node
            } else {
                self.new_error(SemanticError::UndefinedType(func_type.type_name.clone()));
                self.get_type(&HulkTypesInfo::Unknown)
            }    
        } else {
            self.new_error(SemanticError::UndeclaredFunction(node.funct_name.clone()));
            self.get_type(&HulkTypesInfo::Unknown)
        }
    }

    fn visit_while_loop(&mut self, node: &mut WhileLoop) -> TypeNode {
        let condition_type = node.condition.accept(self);
        if condition_type != self.get_type(&HulkTypesInfo::Boolean) {
            self.new_error(SemanticError::InvalidConditionType(condition_type));
        }
        let body_type = node.body.accept(self);
        node.set_expression_type(body_type.clone());
        body_type
    }

    fn visit_code_block(&mut self, node: &mut Block) -> TypeNode {
        self.build_scope();
        let mut last_type = self.get_type(&HulkTypesInfo::Unknown);
        for expr in node.expression_list.expressions.iter_mut() {
            last_type = expr.accept(self);
        }
        self.exit_scope();
        node.set_expression_type(last_type.clone());
        last_type
    }

    fn visit_binary_expr(&mut self, node: &mut BinaryExpr) -> TypeNode {
        let left_type = node.left.accept(self);
        let right_type = node.right.accept(self);

        match node.operator {
            BinaryOperatorToken::Plus
            | BinaryOperatorToken::Minus
            | BinaryOperatorToken::Mul
            | BinaryOperatorToken::Div
            | BinaryOperatorToken::Mod
            | BinaryOperatorToken::Pow => {
                if left_type == self.get_type(&HulkTypesInfo::Number)
                    && right_type == self.get_type(&HulkTypesInfo::Number)
                {
                    node.set_expression_type(self.get_type(&HulkTypesInfo::Number));
                    self.get_type(&HulkTypesInfo::Number)
                } else {
                    self.new_error(SemanticError::InvalidBinaryOperation(
                        left_type,
                        right_type,
                        node.operator.clone(),
                    ));
                    self.get_type(&HulkTypesInfo::Unknown)
                }
            }
            BinaryOperatorToken::Gt
            | BinaryOperatorToken::Gte
            | BinaryOperatorToken::Lt
            | BinaryOperatorToken::Lte
            | BinaryOperatorToken::Eq
            | BinaryOperatorToken::Neq
            | BinaryOperatorToken::Neg => {
                if left_type == self.get_type(&HulkTypesInfo::Number)
                    && right_type == self.get_type(&HulkTypesInfo::Number)
                {
                    node.set_expression_type(self.get_type(&HulkTypesInfo::Boolean));
                    self.get_type(&HulkTypesInfo::Boolean)
                } else {
                    self.new_error(SemanticError::InvalidBinaryOperation(
                        left_type,
                        right_type,
                        node.operator.clone(),
                    ));
                    self.get_type(&HulkTypesInfo::Unknown)
                }
            }
            BinaryOperatorToken::Concat => {
                if (left_type == self.get_type(&HulkTypesInfo::String)
                    || left_type == self.get_type(&HulkTypesInfo::Boolean)
                    || left_type == self.get_type(&HulkTypesInfo::Number))
                    && (right_type == self.get_type(&HulkTypesInfo::String)
                        || right_type == self.get_type(&HulkTypesInfo::Boolean)
                        || right_type == self.get_type(&HulkTypesInfo::Number))
                {
                    node.set_expression_type(self.get_type(&HulkTypesInfo::String));
                    self.get_type(&HulkTypesInfo::String)
                } else {
                    self.new_error(SemanticError::InvalidBinaryOperation(
                        left_type,
                        right_type,
                        node.operator.clone(),
                    ));
                    self.get_type(&HulkTypesInfo::Unknown)
                }
            }
            BinaryOperatorToken::And | BinaryOperatorToken::Or => {
                if left_type == self.get_type(&HulkTypesInfo::Boolean)
                    && right_type == self.get_type(&HulkTypesInfo::Boolean)
                {
                    node.set_expression_type(self.get_type(&HulkTypesInfo::Boolean));
                    self.get_type(&HulkTypesInfo::Boolean)
                } else {
                    self.new_error(SemanticError::InvalidBinaryOperation(
                        left_type,
                        right_type,
                        node.operator.clone(),
                    ));
                    self.get_type(&HulkTypesInfo::Unknown)
                }
            }
            _ => {
                self.new_error(SemanticError::UnknownError(format!(
                    "Operator ( {:?} ) not supported in binary operation",
                    node.operator
                )));
                self.get_type(&HulkTypesInfo::Unknown)
            }
        }
    }

    fn visit_unary_expr(&mut self, node: &mut UnaryExpr) -> TypeNode {
        let operand_type = node.operand.accept(self);

        match node.operator {
            UnaryOperator::Minus => {
                if operand_type == self.get_type(&HulkTypesInfo::Number) {
                    node.set_expression_type(self.get_type(&HulkTypesInfo::Number));
                    self.get_type(&HulkTypesInfo::Number)
                } else {
                    self.new_error(SemanticError::InvalidUnaryOperation(
                        operand_type,
                        node.operator.clone(),
                    ));
                    self.get_type(&HulkTypesInfo::Unknown)
                }
            }
            UnaryOperator::LogicalNot => {
                if operand_type == self.get_type(&HulkTypesInfo::Boolean) {
                    node.set_expression_type(self.get_type(&HulkTypesInfo::Boolean));
                    self.get_type(&HulkTypesInfo::Boolean)
                } else {
                    self.new_error(SemanticError::InvalidUnaryOperation(
                        operand_type,
                        node.operator.clone(),
                    ));
                    self.get_type(&HulkTypesInfo::Unknown)
                }
            }
            _ => {
                self.new_error(SemanticError::UnknownError(format!(
                    "Operator ( {:?} ) not supported in unary operation",
                    node.operator.clone()
                )));
                self.get_type(&HulkTypesInfo::Unknown)
            }
        }
    }

    fn visit_if_else(&mut self, node: &mut IfExpr) -> TypeNode {
        let condition_type = node.condition.accept(self);
        if condition_type != self.get_type(&HulkTypesInfo::Boolean) {
            self.new_error(SemanticError::InvalidConditionType(condition_type));
        }
        let then_type = node.then_branch.accept(self);
        let else_type = if let Some(else_branch) = &mut node.else_branch {
            else_branch.accept(self)
        } else {
            self.get_type(&HulkTypesInfo::Unknown)
        };
        
        if then_type != else_type {
            let lca = self.type_ast.find_lca(&then_type, &else_type);
            if lca.type_name == "Unknown" {
                self.new_error(SemanticError::UnknownError("Incompatible types in if-else branches".to_string()));
            }
            node.set_expression_type(lca.clone());
            lca
        } else {
            node.set_expression_type(then_type.clone());
            then_type
        }
    }

    fn visit_let_in(&mut self, node: &mut LetIn) -> TypeNode {
        self.build_scope();
        for assig in node.assignment.iter_mut() {
            let expr_type = assig.expression.accept(self);
            if let Some(_) = self.current_scope.variables.get(&assig.identifier.id) {
                self.new_error(SemanticError::RedefinitionOfVariable(
                    assig.identifier.id.clone(),
                ));
            } else {
                self.current_scope
                    .variables
                    .insert(assig.identifier.id.clone(), expr_type);
            }
        }
        let return_type = node.body.accept(self);
        self.exit_scope();
        node.set_expression_type(return_type.clone());
        return_type
    }

    fn visit_program(&mut self, node: &mut ProgramNode) -> TypeNode {
        let mut last_type = self.get_type(&HulkTypesInfo::Unknown);
        for instruction in &mut node.instructions {
            last_type = instruction.accept(self);
        }
        last_type
    }

    fn visit_expression_list(
        &mut self,
        node: &mut crate::hulk_ast_nodes::ExpressionList,
    ) -> TypeNode {
        let mut last_type = self.get_type(&HulkTypesInfo::Unknown);
        for expr in &mut **node.expressions {
            last_type = expr.accept(self);
        }
        last_type
    }

    fn visit_assignment(
        &mut self,
        node: &mut crate::hulk_ast_nodes::Assignment,
    ) -> TypeNode {
        let expr_type = node.expression.accept(self);
        if self.current_scope.variables.contains_key(&node.identifier.id) {
            self.new_error(SemanticError::RedefinitionOfVariable(
                node.identifier.id.clone(),
            ));
        } else {
            self.current_scope
                .variables
                .insert(node.identifier.id.clone(), expr_type.clone());
        }
        node.set_expression_type(expr_type.clone());
        expr_type
    }

    fn visit_else_branch(
        &mut self,
        node: &mut crate::hulk_ast_nodes::ElseBranch,
    ) -> TypeNode {
        node.body.accept(self)
    }

    fn visit_type_def(&mut self, node: &mut HulkTypeNode) -> TypeNode {
        self.build_scope();
        self.current_scope.current_type = Some(node.type_name.clone());
        for param in &node.parameters {
            if self.current_scope.variables.contains_key(&param.name) {
            self.new_error(SemanticError::ParamNameAlreadyExist(param.name.clone(), node.type_name.clone(), "type".to_string()));
            }
            if let Some(type_node) = self.type_ast.get_type(&param.signature) {
            self.current_scope.variables.insert(param.name.clone(), type_node.clone());
            } else {
            self.new_error(SemanticError::UndefinedType(param.signature.clone()));
            self.current_scope.variables.insert(param.name.clone(), self.get_type(&HulkTypesInfo::Unknown));
            }
        }
        if let Some(parent_name) = &node.parent {
            if let Some(parent_node) = self.type_ast.get_type(&parent_name) {
            if parent_node.params.len() != node.parent_args.len() && !node.parent_args.is_empty() {
                self.new_error(SemanticError::InvalidTypeArgumentCount(node.parent_args.len(), parent_node.params.len(), parent_node.type_name.clone()));
            } else {
                for (index, arg) in node.parent_args.iter_mut().enumerate() {
                let arg_type = arg.accept(self);
                if arg_type.type_name != parent_node.params[index].signature {
                    self.new_error(SemanticError::InvalidTypeArgument(
                    "types".to_string(),
                    arg_type.type_name,
                    parent_node.params[index].name.clone(),
                    index,
                    node.type_name.clone(),
                    ));
                }
                }
            }
            } else {
            self.new_error(SemanticError::UndefinedType(parent_name.clone()));
            }
        }
        for prop in node.attributes.values_mut() {
            let prop_type = prop.init_expr.accept(self);
            if let Some(type_node) = self.type_ast.nodes.get_mut(&node.type_name) {
                type_node.add_variable(prop.name.to_string().clone(), Box::new(prop_type.type_name));
            }
        }
        for method in node.methods.values_mut() {
            self.visit_function_def(method);
        }
        self.exit_scope();
        let return_type = self.type_ast.get_type(&node.type_name).unwrap();
        node.set_expression_type(return_type.clone());
        return_type
        
    }

    fn visit_new_type_instance(&mut self, node: &mut NewTypeInstance) -> TypeNode {
        if let Some(type_node) = self.type_ast.get_type(&node.type_name.id.clone()) {
            if type_node.params.len() != node.arguments.len() {
                self.new_error(SemanticError::InvalidTypeArgumentCount(node.arguments.len(), type_node.params.len(), node.type_name.id.clone()));
                self.get_type(&HulkTypesInfo::Unknown)
            } else {
                for (index, arg ) in node.arguments.iter_mut().enumerate() {
                    let arg_type = arg.accept(self);
                    if arg_type.type_name != type_node.params[index].signature {
                        self.new_error(SemanticError::InvalidTypeArgument("types".to_string(), arg_type.type_name, type_node.params[index].signature.clone(), index, node.type_name.id.clone()));
                    }
                }
                node.set_expression_type(type_node.clone());
                type_node
            }
        } else {
            self.new_error(SemanticError::UndefinedType(node.type_name.id.clone()));
            self.get_type(&HulkTypesInfo::Unknown)
        }
    }

    fn visit_function_access(&mut self, node: &mut FunctionAccess) -> TypeNode {
        let object = node.object.accept(self);
        let member_function = self.type_ast.find_method(object.type_name.clone(),node.member.funct_name.clone());
        if let Some(func) = member_function {
            if func.params.len() != node.member.arguments.len() {
                self.new_error(SemanticError::InvalidArgumentsCount(node.member.arguments.len(), func.params.len(), node.member.funct_name.clone()));
                self.get_type(&HulkTypesInfo::Unknown)
            } else {
                for (index , arg) in node.member.arguments.iter_mut().enumerate() {
                    let arg_type = arg.accept(self);
                    if arg_type.type_name != func.params[index].signature {
                        self.new_error(SemanticError::InvalidTypeArgument("function".to_string(), arg_type.type_name, func.params[index].signature.clone(), index, node.member.funct_name.clone()));
                    }
                }
                if let Some(function_return_type) = self.type_ast.get_type(&func.return_type) {
                    node.set_expression_type(function_return_type.clone());
                    function_return_type
                } else {
                    self.new_error(SemanticError::UndefinedType(func.return_type.clone()));
                    self.get_type(&HulkTypesInfo::Unknown)
                } 
            }
        } else {
            self.new_error(SemanticError::InvalidTypeFunctionAccess(object.type_name.clone(), node.member.funct_name.clone()));
            self.get_type(&HulkTypesInfo::Unknown)
        }
    }

    fn visit_member_access(&mut self, node: &mut MemberAccess) -> TypeNode {
        let object = node.object.accept(self);
        if let Some(current_type) = self.current_scope.current_type.clone() {
            if let Some(type_node) = self.type_ast.nodes.get_mut(&current_type) {
                if let Some(property_type) = type_node.variables.get_mut(&node.member.id) {
                    let property_type_cloned = property_type.clone();
                    let return_type = self.type_ast.get_type(&property_type_cloned).unwrap();
                    node.set_expression_type(return_type.clone());
                    return_type.clone()
                } else {
                    self.new_error(SemanticError::InvalidTypeProperty(object.type_name.clone(), node.member.id.clone()));
                    self.get_type(&HulkTypesInfo::Unknown)
                }
            } else {
                self.new_error(SemanticError::UndefinedType(current_type.clone()));
                self.get_type(&HulkTypesInfo::Unknown)
            }
        } else {
            self.new_error(SemanticError::InvalidTypePropertyAccess(object.type_name.clone(), node.member.id.clone()));
            self.get_type(&HulkTypesInfo::Unknown)
        }
    }
}