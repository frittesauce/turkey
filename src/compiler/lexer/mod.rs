use crate::{
    common::position::Position,
    compiler::lexer::{
        kind::LexerKind,
        reader::Reader,
        token::{Token, TokenType},
    },
};
pub mod kind;
pub mod reader;
pub mod token;

pub fn lexer(source: &str) -> Vec<Token> {
    let mut reader = Reader::new(source);
    let mut tokens: Vec<Token> = Vec::new();

    println!("{:#?}", reader.chars);

    loop {
        let king = match reader.next() {
            Some(k) => k,
            None => break,
        };

        let token = match king {
            LexerKind::String(str) => Token::new(TokenType::Text, str.position, str.raw),
            LexerKind::Identifier(str) => Token::new(TokenType::Identifier, str.position, str.raw),
            LexerKind::Integer(str) => Token::new(TokenType::Integer, str.position, str.raw),
            LexerKind::Operators(mut ops) => Token::new(
                TokenType::OpenCurlyBracket,
                ops.pop().unwrap().position,
                "those who sigma!".to_string(),
            ),
        };

        tokens.push(token);
    }

    println!("{:#?}", tokens);

    todo!()
}
