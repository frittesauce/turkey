use crate::{
    common::position::Span,
    compiler::parser::structs::{
        AssignmentOperator, BinaryOperator, Identifier, LiteralValue, Parameter, Type,
        UnaryOperator,
    },
};

pub type Node = Span<RawNode>;

#[derive(Debug, PartialEq)]
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

    StructInit {
        ty: Type,
        fields: Vec<(Identifier, Node)>,
    },

    Scope(Vec<Node>),

    If {
        condition: Box<Node>,
        body: Box<Node>,
        else_body: Option<Box<Node>>,
    },

    Ternary {
        condition: Box<Node>,
        then: Box<Node>,
        esle: Box<Node>,
    },

    While {
        condition: Box<Node>,
        body: Box<Node>,
    },

    For {
        init: Option<Box<Node>>,
        condition: Option<Box<Node>>,
        increment: Option<Box<Node>>,
        body: Box<Node>,
    },

    Return(Option<Box<Node>>),

    BinaryOp {
        left: Box<Node>,
        op: BinaryOperator,
        right: Box<Node>,
    },

    Assignment {
        target: Box<Node>,
        op: AssignmentOperator,
        value: Box<Node>,
    },

    UnaryOp {
        op: UnaryOperator,
        expr: Box<Node>,
    },

    MemberAccess {
        base: Box<Node>,
        field: Identifier,
    },

    Index {
        base: Box<Node>,
        index: Box<Node>,
    },

    IdentifierNode(Identifier),
    Literal(LiteralValue),
    ArrayLiteral(Vec<Node>),

    Call {
        callee: Box<Node>,
        arguments: Vec<Node>,
    },

    Goto {
        label: Identifier,
    },

    Label {
        identifier: Identifier,
        body: Box<Node>,
    },

    SizeOf(Type),

    Break,
    Continue,
}
