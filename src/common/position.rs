pub struct PositionRange {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub character: usize,
}

impl Position {
    pub fn new(line: usize, column: usize, character: usize) -> Self {
        return Self {
            line,
            column,
            character,
        };
    }
}
