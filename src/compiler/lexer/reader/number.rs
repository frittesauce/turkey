use crate::compiler::lexer::{reader::Reader, token::Token};

impl Reader {
    pub fn parse_number(&mut self) -> Token {
        let mut position_tracker = self.advance().unwrap();
        let mut string = String::new();

        string.push(position_tracker.value);
        todo!();
    }
}
