use crate::compiler::lexer::{kind::LexerKind, reader::Reader};

impl Reader {
    pub fn next(&mut self) -> Option<LexerKind> {
        loop {
            let start = match self.peek() {
                Some(c) => c,
                None => return None,
            };

            return match start.raw {
                '"' | '\'' => self.parse_string(),

                char if char.is_ascii_digit() => self.parse_number(),

                char if char.is_ascii_alphabetic() || char == '_' => self.parse_identifier(),

                char if char.is_whitespace() => {
                    self.advance();
                    continue;
                }

                char if char.is_ascii_punctuation() => self.parse_operators(),

                _ => todo!(),
            };
        }
    }
}
