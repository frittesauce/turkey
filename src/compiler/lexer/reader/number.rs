use crate::compiler::lexer::{
    reader::Reader,
    token::{Token, TokenKind},
};

#[derive(Copy, Clone, PartialEq)]
enum NumberType {
    Float,
    Integer,
    Hex,
}

impl NumberType {
    fn base(&self) -> u32 {
        return match self {
            NumberType::Hex => 16,
            _ => 10,
        };
    }
}

impl Reader {
    pub fn parse_number(&mut self) -> Token {
        let mut position_tracker = self.advance().unwrap();
        let mut string = String::new();
        let mut raw = String::new();

        let mut num_type: NumberType = NumberType::Integer;

        raw.push(position_tracker.value);

        if position_tracker.value == '0' && self.peek().unwrap().value == 'x' {
            num_type = NumberType::Hex;

            let char = self.advance().unwrap();

            raw.push(char.value);

            position_tracker
                .position_range
                .set_end(char.position_range.end);
        } else {
            string.push(position_tracker.value);
        }

        while let Some(next) = self.peek() {
            let c = next.value;

            match c {
                _ if c.is_digit(num_type.base()) => {
                    raw.push(c);
                    string.push(c);

                    position_tracker
                        .position_range
                        .set_end(next.position_range.end);

                    self.advance();
                }

                '.' if num_type != NumberType::Hex => {
                    num_type = NumberType::Float;

                    string.push(c);
                    raw.push(c);

                    position_tracker
                        .position_range
                        .set_end(next.position_range.end);

                    self.advance();
                }

                _ => break,
            }
        }

        let token_kind = match num_type {
            NumberType::Hex => match u64::from_str_radix(&string, num_type.base()) {
                Ok(_) => TokenKind::IntLiteral(string),
                Err(_) => TokenKind::Error("invalid hex literal!".into()),
            },
            NumberType::Integer => match string.parse::<i64>() {
                Ok(_) => TokenKind::IntLiteral(string),
                Err(_) => TokenKind::Error("invalid int literal!".into()),
            },
            NumberType::Float => match string.parse::<f64>() {
                Ok(_) => TokenKind::FloatLiteral(string),
                Err(_) => TokenKind::Error("invalid float literal!".into()),
            },
        };

        return Token::new(token_kind, position_tracker.position_range, raw);
    }
}
