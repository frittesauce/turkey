use crate::compiler::lexer::{
    kind::{LexerKind, LocatedString},
    reader::Reader,
};

impl Reader {
    pub fn parse_string(&mut self) -> Option<LexerKind> {
        let mut body = String::new();
        let mut pos_tracker = self.advance().unwrap();
        let string_type = pos_tracker.raw;

        loop {
            let char = match self.advance() {
                Some(c) => c,
                None => break,
            };
            pos_tracker.position.set_end(char.position.end);

            if char.raw == string_type {
                break;
            }

            body.push(char.raw);
        }

        let string = LocatedString::new(body, pos_tracker.position);

        Some(LexerKind::String(string))
    }
}
