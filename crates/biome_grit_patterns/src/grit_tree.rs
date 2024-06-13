use crate::grit_target_node::GritTargetNode;
use grit_util::Ast;
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq)]
pub struct GritTree {
    root: GritTargetNode,
}

impl GritTree {
    pub fn new(root: GritTargetNode) -> Self {
        Self { root }
    }
}

impl Ast for GritTree {
    type Node<'a> = GritTargetNode
    where
        Self: 'a;

    fn root_node(&self) -> GritTargetNode {
        self.root.clone()
    }

    fn source(&self) -> Cow<str> {
        self.root.text().to_string().into()
    }
}
