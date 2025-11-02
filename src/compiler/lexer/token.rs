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
    Dollar,

    Eof,

    // keywords!
    Auto,
    Break,
    Continue,
    Case,
    Default,
    If,
    Static,
    Float,
    For,
    Goto,
    Return,
    Double,
    Else,
    Enum,
    While,
    Unsigned,
    Switch,
    Sizeof,
    Union,
    Struct,
    Signed,
    Typedef,
    Extern,
    Short,
    Void,
    Volatile,
    Char,
    Int,
    Long,
    Register,
    Do,

    Fun,
    Returns,
    Let,
    Const,

    Identifier(String),
    IntLiteral(String),
    FloatLiteral(String),
    CharLiteral(String),
    StringLiteral(String),

    Comment(String),
    Error(String),
    Unknown,
}

pub fn match_keyword(str: &str) -> Option<TokenKind> {
    use TokenKind::*;

    let kind = match str {
        "auto" => Auto,
        "break" => Break,
        "continue" => Continue,
        "case" => Case,
        "default" => Default,
        "if" => If,
        "static" => Static,
        "float" => Float,
        "for" => For,
        "goto" => Goto,
        "return" => Return,
        "double" => Double,
        "else" => Else,
        "enum" => Enum,
        "while" => While,
        "unsigned" => Unsigned,
        "switch" => Switch,
        "sizeof" => Sizeof,
        "union" => Union,
        "struct" => Struct,
        "signed" => Signed,
        "typedef" => Typedef,
        "extern" => Extern,
        "short" => Short,
        "void" => Void,
        "volatile" => Volatile,
        "char" => Char,
        "int" => Int,
        "long" => Long,
        "register" => Register,
        "do" => Do,
        "fun" => Fun,
        "returns" => Returns,
        "let" => Let,
        "const" => Const,
        _ => return None,
    };

    return Some(kind);
}

pub fn match_operator(str: &str) -> Option<TokenKind> {
    use TokenKind::*;

    let kind = match str {
        "+" => Plus,
        "-" => Minus,
        "*" => Star,
        "/" => Slash,
        "%" => Modulus,

        "++" => PlusPlus,
        "--" => MinusMinus,

        "=" => Equals,
        "+=" => PlusEquals,
        "-=" => MinusEquals,
        "*=" => StarEquals,
        "/=" => SlashEquals,
        "%=" => ModulusEquals,

        "==" => EqualsEquals,
        "!=" => BangEquals,
        "<" => Lesser,
        ">" => More,
        "<=" => LesserEquals,
        ">=" => MoreEquals,

        "&&" => AndAnd,
        "||" => OrOr,
        "!" => Bang,

        "&" => And,
        "|" => Or,
        "^" => Xor,
        "<<" => LeftShift,
        ">>" => RightShift,
        ">>=" => RightShiftEqual,
        "<<=" => LeftShiftEqual,
        "&=" => AndEqual,
        "|=" => OrEqual,
        "^=" => XorEqual,

        "(" => OpenParams,
        ")" => CloseParams,
        "{" => OpenCurlyBracket,
        "}" => CloseCurlyBracket,
        "[" => OpenBracket,
        "]" => CloseBracket,

        ";" => Semicolon,
        "," => Comma,
        "~" => Tilda,
        ":" => Colon,
        "." => Period,
        "->" => Arrow,
        "=>" => FatArrow,
        "#" => Hashtag,
        "$" => Dollar,

        _ => return None,
    };

    return Some(kind);
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
            Identifier(s) => write!(f, "Identifier: {}", s),
            IntLiteral(s) => write!(f, "Int: {}", s),
            FloatLiteral(s) => write!(f, "Float: {}", s),
            CharLiteral(s) => write!(f, "Char: {}", s),
            StringLiteral(s) => write!(f, "String: {}", s),

            Comment(s) => write!(f, "<Comment {}>", s),
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
            Dollar => write!(f, "$"),

            Auto => write!(f, "auto"),
            Break => write!(f, "break"),
            Continue => write!(f, "continue"),
            Case => write!(f, "case"),
            Default => write!(f, "default"),
            If => write!(f, "if"),
            Static => write!(f, "static"),
            Float => write!(f, "float"),
            For => write!(f, "for"),
            Goto => write!(f, "goto"),
            Return => write!(f, "return"),
            Double => write!(f, "double"),
            Else => write!(f, "else"),
            Enum => write!(f, "enum"),
            While => write!(f, "while"),
            Unsigned => write!(f, "unsigned"),
            Switch => write!(f, "switch"),
            Sizeof => write!(f, "sizeof"),
            Union => write!(f, "union"),
            Struct => write!(f, "struct"),
            Signed => write!(f, "signed"),
            Typedef => write!(f, "typedef"),
            Extern => write!(f, "extern"),
            Short => write!(f, "short"),
            Void => write!(f, "void"),
            Volatile => write!(f, "volatile"),
            Char => write!(f, "char"),
            Int => write!(f, "int"),
            Long => write!(f, "long"),
            Register => write!(f, "register"),
            Do => write!(f, "do"),
            Fun => write!(f, "fun"),
            Returns => write!(f, "return"),
            Let => write!(f, "let"),
            Const => write!(f, "const"),

            Eof => write!(f, "<eof>"),
            Unknown => write!(f, "unkown charcter!"),
        }
    }
}
