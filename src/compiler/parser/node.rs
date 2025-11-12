use crate::common::position::Span;

pub type Node = Span<RawNode>;

#[derive(Debug, PartialEq, Eq)]
pub enum RawNode {
    Function {
        identifier: Identifier,
        paramaters: Vec<Paramaters>,
        return_type: Type,
        node: Box<Node>,
    },
}
