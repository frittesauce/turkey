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
        ty: Box<Node>,
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

    UnaryOp {
        op: String,
        expr: Box<Node>,
    },
}
