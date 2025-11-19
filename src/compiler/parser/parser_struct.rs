use std::collections::HashSet;

use crate::compiler::lexer::token::{Token, TokenKind};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub types: HashSet<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut types = HashSet::new();

        for ty in [
            "void", "bool", "char", "u16", "i16", "u32", "i32", "u64", "i64", "f32", "f64", "f128",
        ] {
            types.insert(ty.to_string());
        }

        return Self { tokens, types };
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

    pub fn is_type(&self, tk: &TokenKind) -> bool {
        use TokenKind::*;

        let is_type = match tk {
            Identifier(id) => self.types.contains(id),
            U16 | I16 | U32 | I32 | U64 | I64 | F32 | F64 | F128 | Void | Bool | Char => true,
            _ => false,
        };

        return is_type;
    }
}
