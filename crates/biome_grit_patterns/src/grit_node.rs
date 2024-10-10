use crate::util::TextRangeGritExt;
use biome_grit_syntax::GritSyntaxNode;
use grit_util::{error::GritResult, AstCursor, AstNode as GritAstNode, ByteRange, CodeRange};
use std::{borrow::Cow, ops::Deref};

/// Wrapper around `GritSyntaxNode` as produced by our internal Grit parser.
///
/// This enables us to implement the [`GritAstNode`] trait on Grit nodes, which
/// offers a bunch of utilities used by our node compilers.
#[derive(Clone, Debug, PartialEq)]
pub struct GritNode(GritSyntaxNode);

impl Deref for GritNode {
    type Target = GritSyntaxNode;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<GritSyntaxNode> for GritNode {
    fn from(value: GritSyntaxNode) -> Self {
        Self(value)
    }
}

impl From<&GritSyntaxNode> for GritNode {
    fn from(value: &GritSyntaxNode) -> Self {
        Self(value.clone())
    }
}

impl GritAstNode for GritNode {
    fn ancestors(&self) -> impl Iterator<Item = Self> {
        AncestorIterator::new(self)
    }

    fn children(&self) -> impl Iterator<Item = Self> {
        ChildrenIterator::new(self)
    }

    fn parent(&self) -> Option<Self> {
        self.0.parent().map(Into::into)
    }

    fn next_named_node(&self) -> Option<Self> {
        let mut current_node = Cow::Borrowed(&self.0);
        loop {
            if let Some(sibling) = current_node.next_sibling() {
                return Some(sibling.into());
            }
            current_node = Cow::Owned(current_node.parent()?);
        }
    }

    fn previous_named_node(&self) -> Option<Self> {
        let mut current_node = Cow::Borrowed(&self.0);
        loop {
            if let Some(sibling) = current_node.prev_sibling() {
                return Some(sibling.into());
            }
            current_node = Cow::Owned(current_node.parent()?);
        }
    }

    fn next_sibling(&self) -> Option<Self> {
        self.0.next_sibling().map(Into::into)
    }

    fn previous_sibling(&self) -> Option<Self> {
        self.0.prev_sibling().map(Into::into)
    }

    fn text(&self) -> GritResult<Cow<str>> {
        Ok(Cow::Owned(self.0.text_trimmed().to_string()))
    }

    fn byte_range(&self) -> ByteRange {
        self.0.text_trimmed_range().to_byte_range()
    }

    fn code_range(&self) -> CodeRange {
        let range = self.0.text_trimmed_range();
        CodeRange {
            start: range.start().into(),
            end: range.end().into(),
            // Code ranges contain an address so they can quickly check whether
            // a particular binding belongs to a given range or not.
            address: self
                .0
                .first_token()
                .map(|token| token.text().as_ptr() as usize)
                .unwrap_or_default(),
        }
    }

    fn walk(&self) -> impl AstCursor<Node = Self> {
        GritNodeCursor::new(self)
    }
}

#[derive(Clone)]
pub struct AncestorIterator {
    node: Option<GritNode>,
}

impl AncestorIterator {
    fn new(node: &GritNode) -> Self {
        Self {
            node: Some(node.clone()),
        }
    }
}

impl Iterator for AncestorIterator {
    type Item = GritNode;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node.clone()?;
        self.node = node.parent();
        Some(node)
    }
}

pub struct ChildrenIterator {
    cursor: Option<GritNodeCursor>,
}

impl ChildrenIterator {
    fn new(node: &GritNode) -> Self {
        let mut cursor = GritNodeCursor::new(node);
        Self {
            cursor: cursor.goto_first_child().then_some(cursor),
        }
    }
}

impl Iterator for ChildrenIterator {
    type Item = GritNode;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.cursor.as_mut()?;
        let node = c.node();
        if !c.goto_next_sibling() {
            self.cursor = None;
        }
        Some(node)
    }
}

#[derive(Clone)]
struct GritNodeCursor {
    root: GritNode,
    current: GritNode,
}

impl GritNodeCursor {
    fn new(node: &GritNode) -> Self {
        Self {
            root: node.clone(),
            current: node.clone(),
        }
    }
}

impl AstCursor for GritNodeCursor {
    type Node = GritNode;

    fn goto_first_child(&mut self) -> bool {
        match self.current.first_child() {
            Some(child) => {
                self.current = child.into();
                true
            }
            None => false,
        }
    }

    fn goto_parent(&mut self) -> bool {
        if self.current == self.root {
            return false;
        }

        match self.current.parent() {
            Some(parent) => {
                self.current = parent;
                true
            }
            None => false,
        }
    }

    fn goto_next_sibling(&mut self) -> bool {
        if self.current == self.root {
            return false;
        }

        match self.current.next_sibling() {
            Some(sibling) => {
                self.current = sibling;
                true
            }
            None => false,
        }
    }

    fn node(&self) -> Self::Node {
        self.current.clone()
    }
}
