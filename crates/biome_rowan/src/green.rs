mod element;
mod node;
mod node_cache;
mod token;
mod trivia;

pub(crate) use self::{
    element::{GreenElement, GreenElementRef},
    node::{Child, Children, GreenNode, GreenNodeData, Slot},
    token::{GreenToken, GreenTokenData},
    trivia::GreenTrivia,
};

/// Summary of trivia that is present in a green element's subtree.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub(crate) struct GreenElementFlags(u8);

impl GreenElementFlags {
    pub(crate) const HAS_COMMENTS: Self = Self(1 << 0);
    pub(crate) const HAS_SKIPPED: Self = Self(1 << 1);
    pub(crate) const HAS_COMMENTS_AND_SKIPPED: Self =
        Self(Self::HAS_COMMENTS.0 | Self::HAS_SKIPPED.0);

    #[inline]
    pub(crate) fn contains(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    #[inline]
    pub(crate) fn contains_all(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    #[inline]
    pub(crate) fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }

    #[inline]
    pub(crate) fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub(crate) fn from_trivia_kind(kind: crate::TriviaPieceKind) -> Self {
        if kind.is_comment() {
            Self::HAS_COMMENTS
        } else if kind.is_skipped() {
            Self::HAS_SKIPPED
        } else {
            Self::default()
        }
    }
}

pub use self::node_cache::NodeCache;
pub(crate) use self::node_cache::NodeCacheNodeEntryMut;

/// RawSyntaxKind is a type tag for each token or node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawSyntaxKind(pub u16);

#[cfg(feature = "countme")]
pub(crate) fn has_live() -> bool {
    node::has_live() || token::has_live() || trivia::has_live()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::green::trivia::GreenTrivia;

    #[test]
    fn assert_send_sync() {
        fn f<T: Send + Sync>() {}
        f::<GreenNode>();
        f::<GreenToken>();
        f::<GreenElement>();
    }

    #[test]
    fn test_size_of() {
        use std::mem::size_of;

        assert_eq!(8, size_of::<GreenNode>());
        assert_eq!(8, size_of::<GreenToken>());
        assert_eq!(8, size_of::<GreenTrivia>());
        assert_eq!(16, size_of::<GreenElement>());
    }
}
