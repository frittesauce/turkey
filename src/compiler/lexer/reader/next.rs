use crate::compiler::lexer::{
    reader::Reader,
    token::{Token, TokenKind, match_operator},
};

impl Reader {
    pub fn next(&mut self) -> Token {
        loop {
            let chr = match self.peek() {
                Some(c) => c,
                None => {
                    todo!()
                }
            };

            return match chr.value {
                '"' => self.parse_string(),
                '\'' => self.parse_char(),
                char if match_operator(char.to_string().as_str()) != None => self.parse_operator(),
                char if char.is_digit(10) => self.parse_number(),
                char if char.is_alphabetic() || char == '_' => self.parse_identifier(),

                char if char.is_whitespace() => {
                    self.advance();
                    continue;
                }
                _ => {
                    let chr = self.advance().unwrap();
                    Token::new(
                        TokenKind::Unknown,
                        chr.position_range,
                        chr.value.to_string(),
                    )
                }
            };
        }
    }
}
