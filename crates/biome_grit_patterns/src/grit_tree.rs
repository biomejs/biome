use crate::grit_target_node::GritTargetNode;
use grit_util::Ast;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct GritTree {
    root: GritTargetNode,
}

impl Ast for GritTree {
    type Node<'a> = GritTargetNode
    where
        Self: 'a;

    fn root_node(&self) -> GritTargetNode {
        self.root.clone()
    }
}
