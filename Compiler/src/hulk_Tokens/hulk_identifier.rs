use super::*;

pub struct Identifier {
    pub position: TokenPos,
    pub id: String,
}

impl Identifier {
    pub fn new(start: usize, end: usize, id: &str) -> Self {
        Identifier {
            position: TokenPos::new(start, end),
            id: id.to_string(),
        }
    }
}
impl Identifier {
    pub fn get_position(&self) -> TokenPos {
        self.position
    }
}