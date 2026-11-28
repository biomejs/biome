use crate::{
    NodeOrToken, TextSize,
    green::{GreenNode, GreenNodeRef, GreenToken, GreenTokenRef, RawSyntaxKind},
};

pub(crate) type GreenElement = NodeOrToken<GreenNode, GreenToken>;
pub(crate) type GreenElementRef<'a> = NodeOrToken<GreenNodeRef<'a>, GreenTokenRef<'a>>;

impl From<GreenNode> for GreenElement {
    #[inline]
    fn from(node: GreenNode) -> Self {
        Self::Node(node)
    }
}

impl<'a> From<&'a GreenNode> for GreenElementRef<'a> {
    #[inline]
    fn from(node: &'a GreenNode) -> Self {
        NodeOrToken::Node(node.as_ref())
    }
}

impl From<GreenToken> for GreenElement {
    #[inline]
    fn from(token: GreenToken) -> Self {
        Self::Token(token)
    }
}

impl<'a> From<&'a GreenToken> for GreenElementRef<'a> {
    #[inline]
    fn from(token: &'a GreenToken) -> Self {
        NodeOrToken::Token(token.as_ref())
    }
}

impl GreenElementRef<'_> {
    #[inline]
    pub fn to_owned(self) -> GreenElement {
        match self {
            NodeOrToken::Node(it) => NodeOrToken::Node(it.to_owned()),
            NodeOrToken::Token(it) => NodeOrToken::Token(it.to_owned()),
        }
    }
}

impl GreenElement {
    /// Borrows the element without incrementing its reference count.
    #[inline]
    pub(crate) fn as_ref(&self) -> GreenElementRef<'_> {
        match self {
            Self::Node(node) => NodeOrToken::Node(node.as_ref()),
            Self::Token(token) => NodeOrToken::Token(token.as_ref()),
        }
    }

    /// Returns kind of this element.
    #[inline]
    pub fn kind(&self) -> RawSyntaxKind {
        self.as_ref().kind()
    }

    /// Returns the length of the text covered by this element.
    #[inline]
    pub fn text_len(&self) -> TextSize {
        self.as_ref().text_len()
    }
}

impl GreenElementRef<'_> {
    /// Returns kind of this element.
    #[inline]
    pub fn kind(&self) -> RawSyntaxKind {
        match self {
            NodeOrToken::Node(it) => it.kind(),
            NodeOrToken::Token(it) => it.kind(),
        }
    }

    /// Returns the length of the text covered by this element.
    #[inline]
    pub fn text_len(self) -> TextSize {
        match self {
            NodeOrToken::Node(it) => it.text_len(),
            NodeOrToken::Token(it) => it.text_len(),
        }
    }
}
