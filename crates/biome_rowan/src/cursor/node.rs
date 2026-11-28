use crate::cursor::node_rc::NodeRc;
use crate::cursor::{NodeData, SyntaxElement, SyntaxToken, SyntaxTrivia};
use crate::green::{Child, Children, GreenElementRef, Slot};
use crate::{
    Direction, GreenNode, GreenNodeRef, Language, NodeOrToken, RawSyntaxKind, RewriteSignal,
    SyntaxKindSet, SyntaxNodeText, TokenAtOffset, WalkEvent,
};
use biome_text_size::{TextRange, TextSize};
use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::iter::FusedIterator;
use std::ops;
use std::ptr::NonNull;
use std::{fmt, iter};

use super::GreenElement;

#[derive(Clone)]
pub(crate) struct SyntaxNode {
    pub(super) ptr: NodeRc,
}

impl SyntaxNode {
    pub(crate) fn new_root(green: GreenNode) -> Self {
        Self {
            ptr: NodeRc::new(NodeData::new_root(GreenElement::Node(green))),
        }
    }

    pub(super) fn new_child(
        green: GreenNodeRef<'_>,
        parent: Self,
        slot: u32,
        offset: TextSize,
    ) -> Self {
        Self {
            ptr: NodeRc::new(NodeData::new_child(
                GreenElementRef::Node(green),
                parent.ptr,
                slot,
                offset,
            )),
        }
    }

    pub fn clone_subtree(&self) -> Self {
        Self::new_root(self.green().into())
    }

    #[inline]
    pub(super) fn data(&self) -> &NodeData {
        &self.ptr
    }

    #[inline]
    pub fn kind(&self) -> RawSyntaxKind {
        self.data().kind()
    }

    #[inline]
    pub(super) fn offset(&self) -> TextSize {
        self.data().offset()
    }

    pub(crate) fn element_in_slot(&self, slot_index: u32) -> Option<SyntaxElement> {
        let slot = self
            .slots()
            .nth(slot_index as usize)
            .expect("Slot index out of bounds");

        slot.map(|element| element)
    }

    #[inline]
    pub(crate) fn slots(&self) -> SyntaxSlots {
        SyntaxSlots::new(self.clone())
    }

    /// Returns the slot at `slot_index`, or [None] if this node has no slot with that index.
    #[inline]
    pub(crate) fn slot(&self, slot_index: usize) -> Option<SyntaxSlot> {
        let slot = self.green().slice().get(slot_index)?;
        Some(SyntaxSlot::new(self, slot, slot_index as u32))
    }

    #[inline]
    pub fn text_range(&self) -> TextRange {
        self.data().text_range()
    }

    /// Returns the range of this node's text without the leading trivia of
    /// its first token and the trailing trivia of its last token.
    ///
    /// Tokens whose text without trivia is empty, such as the EOF token, are
    /// skipped over. When every token of the node is empty, the search
    /// continues past the node's bounds, so the range can lie outside the
    /// node's own range.
    pub fn text_trimmed_range(&self) -> TextRange {
        let green = self.green();

        match (
            trimmed_start_in_subtree(green),
            trimmed_end_in_subtree(green),
        ) {
            (Some(start), Some(end)) => {
                let offset = self.offset();
                TextRange::new(offset + start, offset + end)
            }
            _ => self.text_trimmed_range_from_tokens(),
        }
    }

    /// Computes [Self::text_trimmed_range] by walking red tokens, which can
    /// leave this node. Handles the cases the green-tree search gives up on:
    /// nodes without a token with non-empty text, and nodes where such a
    /// token comes after a child node whose text is all trivia.
    #[cold]
    fn text_trimmed_range_from_tokens(&self) -> TextRange {
        let range = self.text_range();
        let mut start = range.start();
        let mut end = range.end();

        // Remove all trivia from the start of the node
        let mut token = self.first_token();
        while let Some(t) = token.take() {
            let (leading_len, trailing_len, total_len) = t.green().leading_trailing_total_len();
            let token_len: u32 = (total_len - leading_len - trailing_len).into();
            if token_len == 0 {
                start += total_len;
                token = t.next_token();
            } else {
                start += leading_len;
            }
        }

        // Remove all trivia from the end of the node
        let mut token = self.last_token();
        while let Some(t) = token.take() {
            let (leading_len, trailing_len, total_len) = t.green().leading_trailing_total_len();
            let token_len: u32 = (total_len - leading_len - trailing_len).into();
            if token_len == 0 {
                end -= total_len;
                token = t.prev_token();
            } else {
                end -= trailing_len;
            }
        }

        TextRange::new(start, end.max(start))
    }

    pub fn first_leading_trivia(&self) -> Option<SyntaxTrivia> {
        self.first_token().map(|x| x.leading_trivia())
    }

    pub fn last_trailing_trivia(&self) -> Option<SyntaxTrivia> {
        self.last_token().map(|x| x.trailing_trivia())
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.data().slot() as usize
    }

    #[inline]
    pub fn text(&self) -> SyntaxNodeText {
        SyntaxNodeText::new(self.clone())
    }

    #[inline]
    pub fn text_trimmed(&self) -> SyntaxNodeText {
        SyntaxNodeText::with_range(self.clone(), self.text_trimmed_range())
    }

    #[inline]
    pub(crate) fn key(&self) -> (NonNull<()>, TextSize) {
        self.data().key()
    }

    #[inline]
    pub(crate) fn green(&self) -> GreenNodeRef<'_> {
        self.data().green().into_node().unwrap()
    }

    #[inline]
    pub fn parent(&self) -> Option<Self> {
        self.data().parent_node()
    }

    #[inline]
    pub fn grand_parent(&self) -> Option<Self> {
        self.data().parent()?.parent_node()
    }

    #[inline]
    pub fn ancestors(&self) -> impl Iterator<Item = Self> + use<> {
        iter::successors(Some(self.clone()), Self::parent)
    }

    #[inline]
    pub fn tree_top(&self) -> Self {
        let mut top = self.clone();
        while let Some(parent) = top.parent() {
            top = parent;
        }
        top
    }

    #[inline]
    pub fn children(&self) -> SyntaxNodeChildren {
        SyntaxNodeChildren::new(self.clone())
    }

    #[inline]
    pub fn children_of_kinds<L: Language>(
        &self,
        kinds: SyntaxKindSet<L>,
    ) -> SyntaxNodeChildrenOfKinds<L> {
        SyntaxNodeChildrenOfKinds::new(self.clone(), kinds)
    }

    #[inline]
    pub fn children_with_tokens(&self) -> SyntaxElementChildren {
        SyntaxElementChildren::new(self.clone())
    }

    #[inline]
    pub fn children_with_tokens_of_kinds<L: Language>(
        &self,
        kinds: SyntaxKindSet<L>,
    ) -> SyntaxElementChildrenOfKinds<L> {
        SyntaxElementChildrenOfKinds::new(self.clone(), kinds)
    }

    #[inline]
    pub fn tokens(&self) -> impl DoubleEndedIterator<Item = SyntaxToken> + '_ {
        self.green().children().filter_map(|child| {
            child.element().into_token().map(|token| {
                SyntaxToken::new(
                    token,
                    self.clone(),
                    child.slot(),
                    self.offset() + child.rel_offset(),
                )
            })
        })
    }

    pub fn first_child(&self) -> Option<Self> {
        self.green().children().find_map(|child| {
            child.element().into_node().map(|green| {
                Self::new_child(
                    green,
                    self.clone(),
                    child.slot(),
                    self.offset() + child.rel_offset(),
                )
            })
        })
    }

    pub fn first_child_of_kinds<L: Language>(&self, kinds: SyntaxKindSet<L>) -> Option<Self> {
        self.children_of_kinds(kinds).next()
    }

    pub fn last_child(&self) -> Option<Self> {
        self.green().children().rev().find_map(|child| {
            child.element().into_node().map(|green| {
                Self::new_child(
                    green,
                    self.clone(),
                    child.slot(),
                    self.offset() + child.rel_offset(),
                )
            })
        })
    }

    pub fn first_child_or_token(&self) -> Option<SyntaxElement> {
        self.green().children().next().map(|child| {
            SyntaxElement::new(
                child.element(),
                self.clone(),
                child.slot(),
                self.offset() + child.rel_offset(),
            )
        })
    }

    pub fn first_child_or_token_of_kinds<L: Language>(
        &self,
        kinds: SyntaxKindSet<L>,
    ) -> Option<SyntaxElement> {
        self.children_with_tokens_of_kinds(kinds).next()
    }

    pub fn last_child_or_token(&self) -> Option<SyntaxElement> {
        self.green().children().next_back().map(|child| {
            SyntaxElement::new(
                child.element(),
                self.clone(),
                child.slot(),
                self.offset() + child.rel_offset(),
            )
        })
    }

    pub fn next_sibling(&self) -> Option<Self> {
        self.data().next_sibling()
    }

    pub fn next_sibling_of_kinds<L: Language>(&self, kinds: SyntaxKindSet<L>) -> Option<Self> {
        self.data().green_siblings()?.following().find_map(|child| {
            let green = child.element().into_node()?;
            if !kinds.matches_raw(green.kind()) {
                return None;
            }
            let parent = self.parent()?;
            let offset = parent.offset() + child.rel_offset();
            Some(Self::new_child(green, parent, child.slot(), offset))
        })
    }

    pub fn prev_sibling(&self) -> Option<Self> {
        self.data().prev_sibling()
    }

    pub fn next_sibling_or_token(&self) -> Option<SyntaxElement> {
        self.data().next_sibling_or_token()
    }

    pub fn next_sibling_or_token_of_kinds<L: Language>(
        &self,
        kinds: SyntaxKindSet<L>,
    ) -> Option<SyntaxElement> {
        self.data().green_siblings()?.following().find_map(|child| {
            let element = child.element();
            if !kinds.matches_raw(element.kind()) {
                return None;
            }
            let parent = self.parent()?;
            let offset = parent.offset() + child.rel_offset();
            Some(SyntaxElement::new(element, parent, child.slot(), offset))
        })
    }
    pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement> {
        self.data().prev_sibling_or_token()
    }

    pub fn first_token(&self) -> Option<SyntaxToken> {
        self.edge_token(Direction::Next)
    }

    pub fn last_token(&self) -> Option<SyntaxToken> {
        self.edge_token(Direction::Prev)
    }

    pub fn next_token(&self) -> Option<SyntaxToken> {
        SyntaxElement::Node(self.clone()).next_token()
    }

    pub fn prev_token(&self) -> Option<SyntaxToken> {
        SyntaxElement::Node(self.clone()).prev_token()
    }

    /// Starts at this node and searches its subtree left to right (`Next`) or
    /// right to left (`Prev`), returning the first token found, if any.
    /// For the whole `a + b` expression, `Next` finds `a` and `Prev` finds `b`.
    fn edge_token(&self, direction: Direction) -> Option<SyntaxToken> {
        let start: SyntaxElement = self.clone().into();
        let mut current = start.clone();

        loop {
            match current {
                NodeOrToken::Token(token) => return Some(token),
                NodeOrToken::Node(node) => {
                    let child = match direction {
                        Direction::Next => node.first_child_or_token(),
                        Direction::Prev => node.last_child_or_token(),
                    };

                    if let Some(child) = child {
                        current = child;
                        continue;
                    }

                    current = node.into();
                }
            }

            loop {
                if current == start {
                    return None;
                }

                let sibling = match direction {
                    Direction::Next => current.next_sibling_or_token(),
                    Direction::Prev => current.prev_sibling_or_token(),
                };

                if let Some(sibling) = sibling {
                    current = sibling;
                    break;
                }

                current = current.parent()?.into();
            }
        }
    }

    #[inline]
    pub fn siblings(&self, direction: Direction) -> impl Iterator<Item = Self> + use<> {
        iter::successors(Some(self.clone()), move |node| match direction {
            Direction::Next => node.next_sibling(),
            Direction::Prev => node.prev_sibling(),
        })
    }

    #[inline]
    pub fn siblings_with_tokens(
        &self,
        direction: Direction,
    ) -> impl Iterator<Item = SyntaxElement> + use<> {
        let me: SyntaxElement = self.clone().into();
        iter::successors(Some(me), move |el| match direction {
            Direction::Next => el.next_sibling_or_token(),
            Direction::Prev => el.prev_sibling_or_token(),
        })
    }

    #[inline]
    pub fn descendants(&self) -> impl Iterator<Item = Self> + use<> {
        self.preorder().filter_map(|event| match event {
            WalkEvent::Enter(node) => Some(node),
            WalkEvent::Leave(_) => None,
        })
    }

    #[inline]
    pub fn descendants_with_tokens(
        &self,
        direction: Direction,
    ) -> impl Iterator<Item = SyntaxElement> + use<> {
        self.preorder_with_tokens(direction)
            .filter_map(|event| match event {
                WalkEvent::Enter(it) => Some(it),
                WalkEvent::Leave(_) => None,
            })
    }

    #[inline]
    pub fn preorder(&self) -> Preorder {
        Preorder::new(self.clone())
    }

    #[inline]
    pub fn preorder_matching<L: Language>(
        &self,
        kinds: SyntaxKindSet<L>,
    ) -> PreorderMatching<'_, L> {
        PreorderMatching::new(self, kinds)
    }

    /// Implements [crate::SyntaxNode::rewrite_of_kinds].
    pub fn rewrite_of_kinds<L: Language>(
        &self,
        kinds: SyntaxKindSet<L>,
        rewrite: impl FnMut(Self) -> RewriteSignal<L>,
    ) -> Option<Self> {
        if kinds == SyntaxKindSet::EMPTY {
            return None;
        }
        RewriteOfKinds::new(self, kinds, rewrite).run()
    }

    #[inline]
    pub fn preorder_with_tokens(&self, direction: Direction) -> PreorderWithTokens {
        PreorderWithTokens::new(self.clone(), direction)
    }

    pub fn preorder_tokens(&self, direction: Direction) -> PreorderTokens {
        PreorderTokens::new(self.clone(), direction)
    }

    pub(crate) fn preorder_slots(&self) -> SlotsPreorder {
        SlotsPreorder::new(self.clone())
    }

    pub fn token_at_offset(&self, offset: TextSize) -> TokenAtOffset<SyntaxToken> {
        let mut node = Cow::Borrowed(self);
        loop {
            let range = node.text_range();
            if range.is_empty() || offset < range.start() || offset > range.end() {
                return TokenAtOffset::None;
            }

            let (left, right) = node.green().children_at_offset(offset - node.offset());
            let (left, right) = match (left, right) {
                (Some(left), right) => (left, right),
                (None, Some(right)) => (right, None),
                (None, None) => return TokenAtOffset::None,
            };
            let left = SyntaxElement::new(
                left.element(),
                node.clone().into_owned(),
                left.slot(),
                node.offset() + left.rel_offset(),
            );

            if let Some(right) = right {
                let right = SyntaxElement::new(
                    right.element(),
                    node.clone().into_owned(),
                    right.slot(),
                    node.offset() + right.rel_offset(),
                );
                let token_at_offset =
                    |node: NodeOrToken<Self, SyntaxToken>| -> TokenAtOffset<SyntaxToken> {
                        match node {
                            NodeOrToken::Token(token) => TokenAtOffset::Single(token),
                            NodeOrToken::Node(node) => node.token_at_offset(offset),
                        }
                    };

                return match (token_at_offset(left), token_at_offset(right)) {
                    (TokenAtOffset::Single(left), TokenAtOffset::Single(right)) => {
                        TokenAtOffset::Between(left, right)
                    }
                    _ => TokenAtOffset::None,
                };
            }

            match left {
                NodeOrToken::Node(left) => node = Cow::Owned(left),
                NodeOrToken::Token(left) => return TokenAtOffset::Single(left),
            }
        }
    }

    pub fn covering_element(&self, range: TextRange) -> SyntaxElement {
        let mut res: SyntaxElement = self.clone().into();
        loop {
            assert!(
                res.text_range().contains_range(range),
                "Bad range: node range {:?}, range {:?}",
                res.text_range(),
                range,
            );
            res = match &res {
                NodeOrToken::Token(_) => return res,
                NodeOrToken::Node(node) => match node.child_or_token_at_range(range) {
                    Some(it) => it,
                    None => return res,
                },
            };
        }
    }

    pub fn child_or_token_at_range(&self, range: TextRange) -> Option<SyntaxElement> {
        let rel_range = range - self.offset();
        self.green()
            .slot_at_range(rel_range)
            .and_then(|(index, rel_offset, slot)| {
                slot.as_ref().map(|green| {
                    SyntaxElement::new(
                        green,
                        self.clone(),
                        index as u32,
                        self.offset() + rel_offset,
                    )
                })
            })
    }

    /// Returns the first node with the given range and kind on the path that
    /// repeated calls to [Self::child_or_token_at_range] take from this node,
    /// this node included. The path ends at the first slot that is empty or
    /// holds a token.
    ///
    /// Each level's red node is created once, and the search stops at the
    /// match without looking at its children.
    pub(crate) fn descendant_at_range_with_kind(
        &self,
        range: TextRange,
        kind: RawSyntaxKind,
    ) -> Option<Self> {
        if self.text_range() == range && self.kind() == kind {
            return Some(self.clone());
        }

        // `green` borrows from `self`, which keeps the whole tree alive, so
        // `node` can move into its child without being borrowed.
        let mut green = self.green();
        let mut node = self.clone();
        loop {
            let offset = node.offset();
            let (index, rel_offset, slot) = green.slot_at_range(range - offset)?;
            let child = slot.as_ref()?.into_node()?;
            let child_offset = offset + rel_offset;
            node = Self::new_child(child, node, index as u32, child_offset);
            if TextRange::at(child_offset, child.text_len()) == range && child.kind() == kind {
                return Some(node);
            }
            green = child;
        }
    }

    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub fn detach(self) -> Self {
        Self {
            ptr: self.ptr.detach(),
        }
    }

    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub fn splice_slots<R, I>(self, range: R, replace_with: I) -> Self
    where
        R: ops::RangeBounds<usize>,
        I: Iterator<Item = Option<SyntaxElement>>,
    {
        Self {
            ptr: self.ptr.splice_slots(
                range,
                replace_with.into_iter().map(|element| {
                    element.map(|child| match child.detach() {
                        NodeOrToken::Node(it) => it.ptr.into_green(),
                        NodeOrToken::Token(it) => it.into_green(),
                    })
                }),
            ),
        }
    }

    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub(crate) fn splice_many_slots<E, R>(self, edits: E) -> Self
    where
        E: IntoIterator<Item = (usize, R)>,
        R: IntoIterator<Item = Option<SyntaxElement>>,
    {
        let green =
            self.green()
                .splice_many_slots(edits.into_iter().map(|(index, replacement)| {
                    (
                        index,
                        replacement.into_iter().map(|element| {
                            element.map(|child| match child.detach() {
                                NodeOrToken::Node(it) => it.ptr.into_green(),
                                NodeOrToken::Token(it) => it.into_green(),
                            })
                        }),
                    )
                }));

        Self {
            ptr: self.ptr.into_root_with_green(GreenElement::Node(green)),
        }
    }

    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub fn replace_child(self, prev_elem: SyntaxElement, next_elem: SyntaxElement) -> Option<Self> {
        Some(Self {
            ptr: self.ptr.replace_child(prev_elem, next_elem)?,
        })
    }
}

/// Returns the offset, relative to `node`, at which the text of the first
/// token with non-empty text starts, leading trivia excluded.
///
/// Descends into the first child node with a non-zero length without
/// remembering the siblings that follow it, so the search runs in constant
/// memory on deep trees. Returns `None` when the text of that child is all
/// trivia, as well as when the subtree has no token with non-empty text.
fn trimmed_start_in_subtree(mut node: GreenNodeRef<'_>) -> Option<TextSize> {
    let mut offset = TextSize::from(0);

    'descend: loop {
        for child in node.children() {
            match child.element() {
                NodeOrToken::Token(token) => {
                    let (leading_len, trailing_len, total_len) = token.leading_trailing_total_len();
                    if total_len > leading_len + trailing_len {
                        return Some(offset + child.rel_offset() + leading_len);
                    }
                }
                NodeOrToken::Node(child_node) => {
                    // A node without text only holds empty tokens.
                    if child_node.text_len() > TextSize::from(0) {
                        offset += child.rel_offset();
                        node = child_node;
                        continue 'descend;
                    }
                }
            }
        }

        return None;
    }
}

/// Returns the offset, relative to `node`, at which the text of the last
/// token with non-empty text ends, trailing trivia excluded.
///
/// The mirror image of [trimmed_start_in_subtree], with the same limits.
fn trimmed_end_in_subtree(mut node: GreenNodeRef<'_>) -> Option<TextSize> {
    let mut offset = TextSize::from(0);

    'descend: loop {
        for child in node.children().rev() {
            match child.element() {
                NodeOrToken::Token(token) => {
                    let (leading_len, trailing_len, total_len) = token.leading_trailing_total_len();
                    if total_len > leading_len + trailing_len {
                        return Some(offset + child.rel_offset() + total_len - trailing_len);
                    }
                }
                NodeOrToken::Node(child_node) => {
                    if child_node.text_len() > TextSize::from(0) {
                        offset += child.rel_offset();
                        node = child_node;
                        continue 'descend;
                    }
                }
            }
        }

        return None;
    }
}

// Identity semantics for hash & eq
impl PartialEq for SyntaxNode {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.data().key() == other.data().key()
    }
}

impl Eq for SyntaxNode {}

impl Hash for SyntaxNode {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data().key().hash(state);
    }
}

impl fmt::Debug for SyntaxNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SyntaxNode")
            .field("kind", &self.kind())
            .field("text_range", &self.text_range())
            .finish()
    }
}

impl fmt::Display for SyntaxNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.preorder_with_tokens(Direction::Next)
            .filter_map(|event| match event {
                WalkEvent::Enter(NodeOrToken::Token(token)) => Some(token),
                _ => None,
            })
            .try_for_each(|it| fmt::Display::fmt(&it, f))
    }
}

// #region: iterators

#[derive(Clone, Debug)]
pub(crate) struct SyntaxNodeChildren {
    next: Option<SyntaxNode>,
}

impl SyntaxNodeChildren {
    fn new(parent: SyntaxNode) -> Self {
        Self {
            next: parent.first_child(),
        }
    }
}

impl Iterator for SyntaxNodeChildren {
    type Item = SyntaxNode;
    fn next(&mut self) -> Option<SyntaxNode> {
        self.next.take().inspect(|next| {
            self.next = next.next_sibling();
        })
    }
}

impl FusedIterator for SyntaxNodeChildren {}

#[derive(Clone, Debug)]
pub(crate) struct SyntaxNodeChildrenOfKinds<L: Language> {
    parent: SyntaxNode,
    kinds: SyntaxKindSet<L>,
    next_slot: usize,
}

impl<L: Language> SyntaxNodeChildrenOfKinds<L> {
    fn new(parent: SyntaxNode, kinds: SyntaxKindSet<L>) -> Self {
        Self {
            parent,
            kinds,
            next_slot: 0,
        }
    }
}

impl<L: Language> Iterator for SyntaxNodeChildrenOfKinds<L> {
    type Item = SyntaxNode;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let index = self.next_slot;
            let slot = self.parent.green().slice().get(index)?;
            self.next_slot += 1;

            let Slot::Node { node, rel_offset } = slot else {
                continue;
            };
            let node = node.as_ref();
            if !self.kinds.matches_raw(node.kind()) {
                continue;
            }

            return Some(SyntaxNode::new_child(
                node,
                self.parent.clone(),
                index as u32,
                self.parent.offset() + *rel_offset,
            ));
        }
    }
}

impl<L: Language> FusedIterator for SyntaxNodeChildrenOfKinds<L> {}

#[derive(Clone, Debug, Default)]
pub(crate) struct SyntaxElementChildren {
    next: Option<SyntaxElement>,
}

impl SyntaxElementChildren {
    fn new(parent: SyntaxNode) -> Self {
        Self {
            next: parent.first_child_or_token(),
        }
    }
}

impl Iterator for SyntaxElementChildren {
    type Item = SyntaxElement;
    fn next(&mut self) -> Option<SyntaxElement> {
        self.next.take().inspect(|next| {
            self.next = next.next_sibling_or_token();
        })
    }
}

impl FusedIterator for SyntaxElementChildren {}

#[derive(Clone, Debug)]
pub(crate) struct SyntaxElementChildrenOfKinds<L: Language> {
    parent: SyntaxNode,
    kinds: SyntaxKindSet<L>,
    next_slot: usize,
}

impl<L: Language> SyntaxElementChildrenOfKinds<L> {
    fn new(parent: SyntaxNode, kinds: SyntaxKindSet<L>) -> Self {
        Self {
            parent,
            kinds,
            next_slot: 0,
        }
    }
}

impl<L: Language> Iterator for SyntaxElementChildrenOfKinds<L> {
    type Item = SyntaxElement;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let index = self.next_slot;
            let slot = self.parent.green().slice().get(index)?;
            self.next_slot += 1;

            let Some(element) = slot.as_ref() else {
                continue;
            };
            if !self.kinds.matches_raw(element.kind()) {
                continue;
            }

            return Some(SyntaxElement::new(
                element,
                self.parent.clone(),
                index as u32,
                self.parent.offset() + slot.rel_offset(),
            ));
        }
    }
}

impl<L: Language> FusedIterator for SyntaxElementChildrenOfKinds<L> {}

#[derive(Clone, Debug)]
pub(crate) struct Preorder {
    start: SyntaxNode,
    next: Option<WalkEvent<SyntaxNode>>,
    skip_subtree: bool,
}

impl Preorder {
    fn new(start: SyntaxNode) -> Self {
        let next = Some(WalkEvent::Enter(start.clone()));
        Self {
            start,
            next,
            skip_subtree: false,
        }
    }

    pub fn skip_subtree(&mut self) {
        self.skip_subtree = true;
    }

    #[cold]
    fn do_skip(&mut self) {
        self.next = self.next.take().map(|next| match next {
            WalkEvent::Enter(first_child) => WalkEvent::Leave(first_child.parent().unwrap()),
            WalkEvent::Leave(parent) => WalkEvent::Leave(parent),
        })
    }
}

impl Iterator for Preorder {
    type Item = WalkEvent<SyntaxNode>;

    fn next(&mut self) -> Option<WalkEvent<SyntaxNode>> {
        if self.skip_subtree {
            self.do_skip();
            self.skip_subtree = false;
        }
        let next = self.next.take();
        self.next = next.as_ref().and_then(|next| {
            Some(match next {
                WalkEvent::Enter(node) => match node.first_child() {
                    Some(child) => WalkEvent::Enter(child),
                    None => WalkEvent::Leave(node.clone()),
                },
                WalkEvent::Leave(node) => {
                    if node == &self.start {
                        return None;
                    }
                    match node.next_sibling() {
                        Some(sibling) => WalkEvent::Enter(sibling),
                        None => WalkEvent::Leave(node.parent()?),
                    }
                }
            })
        });
        next
    }
}

impl FusedIterator for Preorder {}

/// Preorder traversal that yields only the nodes whose kind is in `kinds`.
///
/// The traversal reads the green tree. It allocates a cursor node for each
/// yielded node and for the ancestors of that node, and for nothing else.
pub(crate) struct PreorderMatching<'a, L: Language> {
    start: &'a SyntaxNode,
    kinds: SyntaxKindSet<L>,
    /// Path from `start` to the node whose children are being read. Empty before
    /// the first call to `next` and after the traversal has left `start`.
    ///
    /// Below `start`, a node is on the path only if it matches `kinds` or has a
    /// child node: any other node has no events to yield and nothing below it
    /// to read.
    path: Vec<PathSegment<'a>>,
    entered_start: bool,
}

struct PathSegment<'a> {
    green: GreenNodeRef<'a>,
    /// Index of the next slot of `green` to read.
    next_slot: u32,
    slot: u32,
    offset: TextSize,
    is_match: bool,
    /// Allocated when this node or one of its descendants is yielded.
    node: Option<SyntaxNode>,
}

impl<'a, L: Language> PreorderMatching<'a, L> {
    fn new(start: &'a SyntaxNode, kinds: SyntaxKindSet<L>) -> Self {
        Self {
            start,
            kinds,
            path: Vec::new(),
            entered_start: false,
        }
    }

    pub fn skip_subtree(&mut self) {
        if let Some(segment) = self.path.last_mut() {
            segment.next_slot = u32::MAX;
        }
    }

    /// Returns the cursor node of the last path segment, allocating the cursor
    /// nodes that the path is still missing.
    fn current_node(&mut self) -> Option<SyntaxNode> {
        let allocated = self
            .path
            .iter()
            .rposition(|segment| segment.node.is_some())?;
        let (ancestors, missing) = self.path.split_at_mut(allocated + 1);
        let mut node = ancestors.last()?.node.clone()?;
        for segment in missing {
            node = SyntaxNode::new_child(segment.green, node, segment.slot, segment.offset);
            segment.node = Some(node.clone());
        }
        Some(node)
    }
}

impl<L: Language> Iterator for PreorderMatching<'_, L> {
    type Item = WalkEvent<SyntaxNode>;

    fn next(&mut self) -> Option<WalkEvent<SyntaxNode>> {
        if !self.entered_start {
            self.entered_start = true;
            let green = self.start.green();
            let is_match = self.kinds.matches_raw(green.kind());
            self.path.push(PathSegment {
                green,
                next_slot: 0,
                slot: self.start.data().slot(),
                offset: self.start.offset(),
                is_match,
                node: Some(self.start.clone()),
            });
            if is_match {
                return Some(WalkEvent::Enter(self.start.clone()));
            }
        }

        loop {
            let parent = self.path.last_mut()?;
            let slots = parent.green.slice();
            let next_child = slots
                .get(parent.next_slot as usize..)
                .unwrap_or_default()
                .iter()
                .enumerate()
                .find_map(|(index, slot)| match slot {
                    Slot::Node { rel_offset, node } => Some((index, *rel_offset, node.as_ref())),
                    Slot::Token { .. } | Slot::Empty { .. } => None,
                });
            let Some((index, rel_offset, green)) = next_child else {
                // Only `is_match` and `node` are needed, so they are read in
                // place instead of copying the whole segment out with `pop`.
                let node = if parent.is_match {
                    parent.node.take()
                } else {
                    None
                };
                self.path.truncate(self.path.len() - 1);
                match node {
                    Some(node) => return Some(WalkEvent::Leave(node)),
                    None => continue,
                }
            };
            let slot = parent.next_slot + index as u32;
            parent.next_slot = slot + 1;
            let is_match = self.kinds.matches_raw(green.kind());
            let has_child_node = green
                .slice()
                .iter()
                .any(|slot| matches!(slot, Slot::Node { .. }));
            if !is_match && !has_child_node {
                continue;
            }
            let offset = parent.offset + rel_offset;
            self.path.push(PathSegment {
                green,
                next_slot: 0,
                slot,
                offset,
                is_match,
                node: None,
            });
            if is_match {
                return self.current_node().map(WalkEvent::Enter);
            }
        }
    }
}

impl<L: Language> FusedIterator for PreorderMatching<'_, L> {}

/// State of a [crate::SyntaxNode::rewrite_of_kinds] walk.
///
/// The walk records each replacement as an edit of its parent's slot. When it leaves a node with
/// edits, it rebuilds that node once with [GreenNodeRef::splice_many_slots] and records the result
/// as an edit of the node's parent.
struct RewriteOfKinds<'a, L: Language, F> {
    kinds: SyntaxKindSet<L>,
    rewrite: F,
    start: &'a SyntaxNode,
    /// Kept out of `path` so that a walk that pushes no segment doesn't allocate.
    start_segment: RewriteSegment<'a>,
    /// Segments from a child of `start` to the node being read. Only nodes that match `kinds` or
    /// have a child node are pushed.
    path: Vec<RewriteSegment<'a>>,
    /// Slot index and new node of each edit. A segment's edits are the tail from its
    /// `edits_start`, in slot order.
    edits: Vec<(usize, GreenNode)>,
}

struct RewriteSegment<'a> {
    green: GreenNodeRef<'a>,
    /// Index of the next slot of `green` to read.
    next_slot: u32,
    slot: u32,
    offset: TextSize,
    /// Created when this node or a descendant is offered. `None` for `start`, because `start` is
    /// already a cursor node.
    node: Option<SyntaxNode>,
    /// Length of `edits` when the walk entered this node.
    edits_start: usize,
}

impl<'a, L, F> RewriteOfKinds<'a, L, F>
where
    L: Language,
    F: FnMut(SyntaxNode) -> RewriteSignal<L>,
{
    fn new(start: &'a SyntaxNode, kinds: SyntaxKindSet<L>, rewrite: F) -> Self {
        Self {
            kinds,
            rewrite,
            start,
            start_segment: RewriteSegment {
                green: start.green(),
                next_slot: 0,
                slot: start.data().slot(),
                offset: start.offset(),
                node: None,
                edits_start: 0,
            },
            path: Vec::new(),
            edits: Vec::new(),
        }
    }

    fn run(mut self) -> Option<SyntaxNode> {
        if self.kinds.matches_raw(self.start_segment.green.kind()) {
            match (self.rewrite)(self.start.clone()) {
                RewriteSignal::Replace(replacement) => {
                    let replacement = SyntaxNode::from(replacement);
                    return (replacement != *self.start).then(|| replacement.detach());
                }
                RewriteSignal::Skip => return None,
                RewriteSignal::Descend => {}
            }
        }

        loop {
            let parent = self.path.last_mut().unwrap_or(&mut self.start_segment);
            let next_child = parent
                .green
                .slice()
                .get(parent.next_slot as usize..)
                .unwrap_or_default()
                .iter()
                .enumerate()
                .find_map(|(index, slot)| match slot {
                    Slot::Node { rel_offset, node } => Some((index, *rel_offset, node.as_ref())),
                    Slot::Token { .. } | Slot::Empty { .. } => None,
                });
            let Some((index, rel_offset, green)) = next_child else {
                match self.path.pop() {
                    Some(segment) => self.leave(segment),
                    None => {
                        return (!self.edits.is_empty()).then(|| {
                            SyntaxNode::new_root(splice_edits(
                                self.start_segment.green,
                                self.edits.drain(..),
                            ))
                        });
                    }
                }
                continue;
            };
            let slot = parent.next_slot + index as u32;
            parent.next_slot = slot + 1;
            let offset = parent.offset + rel_offset;
            let is_match = self.kinds.matches_raw(green.kind());
            let has_child_node = green
                .slice()
                .iter()
                .any(|slot| matches!(slot, Slot::Node { .. }));
            if !is_match && !has_child_node {
                continue;
            }
            self.path.push(RewriteSegment {
                green,
                next_slot: 0,
                slot,
                offset,
                node: None,
                edits_start: self.edits.len(),
            });
            if !is_match {
                continue;
            }

            let node = self.current_node();
            match (self.rewrite)(node.clone()) {
                RewriteSignal::Descend => {}
                RewriteSignal::Skip => {
                    self.path.pop();
                }
                RewriteSignal::Replace(replacement) => {
                    self.path.pop();
                    let replacement = SyntaxNode::from(replacement);
                    if replacement != node {
                        self.edits
                            .push((slot as usize, replacement.green().to_owned()));
                    }
                }
            }
        }
    }

    /// Records the rebuilt node of `segment` as an edit of its parent, if `segment` has edits.
    fn leave(&mut self, segment: RewriteSegment<'a>) {
        if self.edits.len() > segment.edits_start {
            let green = splice_edits(segment.green, self.edits.drain(segment.edits_start..));
            self.edits.push((segment.slot as usize, green));
        }
    }

    /// Returns the cursor node of the last path segment, allocating the cursor nodes that the
    /// path is still missing.
    fn current_node(&mut self) -> SyntaxNode {
        let (first_missing, mut node) = self
            .path
            .iter()
            .enumerate()
            .rev()
            .find_map(|(index, segment)| Some((index + 1, segment.node.clone()?)))
            .unwrap_or_else(|| (0, self.start.clone()));
        for segment in self.path.iter_mut().skip(first_missing) {
            node = SyntaxNode::new_child(segment.green, node, segment.slot, segment.offset);
            segment.node = Some(node.clone());
        }
        node
    }
}

/// Returns a copy of `green` with the new node of each edit in its slot.
fn splice_edits(
    green: GreenNodeRef<'_>,
    edits: impl Iterator<Item = (usize, GreenNode)>,
) -> GreenNode {
    green.splice_many_slots(
        edits.map(|(slot, node)| (slot, iter::once(Some(NodeOrToken::Node(node))))),
    )
}

#[derive(Clone, Debug)]
pub(crate) struct PreorderWithTokens {
    start: SyntaxElement,
    next: Option<WalkEvent<SyntaxElement>>,
    skip_subtree: bool,
    direction: Direction,
}

impl PreorderWithTokens {
    fn new(start: SyntaxNode, direction: Direction) -> Self {
        let next = Some(WalkEvent::Enter(start.clone().into()));
        Self {
            start: start.into(),
            next,
            direction,
            skip_subtree: false,
        }
    }

    pub fn skip_subtree(&mut self) {
        self.skip_subtree = true;
    }

    #[cold]
    fn do_skip(&mut self) {
        self.next = self.next.take().map(|next| match next {
            WalkEvent::Enter(first_child) => WalkEvent::Leave(first_child.parent().unwrap().into()),
            WalkEvent::Leave(parent) => WalkEvent::Leave(parent),
        })
    }
}

impl Iterator for PreorderWithTokens {
    type Item = WalkEvent<SyntaxElement>;

    fn next(&mut self) -> Option<WalkEvent<SyntaxElement>> {
        if self.skip_subtree {
            self.do_skip();
            self.skip_subtree = false;
        }
        let next = self.next.take();
        self.next = next.as_ref().and_then(|next| {
            Some(match next {
                WalkEvent::Enter(el) => match el {
                    NodeOrToken::Node(node) => {
                        let next = match self.direction {
                            Direction::Next => node.first_child_or_token(),
                            Direction::Prev => node.last_child_or_token(),
                        };
                        match next {
                            Some(child) => WalkEvent::Enter(child),
                            None => WalkEvent::Leave(node.clone().into()),
                        }
                    }
                    NodeOrToken::Token(token) => WalkEvent::Leave(token.clone().into()),
                },
                WalkEvent::Leave(el) if el == &self.start => return None,
                WalkEvent::Leave(el) => {
                    let next = match self.direction {
                        Direction::Next => el.next_sibling_or_token(),
                        Direction::Prev => el.prev_sibling_or_token(),
                    };

                    match next {
                        Some(sibling) => WalkEvent::Enter(sibling),
                        None => WalkEvent::Leave(el.parent()?.into()),
                    }
                }
            })
        });
        next
    }
}

impl FusedIterator for PreorderWithTokens {}

pub(crate) struct PreorderTokens {
    next: Option<SyntaxToken>,
    direction: Direction,
}

impl PreorderTokens {
    fn new(start: SyntaxNode, direction: Direction) -> Self {
        let next = match direction {
            Direction::Next => start.first_token(),
            Direction::Prev => start.last_token(),
        };
        Self { next, direction }
    }
}

impl Iterator for PreorderTokens {
    type Item = SyntaxToken;

    fn next(&mut self) -> Option<SyntaxToken> {
        let next = self.next.take();
        self.next = next.as_ref().and_then(|next| match self.direction {
            Direction::Next => next.next_token(),
            Direction::Prev => next.prev_token(),
        });
        next
    }
}

impl FusedIterator for PreorderTokens {}

/// Represents a cursor to a green node slot. A slot either contains an element or is empty
/// if the child isn't present in the source.
#[derive(Debug, Clone)]
pub(crate) enum SyntaxSlot {
    Node(SyntaxNode),
    Token(SyntaxToken),
    Empty { parent: SyntaxNode, index: u32 },
}

impl From<SyntaxElement> for SyntaxSlot {
    fn from(element: SyntaxElement) -> Self {
        match element {
            SyntaxElement::Node(node) => Self::Node(node),
            SyntaxElement::Token(token) => Self::Token(token),
        }
    }
}

impl SyntaxSlot {
    #[inline]
    fn new(parent: &SyntaxNode, slot: &Slot, slot_index: u32) -> Self {
        match slot {
            Slot::Empty { .. } => Self::Empty {
                parent: parent.clone(),
                index: slot_index,
            },
            Slot::Token { rel_offset, token } => Self::Token(SyntaxToken::new(
                token.as_ref(),
                parent.clone(),
                slot_index,
                parent.offset() + rel_offset,
            )),
            Slot::Node { rel_offset, node } => Self::Node(SyntaxNode::new_child(
                node.as_ref(),
                parent.clone(),
                slot_index,
                parent.offset() + rel_offset,
            )),
        }
    }

    #[inline]
    pub fn map<F, R>(self, mapper: F) -> Option<R>
    where
        F: FnOnce(SyntaxElement) -> R,
    {
        match self {
            Self::Node(node) => Some(mapper(SyntaxElement::Node(node))),
            Self::Token(token) => Some(mapper(SyntaxElement::Token(token))),
            Self::Empty { .. } => None,
        }
    }
}

/// Iterator over a node's slots
#[derive(Debug, Clone)]
pub(crate) struct SyntaxSlots {
    /// Position of the next element to return.
    pos: u32,

    /// Position of the last returned element from the back.
    /// Initially points one element past the last slot.
    ///
    /// [nth_back]: https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html#method.nth_back
    back_pos: u32,
    parent: SyntaxNode,
}

impl SyntaxSlots {
    #[inline]
    fn new(parent: SyntaxNode) -> Self {
        Self {
            pos: 0,
            back_pos: parent.green().slice().len() as u32,
            parent,
        }
    }

    /// Returns a slice containing the remaining elements to iterate over
    /// an empty slice if the iterator reached the end.
    #[inline]
    fn slice(&self) -> &[Slot] {
        if self.pos < self.back_pos {
            &self.parent.green().slice()[self.pos as usize..self.back_pos as usize]
        } else {
            &[]
        }
    }

    fn map_slot(&self, slot: &Slot, slot_index: u32) -> SyntaxSlot {
        SyntaxSlot::new(&self.parent, slot, slot_index)
    }
}

impl Iterator for SyntaxSlots {
    type Item = SyntaxSlot;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let slot = self.slice().first()?;
        let mapped = self.map_slot(slot, self.pos);
        self.pos += 1;
        Some(mapped)
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.slice().len();
        (len, Some(len))
    }

    #[inline(always)]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.len()
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.next_back()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.pos += n as u32;
        self.next()
    }
}

impl ExactSizeIterator for SyntaxSlots {
    #[inline(always)]
    fn len(&self) -> usize {
        self.slice().len()
    }
}

impl FusedIterator for SyntaxSlots {}

impl DoubleEndedIterator for SyntaxSlots {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let slot = self.slice().last()?;
        let mapped = self.map_slot(slot, self.back_pos - 1);
        self.back_pos -= 1;
        Some(mapped)
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.back_pos -= n as u32;
        self.next_back()
    }
}

/// Iterator to visit a node's slots in pre-order.
pub(crate) struct SlotsPreorder {
    start: SyntaxNode,
    next: Option<WalkEvent<SyntaxSlot>>,
}

impl SlotsPreorder {
    fn new(start: SyntaxNode) -> Self {
        let next = Some(WalkEvent::Enter(SyntaxSlot::Node(start.clone())));
        Self { start, next }
    }
}

impl Iterator for SlotsPreorder {
    type Item = WalkEvent<SyntaxSlot>;

    fn next(&mut self) -> Option<WalkEvent<SyntaxSlot>> {
        let next = self.next.take();
        self.next = next.as_ref().and_then(|next| {
            Some(match next {
                WalkEvent::Enter(slot) => match slot {
                    SyntaxSlot::Empty { .. } | SyntaxSlot::Token(_) => {
                        WalkEvent::Leave(slot.clone())
                    }
                    SyntaxSlot::Node(node) => match node.slots().next() {
                        None => WalkEvent::Leave(SyntaxSlot::Node(node.clone())),
                        Some(first_slot) => WalkEvent::Enter(first_slot),
                    },
                },
                WalkEvent::Leave(slot) => {
                    let (parent, slot_index) = match slot {
                        SyntaxSlot::Empty { parent, index } => (parent.clone(), *index as usize),
                        SyntaxSlot::Token(token) => (token.parent()?, token.index()),
                        SyntaxSlot::Node(node) => {
                            if node == &self.start {
                                return None;
                            }

                            (node.parent()?, node.index())
                        }
                    };

                    let next_slot = parent.slots().nth(slot_index + 1);
                    match next_slot {
                        Some(slot) => WalkEvent::Enter(slot),
                        None => WalkEvent::Leave(SyntaxSlot::Node(parent)),
                    }
                }
            })
        });
        next
    }
}

impl FusedIterator for SlotsPreorder {}

#[derive(Debug, Clone)]
pub(crate) struct Siblings<'a> {
    parent: GreenNodeRef<'a>,
    start_slot: u32,
}

impl<'a> Siblings<'a> {
    pub fn new(parent: GreenNodeRef<'a>, start_slot: u32) -> Self {
        assert!(
            (start_slot as usize) < parent.slots().len(),
            "Start slot {} out of bounds {}",
            start_slot,
            parent.slots().len()
        );

        Self { parent, start_slot }
    }

    /// Creates an iterator over the siblings following the start node.
    /// For example, the following siblings of the if statement's condition are
    /// * the consequence
    /// * potentially the else clause
    pub fn following(&self) -> Children<'a> {
        let mut slots = self.parent.slots().enumerate();

        // Navigate to the start slot so that calling `next` returns the first following sibling
        slots.nth(self.start_slot as usize);

        Children::new(slots)
    }

    /// Creates an iterator over the siblings preceding the start node in reverse order.
    /// For example, the preceding siblings of the if statement's condition are:
    /// * opening parentheses: (
    /// * if keyword: if
    pub fn previous(&self) -> impl Iterator<Item = Child<'a>> + use<'a> {
        let mut slots = self.parent.slots().enumerate();

        // Navigate to the start slot from the back so that calling `next_back` (or rev().next()) returns
        // the first slot preceding the start node
        slots.nth_back(slots.len() - 1 - self.start_slot as usize);

        Children::new(slots).rev()
    }
}

// #endregion

#[cfg(test)]
mod tests {
    use crate::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    use crate::{
        SyntaxKindSet, SyntaxNode, SyntaxSlot, SyntaxToken, TextRange, TextSize, TokenAtOffset,
        TriviaPiece, WalkEvent,
    };

    fn token_texts_at_offset(node: &SyntaxNode<RawLanguage>, offset: u32) -> Vec<String> {
        node.token_at_offset(TextSize::from(offset))
            .map(|token| token.text_trimmed().to_string())
            .collect()
    }

    fn detached_node(
        kind: RawLanguageKind,
        slots: Vec<Option<crate::SyntaxElement<RawLanguage>>>,
    ) -> SyntaxNode<RawLanguage> {
        SyntaxNode::new_detached(kind, slots)
    }

    fn literal(text: &str) -> Option<crate::SyntaxElement<RawLanguage>> {
        let token = SyntaxToken::new_detached(RawLanguageKind::NUMBER_TOKEN, text, [], []);
        Some(
            detached_node(
                RawLanguageKind::LITERAL_EXPRESSION,
                vec![Some(token.into())],
            )
            .into(),
        )
    }

    /// What a consumer can observe of a node yielded by a traversal.
    #[derive(Debug, PartialEq)]
    struct ObservedEvent {
        is_enter: bool,
        node: SyntaxNode<RawLanguage>,
        range: TextRange,
        index: usize,
        ancestors: Vec<SyntaxNode<RawLanguage>>,
    }

    fn observed(
        events: impl Iterator<Item = WalkEvent<SyntaxNode<RawLanguage>>>,
    ) -> Vec<ObservedEvent> {
        events
            .map(|event| {
                let (is_enter, node) = match event {
                    WalkEvent::Enter(node) => (true, node),
                    WalkEvent::Leave(node) => (false, node),
                };
                ObservedEvent {
                    is_enter,
                    range: node.text_range_with_trivia(),
                    index: node.index(),
                    ancestors: node.ancestors().collect(),
                    node,
                }
            })
            .collect()
    }

    fn preorder_test_tree() -> SyntaxNode<RawLanguage> {
        use RawLanguageKind::{CONDITION, EXPRESSION_LIST, LITERAL_EXPRESSION, ROOT};

        let nested_literal = detached_node(
            LITERAL_EXPRESSION,
            vec![Some(
                detached_node(EXPRESSION_LIST, vec![literal("3")]).into(),
            )],
        );
        let condition = detached_node(
            CONDITION,
            vec![
                None,
                literal("1"),
                Some(
                    detached_node(
                        EXPRESSION_LIST,
                        vec![Some(
                            detached_node(CONDITION, vec![None, literal("22")]).into(),
                        )],
                    )
                    .into(),
                ),
            ],
        );
        // Nodes without child nodes, between and after nodes of the kinds they
        // contain elsewhere.
        let childless = detached_node(
            EXPRESSION_LIST,
            vec![
                literal("4"),
                Some(detached_node(CONDITION, vec![None]).into()),
                Some(SyntaxToken::new_detached(RawLanguageKind::PLUS_TOKEN, "+", [], []).into()),
                literal("5"),
                Some(detached_node(CONDITION, vec![]).into()),
            ],
        );
        let leading_token = SyntaxToken::new_detached(RawLanguageKind::STRING_TOKEN, "'a'", [], []);
        detached_node(
            ROOT,
            vec![
                Some(leading_token.into()),
                Some(condition.into()),
                None,
                Some(nested_literal.into()),
                Some(childless.into()),
                Some(detached_node(EXPRESSION_LIST, vec![]).into()),
            ],
        )
    }

    fn preorder_kind_sets() -> [SyntaxKindSet<RawLanguage>; 6] {
        use RawLanguageKind::{CONDITION, EXPRESSION_LIST, LITERAL_EXPRESSION, ROOT};

        [
            SyntaxKindSet::EMPTY,
            SyntaxKindSet::of(LITERAL_EXPRESSION),
            SyntaxKindSet::of(CONDITION),
            SyntaxKindSet::of(ROOT).union(SyntaxKindSet::of(EXPRESSION_LIST)),
            SyntaxKindSet::of(LITERAL_EXPRESSION).union(SyntaxKindSet::of(CONDITION)),
            SyntaxKindSet::of(ROOT)
                .union(SyntaxKindSet::of(EXPRESSION_LIST))
                .union(SyntaxKindSet::of(LITERAL_EXPRESSION))
                .union(SyntaxKindSet::of(CONDITION)),
        ]
    }

    #[test]
    fn preorder_of_kinds_yields_the_matching_preorder_events() {
        let root = preorder_test_tree();
        let mut yielded = 0;
        for start in root.descendants() {
            for kinds in preorder_kind_sets() {
                let expected = observed(start.preorder().filter(|event| match event {
                    WalkEvent::Enter(node) | WalkEvent::Leave(node) => kinds.matches(node.kind()),
                }));
                let actual = observed(start.preorder_of_kinds(kinds));
                assert_eq!(actual, expected, "from {start:?}");
                yielded += actual.len();
            }
        }
        assert!(yielded > 0);
    }

    #[test]
    fn preorder_of_kinds_skips_subtrees_like_preorder() {
        use RawLanguageKind::{CONDITION, EXPRESSION_LIST, LITERAL_EXPRESSION, ROOT};

        let root = preorder_test_tree();
        let mut skipped = 0;
        for start in root.descendants() {
            for kinds in preorder_kind_sets() {
                for skipped_kind in [ROOT, CONDITION, EXPRESSION_LIST, LITERAL_EXPRESSION] {
                    let skips = |event: &WalkEvent<SyntaxNode<RawLanguage>>| matches!(event, WalkEvent::Enter(node) if node.kind() == skipped_kind);

                    let mut preorder = start.preorder();
                    let mut expected = Vec::new();
                    while let Some(event) = preorder.next() {
                        let node = match &event {
                            WalkEvent::Enter(node) | WalkEvent::Leave(node) => node,
                        };
                        if !kinds.matches(node.kind()) {
                            continue;
                        }
                        if skips(&event) {
                            preorder.skip_subtree();
                        }
                        expected.push(event);
                    }

                    let mut preorder = start.preorder_of_kinds(kinds);
                    let mut actual = Vec::new();
                    while let Some(event) = preorder.next() {
                        if skips(&event) {
                            preorder.skip_subtree();
                            skipped += 1;
                        }
                        actual.push(event);
                    }

                    assert_eq!(
                        observed(actual.into_iter()),
                        observed(expected.into_iter()),
                        "from {start:?} skipping {skipped_kind:?}"
                    );
                }
            }
        }
        assert!(skipped > 0);
    }

    mod rewrite_of_kinds {
        use super::{
            ObservedEvent, detached_node, observed, preorder_kind_sets, preorder_test_tree,
        };
        use crate::raw_language::{RawLanguage, RawLanguageKind};
        use crate::{
            RewriteSignal, SyntaxKindSet, SyntaxNode, SyntaxRewriter, SyntaxToken, TextSize,
            VisitNodeSignal, WalkEvent,
        };
        use std::collections::HashSet;
        use std::iter;

        /// Selects what a rewrite does with an offered node. [KindsRewrite] and
        /// [TraverseRewrite] make the same decision for the same node.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        enum Decision {
            Descend,
            Skip,
            ReplaceWithItself,
            ReplaceWithLiteral,
            /// Replaces the node with its first child node, a node of the original tree.
            ReplaceWithFirstChild,
            /// Rewrites the first child node with a nested rewrite, then replaces the node with a
            /// copy that holds the rewritten child.
            RewriteFirstChild,
        }

        const DECISIONS: [Decision; 6] = [
            Decision::Descend,
            Decision::Skip,
            Decision::ReplaceWithItself,
            Decision::ReplaceWithLiteral,
            Decision::ReplaceWithFirstChild,
            Decision::RewriteFirstChild,
        ];

        fn decide(seed: usize, node: &SyntaxNode<RawLanguage>) -> Decision {
            let depth = node.ancestors().count();
            DECISIONS[(seed + depth + node.index()) % DECISIONS.len()]
        }

        fn literal_node(text: &str) -> SyntaxNode<RawLanguage> {
            let token = SyntaxToken::new_detached(RawLanguageKind::NUMBER_TOKEN, text, [], []);
            detached_node(
                RawLanguageKind::LITERAL_EXPRESSION,
                vec![Some(token.into())],
            )
        }

        fn replace_child(
            node: &SyntaxNode<RawLanguage>,
            child: &SyntaxNode<RawLanguage>,
            replacement: SyntaxNode<RawLanguage>,
        ) -> SyntaxNode<RawLanguage> {
            let index = child.index();
            node.clone()
                .splice_slots(index..=index, [Some(replacement.into())])
        }

        fn offered(node: &SyntaxNode<RawLanguage>) -> ObservedEvent {
            observed(iter::once(WalkEvent::Enter(node.clone())))
                .pop()
                .unwrap()
        }

        struct KindsRewrite {
            kinds: SyntaxKindSet<RawLanguage>,
            seed: usize,
            offered: Vec<ObservedEvent>,
            decisions: Vec<Decision>,
        }

        impl KindsRewrite {
            fn new(kinds: SyntaxKindSet<RawLanguage>, seed: usize) -> Self {
                Self {
                    kinds,
                    seed,
                    offered: Vec::new(),
                    decisions: Vec::new(),
                }
            }

            fn visit(&mut self, node: SyntaxNode<RawLanguage>) -> RewriteSignal<RawLanguage> {
                self.offered.push(offered(&node));
                let decision = decide(self.seed, &node);
                self.decisions.push(decision);
                match decision {
                    Decision::Descend => RewriteSignal::Descend,
                    Decision::Skip => RewriteSignal::Skip,
                    Decision::ReplaceWithItself => RewriteSignal::Replace(node),
                    Decision::ReplaceWithLiteral => RewriteSignal::Replace(literal_node("9")),
                    Decision::ReplaceWithFirstChild => {
                        RewriteSignal::Replace(node.first_child().unwrap_or(node))
                    }
                    Decision::RewriteFirstChild => match node.first_child() {
                        Some(child) => {
                            let kinds = self.kinds;
                            let rewritten = child
                                .rewrite_of_kinds(kinds, |node| self.visit(node))
                                .unwrap_or_else(|| child.clone());
                            RewriteSignal::Replace(replace_child(&node, &child, rewritten))
                        }
                        None => RewriteSignal::Skip,
                    },
                }
            }
        }

        /// Runs the rewrite of [KindsRewrite] with [SyntaxRewriter::transform].
        struct TraverseRewrite {
            kinds: SyntaxKindSet<RawLanguage>,
            seed: usize,
            offered: Vec<ObservedEvent>,
        }

        impl SyntaxRewriter for TraverseRewrite {
            type Language = RawLanguage;

            fn visit_node(
                &mut self,
                node: SyntaxNode<RawLanguage>,
            ) -> VisitNodeSignal<RawLanguage> {
                if !self.kinds.matches(node.kind()) {
                    return VisitNodeSignal::Traverse(node);
                }
                self.offered.push(offered(&node));
                match decide(self.seed, &node) {
                    Decision::Descend => VisitNodeSignal::Traverse(node),
                    Decision::Skip | Decision::ReplaceWithItself => VisitNodeSignal::Replace(node),
                    Decision::ReplaceWithLiteral => VisitNodeSignal::Replace(literal_node("9")),
                    Decision::ReplaceWithFirstChild => {
                        VisitNodeSignal::Replace(node.first_child().unwrap_or(node))
                    }
                    Decision::RewriteFirstChild => match node.first_child() {
                        Some(child) => {
                            let rewritten = self.transform(child.clone());
                            VisitNodeSignal::Replace(replace_child(&node, &child, rewritten))
                        }
                        None => VisitNodeSignal::Replace(node),
                    },
                }
            }
        }

        #[test]
        fn matches_syntax_rewriter() {
            let root = preorder_test_tree();
            let mut rewritten = 0;
            let mut decisions = HashSet::new();
            for start in root.descendants() {
                for kinds in preorder_kind_sets() {
                    for seed in 0..DECISIONS.len() {
                        let context = format!("from {start:?} with {kinds:?} and seed {seed}");

                        let mut rewrite = KindsRewrite::new(kinds, seed);
                        let actual = start.rewrite_of_kinds(kinds, |node| rewrite.visit(node));
                        let mut oracle = TraverseRewrite {
                            kinds,
                            seed,
                            offered: Vec::new(),
                        };
                        let expected = oracle.transform(start.clone());

                        assert_eq!(rewrite.offered, oracle.offered, "{context}");
                        match actual {
                            None => assert_eq!(expected, start, "{context}"),
                            Some(actual) => {
                                assert_ne!(expected, start, "{context}");
                                assert!(actual.parent().is_none(), "{context}");
                                assert_eq!(
                                    format!("{actual:#?}"),
                                    format!("{:#?}", expected.detach()),
                                    "{context}"
                                );
                                rewritten += 1;
                            }
                        }
                        decisions.extend(rewrite.decisions);
                    }
                }
            }
            assert!(rewritten > 0);
            for decision in DECISIONS {
                assert!(decisions.contains(&decision), "{decision:?} was never made");
            }
        }

        #[test]
        fn returns_none_for_no_kinds_without_offering_nodes() {
            let root = preorder_test_tree();
            let rewritten =
                root.rewrite_of_kinds(SyntaxKindSet::EMPTY, |node| panic!("offered {node:?}"));
            assert!(rewritten.is_none());
        }

        #[test]
        fn never_offers_tokens() {
            let root = preorder_test_tree();
            let tokens = SyntaxKindSet::of(RawLanguageKind::NUMBER_TOKEN)
                .union(SyntaxKindSet::of(RawLanguageKind::STRING_TOKEN));
            let rewritten = root.rewrite_of_kinds(tokens, |node| panic!("offered {node:?}"));
            assert!(rewritten.is_none());
        }

        #[test]
        fn returns_none_when_nodes_are_replaced_by_themselves() {
            let root = preorder_test_tree();
            let all_kinds = preorder_kind_sets()[5];
            let rewritten = root.rewrite_of_kinds(all_kinds, |node| {
                if node.kind() == RawLanguageKind::ROOT {
                    RewriteSignal::Descend
                } else {
                    RewriteSignal::Replace(node)
                }
            });
            assert!(rewritten.is_none());
        }

        #[test]
        fn detaches_a_replaced_start_node() {
            let root = preorder_test_tree();
            let start = root.first_child().unwrap();
            let replacement = root.children().nth(1).unwrap();
            assert!(start.text_range_with_trivia().start() > TextSize::from(0));

            let rewritten = start
                .rewrite_of_kinds(SyntaxKindSet::of(start.kind()), |_| {
                    RewriteSignal::Replace(replacement.clone())
                })
                .unwrap();

            assert!(rewritten.parent().is_none());
            assert_eq!(
                rewritten.text_range_with_trivia().start(),
                TextSize::from(0)
            );
            assert_eq!(
                format!("{rewritten:#?}"),
                format!("{:#?}", replacement.detach())
            );
        }

        #[test]
        fn offers_nodes_of_the_original_tree() {
            let root = preorder_test_tree();
            let literals = SyntaxKindSet::of(RawLanguageKind::LITERAL_EXPRESSION);
            let outer_literals = root.descendants().filter(|node| {
                node.kind() == RawLanguageKind::LITERAL_EXPRESSION
                    && !node
                        .ancestors()
                        .skip(1)
                        .any(|ancestor| ancestor.kind() == RawLanguageKind::LITERAL_EXPRESSION)
            });

            let mut offered_nodes = Vec::new();
            let rewritten = root.rewrite_of_kinds(literals, |node| {
                offered_nodes.push(offered(&node));
                RewriteSignal::Replace(literal_node("99"))
            });

            assert!(rewritten.is_some());
            assert_eq!(
                offered_nodes,
                outer_literals
                    .map(|node| offered(&node))
                    .collect::<Vec<_>>()
            );
        }

        /// Rewrites literals to `0`. Rewrites any other node by first rewriting its last child node
        /// with a nested call.
        fn rewrite_last_operand_first(
            node: SyntaxNode<RawLanguage>,
            kinds: SyntaxKindSet<RawLanguage>,
            offered_nodes: &mut Vec<SyntaxNode<RawLanguage>>,
        ) -> RewriteSignal<RawLanguage> {
            offered_nodes.push(node.clone());
            if node.kind() == RawLanguageKind::LITERAL_EXPRESSION {
                return RewriteSignal::Replace(literal_node("0"));
            }
            let Some(operand) = node.last_child() else {
                return RewriteSignal::Skip;
            };
            match operand.rewrite_of_kinds(kinds, |node| {
                rewrite_last_operand_first(node, kinds, offered_nodes)
            }) {
                Some(rewritten) => {
                    RewriteSignal::Replace(replace_child(&node, &operand, rewritten))
                }
                None => RewriteSignal::Skip,
            }
        }

        #[test]
        fn supports_nested_rewrites() {
            let root = preorder_test_tree();
            assert_eq!(root.text_trimmed().to_string(), "'a'12234+5");
            let kinds = SyntaxKindSet::of(RawLanguageKind::CONDITION)
                .union(SyntaxKindSet::of(RawLanguageKind::LITERAL_EXPRESSION));

            let mut offered_nodes = Vec::new();
            let rewritten = root
                .rewrite_of_kinds(kinds, |node| {
                    rewrite_last_operand_first(node, kinds, &mut offered_nodes)
                })
                .unwrap();

            assert_eq!(rewritten.text_trimmed().to_string(), "'a'1000+0");
            let unique: HashSet<_> = offered_nodes.iter().collect();
            assert_eq!(unique.len(), offered_nodes.len(), "{offered_nodes:#?}");
            assert_eq!(offered_nodes.len(), 8);
        }
    }

    #[test]
    fn slots_iter() {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::EXPRESSION_LIST);

        for number in [1, 2, 3, 4] {
            builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
            builder.token(RawLanguageKind::NUMBER_TOKEN, &number.to_string());
            builder.finish_node();
        }
        builder.finish_node();

        let list = builder.finish();

        let mut iter = list.slots();

        assert_eq!(iter.size_hint(), (4, Some(4)));

        assert_eq!(
            iter.next()
                .and_then(|slot| slot.into_node())
                .map(|node| node.text_with_trivia().to_string())
                .as_deref(),
            Some("1")
        );

        assert_eq!(iter.size_hint(), (3, Some(3)));

        assert_eq!(
            iter.next_back()
                .and_then(|slot| slot.into_node())
                .map(|node| node.text_with_trivia().to_string())
                .as_deref(),
            Some("4")
        );

        assert_eq!(iter.size_hint(), (2, Some(2)));

        assert_eq!(
            iter.last()
                .and_then(|slot| slot.into_node())
                .map(|node| node.text_with_trivia().to_string())
                .as_deref(),
            Some("3")
        );
    }

    #[test]
    fn slot_returns_the_element_at_the_index() {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        builder.token(RawLanguageKind::STRING_TOKEN, "a");
        builder.start_node(RawLanguageKind::CONDITION);
        builder.token(RawLanguageKind::L_PAREN_TOKEN, "(");
        builder.token(RawLanguageKind::R_PAREN_TOKEN, ")");
        builder.finish_node();
        builder.finish_node();
        let root = builder.finish();

        let condition = root.slot(1).and_then(SyntaxSlot::into_node).unwrap();
        assert_eq!(condition.kind(), RawLanguageKind::CONDITION);
        assert_eq!(condition.index(), 1);
        assert_eq!(condition.parent(), Some(root.clone()));
        assert_eq!(
            condition.text_range_with_trivia(),
            TextRange::new(1.into(), 3.into())
        );

        let r_paren = condition.slot(2).and_then(SyntaxSlot::into_token).unwrap();
        assert_eq!(r_paren.kind(), RawLanguageKind::R_PAREN_TOKEN);
        assert_eq!(r_paren.index(), 2);
        assert_eq!(r_paren.parent(), Some(condition.clone()));
        assert_eq!(r_paren.text_range(), TextRange::new(2.into(), 3.into()));

        assert!(matches!(
            condition.slot(1),
            Some(SyntaxSlot::Empty { index: 1 })
        ));
        assert!(condition.slot(3).is_none());

        for (index, expected) in condition.slots().enumerate() {
            let actual = condition.slot(index).unwrap();
            assert_eq!(actual.kind(), expected.kind());
            assert_eq!(actual.into_syntax_element(), expected.into_syntax_element());
        }
    }

    #[test]
    fn token_at_offset_preserves_nested_boundaries() {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);

        for text in ["a", "b"] {
            builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
            builder.token(RawLanguageKind::STRING_TOKEN, text);
            builder.finish_node();
        }

        builder.finish_node();
        let root = builder.finish();

        assert_eq!(token_texts_at_offset(&root, 0), ["a"]);
        assert_eq!(token_texts_at_offset(&root, 1), ["a", "b"]);
        assert_eq!(token_texts_at_offset(&root, 2), ["b"]);
        assert!(matches!(
            root.token_at_offset(TextSize::from(3)),
            TokenAtOffset::None
        ));
    }

    #[test]
    fn token_at_offset_skips_zero_width_elements_and_includes_trivia() {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        builder.token(RawLanguageKind::STRING_TOKEN, "");
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.finish_node();
        builder.token_with_trivia(
            RawLanguageKind::STRING_TOKEN,
            "a ",
            &[],
            &[TriviaPiece::whitespace(1)],
        );
        builder.token(RawLanguageKind::STRING_TOKEN, "b");
        builder.finish_node();
        let root = builder.finish();

        assert_eq!(token_texts_at_offset(&root, 0), ["a"]);
        assert_eq!(token_texts_at_offset(&root, 1), ["a"]);
        assert_eq!(token_texts_at_offset(&root, 2), ["a", "b"]);
        assert_eq!(token_texts_at_offset(&root, 3), ["b"]);
    }

    #[test]
    fn token_at_offset_skips_zero_width_elements_at_boundaries_and_eof() {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        builder.token(RawLanguageKind::STRING_TOKEN, "a");
        builder.token(RawLanguageKind::STRING_TOKEN, "");
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.finish_node();
        builder.token(RawLanguageKind::STRING_TOKEN, "b");
        builder.token(RawLanguageKind::STRING_TOKEN, "");
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.finish_node();
        builder.finish_node();
        let root = builder.finish();

        assert_eq!(token_texts_at_offset(&root, 1), ["a", "b"]);
        assert_eq!(token_texts_at_offset(&root, 2), ["b"]);
    }

    #[test]
    fn token_at_offset_skips_missing_slots_and_respects_subtree_ranges() {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        builder.token(RawLanguageKind::STRING_TOKEN, "x");
        builder.start_node(RawLanguageKind::CONDITION);
        builder.token(RawLanguageKind::L_PAREN_TOKEN, "x");
        builder.token(RawLanguageKind::R_PAREN_TOKEN, "x");
        builder.finish_node();
        builder.finish_node();
        let root = builder.finish();
        let condition = root.children().next().unwrap();

        let TokenAtOffset::Between(left, right) = condition.token_at_offset(TextSize::from(2))
        else {
            panic!("expected tokens on both sides of the missing slot");
        };

        assert_eq!(left.text_trimmed(), "x");
        assert_eq!(right.text_trimmed(), "x");
        assert_ne!(left, right);
        assert_eq!(left.parent(), Some(condition.clone()));
        assert_eq!(right.parent(), Some(condition.clone()));
        assert_eq!(left.index(), 0);
        assert_eq!(right.index(), 2);
        assert_eq!(left.text_range(), TextRange::new(1.into(), 2.into()));
        assert_eq!(right.text_range(), TextRange::new(2.into(), 3.into()));
        assert_eq!(token_texts_at_offset(&condition, 3), ["x"]);
        assert!(matches!(
            condition.token_at_offset(TextSize::from(0)),
            TokenAtOffset::None
        ));
        assert!(matches!(
            condition.token_at_offset(TextSize::from(4)),
            TokenAtOffset::None
        ));
    }

    #[test]
    fn token_at_offset_returns_none_for_empty_nodes() {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        builder.finish_node();
        let root = builder.finish();

        assert!(matches!(
            root.token_at_offset(TextSize::from(0)),
            TokenAtOffset::None
        ));
    }
}
