use crate::compiler::lexer::{reader::Reader, token::Token};

impl Reader {
    pub fn next(&mut self) -> Option<Token> {
        loop {
            let chr = match self.peek() {
                Some(c) => c,
                None => return None,
            };

            let kind = match chr.value {
                '"' => self.parse_string(),
                char if char.is_whitespace() => {
                    self.advance();
                    continue;
                }
                _ => continue,
            };
        }
    }
}
