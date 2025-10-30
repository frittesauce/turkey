use crate::compiler::lexer::{reader::Reader, token::Token};

pub mod reader;
pub mod token;

pub fn lexer(src: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut reader = Reader::new(src);

    loop {
        let token = match reader.next() {
            Some(tk) => tk,
            None => break,
        };

        tokens.push(token);
    }

    println!("{:?}", tokens);

    return tokens;
}
