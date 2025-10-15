use crate::common::position::{Position, Span};

pub mod identifier;
pub mod next;
pub mod number;
pub mod operators;
pub mod string;

pub type Character = Span<char>;

pub struct Reader {
    pub chars: Vec<Character>,
}

const TAB_SIZE: usize = 4;

impl Reader {
    pub fn new(source: &str) -> Self {
        let mut input = source.chars();
        let mut output: Vec<Character> = Vec::with_capacity(source.len());

        let mut line: usize = 1;
        let mut col: usize = 0;
        let mut character: usize = 0;

        loop {
            let char = match input.next() {
                Some(char) => char,
                None => break,
            };

            let mut newline: bool = false;

            match char {
                '\r' => continue,
                '\n' => {
                    newline = true;
                    col += 1
                }
                '\t' => {
                    col += TAB_SIZE;
                }
                _ => col += 1,
            }
            character += 1;

            let position = Position::new(line, col, character);

            output.push(Character::new(char, position.to_range()));

            if newline {
                line += 1;
                col = 0;
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

    pub fn second_peek(&self) -> Option<&Character> {
        self.chars.get(self.chars.len() - 2)
    }
}
