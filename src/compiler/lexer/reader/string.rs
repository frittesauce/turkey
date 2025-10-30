use crate::{
    common::position::Span,
    compiler::lexer::{reader::Reader, token::TokenKind},
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

    pub fn parse_string(&mut self) -> Span<TokenKind> {
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
                        return Span::new(
                            error_message.position_range,
                            TokenKind::Error(error_message.value),
                        );
                    }
                }
                continue;
            }

            string.push(char.value);
        }

        todo!()
    }
}
