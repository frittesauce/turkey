use crate::compiler::{lexer::token::Token, parser::node::Node};

pub mod node;
pub mod pars;
pub mod structs;

pub fn parser(mut tokens: Vec<Token>) -> Vec<Node> {
    tokens.reverse();
    println!("parsing shit");

    let mut parser = pars::Parser::new(tokens);
    let mut nodes: Vec<Node> = Vec::new();

    loop {
        let peek = parser.peek() {
            
        }
    }

    return nodes;
}
