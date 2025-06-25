// Alfabeto permitido para el lexer
// Puedes modificar este arreglo según tus necesidades

// Letras mayúsculas
pub const UPPERCASE: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

// Letras minúsculas
pub const LOWERCASE: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

// Dígitos
pub const DIGITS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

// Símbolos comunes
pub const SYMBOLS: &[char] = &[
    '+', '-', '*', '/', '=', ';', '.', ',', '(', ')', '{', '}', '[', ']', '_', '<', '>', '!', ':',
    '"', '\\', '|', '&', '%', '?', '#', '^', '$', '\'',
];

// Caracteres acentuados y especiales
pub const ACCENTED_CHARS: &[char] = &[
    'á', 'é', 'í', 'ó', 'ú', 'Á', 'É', 'Í', 'Ó', 'Ú', 'ñ', 'Ñ', 'ü', 'Ü',
];

// Espacio y tabulación
pub const WHITESPACE: &[char] = &[' ', '\t', '\n', '\r'];

// Alfabeto total
pub const ALPHABET: &[char] = &[
    // Mayúsculas
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', // Minúsculas
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', // Dígitos
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', // Símbolos
    '+', '-', '*', '/', '=', ';', '.', ',', '(', ')', '{', '}', '[', ']', '_', '<', '>', '!', ':',
    '"', '\\', '|', '&', '%', '?', '#', '^', '$', '\'', // Caracteres acentuados
    'á', 'é', 'í', 'ó', 'ú', 'Á', 'É', 'Í', 'Ó', 'Ú', 'ñ', 'Ñ', 'ü', 'Ü',
    // Espacio y caracteres de control
    ' ', '\t', '\n', '\r',
];
