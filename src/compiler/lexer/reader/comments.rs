use crate::compiler::lexer::{
    reader::Reader,
    token::{Token, TokenKind},
};

impl Reader {
    pub fn parse_single_comment(&mut self) -> Token {
        let mut postion_tracker = self.advance().unwrap();
        let mut string = String::new();
        let mut raw = String::new();

        raw.push(postion_tracker.value);

        let char = self.advance().unwrap();

        raw.push(char.value);

        loop {
            let char = match self.advance() {
                Some(c) => c,
                None => break,
            };

            raw.push(char.value);

            postion_tracker
                .position_range
                .set_end(char.position_range.end);

            if char.value == '\n' {
                break;
            }

            string.push(char.value);
        }

        return Token::new(
            TokenKind::SingleLineComment(string),
            postion_tracker.position_range,
            raw,
        );
    }

    pub fn parse_multi_comment(&mut self) -> Token {
        let mut postion_tracker = self.advance().unwrap();
        let mut string = String::new();
        let mut raw = String::new();

        raw.push(postion_tracker.value);

        let char = self.advance().unwrap();

        raw.push(char.value);

        loop {
            let char = match self.advance() {
                Some(c) => c,
                None => break,
            };

            raw.push(char.value);
            postion_tracker
                .position_range
                .set_end(char.position_range.end);

            if char.value == '*' {
                if self.peek().unwrap().value == '/' {
                    break;
                }
            }

            string.push(char.value);

            if char.value == '*' {
                if self.peek().unwrap().value == '/' {
                    break;
                }
            }
        }

        let char = self.advance().unwrap();

        raw.push(char.value);

        return Token::new(
            TokenKind::MultiLineComment(string),
            postion_tracker.position_range,
            raw,
        );
    }
}
