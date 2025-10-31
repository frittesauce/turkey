use crate::compiler::lexer::{
    reader::Reader,
    token::{Token, TokenKind},
};

impl Reader {
    pub fn parse_operator(&mut self) -> Token {
        let mut location = self.advance().unwrap();
        let mut string = String::new();

        string.push(location.value);

        loop {
            let char = match self.advance() {
                Some(c) => c,
                None => break,
            };
        }

        return match string.as_str() {
            "<<=" => Token::new(TokenKind::LeftShiftEqual, location.position_range, string),
            ">>=" => Token::new(TokenKind::RightShiftEqual, location.position_range, string),
            "&=" => Token::new(TokenKind::AndEqual, location.position_range, string),
            "^=" => Token::new(TokenKind::XorEqual, location.position_range, string),
            "|=" => Token::new(TokenKind::OrEqual, location.position_range, string),

            "+=" => Token::new(TokenKind::PlusEquals, location.position_range, string),
            "-=" => Token::new(TokenKind::MinusEquals, location.position_range, string),
            "*=" => Token::new(TokenKind::StarEquals, location.position_range, string),
            "/=" => Token::new(TokenKind::SlashEquals, location.position_range, string),
            "%=" => Token::new(TokenKind::ModulusEquals, location.position_range, string),

            ">=" => Token::new(TokenKind::MoreEquals, location.position_range, string),
            "<=" => Token::new(TokenKind::LesserEquals, location.position_range, string),
            "!=" => Token::new(TokenKind::LesserEquals, location.position_range, string),
            "==" => Token::new(TokenKind::EqualsEquals, location.position_range, string),

            "<<" => Token::new(TokenKind::LeftShift, location.position_range, string),
            ">>" => Token::new(TokenKind::RightShift, location.position_range, string),

            "=>" => Token::new(TokenKind::FatArrow, location.position_range, string),
            "->" => Token::new(TokenKind::Arrow, location.position_range, string),
            _ => Token::new(
                TokenKind::Error("oppperator unkown!".to_string()),
                location.position_range,
                string,
            ),
        };
    }
}
