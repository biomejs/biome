use crate::green::{GreenElement, GreenElementRef};
use crate::{GreenNode, GreenNodeData, GreenToken, GreenTokenData, NodeOrToken, SyntaxKind};
use std::marker::PhantomData;

/// New-type wrapper around a `GreenNode`.
///
/// Allows third-party crates to access limited information about a `GreenNode` or construct
/// a `GreenNode` in a limited places.
///
/// While a factory makes a node, the handle doesn't count a reference to its element, see
/// `green/arena.rs`, so a node the factory drops is discarded instead.
#[derive(Debug)]
pub struct RawSyntaxNode<K: SyntaxKind> {
    /// Only `None` once moved out.
    raw: Option<GreenNode>,
    ph: PhantomData<K>,
}

impl<K: SyntaxKind> RawSyntaxNode<K> {
    /// Creates a new node with the given `kind` and `slots`.
    #[inline]
    pub fn new<I>(kind: K, slots: I) -> Self
    where
        I: IntoIterator<Item = Option<RawSyntaxElement<K>>>,
        I::IntoIter: ExactSizeIterator,
    {
        let slots = slots
            .into_iter()
            .map(|slot| slot.map(|element| element.into_green()));
        // A tree builder makes its arena current while a factory makes a node.
        let raw = crate::green::new_node_in_current(kind.to_raw(), slots);
        Self::from(raw)
    }

    #[inline]
    pub fn kind(&self) -> K {
        K::from_raw(self.green().kind())
    }

    #[inline]
    fn green(&self) -> &GreenNode {
        self.raw.as_ref().expect("the node was moved out")
    }

    /// Unwraps this raw syntax into it's underlying green node.
    #[inline]
    pub(crate) fn into_green(mut self) -> GreenNode {
        self.raw.take().expect("the node was moved out")
    }
}

impl<K: SyntaxKind> Drop for RawSyntaxNode<K> {
    #[inline]
    fn drop(&mut self) {
        if let Some(node) = self.raw.take() {
            crate::green::discard_element(NodeOrToken::Node(node));
        }
    }
}

impl<K: SyntaxKind> From<GreenNode> for RawSyntaxNode<K> {
    #[inline]
    fn from(node: GreenNode) -> Self {
        Self {
            raw: Some(node),
            ph: PhantomData,
        }
    }
}

/// New-type wrapper around a `GreenToken`. Allows third-party crates to access limited information
/// on not yet fully constructed nodes.
///
/// As [RawSyntaxNode], a token a factory drops is discarded.
#[derive(Debug)]
pub struct RawSyntaxToken<K: SyntaxKind> {
    /// Only `None` once moved out.
    raw: Option<GreenToken>,
    ph: PhantomData<K>,
}

impl<K: SyntaxKind> RawSyntaxToken<K> {
    #[inline]
    pub fn kind(&self) -> K {
        K::from_raw(self.raw.as_ref().expect("the token was moved out").kind())
    }

    #[inline]
    fn into_green(mut self) -> GreenToken {
        self.raw.take().expect("the token was moved out")
    }
}

impl<K: SyntaxKind> Drop for RawSyntaxToken<K> {
    #[inline]
    fn drop(&mut self) {
        if let Some(token) = self.raw.take() {
            crate::green::discard_element(NodeOrToken::Token(token));
        }
    }
}

impl<K: SyntaxKind> From<GreenToken> for RawSyntaxToken<K> {
    fn from(token: GreenToken) -> Self {
        Self {
            raw: Some(token),
            ph: PhantomData,
        }
    }
}

pub type RawSyntaxElement<K> = NodeOrToken<RawSyntaxNode<K>, RawSyntaxToken<K>>;

impl<K: SyntaxKind> RawSyntaxElement<K> {
    #[inline]
    pub fn kind(&self) -> K {
        match self {
            Self::Node(node) => node.kind(),
            Self::Token(token) => token.kind(),
        }
    }

    #[inline]
    fn into_green(self) -> GreenElement {
        match self {
            Self::Node(node) => NodeOrToken::Node(node.into_green()),
            Self::Token(token) => NodeOrToken::Token(token.into_green()),
        }
    }
}

impl<K: SyntaxKind> From<GreenElement> for RawSyntaxElement<K> {
    #[inline]
    fn from(element: GreenElement) -> Self {
        match element {
            NodeOrToken::Node(node) => Self::Node(RawSyntaxNode::from(node)),
            NodeOrToken::Token(token) => Self::Token(RawSyntaxToken::from(token)),
        }
    }
}

/// New-type wrapper to a reference of a `GreenNode`.
#[derive(Debug)]
pub struct RawSyntaxNodeRef<'a, K: SyntaxKind> {
    raw: &'a GreenNodeData,
    ph: PhantomData<K>,
}

impl<K: SyntaxKind> RawSyntaxNodeRef<'_, K> {
    #[inline]
    pub fn kind(&self) -> K {
        K::from_raw(self.raw.kind())
    }
}

impl<'a, K: SyntaxKind> From<&'a GreenNodeData> for RawSyntaxNodeRef<'a, K> {
    #[inline]
    fn from(node: &'a GreenNodeData) -> Self {
        Self {
            raw: node,
            ph: PhantomData,
        }
    }
}

/// New-type wrapper to a reference of a `GreenToken`
#[derive(Debug)]
pub struct RawSyntaxTokenRef<'a, K: SyntaxKind> {
    raw: &'a GreenTokenData,
    ph: PhantomData<K>,
}

impl<K: SyntaxKind> RawSyntaxTokenRef<'_, K> {
    #[inline]
    pub fn kind(&self) -> K {
        K::from_raw(self.raw.kind())
    }
}

impl<'a, K: SyntaxKind> From<&'a GreenTokenData> for RawSyntaxTokenRef<'a, K> {
    #[inline]
    fn from(token: &'a GreenTokenData) -> Self {
        Self {
            raw: token,
            ph: PhantomData,
        }
    }
}

pub type RawSyntaxElementRef<'a, K> =
    NodeOrToken<RawSyntaxNodeRef<'a, K>, RawSyntaxTokenRef<'a, K>>;

impl<K: SyntaxKind> RawSyntaxElementRef<'_, K> {
    #[inline]
    pub fn kind(&self) -> K {
        match self {
            NodeOrToken::Node(node) => node.kind(),
            NodeOrToken::Token(token) => token.kind(),
        }
    }
}

impl<'a, K: SyntaxKind> From<GreenElementRef<'a>> for RawSyntaxElementRef<'a, K> {
    #[inline]
    fn from(element: GreenElementRef<'a>) -> Self {
        match element {
            NodeOrToken::Node(node) => NodeOrToken::Node(RawSyntaxNodeRef::from(node)),
            NodeOrToken::Token(token) => NodeOrToken::Token(RawSyntaxTokenRef::from(token)),
        }
    }
}
