use crate::common::position::Span;

pub type Identifier = Span<String>;
pub type Type = Span<RawType>;

pub enum RawType {
    Void,
    Bool,

    Char,
    String,

    U16,
    S16,
    U32,
    S32,
    U64,
    S64,

    F32,
    F64,
    F128,

    Unsigned(Box<RawType>),
    Signed(Box<RawType>),
}
