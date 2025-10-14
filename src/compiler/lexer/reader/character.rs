use crate::common::position::Position;

#[derive(Debug)]
pub struct Character {
    char: char,
    position: Position,
}

impl Character {
    pub fn new(char: char, position: Position) -> Self {
        return Self {
            char: char,
            position: position,
        };
    }
}

impl std::fmt::Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.char)
    }
}
