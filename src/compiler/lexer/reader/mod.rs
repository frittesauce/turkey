use crate::common::position::{Position, Span};

mod next;
mod operator;
mod string;

const TAB_SIZE: usize = 4;

pub type Character = Span<char>;

pub struct Reader {
    pub chars: Vec<Character>,
}

impl Reader {
    pub fn new(source: &str) -> Self {
        let mut input = source.chars();
        let mut output: Vec<Character> = Vec::with_capacity(source.len());

        let mut line: usize = 1;
        let mut column: usize = 0;
        let mut character: usize = 0;

        loop {
            let char = match input.next() {
                Some(char) => char,
                None => break,
            };

            match char {
                '\r' => continue,
                '\t' => column += TAB_SIZE,
                _ => column += 1,
            }
            character += 1;

            let position = Position::new(line, column, character);

            output.push(Character::new(position.to_range(), char));

            if char == '\n' {
                line += 1;
                column = 0;
                character = 0;
            }
        }

        output.reverse();

        return Self { chars: output };
    }

    pub fn advance(&mut self) -> Option<Character> {
        self.chars.pop()
    }

    pub fn peek(&self) -> Option<&Character> {
        self.chars.last()
    }
}
