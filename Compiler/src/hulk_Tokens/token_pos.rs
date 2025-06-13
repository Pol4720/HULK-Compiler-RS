#[derive(Debug,Copy, Clone, PartialEq, Eq)]
pub struct TokenPos {
    pub start: usize,
    pub end: usize,
}

impl TokenPos {
    pub fn new(start: usize, end: usize) -> Self {
        TokenPos { start, end }
    }
}
