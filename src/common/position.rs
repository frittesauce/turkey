use std::fmt::Debug;

#[derive(Debug)]
pub struct PositionRange {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub char: usize,
}

impl Position {
    pub fn new(line: usize, column: usize, char: usize) -> Self {
        return Self { line, column, char };
    }

    pub fn to_range(self) -> PositionRange {
        return PositionRange::new(self.clone(), self);
    }
}

impl PositionRange {
    pub fn new(start: Position, end: Position) -> Self {
        return Self { start, end };
    }

    pub fn set_end(&mut self, position: Position) {
        self.end = position;
    }
}

pub struct Span<T = ()> {
    pub position_range: PositionRange,
    pub value: T,
}

impl<T> Span<T> {
    pub fn new(position_range: PositionRange, value: T) -> Self {
        return Self {
            position_range,
            value,
        };
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Column: {}\n Line: {}", self.column, self.line)
    }
}

impl std::fmt::Display for PositionRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "col {}-{}, ln {}-{}",
            self.start.column, self.end.column, self.start.line, self.end.line
        )
    }
}

impl<T: Debug> Debug for Span<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?} (){})", self.value, self.position_range)
    }
}
