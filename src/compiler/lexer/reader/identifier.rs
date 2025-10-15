use crate::compiler::lexer::{
    kind::{LexerKind, LocatedString},
    reader::Reader,
};

impl Reader {
    pub fn parse_identifier(&mut self) -> Option<LexerKind> {
        let mut body = String::new();
        let mut pos_tracker = self.advance().unwrap();
        body.push(pos_tracker.raw);

        loop {
            let peek = match self.peek() {
                Some(c) => c,
                None => break,
            };

            if peek.raw.is_ascii_alphabetic() || peek.raw.is_ascii_digit() || peek.raw == '_' {
                let char = self.advance().unwrap();
                pos_tracker.position.set_end(char.position.end);
                body.push(char.raw);
            } else {
                break;
            }
        }

        let string = LocatedString::new(body, pos_tracker.position);

        return Some(LexerKind::Identifier(string));
    }
}
