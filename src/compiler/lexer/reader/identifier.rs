use crate::compiler::lexer::{
    reader::Reader,
    token::{Token, TokenKind, match_keyword},
};

impl Reader {
    pub fn parse_identifier(&mut self) -> Token {
        let mut postion_tracker = self.advance().unwrap();
        let mut raw = String::new();

        raw.push(postion_tracker.value);

        loop {
            let char = match self.advance_if(|c| c.is_alphanumeric() || c == &'_') {
                Some(c) => c,
                None => break,
            };

            raw.push(char.value);
            postion_tracker
                .position_range
                .set_end(char.position_range.end);
        }
        match match_keyword(raw.as_str()) {
            Some(kind) => return Token::new(kind, postion_tracker.position_range, raw),
            None => {
                return Token::new(
                    TokenKind::Identifier(raw.clone()),
                    postion_tracker.position_range,
                    raw,
                );
            }
        }
    }
}
