use crate::compiler::lexer::{reader::Reader, token::Token};

pub mod reader;
pub mod token;

pub fn lexer(src: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut reader = Reader::new(src);

    println!("going to start looping through chars for the tokens now!");
    loop {
        let token = reader.next();

        println!("{}\n", token);

        tokens.push(token);
    }

    println!("{:?}", tokens);

    return tokens;
}
