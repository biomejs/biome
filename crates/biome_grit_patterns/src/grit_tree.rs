use crate::grit_target_node::{GritTargetLanguageNode, GritTargetNode};
use biome_diagnostics::{LineIndex, LineIndexBuf};
use grit_util::Ast;
use std::borrow::Cow;
use std::sync::OnceLock;

#[derive(Clone, Debug)]
pub struct GritTargetTree {
    root: GritTargetLanguageNode,
    source: String,

    /// Caches the line starts of `source`, computed on first use. Every
    /// binding converted to a line and column position reads them, so
    /// building them once per tree avoids rescanning the source per binding.
    line_index: OnceLock<LineIndexBuf>,
}

impl GritTargetTree {
    pub fn new(root: GritTargetLanguageNode) -> Self {
        let source = root.owned_text().into_owned();
        Self {
            root,
            source,
            line_index: OnceLock::new(),
        }
    }

    pub fn text(&self) -> &str {
        &self.source
    }

    pub fn line_index(&self) -> &LineIndex {
        self.line_index
            .get_or_init(|| LineIndexBuf::from_source_text(&self.source))
    }
}

impl PartialEq for GritTargetTree {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root && self.source == other.source
    }
}

impl Ast for GritTargetTree {
    type Node<'a>
        = GritTargetNode<'a>
    where
        Self: 'a;

    fn root_node(&self) -> GritTargetNode<'_> {
        GritTargetNode::new(self.root.clone(), self)
    }

    fn source(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.source)
    }
}
