use std::fmt::{self, Display, Formatter};
use crate::visitor::hulk_accept::Accept;
use crate::visitor::hulk_visitor::Visitor;
use crate::codegen::traits::Codegen;
use crate::codegen::context::CodegenContext;

#[derive(Debug, Clone, PartialEq)]
pub struct NumberLiteral {
    pub value: f64,
}

impl NumberLiteral {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.parse().unwrap(),
        }
    }
}

impl Display for NumberLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Codegen for NumberLiteral {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // Genera un nuevo registro temporal
        let result_reg = context.generate_temp();

        // LLVM requiere literales double con formato decimal o exponencial
        let formatted = format!("{:.16E}", self.value);

        // Emite la instrucción: fadd double 0.0, literal
        // para asignar el valor a un registro (truco común para literales float)
        let line = format!("  {} = fadd double 0.0, {}", result_reg, formatted);
        context.emit(&line);

        result_reg
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct BooleanLiteral {
    pub value: bool,
}

impl BooleanLiteral {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.parse().unwrap(),
        }
    }
}

impl Display for BooleanLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Codegen for BooleanLiteral {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // Convierte true/false a 1/0
        let llvm_bool = if self.value { "1" } else { "0" };

        // Crea un nuevo registro temporal
        let result_reg = context.generate_temp();

        // Emite una instrucción de mov para asignar el literal al registro
        // LLVM no tiene una instrucción de "mov", usamos `add 0, x` como truco
        let line = format!("  {} = add i1 0, {}", result_reg, llvm_bool);
        context.emit(&line);

        result_reg
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral {
    pub value: String,
}

impl StringLiteral {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl Display for StringLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Codegen for StringLiteral {
    fn codegen(&self, context: &mut CodegenContext) -> String {
        // Escape comillas, saltos de línea, etc.
        let escaped = self
            .value
            .replace("\\", "\\5C")
            .replace("\n", "\\0A")
            .replace("\"", "\\22");

        // Asegura que termine en nulo para C strings
        let null_terminated = format!("{}\\00", escaped);
        let byte_count = null_terminated.len();

        // Nombre único para la constante
        let const_name = context.generate_string_const_name();

        // Define la constante global
        let global_def = format!(
            "@{} = private unnamed_addr constant [{} x i8] c\"{}\"",
            const_name, byte_count, null_terminated
        );
        context.emit_global(&global_def);

        // Prepara el puntero con getelementptr
        let ptr_reg = context.generate_temp();
        let gep_inst = format!(
            "  {} = getelementptr inbounds [{} x i8], [{} x i8]* @{}, i32 0, i32 0",
            ptr_reg, byte_count, byte_count, const_name
        );
        context.emit(&gep_inst);

        ptr_reg // Devuelve el nombre del registro con la dirección del string
    }
}

