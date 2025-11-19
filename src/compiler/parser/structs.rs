use crate::common::position::Span;

pub type Identifier = Span<String>;
pub type Type = Span<RawType>;
pub type Parameter = Span<RawParameter>;

#[derive(Debug, PartialEq, Eq)]
pub struct RawParameter {
    pub identifier: Identifier,
    pub ty: Type,
}

#[derive(Eq, PartialEq, Debug)]
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

    Other(Identifier),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Int(i64),
    Float(f64),
    String(String),
    Char(char),
    Bool(bool),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BinaryOperator {
    Add,    // +
    Sub,    // -
    Mul,    // *
    Div,    // /
    Mod,    // %
    Eq,     // ==
    Ne,     // !=
    Lt,     // <
    Gt,     // >
    Le,     // <=
    Ge,     // >=
    And,    // &&
    Or,     // ||
    BitAnd, // &
    BitOr,  // |
    BitXor, // ^
    Shr,    // >>
    Shl,    // <<
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AssignmentOperator {
    Assign,       // =
    AddAssign,    // +=
    SubAssign,    // -=
    MulAssign,    // *=
    DivAssign,    // /=
    ModAssign,    // %=
    BitAndAssign, // &=
    BitOrAssign,  // |=
    BitXorAssign, // ^=
    ShlAssign,    // <<=
    ShrAssign,    // >>=
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum UnaryOperator {
    Neg,     // -
    Not,     // !
    BitNot,  // ~
    Deref,   // *
    AddrOf,  // &
    PreInc,  // ++x
    PreDec,  // --x
    PostInc, // x++
    PostDec, // x--
}
