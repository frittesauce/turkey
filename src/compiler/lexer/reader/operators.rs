use crate::compiler::lexer::{
    kind::LexerKind,
    reader::{Reader, operators},
};

impl Reader {
    pub fn parse_operators(&mut self) -> Option<LexerKind> {
        let mut operators = Vec::new();
        operators.push(self.advance().unwrap());

        loop {
            let peek = match self.peek() {
                Some(c) => c,
                None => break,
            };

            if peek.raw.is_ascii_punctuation() && !(peek.raw == '"' || peek.raw == '\'') {
                operators.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        Some(LexerKind::Operators(operators))
    }
}
