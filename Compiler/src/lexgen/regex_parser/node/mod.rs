// ===============================
// Módulo principal de nodos AST para regex
// ===============================
// Expone todos los tipos de nodos y enums usados en el AST de expresiones regulares.

pub mod ast_node_impl; // Definición del nodo base y trait AstNode
pub mod bin_op; // Operadores binarios (Concat, Or)
pub mod group; // Agrupaciones (paréntesis)
pub mod regex_char; // Caracteres y escapes
pub mod regex_class; // Clases de caracteres ([abc], [a-z], .)
pub mod regex_escape; // Escapes especiales (\n, \t, etc)
pub mod un_op; // Operadores unarios (*, +, ?)
