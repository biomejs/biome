use crate::grit_target_node::{GritTargetLanguageNode, GritTargetNode};
use grit_util::Ast;
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq)]
pub struct GritTargetTree {
    root: GritTargetLanguageNode,
    source: String,
}

impl GritTargetTree {
    pub fn new(root: GritTargetLanguageNode) -> Self {
        let source = root.owned_text().into_owned();
        Self { root, source }
    }

    pub fn text(&self) -> &str {
        &self.source
    }
}

impl Ast for GritTargetTree {
    type Node<'a>
        = GritTargetNode<'a>
    where
        Self: 'a;

    fn root_node(&self) -> GritTargetNode {
        GritTargetNode::new(self.root.clone(), self)
    }

    fn source(&self) -> Cow<str> {
        Cow::Borrowed(&self.source)
    }
}
