use crate::grit_node::GritNode;
use grit_util::Ast;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct GritTree;

impl Ast for GritTree {
    type Node<'a> = GritNode
    where
        Self: 'a;

    fn root_node(&self) -> GritNode {
        todo!()
    }
}
