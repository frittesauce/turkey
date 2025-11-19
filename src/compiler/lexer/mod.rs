use crate::compiler::lexer::{
    reader::Reader,
    token::{Token, TokenKind},
};

pub mod reader;
pub mod token;

pub fn lexer(src: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut reader = Reader::new(src);

    loop {
        let token = reader.next();

        if token.token_kind == TokenKind::Eof {
            break;
        };

        tokens.push(token);
    }

    tokens
}
