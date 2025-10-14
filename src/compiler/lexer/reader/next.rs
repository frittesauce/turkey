use crate::compiler::lexer::{kind::LexerKind, reader::Reader};

impl Reader {
    pub fn next(&mut self) -> Option<LexerKind> {
        loop {
            let start = match self.peek() {
                Some(c) => c,
                None => return None,
            };

            return match start.raw {};
        }
    }
}
