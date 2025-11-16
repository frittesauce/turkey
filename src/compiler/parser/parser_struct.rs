use crate::compiler::lexer::token::Token;

pub struct Parser {
    pub tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        return Self { tokens };
    }

    pub fn advance(&mut self) -> Option<Token> {
        self.tokens.pop()
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.last()
    }

    pub fn advance_if<C>(&mut self, condition: C) -> Option<Token>
    where
        C: FnOnce(&Token) -> bool,
    {
        let peek = match self.peek() {
            Some(p) => p,
            None => return None,
        };

        if condition(&peek) {
            return self.advance();
        }
        return None;
    }
}
