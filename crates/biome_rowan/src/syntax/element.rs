use crate::syntax::SyntaxTrivia;
use crate::{Language, NodeOrToken, SyntaxNode, SyntaxToken, cursor};
use biome_text_size::{TextRange, TextSize};
use std::iter;
use std::ptr::NonNull;

pub type SyntaxElement<L> = NodeOrToken<SyntaxNode<L>, SyntaxToken<L>>;

impl<L: Language> SyntaxElement<L> {
    pub fn key(&self) -> SyntaxElementKey {
        match self {
            Self::Node(it) => it.key(),
            Self::Token(it) => it.key(),
        }
    }

    pub fn text_range(&self) -> TextRange {
        match self {
            Self::Node(it) => it.text_range_with_trivia(),
            Self::Token(it) => it.text_range(),
        }
    }

    pub fn text_trimmed_range(&self) -> TextRange {
        match self {
            Self::Node(it) => it.text_trimmed_range(),
            Self::Token(it) => it.text_trimmed_range(),
        }
    }

    pub fn leading_trivia(&self) -> Option<SyntaxTrivia<L>> {
        match self {
            Self::Node(it) => it.first_leading_trivia(),
            Self::Token(it) => Some(it.leading_trivia()),
        }
    }

    pub fn trailing_trivia(&self) -> Option<SyntaxTrivia<L>> {
        match self {
            Self::Node(it) => it.last_trailing_trivia(),
            Self::Token(it) => Some(it.trailing_trivia()),
        }
    }

    pub fn kind(&self) -> L::Kind {
        match self {
            Self::Node(it) => it.kind(),
            Self::Token(it) => it.kind(),
        }
    }

    pub fn parent(&self) -> Option<SyntaxNode<L>> {
        match self {
            Self::Node(it) => it.parent(),
            Self::Token(it) => it.parent(),
        }
    }

    pub(crate) fn index(&self) -> usize {
        match self {
            Self::Node(it) => it.index(),
            Self::Token(it) => it.index(),
        }
    }

    pub fn ancestors(&self) -> impl Iterator<Item = SyntaxNode<L>> + use<L> {
        let first = match self {
            Self::Node(it) => Some(it.clone()),
            Self::Token(it) => it.parent(),
        };
        iter::successors(first, SyntaxNode::parent)
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
}

impl<L: Language> From<cursor::SyntaxElement> for SyntaxElement<L> {
    fn from(raw: cursor::SyntaxElement) -> Self {
        match raw {
            NodeOrToken::Node(it) => Self::Node(it.into()),
            NodeOrToken::Token(it) => Self::Token(it.into()),
        }
    }
}

impl<L: Language> From<SyntaxElement<L>> for cursor::SyntaxElement {
    fn from(element: SyntaxElement<L>) -> Self {
        match element {
            NodeOrToken::Node(it) => Self::Node(it.into()),
            NodeOrToken::Token(it) => Self::Token(it.into()),
        }
    }
}

impl<L: Language> From<SyntaxToken<L>> for SyntaxElement<L> {
    fn from(token: SyntaxToken<L>) -> Self {
        Self::Token(token)
    }
}

impl<L: Language> From<SyntaxNode<L>> for SyntaxElement<L> {
    fn from(node: SyntaxNode<L>) -> Self {
        Self::Node(node)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SyntaxElementKey {
    node_data: NonNull<()>,
    offset: TextSize,
}

impl SyntaxElementKey {
    pub(crate) fn new(node_data: NonNull<()>, offset: TextSize) -> Self {
        Self { node_data, offset }
    }
}
