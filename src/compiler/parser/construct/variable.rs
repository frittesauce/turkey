use crate::compiler::{
    lexer::token::{Token, TokenKind},
    parser::{node::Node, parser_struct::Parser},
};

impl Parser {
    pub fn construct_var(&mut self) -> Option<Node> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut pos = self.advance().unwrap();
        tokens.push(pos.clone());

        let next = match self.peek() {
            Some(tk) => tk,
            None => return None,
        };

        match next.token_kind {
            TokenKind::Identifier(_) => {
                let tk = self.advance().unwrap();
                pos.position_range.set_end(tk.position_range.end);
                tokens.push(tk);
            }
            _ => return None,
        }

        todo!();
    }
}
