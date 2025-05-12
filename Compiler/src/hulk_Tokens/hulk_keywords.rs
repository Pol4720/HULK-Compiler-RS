use std::fmt::Display;


#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum KeywordToken {
    Print,
    While,
    Elif,
    Else,
    If,
    In,
    Function,
    Let,
    Class,
    Return,
    Break,
    Protocol,
    Continue,
    Import,
    For,
}

impl Display for KeywordToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeywordToken::Let => write!(f, "let"),
            KeywordToken::If => write!(f, "if"),
            KeywordToken::Else => write!(f, "else"),
            KeywordToken::While => write!(f, "while"),
            KeywordToken::Print => write!(f, "print"),
            KeywordToken::In => write!(f, "in"),    
            KeywordToken::Function => write!(f, "function"),
            KeywordToken::Class => write!(f, "class"),
            KeywordToken::Protocol => write!(f, "protocol"),
            KeywordToken::Return => write!(f, "return"),
            KeywordToken::Break => write!(f, "break"),
            KeywordToken::Continue => write!(f, "continue"),
            KeywordToken::Import => write!(f, "import"),
            KeywordToken::For => write!(f, "for"),
            KeywordToken::Elif => write!(f, "elif"),
        }
    }
}