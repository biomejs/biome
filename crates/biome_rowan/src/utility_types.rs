use std::{fmt, ops::Deref};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeOrToken<N, T> {
    Node(N),
    Token(T),
}

impl<N, T> NodeOrToken<N, T> {
    pub fn into_node(self) -> Option<N> {
        match self {
            Self::Node(node) => Some(node),
            Self::Token(_) => None,
        }
    }

    pub fn into_token(self) -> Option<T> {
        match self {
            Self::Node(_) => None,
            Self::Token(token) => Some(token),
        }
    }

    pub fn as_node(&self) -> Option<&N> {
        match self {
            Self::Node(node) => Some(node),
            Self::Token(_) => None,
        }
    }

    pub fn as_token(&self) -> Option<&T> {
        match self {
            Self::Node(_) => None,
            Self::Token(token) => Some(token),
        }
    }
}

impl<N: Deref, T: Deref> NodeOrToken<N, T> {
    pub(crate) fn as_deref(&self) -> NodeOrToken<&N::Target, &T::Target> {
        match self {
            Self::Node(node) => NodeOrToken::Node(&**node),
            Self::Token(token) => NodeOrToken::Token(&**token),
        }
    }
}

impl<N: fmt::Display, T: fmt::Display> fmt::Display for NodeOrToken<N, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Node(node) => fmt::Display::fmt(node, f),
            Self::Token(token) => fmt::Display::fmt(token, f),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Next,
    Prev,
}

/// `WalkEvent` describes tree walking process.
#[derive(Debug, Copy, Clone)]
pub enum WalkEvent<T> {
    /// Fired before traversing the node.
    Enter(T),
    /// Fired after the node is traversed.
    Leave(T),
}

impl<T> WalkEvent<T> {
    pub fn map<F: FnOnce(T) -> U, U>(self, f: F) -> WalkEvent<U> {
        match self {
            Self::Enter(it) => WalkEvent::Enter(f(it)),
            Self::Leave(it) => WalkEvent::Leave(f(it)),
        }
    }
}

/// There might be zero, one or two leaves at a given offset.
#[derive(Clone, Debug)]
pub enum TokenAtOffset<T> {
    /// No leaves at offset -- possible for the empty file.
    None,
    /// Only a single leaf at offset.
    Single(T),
    /// Offset is exactly between two leaves.
    Between(T, T),
}

impl<T> TokenAtOffset<T> {
    pub fn map<F: Fn(T) -> U, U>(self, f: F) -> TokenAtOffset<U> {
        match self {
            Self::None => TokenAtOffset::None,
            Self::Single(it) => TokenAtOffset::Single(f(it)),
            Self::Between(l, r) => TokenAtOffset::Between(f(l), f(r)),
        }
    }

    /// Convert to option, preferring the right leaf in case of a tie.
    pub fn right_biased(self) -> Option<T> {
        match self {
            Self::None => None,
            Self::Single(node) => Some(node),
            Self::Between(_, right) => Some(right),
        }
    }

    /// Convert to option, preferring the left leaf in case of a tie.
    pub fn left_biased(self) -> Option<T> {
        match self {
            Self::None => None,
            Self::Single(node) => Some(node),
            Self::Between(left, _) => Some(left),
        }
    }
}

impl<T> Iterator for TokenAtOffset<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match std::mem::replace(self, Self::None) {
            Self::None => None,
            Self::Single(node) => {
                *self = Self::None;
                Some(node)
            }
            Self::Between(left, right) => {
                *self = Self::Single(right);
                Some(left)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::None => (0, Some(0)),
            Self::Single(_) => (1, Some(1)),
            Self::Between(_, _) => (2, Some(2)),
        }
    }
}

impl<T> ExactSizeIterator for TokenAtOffset<T> {}

#[cfg(target_pointer_width = "64")]
#[macro_export]
macro_rules! static_assert {
    ($expr:expr) => {
        const _: i32 = 0 / $expr as i32;
    };
}

#[cfg(target_pointer_width = "64")]
pub use static_assert;
