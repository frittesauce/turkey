use crate::compiler::lexer::{reader::Reader, token::Token};
pub mod reader;
pub mod token;

pub fn lexer(source: &str) -> Vec<Token> {
    let mut reader = Reader::new(source);

    println!("{:#?}", reader.chars);

    todo!()
}
