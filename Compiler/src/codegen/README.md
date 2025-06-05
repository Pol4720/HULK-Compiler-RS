# 🧠 Hulk Compiler – Code Generation Phase

Este módulo implementa la **fase de generación de código** del compilador de Hulk, un lenguaje experimental basado en Rust. Utiliza LLVM a través del crate [`inkwell`](https://crates.io/crates/inkwell) para generar código LLVM IR o incluso código de máquina listo para ejecutar en la arquitectura del sistema anfitrión.

---

## 📐 Arquitectura General

```text
Compiler
└── Codegen
     ├── CodeGenerator (entry point)
     ├── ContextManager (manejo de LLVM context/module/builder)
     ├── ValueMap (tabla de símbolos)
     └── traits/
          └── CodegenNode (trait para nodos del AST)
```

---

## 📁 Estructura del Módulo

### `codegen/mod.rs`
Archivo de reexportación. Incluye todos los submódulos necesarios para que el generador de código sea accesible desde fuera del módulo.

### `codegen/generator.rs`
Define la estructura `CodeGenerator`, el punto de entrada principal de la fase de generación de código.

**Componentes:**
- `generate()`: acepta un nodo raíz del AST y comienza el proceso de generación.
- `print_ir()`: imprime el LLVM IR generado a stderr.
- `emit_machine_code()`: (placeholder) emitirá código nativo en futuras versiones.

### `codegen/context.rs`
Define `LLVMContext`, que encapsula:
- `context`: el objeto central de LLVM.
- `builder`: para construir instrucciones.
- `module`: el contenedor del código LLVM generado.
- `engine`: el motor JIT (just-in-time) para pruebas o ejecución inmediata.

```rust
pub struct LLVMContext<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub engine: ExecutionEngine<'ctx>,
}
```

### `codegen/value_map.rs`
Implementa `ValueMap`, una tabla de símbolos que mapea identificadores del lenguaje Hulk a valores de LLVM (`BasicValueEnum`).

**Útil para:**
- Variables locales.
- Argumentos de funciones.
- Seguimiento de valores generados.

### `codegen/traits.rs`
Define el trait `CodegenNode`, que debe implementarse para cada tipo de nodo del AST:

```rust
pub trait CodegenNode<'ctx> {
    fn codegen(
        &self,
        llvm: &mut LLVMContext<'ctx>,
        values: &mut ValueMap<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String>;
}
```

Esto permite que el generador sea modular y extensible: cada tipo de nodo AST tiene su propia lógica de generación de código.

---

## 🧩 Cómo agregar soporte para un nuevo nodo AST

1. Define tu nodo en la carpeta `hulk_Tokens/` (por ejemplo: `hulk_if_expr.rs`, `hulk_while_loop.rs`, etc.).
2. Implementa el trait `CodegenNode` para ese tipo.
3. Dentro del método `codegen()` del trait, utiliza:
   - `builder` para construir instrucciones.
   - `module` para definir funciones o variables globales.
   - `value_map` para leer/escribir símbolos.

### Ejemplo mínimo para una expresión binaria

Supón que tienes la siguiente definición en `hulk_Tokens/hulk_binary_expr.rs`:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: BinaryOperatorToken,
    pub right: Box<Expr>,
}
impl BinaryExpr {
    pub fn new(left: Box<Expr>, operator: BinaryOperatorToken, right: Box<Expr>) -> Self {
        BinaryExpr { left, operator, right }
    }
}
```

La implementación del trait `CodegenNode` podría verse así:

```rust
use crate::codegen::{CodegenNode, CodegenContext};
use inkwell::values::BasicValueEnum;

impl CodegenNode for BinaryExpr {
    fn codegen(&self, context: &mut CodegenContext) -> Result<BasicValueEnum, String> {
        let lhs = self.left.codegen(context)?;
        let rhs = self.right.codegen(context)?;
        let result = match self.operator {
            BinaryOperatorToken::Plus => context.builder.build_int_add(
                lhs.into_int_value(),
                rhs.into_int_value(),
                "addtmp"
            ),
            BinaryOperatorToken::Minus => context.builder.build_int_sub(
                lhs.into_int_value(),
                rhs.into_int_value(),
                "subtmp"
            ),
            // Agrega más operadores según sea necesario
            _ => return Err(format!("Operador no soportado: {:?}", self.operator)),
        };
        Ok(result.as_basic_value_enum())
    }
}
```

---

## 🚀 Uso básico

A continuación se muestra cómo integrar la fase de codegen en el flujo principal del compilador, adaptado a la estructura real de  `main.rs`. El codegen se puede invocar después del análisis semántico exitoso, sin afectar el funcionamiento actual.

Supón que tienes un módulo `codegen` y un AST compatible con el trait `CodegenNode`:

```rust
use inkwell::context::Context;
use codegen::generator::CodeGenerator;
// ...otros imports...

fn main() {
    let parser = ProgramParser::new();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }

        let parsed_expr = parser.parse(&input).unwrap();
        let mut print_visitor = PreetyPrintVisitor;
        let mut semantic_visitor = SemanticVisitor::new();
        let res = semantic_visitor.analyze(&parsed_expr);
        match res {
            Ok(_) => {
                println!("Parsed successfully!");

                // --- INTEGRACIÓN CODEGEN ---
                // Solo ejecutar codegen si el análisis semántico fue exitoso
                let context = Context::create();
                let mut generator = CodeGenerator::new("hulk_module", &context);
                match generator.generate(&parsed_expr) {
                    Ok(_) => generator.print_ir(), // Muestra el LLVM IR generado
                    Err(e) => eprintln!("Error en codegen: {}", e),
                }
                // --- FIN CODEGEN ---
            }
            Err(errors) => {
                println!("\x1b[31mErrors:");
                for err in errors.iter() {
                    println!("{}", err.message());
                }
                println!("\x1b[0m");
            }
        }
        println!("");
        println!("\x1b[34m{}\x1b[0m", print_visitor.visit_program(&parsed_expr));
    }
}
```

**Notas:**
- El bloque de codegen se ejecuta solo si el análisis semántico es exitoso.
- El LLVM IR generado se imprime en consola.
- No se altera el flujo actual de impresión ni el análisis semántico.

---

## 📌 Requisitos

- Rust (>= 1.70)
- LLVM (>= 14.0 instalado en el sistema)
- Crate `inkwell`

### Para instalar LLVM:

```bash
# Ubuntu/Debian
sudo apt install llvm-14-dev

# Mac (Homebrew)
brew install llvm

# Windows (choco)
choco install llvm
```

---



## 📚 Recursos recomendados

- [LLVM Language Reference Manual](https://llvm.org/docs/LangRef.html)
- [Inkwell Documentation](https://thedan64.github.io/inkwell/)
- [Crafting Interpreters](https://craftinginterpreters.com/)
- [LLVM by Example (Rust)](https://github.com/maekawatoshiki/llvm-ir-tutorial)

---

## 🧠 Licencia

Richard. ❤️







