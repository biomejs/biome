use crate::cursor::{SyntaxNode, SyntaxToken};
use crate::green::{GreenElement, GreenElementRef};
use crate::{NodeOrToken, RawSyntaxKind};
use biome_text_size::{TextRange, TextSize};
use std::iter;

pub(crate) type SyntaxElement = NodeOrToken<SyntaxNode, SyntaxToken>;

impl SyntaxElement {
    pub(super) fn new(
        element: GreenElementRef<'_>,
        parent: SyntaxNode,
        slot: u32,
        offset: TextSize,
    ) -> Self {
        match element {
            NodeOrToken::Node(node) => SyntaxNode::new_child(node, parent, slot, offset).into(),
            NodeOrToken::Token(token) => SyntaxToken::new(token, parent, slot, offset).into(),
        }
    }

    #[inline]
    pub fn text_range(&self) -> TextRange {
        match self {
            Self::Node(it) => it.text_range(),
            Self::Token(it) => it.text_range(),
        }
    }

    #[inline]
    pub fn index(&self) -> usize {
        match self {
            Self::Node(it) => it.index(),
            Self::Token(it) => it.index(),
        }
    }

    #[inline]
    pub fn kind(&self) -> RawSyntaxKind {
        match self {
            Self::Node(it) => it.kind(),
            Self::Token(it) => it.kind(),
        }
    }

    #[inline]
    pub fn parent(&self) -> Option<SyntaxNode> {
        match self {
            Self::Node(it) => it.parent(),
            Self::Token(it) => it.parent(),
        }
    }

    #[inline]
    pub fn ancestors(&self) -> impl Iterator<Item = SyntaxNode> + use<> {
        let first = match self {
            Self::Node(it) => Some(it.clone()),
            Self::Token(it) => it.parent(),
        };
        iter::successors(first, SyntaxNode::parent)
    }

    pub fn first_token(&self) -> Option<SyntaxToken> {
        match self {
            Self::Node(it) => it.first_token(),
            Self::Token(it) => Some(it.clone()),
        }
    }
    pub fn last_token(&self) -> Option<SyntaxToken> {
        match self {
            Self::Node(it) => it.last_token(),
            Self::Token(it) => Some(it.clone()),
        }
    }

    pub fn next_sibling_or_token(&self) -> Option<Self> {
        match self {
            Self::Node(it) => it.next_sibling_or_token(),
            Self::Token(it) => it.next_sibling_or_token(),
        }
    }
    pub fn prev_sibling_or_token(&self) -> Option<Self> {
        match self {
            Self::Node(it) => it.prev_sibling_or_token(),
            Self::Token(it) => it.prev_sibling_or_token(),
        }
    }

    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub fn detach(self) -> Self {
        match self {
            Self::Node(it) => Self::Node(it.detach()),
            Self::Token(it) => Self::Token(it.detach()),
        }
    }

    pub(crate) fn into_green(self) -> GreenElement {
        match self {
            Self::Node(it) => it.ptr.into_green(),
            Self::Token(it) => it.into_green(),
        }
    }
}

// #region: impls

impl From<SyntaxNode> for SyntaxElement {
    #[inline]
    fn from(node: SyntaxNode) -> Self {
        Self::Node(node)
    }
}

impl From<SyntaxToken> for SyntaxElement {
    #[inline]
    fn from(token: SyntaxToken) -> Self {
        Self::Token(token)
    }
}

// #endregion
