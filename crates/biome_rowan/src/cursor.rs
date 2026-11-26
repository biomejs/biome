//! Implementation of the cursors -- API for convenient access to syntax trees.
//!
//! Functional programmers will recognize that this module implements a zipper
//! for a purely functional (green) tree.
//!
//! A cursor node (`SyntaxNode`) points to a `GreenNode` and a parent
//! `SyntaxNode`. This allows cursor to provide iteration over both ancestors
//! and descendants, as well as a cheep access to absolute offset of the node in
//! file.
//!

// Implementation notes:
//
// The implementation is utterly and horribly unsafe. This whole module is an
// unsafety boundary. It is believed that the API here is, in principle, sound,
// but the implementation might have bugs.
//
// The core type is `NodeData` -- a heap-allocated reference counted object,
// which points to a green node or a green token, and to the parent `NodeData`.
// Publicly-exposed `SyntaxNode` and `SyntaxToken` own a reference to
// `NodeData` through a `NodeRc`, whose blocks come from a per-thread pool
// (see `node_rc`).
//
// `NodeData`s are transient, and are created and destroyed during tree
// traversals. In general, only currently referenced nodes and their ancestors
// are alive at any given moment.
//
// More specifically, `NodeData`'s ref count is equal to the number of
// outstanding `SyntaxNode` and `SyntaxToken` plus the number of children with
// non-zero ref counts. For example, if the user has only a single `SyntaxNode`
// pointing somewhere in the middle of the tree, then all `NodeData` on the path
// from that point towards the root have ref count equal to one.
//
// `NodeData` which doesn't have a parent (is a root) owns the corresponding
// green node or token, and is responsible for freeing it. For child `NodeData`
// however since they hold a strong reference to their parent node and thus
// to the root, their corresponding green node is guaranteed to be alive as
// a reference cycle to is know to exist (child `NodeData` -> root `NodeData`
// -> root `GreenNode` -> child `GreenNode`) and they can safely use a "weak
// reference" (raw pointer) to the corresponding green node as an optimization
// to avoid having to track atomic references on the traversal hot path

mod element;
mod node;
mod node_rc;
mod token;
mod trivia;

use std::{iter, ops, ptr};

pub(crate) use trivia::{SyntaxTrivia, SyntaxTriviaPiecesIterator};

use crate::cursor::node::Siblings;
use crate::cursor::node_rc::{NodeRc, OwnedOrShared};
pub(crate) use crate::cursor::token::SyntaxToken;
use crate::green::{self, GreenElement, GreenNode, GreenNodeRef, GreenToken, GreenTokenRef};
use crate::{
    NodeOrToken, TextRange, TextSize,
    green::{GreenElementRef, RawSyntaxKind},
};
pub(crate) use element::SyntaxElement;
pub(crate) use node::{
    Preorder, PreorderMatching, PreorderTokens, PreorderWithTokens, SyntaxElementChildren,
    SyntaxNode, SyntaxNodeChildren, SyntaxSlot, SyntaxSlots,
};

#[derive(Debug)]
struct _SyntaxElement;

#[cfg(feature = "countme")]
pub(crate) fn has_live() -> bool {
    countme::get::<_SyntaxElement>().live > 0
}

#[derive(Debug)]
struct NodeData {
    #[cfg(feature = "countme")]
    _c: countme::Count<_SyntaxElement>,

    green: Green,
    /// The parent of a child node, or `None` for a root node.
    parent: Option<NodeRc>,
    slot: u32,

    /// Absolute offset for immutable nodes, unused for mutable nodes.
    offset: TextSize,
}

/// The green element of a [NodeData] (red node).
///
/// A red node is either a "root node" (no parent node and holds a strong
/// reference to the root of the green tree) or a "child node" (holds a strong
/// reference to its parent red node and a weak reference to its counterpart
/// green node)
///
/// Child SyntaxNodes use "unsafe" weak pointers to refer to their green node.
/// Unlike the safe [std::sync::Weak] these are just a raw pointer: the
/// corresponding [ThinArc](crate::arc::ThinArc) doesn't keep a counter of
/// outstanding weak references or defer the release of the underlying memory
/// until the last `Weak` is dropped. On the other hand, a weak reference to a
/// released green node points to deallocated memory and it is undefined
/// behavior to dereference it, but in the context of `NodeData` this is
/// statically known to never happen
#[derive(Debug)]
enum Green {
    // The variants are flat instead of nesting a node-or-token enum in a root
    // and a child variant. Every variant then holds a single pointer, which
    // can be read without checking the variant first. The node variants come
    // before the token variants, so telling nodes from tokens takes a single
    // comparison of the discriminant.
    RootNode(GreenNode),
    ChildNode(ptr::NonNull<()>),
    RootToken(GreenToken),
    ChildToken(ptr::NonNull<()>),
}

impl NodeData {
    #[inline]
    fn new_root(green: GreenElement) -> Self {
        Self {
            #[cfg(feature = "countme")]
            _c: countme::Count::new(),
            green: match green {
                NodeOrToken::Node(green) => Green::RootNode(green),
                NodeOrToken::Token(green) => Green::RootToken(green),
            },
            parent: None,
            slot: 0,
            offset: TextSize::from(0),
        }
    }

    /// Creates the node for `green`, which is the element in the slot `slot`
    /// of the green node of `parent`.
    #[inline]
    fn new_child(green: GreenElementRef<'_>, parent: NodeRc, slot: u32, offset: TextSize) -> Self {
        Self {
            #[cfg(feature = "countme")]
            _c: countme::Count::new(),
            green: match green {
                NodeOrToken::Node(green) => Green::ChildNode(green.as_ptr()),
                NodeOrToken::Token(green) => Green::ChildToken(green.as_ptr()),
            },
            parent: Some(parent),
            slot,
            offset,
        }
    }

    /// Drops the node and returns the reference it held to its parent, if any.
    #[inline]
    fn into_parent(self) -> Option<NodeRc> {
        self.parent
    }

    /// Drops the node and returns an owned handle to its green element.
    #[inline]
    fn into_green(self) -> GreenElement {
        match self.green {
            Green::RootNode(green) => GreenElement::Node(green),
            Green::RootToken(green) => GreenElement::Token(green),
            Green::ChildNode(_) | Green::ChildToken(_) => self.green().to_owned(),
        }
    }

    #[inline]
    fn key(&self) -> (ptr::NonNull<()>, TextSize) {
        let ptr = match self.green() {
            NodeOrToken::Node(node) => node.as_ptr(),
            NodeOrToken::Token(token) => token.as_ptr(),
        };
        (ptr, self.offset())
    }

    #[inline]
    fn parent_node(&self) -> Option<SyntaxNode> {
        debug_assert!(matches!(
            self.parent()?.green(),
            GreenElementRef::Node { .. }
        ));
        self.parent.as_ref().map(|parent| SyntaxNode {
            ptr: parent.clone(),
        })
    }

    #[inline]
    fn parent(&self) -> Option<&Self> {
        self.parent.as_deref()
    }

    #[inline]
    fn green(&self) -> GreenElementRef<'_> {
        match &self.green {
            Green::RootNode(green) => NodeOrToken::Node(green.as_ref()),
            Green::RootToken(green) => NodeOrToken::Token(green.as_ref()),
            // SAFETY: The parent keeps the green tree that contains the
            // element alive for as long as `self` exists.
            Green::ChildNode(green) => NodeOrToken::Node(unsafe { GreenNodeRef::from_ptr(*green) }),
            // SAFETY: See `ChildNode`.
            Green::ChildToken(green) => {
                NodeOrToken::Token(unsafe { GreenTokenRef::from_ptr(*green) })
            }
        }
    }

    /// Returns an iterator over the siblings of this node. The iterator is positioned at the current node.
    #[inline]
    fn green_siblings(&self) -> Option<Siblings<'_>> {
        match self.parent()?.green() {
            GreenElementRef::Node(node) => Some(Siblings::new(node, self.slot())),
            GreenElementRef::Token(_) => {
                debug_assert!(
                    false,
                    "A token should never be a parent of a token or node."
                );
                None
            }
        }
    }
    #[inline]
    fn slot(&self) -> u32 {
        self.slot
    }

    #[inline]
    fn offset(&self) -> TextSize {
        self.offset
    }

    #[inline]
    fn text_range(&self) -> TextRange {
        let offset = self.offset();
        let len = self.green().text_len();
        TextRange::at(offset, len)
    }

    #[inline]
    fn kind(&self) -> RawSyntaxKind {
        self.green().kind()
    }

    fn next_sibling(&self) -> Option<SyntaxNode> {
        let siblings = self.green_siblings()?;
        siblings.following().find_map(|child| {
            child.element().into_node().and_then(|green| {
                let parent = self.parent_node()?;
                let offset = parent.offset() + child.rel_offset();
                Some(SyntaxNode::new_child(green, parent, child.slot(), offset))
            })
        })
    }
    fn prev_sibling(&self) -> Option<SyntaxNode> {
        let siblings = self.green_siblings()?;
        siblings.previous().find_map(|child| {
            child.element().into_node().and_then(|green| {
                let parent = self.parent_node()?;
                let offset = parent.offset() + child.rel_offset();
                Some(SyntaxNode::new_child(green, parent, child.slot(), offset))
            })
        })
    }

    fn next_sibling_or_token(&self) -> Option<SyntaxElement> {
        let siblings = self.green_siblings()?;

        siblings.following().next().and_then(|child| {
            let parent = self.parent_node()?;
            let offset = parent.offset() + child.rel_offset();
            Some(SyntaxElement::new(
                child.element(),
                parent,
                child.slot(),
                offset,
            ))
        })
    }
    fn prev_sibling_or_token(&self) -> Option<SyntaxElement> {
        let siblings = self.green_siblings()?;

        siblings.previous().next().and_then(|child| {
            let parent = self.parent_node()?;
            let offset = parent.offset() + child.rel_offset();
            Some(SyntaxElement::new(
                child.element(),
                parent,
                child.slot(),
                offset,
            ))
        })
    }
}

impl NodeRc {
    fn into_green(self) -> GreenElement {
        match Self::into_data(self) {
            OwnedOrShared::Owned(data) => data.into_green(),
            OwnedOrShared::Shared(ptr) => ptr.green().to_owned(),
        }
    }

    /// Return a clone of this subtree detached from its parent
    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    fn detach(self) -> Self {
        match &self.parent {
            Some(_) => Self::new(NodeData::new_root(self.green().to_owned())),
            // If this node is already detached, increment the reference count and return a clone
            None => self.clone(),
        }
    }

    /// Return a clone of this node with the specified range of slots replaced
    /// with the elements of the provided iterator
    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    fn splice_slots<R, I>(self, range: R, replace_with: I) -> Self
    where
        R: ops::RangeBounds<usize>,
        I: Iterator<Item = Option<green::GreenElement>>,
    {
        let green = match self.green() {
            NodeOrToken::Node(green) => green.splice_slots(range, replace_with).into(),
            NodeOrToken::Token(_) => panic!("called splice_slots on a token node"),
        };

        self.into_root_with_green(green)
    }

    /// Returns a root node holding `green`.
    fn into_root_with_green(mut self, green: GreenElement) -> Self {
        // Try to reuse the underlying memory allocation if self is the only
        // outstanding reference to this NodeData
        match self.get_mut() {
            Some(node) => {
                *node = NodeData::new_root(green);
                self
            }
            None => Self::new(NodeData::new_root(green)),
        }
    }

    /// Return a new version of this node with the element `prev_elem` replaced with `next_elem`
    ///
    /// `prev_elem` can be a direct child of this node, or an indirect child through any descendant node
    ///
    /// Returns `None` if `prev_elem` is not a descendant of this node
    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    fn replace_child(mut self, prev_elem: SyntaxElement, next_elem: SyntaxElement) -> Option<Self> {
        let mut green = next_elem.into_green();
        let mut elem = prev_elem;

        loop {
            let node = elem.parent()?;
            let is_self = node.key() == self.key();

            let index = elem.index();
            let range = index..=index;

            let replace_with = iter::once(Some(green));
            green = node.green().splice_slots(range, replace_with).into();
            elem = node.into();

            if is_self {
                break;
            }
        }

        // Try to reuse the underlying memory allocation if self is the only
        // outstanding reference to this NodeData
        let result = match self.get_mut() {
            Some(node) => {
                *node = NodeData::new_root(green);
                self
            }
            None => Self::new(NodeData::new_root(green)),
        };

        Some(result)
    }
}
