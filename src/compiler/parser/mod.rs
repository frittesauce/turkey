use crate::compiler::{lexer::token::Token, parser::node::Node};

pub mod node;
pub mod parser_struct;
pub mod structs;

pub fn pars(mut tokens: Vec<Token>) -> Vec<Node> {
    tokens.reverse();

    let mut parser = parser_struct::Parser::new(tokens);
    let mut nodes: Vec<Node>;

    todo!()
}
