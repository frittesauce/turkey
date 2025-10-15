use crate::common::position::PositionRange;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub position: PositionRange,
    pub string: String,
}

impl Token {
    pub fn new(token_type: TokenType, position: PositionRange, string: String) -> Self {
        return Self {
            token_type,
            position,
            string,
        };
    }
}

#[derive(Debug)]
pub enum TokenType {
    EOF,
    Unknown,

    OpenCurlyBracket,  // {
    CloseCurlyBracket, // }
    CloseParen,        // )
    CloseBracket,      // ]
    OpenBracket,       // [
    OpenParen,         // (

    Pub,    // pub
    Extern, // extern
    Async,  // async
    Unsafe, // unsafe
    Static, // Static

    Import,  // import
    Use,     // use
    Mutable, // mut
    Var,     // var
    Enum,    // enum
    Struct,  // struct

    Return, // return
    Result, // result

    Ampersand,       // &
    CommercialAt,    // @
    NumberSign,      // #
    SemiColon,       // ;
    DoubleColon,     // ::
    Dot,             // .
    Underscore,      // _
    Colon,           // :
    Equals,          // =
    Comma,           // ,
    ExclamationMark, // !
    Arrow,           // ->
    FatArrow,        // =>

    SelfKeyword, // self
    Super,       // super

    If,     // if
    ElseIf, // else if
    Else,   // else

    LeftBitshift,  // <<
    RightBitshift, // >>
    Plus,          // +
    Minus,         // -
    ForwardSlash,  // /
    Asterisk,      // *
    Percent,       // %
    // Increment,     // ++
    // Decrement,     // --
    Apostrophe, // '

    Loop,     // loop
    While,    // while
    Continue, // continue
    Break,    // break
    Function, // func

    Range,               // ..
    RangeEquals,         // ..=
    Compare,             // ==
    NotEquals,           // !=
    LessThan,            // <
    LessThanOrEquals,    // <=
    GreaterThan,         // >
    GreaterThanOrEquals, // >=
    And,                 // &&
    Or,                  // ||

    PlusEquals,      // +=
    SubtractEquals,  // -=
    DivideEquals,    // /=
    MultiplyEquals,  // *=
    RemainderEquals, // %=

    False,      // false
    True,       // true
    Character,  // char
    Text,       // string
    Integer,    // int
    Float,      // float
    Identifier, // identifier
}
