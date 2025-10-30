use crate::{
    common::position::Span,
    compiler::lexer::{
        reader::Reader,
        token::{Token, TokenKind},
    },
};

impl Reader {
    fn parse_escape(&mut self) -> Result<Span<char>, Span<String>> {
        let next = match self.advance() {
            Some(chr) => chr,
            None => panic!("Error something went wrong when parsing an escape thing"),
        };

        return match next.value {
            'n' => Ok(Span::new(next.position_range, '\n')),
            'r' => Ok(Span::new(next.position_range, '\r')),
            't' => Ok(Span::new(next.position_range, '\t')),
            '\\' => Ok(Span::new(next.position_range, '\\')),
            '"' => Ok(Span::new(next.position_range, '"')),
            '\'' => Ok(Span::new(next.position_range, '\'')),
            '0' => Ok(Span::new(next.position_range, '\0')),

            'x' | 'X' => {
                let mut val: u32 = 0;
                let mut count = 0;

                let mut d: Span<char>;

                while count < 2 {
                    match self.peek() {
                        Some(p) if p.value.is_ascii_hexdigit() => {
                            d = self.advance().unwrap();
                            val = val * 16 + d.value.to_digit(16).unwrap();
                            count += 1;
                        }
                        _ => break,
                    }
                }
                if count == 0 {
                    return Err(Span::new(
                        next.position_range,
                        "Invalid hex escape: expected 1-2 hex digits after \\x".to_string(),
                    ));
                } else {
                    return Ok(Span::new(
                        next.position_range,
                        std::char::from_u32(val).unwrap_or('\u{FFFD}'),
                    ));
                }
            }

            c if c.is_digit(8) => {
                let mut val = c.to_digit(8).unwrap();
                let mut count = 1;
                let mut d: Span<char>;

                while count < 3 {
                    match self.peek() {
                        Some(p) if p.value.is_digit(8) => {
                            d = self.advance().unwrap();
                            val = val * 8 + d.value.to_digit(8).unwrap();
                            count += 1;
                        }
                        _ => break,
                    }
                }
                return Ok(Span::new(
                    next.position_range,
                    std::char::from_u32(val).unwrap_or('\u{FFD}'),
                ));
            }

            other => Err(Span::new(
                next.position_range,
                format!("unkown escape sequence: \\{}", other),
            )),
        };
    }

    pub fn parse_string(&mut self) -> Token {
        let mut position_tracker = self.advance().unwrap();
        let mut string = String::new();

        loop {
            let char = match self.advance() {
                Some(c) => c,
                None => break,
            };

            position_tracker
                .position_range
                .set_end(char.position_range.end);

            if char.value == '\\' {
                match self.parse_escape() {
                    Ok(decoded) => {
                        string.push(decoded.value);
                        position_tracker
                            .position_range
                            .set_end(decoded.position_range.end);
                    }
                    Err(error_message) => {
                        return Token::new(
                            TokenKind::Error(error_message.value),
                            error_message.position_range,
                            string,
                        );
                    }
                }
                continue;
            }

            if char.value == '"' {
                break;
            }

            string.push(char.value);
        }
        return Token::new(
            TokenKind::StringLiteral(string),
            position_tracker.position_range,
            "".to_string(),
        );
    }

    pub fn parse_char(&mut self) -> Token {
        let mut position_tracker = self.advance().unwrap();
        let mut string = String::new();

        string.push(position_tracker.value);

        let char = match self.advance() {
            Some(c) => c,
            None => panic!("something went wrong whilest parsing a char!"),
        };

        position_tracker
            .position_range
            .set_end(char.position_range.end);

        match char.value {
            '\\' => match self.parse_escape() {
                Ok(decoded) => {
                    position_tracker
                        .position_range
                        .set_end(decoded.position_range.end);

                    string.push(decoded.value);

                    return Token::new(
                        TokenKind::StringLiteral(decoded.value.to_string()),
                        position_tracker.position_range,
                        string,
                    );
                }
                Err(error_message) => {
                    return Token::new(
                        TokenKind::Error(error_message.value),
                        error_message.position_range,
                        string,
                    );
                }
            },
            '\'' => {
                return Token::new(
                    TokenKind::CharLiteral("".to_string()),
                    position_tracker.position_range,
                    "".to_string(),
                );
            }

            other => {
                string.push(other);

                match self.advance().unwrap() {
                    c if c.value == '\'' => {
                        position_tracker
                            .position_range
                            .set_end(c.position_range.end);

                        return Token::new(
                            TokenKind::CharLiteral(other.to_string()),
                            position_tracker.position_range,
                            other.to_string(),
                        );
                    }
                    c => {
                        position_tracker
                            .position_range
                            .set_end(c.position_range.end);

                        return Token::new(
                            TokenKind::Error("Char never closes!".to_string()),
                            position_tracker.position_range,
                            string,
                        );
                    }
                }
            }
        }
    }
}
