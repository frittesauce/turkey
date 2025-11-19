use crate::compiler::lexer::{
    reader::Reader,
    token::{Token, TokenKind},
};

impl Reader {
    pub fn parse_single_comment(&mut self) -> Token {
        let mut position_tracker = self.advance().unwrap();
        let mut string = String::new();
        let mut raw = String::new();

        raw.push(position_tracker.value);

        let char = self.advance().unwrap();

        raw.push(char.value);

        while self.advance().is_some() {
            raw.push(char.value);

            position_tracker
                .position_range
                .set_end(char.position_range.end);

            if char.value == '\n' {
                break;
            }

            string.push(char.value);
        }

        Token::new(
            TokenKind::SingleLineComment(string),
            position_tracker.position_range,
            raw,
        )
    }

    pub fn parse_multi_comment(&mut self) -> Token {
        let mut position_tracker = self.advance().unwrap();
        let mut string = String::new();
        let mut raw = String::new();

        raw.push(position_tracker.value);

        let char = self.advance().unwrap();

        raw.push(char.value);

        loop {
            let char = match self.advance() {
                Some(c) => c,
                None => {
                    return Token::new(
                        TokenKind::Error("Unterminated multi-line comment".to_string()),
                        position_tracker.position_range,
                        raw,
                    );
                }
            };

            raw.push(char.value);
            position_tracker
                .position_range
                .set_end(char.position_range.end);

            if char.value == '*'
                && let Some(next) = self.peek()
                && next.value == '/'
            {
                string.push(char.value);
                break;
            }

            string.push(char.value);
        }

        let char = self.advance().unwrap();

        raw.push(char.value);

        Token::new(
            TokenKind::MultiLineComment(string),
            position_tracker.position_range,
            raw,
        )
    }
}
