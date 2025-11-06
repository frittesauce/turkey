use crate::common::position::Span;

pub type Identifier = Span<String>;
pub type Type = Span<RawType>;

pub enum RawType {
    Void,
    Bool,

    Char,
    String,

    Int,

    Float,
    Double,

    Short(Box<RawType>),
    Long(Box<RawType>),
    Unsigned(Box<RawType>),
    Signed(Box<RawType>),
}
