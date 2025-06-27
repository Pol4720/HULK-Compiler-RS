/// Módulo para manejar los escapes de caracteres en expresiones regulares.
/// Soporta los escapes clásicos: \n, \t, \\, \r, \0, y escapes de operaciones: \d, \D, \w, \W, \s, \S, etc.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum RegexEscape {
    Newline,   // \n
    Tab,       // \t
    Backslash, // \\
    Return,    // \r
    Null,      // \0
    Digit,     // \d
    NotDigit,  // \D
    Word,      // \w
    NotWord,   // \W
    Space,     // \s
    NotSpace,  // \S
    LBracket,  // \[
    RBracket,  // \]
    LParen,    // \(
    RParen,    // \)
    LBrace,    // \{
    RBrace,    // \}
    Dot,       // \.
    Plus,      // \+
    Star,      // \*
    Question,  // \?
    Pipe,      // \|
    Caret,     // \^
    Dollar,    // \$
               // Puedes agregar más escapes según sea necesario
}

impl RegexEscape {
    /// Intenta convertir un carácter de escape en su representación enum.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'n' => Some(RegexEscape::Newline),
            't' => Some(RegexEscape::Tab),
            '\\' => Some(RegexEscape::Backslash),
            'r' => Some(RegexEscape::Return),
            '0' => Some(RegexEscape::Null),
            'd' => Some(RegexEscape::Digit),
            'D' => Some(RegexEscape::NotDigit),
            'w' => Some(RegexEscape::Word),
            'W' => Some(RegexEscape::NotWord),
            's' => Some(RegexEscape::Space),
            'S' => Some(RegexEscape::NotSpace),
            '[' => Some(RegexEscape::LBracket),
            ']' => Some(RegexEscape::RBracket),
            '(' => Some(RegexEscape::LParen),
            ')' => Some(RegexEscape::RParen),
            '{' => Some(RegexEscape::LBrace),
            '}' => Some(RegexEscape::RBrace),
            '.' => Some(RegexEscape::Dot),
            '+' => Some(RegexEscape::Plus),
            '*' => Some(RegexEscape::Star),
            '?' => Some(RegexEscape::Question),
            '|' => Some(RegexEscape::Pipe),
            '^' => Some(RegexEscape::Caret),
            '$' => Some(RegexEscape::Dollar),
            _ => None,
        }
    }

    /// Devuelve el carácter real representado por el escape (solo para escapes literales).
    pub fn as_char(&self) -> Option<char> {
        match self {
            RegexEscape::Newline => Some('\n'),
            RegexEscape::Tab => Some('\t'),
            RegexEscape::Backslash => Some('\\'),
            RegexEscape::Return => Some('\r'),
            RegexEscape::Null => Some('\0'),
            RegexEscape::LBracket => Some('['),
            RegexEscape::RBracket => Some(']'),
            RegexEscape::LParen => Some('('),
            RegexEscape::RParen => Some(')'),
            RegexEscape::LBrace => Some('{'),
            RegexEscape::RBrace => Some('}'),
            RegexEscape::Dot => Some('.'),
            RegexEscape::Plus => Some('+'),
            RegexEscape::Star => Some('*'),
            RegexEscape::Question => Some('?'),
            RegexEscape::Pipe => Some('|'),
            RegexEscape::Caret => Some('^'),
            RegexEscape::Dollar => Some('$'),
            // Los escapes de clase no tienen un solo char
            _ => None,
        }
    }
}
