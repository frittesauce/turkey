use crate::compiler::lexer::{
    reader::Reader,
    token::{Token, match_operator},
};

impl Reader {
    pub fn next(&mut self) -> Token {
        println!("new token!");
        loop {
            let chr = match self.peek() {
                Some(c) => c,
                None => return todo!(),
            };

            return match chr.value {
                '"' => self.parse_string(),
                '\'' => self.parse_char(),
                char if match_operator(char.to_string().as_str()) != None => self.parse_operator(),

                char if char.is_whitespace() => {
                    self.advance();
                    continue;
                }
                _ => todo!(),
            };
        }
    }
}
