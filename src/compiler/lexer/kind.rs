use crate::{common::position::Span, compiler::lexer::reader::Character};

pub type LocatedString = Span<String>;

#[derive(Debug)]
pub enum LexerKind {
    String(LocatedString),
    Character(LocatedString),
    Identifier(LocatedString),
    Integer(LocatedString),
    Float(LocatedString),
    Operators(Vec<Character>),
}
