use crate::compiler::{
    lexer::token::{Token, TokenKind},
    parser::node::Node,
};

pub mod construct;
pub mod node;
pub mod parser_struct;
pub mod structs;

pub fn pars(mut tokens: Vec<Token>) -> Vec<Node> {
    tokens.reverse();
    println!("parsing shit");

    let mut parser = parser_struct::Parser::new(tokens);
    let mut nodes: Vec<Node> = Vec::new();

    loop {
        let peek = match parser.peek() {
            Some(tk) => tk,
            None => break,
        };

        use TokenKind::*;
        let node: Option<Node> = match peek.token_kind {
            tk if parser.is_type(&tk) => parser.construct_var(),

            MultiLineComment(_) | SingleLineComment(_) | _ => {
                parser.advance();
                None
            }
            Eof => break,
        };
    }

    return nodes;
}
