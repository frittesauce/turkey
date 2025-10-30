use crate::compiler::lexer::{reader::Reader, token::Token};

pub mod reader;
pub mod token;

pub fn lexer(src: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let reader = Reader::new(src);

    return tokens;
}
