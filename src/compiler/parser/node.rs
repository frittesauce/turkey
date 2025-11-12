use crate::{
    common::position::Span,
    compiler::parser::structs::{Identifier, Parameter, Type},
};

pub type Node = Span<RawNode>;

#[derive(Debug, PartialEq, Eq)]
pub enum RawNode {
    FunctionDecl {
        identifier: Identifier,
        paramaters: Vec<Parameter>,
        return_type: Type,
        body: Box<Node>,
    },

    VarDecl {
        is_const: bool,
        identifier: Identifier,
        var_type: Type,
        value: Option<Box<Node>>,
    },

    TypeDef {
        identifier: Identifier,
        ty: Type,
    },

    StructDecl {
        identifier: Identifier,
        fields: Vec<Node>,
    },

    Scope(Vec<Node>),

    If {
        condition: Box<Node>,
        body: Box<Node>,
        else_body: Option<Box<Node>>,
    },

    While {
        condition: Box<Node>,
        body: Box<Node>,
    },

    Return(Option<Box<Node>>),

    BinaryOp {
        left: Box<Node>,
        op: String,
        right: Box<Node>,
    },

    Asignment {
        target: Box<Node>,
        op: AsignmentOperator,
        value: Box<Node>,
    },

    UnaryOp {
        op: String,
        expr: Box<Node>,
    },

    Call {
        identifier: Identifier,
        argments: Vec<Node>,
    },
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
pub enum AsignmentOperator {
    Assign,       // =
    AddAssign,    // +=
    SubAssign,    // -=
    MulAssign,    // *=
    DivAssing,    // /=
    ModAssing,    // %=
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
