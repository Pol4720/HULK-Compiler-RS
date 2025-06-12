use crate::hulk_ast_nodes::hulk_expression::Expr;
use crate::codegen::traits::Codegen;
use crate::codegen::context::CodegenContext;
use crate::typings::types_node::TypeNode;

#[derive(Debug, PartialEq,Clone)]
pub struct ForExpr {
    pub variable: String,
    pub start: Box<Expr>,
    pub end: Box<Expr>,
    pub body: Box<Expr>,
    pub _type: Option<TypeNode>,
}

impl ForExpr {
    pub fn new(variable: String, start: Expr, end: Expr, body: Expr) -> Self {
        ForExpr {
            variable,
            start: Box::new(start),
            end: Box::new(end),
            body: Box::new(body),
            _type: None,
        }
    }
    pub fn set_expression_type(&mut self, _type: TypeNode) {
        self._type = Some(_type);
    }
}

impl Codegen for ForExpr {
    fn codegen(&self, context: &mut CodegenContext) -> String {
         // Genera valores de inicio y fin
         let start_val = self.start.codegen(context);
         let end_val = self.end.codegen(context);
 
         // Aloca espacio para la variable del bucle y almacena el valor inicial
         let loop_var_alloc = context.generate_temp();
         context.emit(&format!("  {} = alloca i32", loop_var_alloc));
         context.emit(&format!("  store i32 {}, i32* {}", start_val, loop_var_alloc));
 
         // Etiquetas
         let loop_cond_label = context.generate_label("loop_cond");
         let loop_body_label = context.generate_label("loop_body");
         let loop_inc_label = context.generate_label("loop_inc");
         let loop_end_label = context.generate_label("loop_end");
 
         // Salta a condición
         context.emit(&format!("  br label %{}", loop_cond_label));
 
         // loop_cond:
         context.emit(&format!("{}:", loop_cond_label));
         let loop_var = context.generate_temp();
         context.emit(&format!("  {} = load i32, i32* {}", loop_var, loop_var_alloc));
 
         let cond_temp = context.generate_temp();
         context.emit(&format!(
             "  {} = icmp sle i32 {}, {}",
             cond_temp, loop_var, end_val
         ));
         context.emit(&format!(
             "  br i1 {}, label %{}, label %{}",
             cond_temp, loop_body_label, loop_end_label
         ));
 
         // loop_body:
         context.emit(&format!("{}:", loop_body_label));
 
         // Aquí puedes registrar el nombre de variable si usas un entorno de símbolos
         // por ahora solo generamos el cuerpo normalmente
         let _ = self.body.codegen(context);
 
         context.emit(&format!("  br label %{}", loop_inc_label));
 
         // loop_inc:
         context.emit(&format!("{}:", loop_inc_label));
         let next_val = context.generate_temp();
         context.emit(&format!("  {} = add i32 {}, 1", next_val, loop_var));
         context.emit(&format!("  store i32 {}, i32* {}", next_val, loop_var_alloc));
         context.emit(&format!("  br label %{}", loop_cond_label));
 
         // loop_end:
         context.emit(&format!("{}:", loop_end_label));
 
         // Los bucles no producen valor , así que devolvemos "void"
         String::from("void")
    }
}