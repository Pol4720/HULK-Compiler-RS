# HULK Compiler

<div align="center">

**A Full Compiler for the HULK Language, Written in Rust**

A complete compiler implementation featuring lexical analysis, LALRPOP-based parsing, semantic analysis with type inference, and LLVM IR code generation — targeting the HULK educational programming language.

[![Rust](https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![LLVM](https://img.shields.io/badge/LLVM-262D3A?style=flat-square&logo=llvm&logoColor=white)](https://llvm.org)

</div>

---

## Overview

HULK (Havana University Language for Kompilers) is an educational programming language designed at MATCOM for teaching compiler construction. This project implements a **complete compiler pipeline** in Rust, from source code to LLVM IR, with full semantic analysis and type checking.

## Compiler Pipeline

```
Source Code (.hulk)
       │
       ▼
┌──────────────┐
│    Lexer     │  Tokenization (regex-based)
└──────┬───────┘
       ▼
┌──────────────┐
│   Parser     │  LALRPOP grammar → AST
└──────┬───────┘
       ▼
┌──────────────┐
│  Semantic    │  Type checking, scope resolution,
│  Analysis    │  inheritance validation
└──────┬───────┘
       ▼
┌──────────────┐
│  Code Gen    │  AST → LLVM IR
└──────┬───────┘
       ▼
   LLVM IR (.ll)
       │
       ▼
  Native Binary (via llc + gcc/clang)
```

## Language Features Supported

### Type System
- Primitive types: `Number`, `String`, `Boolean`
- User-defined types with inheritance
- Type inference for `let` bindings
- Protocol conformance (structural typing)

### Control Flow
- `if`/`elif`/`else` expressions
- `while` and `for` loops
- Pattern matching with `is` expressions

### Functions & Methods
- First-class functions
- Method dispatch with virtual tables
- Operator overloading
- Built-in mathematical functions (`sin`, `cos`, `sqrt`, `log`, `exp`)

### Object-Oriented
- Single inheritance with `inherits`
- Constructor initialization
- `self` references
- Protocol declarations (interfaces)

### Other Features
- String interpolation
- Let-in expressions with destructuring
- Type annotations (optional)
- Print and input built-ins

## Tech Stack

| Component | Technology |
|-----------|-----------|
| **Language** | Rust |
| **Parser Generator** | LALRPOP |
| **Code Generation** | LLVM IR (inkwell / llvm-sys) |
| **Build System** | Cargo |

## Building & Running

### Prerequisites

- Rust toolchain (rustup)
- LLVM 14+ (for code generation backend)
- Clang or GCC (for linking final binaries)

### Build

```bash
git clone https://github.com/Pol4720/HULK-Compiler-RS.git
cd HULK-Compiler-RS

cargo build --release
```

### Usage

```bash
# Compile a HULK source file to LLVM IR
cargo run -- input.hulk -o output.ll

# Generate native binary
llc output.ll -o output.s
gcc output.s -o output -lm
./output
```

### Example

```hulk
type Point(x: Number, y: Number) {
    norm(): Number => (self.x ^ 2 + self.y ^ 2) ^ 0.5;
}

let p = new Point(3, 4) in
    print("Distance: " @@ p.norm());
```

## Academic Context

Developed as a **Compilers** course project at the University of Havana, Faculty of Mathematics and Computer Science (MATCOM).

## License

This project is licensed under the MIT License.
