use std::fmt::{self, Display};

use crate::common::position::PositionRange;

#[derive(Debug)]
pub struct Token {
    pub token_kind: TokenKind,
    pub position_range: PositionRange,
    pub string: String,
}

impl Token {
    pub fn new(token_kind: TokenKind, position_range: PositionRange, string: String) -> Self {
        return Self {
            token_kind,
            position_range,
            string,
        };
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Plus,
    Minus,
    Star,
    Slash,
    Modulus,

    PlusPlus,
    MinusMinus,

    Equals,
    PlusEquals,
    MinusEquals,
    StarEquals,
    SlashEquals,
    ModulusEquals,

    EqualsEquals,
    BangEquals,
    Lesser,
    More,
    LesserEquals,
    MoreEquals,

    AndAnd,
    OrOr,
    Bang,

    And,
    Or,
    Xor,
    LeftShift,
    RightShift,
    RightShiftEqual,
    LeftShiftEqual,
    AndEqual,
    OrEqual,
    XorEqual,

    OpenParams,
    CloseParams,
    OpenCurlyBracket,
    CloseCurlyBracket,
    OpenBracket,
    CloseBracket,
    Semicolon,
    Comma,
    Tilda,
    Colon,
    Period,
    Arrow,
    FatArrow,
    Hashtag,

    Eof,

    KeyWord(&'static str),
    Identifier(String),
    IntLiteral(String),
    FloatLiteral(String),
    CharLiteral(String),
    StringLiteral(String),

    Error(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Token: {}\nRaw: {}\nLocation: {}",
            self.token_kind, self.string, self.position_range
        )
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenKind::*;
        match self {
            KeyWord(s) => write!(f, "Keyword: {}", s),
            Identifier(s) => write!(f, "Identifier: {}", s),
            IntLiteral(s) => write!(f, "Int: {}", s),
            FloatLiteral(s) => write!(f, "Float: {}", s),
            CharLiteral(s) => write!(f, "Char: {}", s),
            StringLiteral(s) => write!(f, "String: {}", s),

            Error(s) => write!(f, "<error {}>", s),

            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Star => write!(f, "*"),
            Slash => write!(f, "/"),
            Modulus => write!(f, "%"),

            PlusPlus => write!(f, "++"),
            MinusMinus => write!(f, "--"),

            Equals => write!(f, "="),
            PlusEquals => write!(f, "+="),
            MinusEquals => write!(f, "-="),
            StarEquals => write!(f, "*="),
            SlashEquals => write!(f, "/="),
            ModulusEquals => write!(f, "%="),

            EqualsEquals => write!(f, "=="),
            BangEquals => write!(f, "!="),
            Lesser => write!(f, "<"),
            More => write!(f, ">"),
            LesserEquals => write!(f, "<="),
            MoreEquals => write!(f, ">="),

            AndAnd => write!(f, "&&"),
            OrOr => write!(f, "||"),
            Bang => write!(f, "!"),

            And => write!(f, "&"),
            Or => write!(f, "|"),
            Xor => write!(f, "^"),
            LeftShift => write!(f, "<<"),
            RightShift => write!(f, ">>"),

            RightShiftEqual => write!(f, ">>="),
            LeftShiftEqual => write!(f, "<<="),
            AndEqual => write!(f, "&="),
            OrEqual => write!(f, "|="),
            XorEqual => write!(f, "^="),

            OpenParams => write!(f, "("),
            CloseParams => write!(f, ")"),
            OpenCurlyBracket => write!(f, "{{"),
            CloseCurlyBracket => write!(f, "}}"),
            OpenBracket => write!(f, "["),
            CloseBracket => write!(f, "]"),

            Semicolon => write!(f, ";"),
            Comma => write!(f, ","),
            Tilda => write!(f, "~"),
            Colon => write!(f, ":"),
            Period => write!(f, "."),
            Arrow => write!(f, "->"),
            FatArrow => write!(f, "=>"),
            Hashtag => write!(f, "#"),

            Eof => write!(f, "<eof>"),
        }
    }
}
