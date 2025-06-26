// ===============================
// extractor.rs: Extracción de Lexemas usando DFA
// ===============================
// Recorre el texto de entrada y extrae tokens reconocidos por el DFA.

use super::error::LexicalError;
use super::lexeme::Lexeme;
use crate::dfa::dfa::{DFA, DFAState};
use crate::nfa::join_nfa::AcceptInfo;
use crate::regex_parser::node::regex_char::RegexChar;

/// Extrae los lexemas de un texto usando un DFA.
/// Devuelve Ok(lexemas) o Err(errores léxicos).
pub fn extract_lexemes(text: &str, dfa: &DFA) -> Result<Vec<Lexeme>, Vec<LexicalError>> {
    let chars: Vec<char> = text.chars().collect();
    let mut index = 0;
    let mut line = 1;
    let mut column = 1;
    let mut lexemes = Vec::new();
    let mut errors = Vec::new();
    let len = chars.len();

    while index < len {
        // Saltar espacios, tabs, saltos de línea y retorno de carro como separadores de lexemas
        while index < len
            && (chars[index] == ' '
                || chars[index] == '\t'
                || chars[index] == '\n'
                || chars[index] == '\r')
        {
            if chars[index] == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
            index += 1;
        }
        if index >= len {
            break;
        }
        let mut state_key = dfa.start.clone();
        let mut last_accept: Option<(usize, &DFAState)> = None;
        let mut i = index;
        let mut col = column;
        let mut last_accept_col = col;
        // Buscar el match más largo (greedy)
        while i < len {
            let c = chars[i];
            // Mapeo especial para metacaracteres reconocidos por el DFA
            let symbol = RegexChar::Literal(c);
            if let Some(next_key) = dfa.transitions.get(&(state_key.clone(), symbol.clone())) {
                state_key = next_key.clone();
                if let Some(state) = dfa.states.get(&state_key) {
                    if let Some(ref accept) = state.accept {
                        last_accept = Some((i, state));
                        last_accept_col = col;
                    }
                }
                if c == '\n' {
                    col = 1;
                } else {
                    col += 1;
                }
                i += 1;
            } else {
                // Simulación de $ (fin de línea/texto)
                let at_end_of_line = i == len || (i < len && chars[i] == '\n');
                if at_end_of_line {
                    if let Some(next_key) =
                        dfa.transitions.get(&(state_key.clone(), RegexChar::End))
                    {
                        state_key = next_key.clone();
                        if let Some(state) = dfa.states.get(&state_key) {
                            if let Some(ref accept) = state.accept {
                                last_accept = Some((i - 1, state));
                                last_accept_col = col;
                            }
                        }
                    }
                }
                break;
            }
        }
        // --- ARREGLO: Si llegamos al final del texto, intentamos la transición con $ ---
        if i == len {
            if let Some(next_key) = dfa.transitions.get(&(state_key.clone(), RegexChar::End)) {
                state_key = next_key.clone();
                if let Some(state) = dfa.states.get(&state_key) {
                    if let Some(ref accept) = state.accept {
                        last_accept = Some((i - 1, state));
                        last_accept_col = col;
                    }
                }
            }
        }
        if let Some((end, state)) = last_accept {
            let accept = state.accept.as_ref().unwrap();
            let value: String = chars[index..=end].iter().collect();
            lexemes.push(Lexeme {
                token_type: accept.token_type.clone(),
                value,
                line,
                column_start: column,
                column_end: last_accept_col + (end - index),
            });
            // Actualizar posición y columna
            for c in &chars[index..=end] {
                if *c == '\n' {
                    line += 1;
                    column = 1;
                } else {
                    column += 1;
                }
            }
            index = end + 1;
        } else {
            // Error léxico: carácter inesperado
            errors.push(LexicalError {
                message: format!("Error léxico: carácter inesperado '{}'.", chars[index]),
                line,
                column,
            });
            if chars[index] == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
            index += 1;
        }
    }
    if errors.is_empty() {
        Ok(lexemes)
    } else {
        Err(errors)
    }
}
