use crate::compiler::lexer::token::Token;

pub mod node;
pub mod structs;

pub fn parser(mut tokens: Vec<Token>) {
    tokens.reverse();
    println!("parsing shit");
}
