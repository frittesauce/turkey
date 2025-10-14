use crate::{
    common::position::Position,
    compiler::lexer::{reader::Reader, token::Token},
};
pub mod kind;
pub mod reader;
pub mod token;

pub fn lexer(source: &str) -> Vec<Token> {
    let mut reader = Reader::new(source);
    let mut token: Vec<Token> = Vec::new();

    println!("{:#?}", reader.chars);

    loop {
        println!("{:#?}", reader.next());
    }

    todo!()
}
