use crate::compiler::lexer::{
    reader::Reader,
    token::{Token, TokenKind, match_operator},
};

impl Reader {
    pub fn parse_operator(&mut self) -> Token {
        let mut location = self.advance().unwrap();
        let mut string = String::new();

        let kind: TokenKind;

        string.push(location.value);

        loop {
            let char = match self.advance_if(|c| -> bool {
                let mut temp_string = string.clone();
                temp_string.push(*c);

                return match_operator(temp_string.as_str()) != None;
            }) {
                Some(c) => c,
                None => {
                    kind = match_operator(&string).unwrap();
                    break;
                }
            };

            string.push(char.value);
            location.position_range.set_end(char.position_range.end);
        }

        return Token::new(kind, location.position_range, string);
    }
}
