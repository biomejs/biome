use std::convert::TryFrom;
use std::fmt::Formatter;
use std::iter::Enumerate;
use std::{fmt, iter::FusedIterator, ops, ptr, slice};

#[cfg(target_pointer_width = "64")]
use crate::utility_types::static_assert;
#[cfg(target_pointer_width = "64")]
use std::mem;

use crate::{
    GreenToken, NodeOrToken, TextRange, TextSize,
    arc::{ThinArc, ThinArcBorrow},
    green::{GreenElement, GreenElementFlags, GreenElementRef, RawSyntaxKind},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct GreenNodeHead {
    kind: RawSyntaxKind,
    flags: GreenElementFlags,
    text_len: TextSize,
    #[cfg(feature = "countme")]
    _c: countme::Count<GreenNode>,
}

#[cfg(target_pointer_width = "64")]
static_assert!(mem::size_of::<GreenElementFlags>() == 1);
#[cfg(target_pointer_width = "64")]
static_assert!(mem::size_of::<GreenNodeHead>() == 8);

#[cfg(feature = "countme")]
pub(crate) fn has_live() -> bool {
    countme::get::<GreenNode>().live > 0
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Slot {
    Node {
        rel_offset: TextSize,
        node: GreenNode,
    },
    Token {
        rel_offset: TextSize,
        token: GreenToken,
    },
    /// An empty slot for a child that was missing in the source because:
    /// * it's an optional child which is missing for this node
    /// * it's a mandatory child but it's missing because of a syntax error
    Empty { rel_offset: TextSize },
}

impl std::fmt::Display for Slot {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty { .. } => write!(f, "∅"),
            Self::Node { node, .. } => std::fmt::Display::fmt(node, f),
            Self::Token { token, .. } => std::fmt::Display::fmt(token, f),
        }
    }
}

#[cfg(target_pointer_width = "64")]
static_assert!(mem::size_of::<Slot>() == mem::size_of::<usize>() * 2);

/// A [GreenNode] borrowed for `'a`.
///
/// It is a copy of the pointer of the node that doesn't own a reference, so
/// copying it doesn't touch the reference count of the node.
#[derive(Clone, Copy)]
pub(crate) struct GreenNodeRef<'a> {
    ptr: ThinArcBorrow<'a, GreenNodeHead, Slot>,
}

impl PartialEq for GreenNodeRef<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.header() == other.header() && self.slice() == other.slice()
    }
}

/// Internal node in the immutable tree.
/// It has other nodes and tokens as children.
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub(crate) struct GreenNode {
    ptr: ThinArc<GreenNodeHead, Slot>,
}

impl From<GreenNodeRef<'_>> for GreenNode {
    #[inline]
    fn from(borrow: GreenNodeRef<'_>) -> Self {
        borrow.to_owned()
    }
}

impl fmt::Debug for GreenNodeRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GreenNode")
            .field("kind", &self.kind())
            .field("text_len", &self.text_len())
            .field("n_slots", &self.slots().len())
            .finish()
    }
}

impl fmt::Debug for GreenNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.as_ref(), f)
    }
}

impl fmt::Display for GreenNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.as_ref(), f)
    }
}

impl fmt::Display for GreenNodeRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for child in self.slots() {
            write!(f, "{child}")?;
        }
        Ok(())
    }
}

impl<'a> GreenNodeRef<'a> {
    /// Returns an owned handle to the node, incrementing its reference count.
    #[inline]
    pub(crate) fn to_owned(self) -> GreenNode {
        GreenNode {
            ptr: self.ptr.to_owned(),
        }
    }

    /// Returns the address of the node, which identifies it for as long as
    /// it is alive. [GreenNode::into_raw] returns the same pointer.
    #[inline]
    pub(crate) fn as_ptr(self) -> ptr::NonNull<()> {
        self.ptr.as_ptr()
    }

    /// Creates a borrow from a pointer returned by [Self::as_ptr] or
    /// [GreenNode::into_raw].
    ///
    /// # Safety
    ///
    /// `ptr` must point to a green node that stays alive for `'a`.
    #[inline]
    pub(crate) unsafe fn from_ptr(ptr: ptr::NonNull<()>) -> Self {
        Self {
            // SAFETY: Guaranteed by the caller.
            ptr: unsafe { ThinArcBorrow::from_ptr(ptr) },
        }
    }

    pub(crate) fn write_source_text(self, output: &mut String) {
        for slot in self.slots() {
            match slot {
                Slot::Node { node, .. } => node.as_ref().write_source_text(output),
                Slot::Token { token, .. } => output.push_str(token.as_ref().text()),
                Slot::Empty { .. } => {}
            }
        }
    }

    #[inline]
    fn header(self) -> &'a GreenNodeHead {
        self.ptr.header()
    }

    #[inline]
    pub(crate) fn slice(self) -> &'a [Slot] {
        self.ptr.slice()
    }

    /// Kind of this node.
    #[inline]
    pub fn kind(self) -> RawSyntaxKind {
        self.header().kind
    }

    /// Returns the length of the text covered by this node.
    #[inline]
    pub fn text_len(self) -> TextSize {
        self.header().text_len
    }

    #[inline]
    pub(crate) fn has_comments(self) -> bool {
        self.flags().contains(GreenElementFlags::HAS_COMMENTS)
    }

    #[inline]
    pub(crate) fn has_skipped(self) -> bool {
        self.flags().contains(GreenElementFlags::HAS_SKIPPED)
    }

    #[inline]
    pub(crate) fn flags(self) -> GreenElementFlags {
        self.header().flags
    }

    /// Children of this node.
    #[inline]
    pub fn children(self) -> Children<'a> {
        Children::new(self.slots().enumerate())
    }

    /// Returns the slots of this node. Every node of a specific kind has the same number of slots
    /// to allow using fixed offsets to retrieve a specific child even if some other child is missing.
    #[inline]
    pub fn slots(self) -> Slots<'a> {
        Slots {
            raw: self.slice().iter(),
        }
    }

    /// Returns the slot whose range contains `rel_range`, with its index and
    /// offset relative to this node.
    ///
    /// For a non-empty range at most one slot contains it. An empty range can
    /// sit on the boundary of several slots. Then a slot that strictly
    /// contains it wins, otherwise the last slot that ends at it, otherwise
    /// the first slot when the range is at the start of this node.
    pub(crate) fn slot_at_range(self, rel_range: TextRange) -> Option<(usize, TextSize, &'a Slot)> {
        if rel_range.is_empty() {
            return self.slot_at_range_by_lengths(rel_range);
        }

        // Slots are contiguous: each one ends where the next one starts, and
        // the last one ends where this node ends. Reading the ends from the
        // offsets avoids loading every probed child to read its length.
        let slots = self.slice();
        let idx = slots
            .partition_point(|slot| slot.rel_offset() <= rel_range.start())
            .checked_sub(1)?;
        let slot = &slots[idx];
        let end = slots.get(idx + 1).map_or(self.text_len(), Slot::rel_offset);
        (rel_range.end() <= end).then_some((idx, slot.rel_offset(), slot))
    }

    /// Returns the same slot as [Self::slot_at_range], for empty and non-empty
    /// ranges, but loads each child the binary search probes to read its
    /// length.
    fn slot_at_range_by_lengths(self, rel_range: TextRange) -> Option<(usize, TextSize, &'a Slot)> {
        let idx = self
            .slice()
            .binary_search_by(|it| {
                let child_range = it.rel_range();
                TextRange::ordering(child_range, rel_range)
            })
            // XXX: this handles empty ranges
            .unwrap_or_else(|it| it.saturating_sub(1));
        let slot = &self
            .slice()
            .get(idx)
            .filter(|it| it.rel_range().contains_range(rel_range))?;
        Some((idx, slot.rel_offset(), slot))
    }

    /// Returns a pair of optional children around `offset`:
    ///
    /// - Left: starts before `offset` and ends at or after it.
    /// - Right: starts exactly at `offset`.
    ///
    /// Missing slots and children with zero-length text ranges are skipped,
    /// including zero-length tokens and nodes with no source text.
    ///
    /// For the JavaScript call `foo(1)`, the two children cover `foo` (`0..3`)
    /// and `(1)` (`3..6`). The pairs below show each child's source text:
    ///
    /// ```text
    /// offset 0 → (None, Some("foo"))
    /// offset 2 → (Some("foo"), None)
    /// offset 3 → (Some("foo"), Some("(1)"))
    /// offset 4 → (Some("(1)"), None)
    /// offset 6 → (Some("(1)"), None)
    /// ```
    pub(crate) fn children_at_offset(
        self,
        offset: TextSize,
    ) -> (Option<Child<'a>>, Option<Child<'a>>) {
        let slots = self.slice();
        let offset_index = slots.partition_point(|slot| slot.rel_offset() < offset);
        let is_at_offset = |slot: &Slot| {
            let range = slot.rel_range();
            !range.is_empty() && range.contains_inclusive(offset)
        };

        let left = slots[..offset_index]
            .iter()
            .enumerate()
            .rev()
            .find(|(_, slot)| is_at_offset(slot))
            .and_then(|(index, slot)| Child::try_from((index, slot)).ok());
        let right = slots[offset_index..]
            .iter()
            .enumerate()
            .take_while(|(_, slot)| slot.rel_offset() == offset)
            .find(|(_, slot)| is_at_offset(slot))
            .and_then(|(index, slot)| Child::try_from((offset_index + index, slot)).ok());

        (left, right)
    }

    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub(crate) fn splice_slots<R, I>(self, range: R, replace_with: I) -> GreenNode
    where
        R: ops::RangeBounds<usize>,
        I: Iterator<Item = Option<GreenElement>>,
    {
        let mut slots: Vec<_> = self
            .slots()
            .map(|slot| match slot {
                Slot::Empty { .. } => None,
                Slot::Node { node, .. } => Some(NodeOrToken::Node(node.clone())),
                Slot::Token { token, .. } => Some(NodeOrToken::Token(token.clone())),
            })
            .collect();

        slots.splice(range, replace_with);
        GreenNode::new(self.kind(), slots)
    }

    /// Returns a copy of this node in which each slot named in `edits` is
    /// replaced with the elements of its replacement, which may be empty to
    /// remove the slot.
    ///
    /// `edits` is expected in ascending slot order. An edit is ignored if its
    /// index is not greater than the index of an earlier edit, or if it is past
    /// the last slot. For edits in ascending order without duplicates that
    /// only name existing slots, the result is the same as calling
    /// [Self::splice_slots] once per edit in descending order, but the slots
    /// are copied only once.
    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub(crate) fn splice_many_slots<E, R>(self, edits: E) -> GreenNode
    where
        E: IntoIterator<Item = (usize, R)>,
        R: IntoIterator<Item = Option<GreenElement>>,
    {
        let mut edits = edits.into_iter().peekable();
        let mut slots = Vec::with_capacity(self.slice().len());

        for (index, slot) in self.slots().enumerate() {
            // Skip the edits of slots that were already passed
            while edits
                .next_if(|(edit_index, _)| *edit_index < index)
                .is_some()
            {}
            if let Some((_, replacement)) = edits.next_if(|(edit_index, _)| *edit_index == index) {
                slots.extend(replacement);
            } else {
                slots.push(match slot {
                    Slot::Empty { .. } => None,
                    Slot::Node { node, .. } => Some(NodeOrToken::Node(node.clone())),
                    Slot::Token { token, .. } => Some(NodeOrToken::Token(token.clone())),
                });
            }
        }

        GreenNode::new(self.kind(), slots)
    }
}

impl GreenNode {
    /// Creates new Node.
    #[inline]
    pub fn new<I>(kind: RawSyntaxKind, slots: I) -> Self
    where
        I: IntoIterator<Item = Option<GreenElement>>,
        I::IntoIter: ExactSizeIterator,
    {
        let data = ThinArc::from_header_and_iter_with(
            GreenNodeHead {
                kind,
                flags: GreenElementFlags::default(),
                text_len: 0.into(),
                #[cfg(feature = "countme")]
                _c: countme::Count::new(),
            },
            slots.into_iter(),
            |head, el| {
                let rel_offset = head.text_len;
                match el {
                    Some(NodeOrToken::Node(node)) => {
                        let node_ref = node.as_ref();
                        head.text_len += node_ref.text_len();
                        head.flags.insert(node_ref.flags());
                        Slot::Node { rel_offset, node }
                    }
                    Some(NodeOrToken::Token(token)) => {
                        let token_ref = token.as_ref();
                        head.text_len += token_ref.text_len();
                        head.flags.insert(token_ref.flags());
                        Slot::Token { rel_offset, token }
                    }
                    None => Slot::Empty { rel_offset },
                }
            },
        );

        Self { ptr: data }
    }

    /// Borrows the node without incrementing its reference count.
    #[inline]
    pub(crate) fn as_ref(&self) -> GreenNodeRef<'_> {
        GreenNodeRef {
            ptr: self.ptr.borrow_arc(),
        }
    }

    /// Converts the node into a pointer without decrementing its reference
    /// count. The reference leaks unless the pointer is passed to
    /// [Self::from_raw].
    #[inline]
    pub(crate) fn into_raw(self) -> ptr::NonNull<()> {
        self.ptr.into_raw()
    }

    /// Reconstructs the node from a pointer returned by [Self::into_raw].
    ///
    /// # Safety
    ///
    /// `ptr` must come from [Self::into_raw], and each call to
    /// [Self::into_raw] allows a single call to this function.
    #[inline]
    pub(crate) unsafe fn from_raw(ptr: ptr::NonNull<()>) -> Self {
        Self {
            // SAFETY: Guaranteed by the caller.
            ptr: unsafe { ThinArc::from_raw(ptr) },
        }
    }
}

impl Slot {
    #[inline]
    pub(crate) fn as_ref(&self) -> Option<GreenElementRef<'_>> {
        match self {
            Self::Node { node, .. } => Some(NodeOrToken::Node(node.as_ref())),
            Self::Token { token, .. } => Some(NodeOrToken::Token(token.as_ref())),
            Self::Empty { .. } => None,
        }
    }
    #[inline]
    pub(crate) fn rel_offset(&self) -> TextSize {
        match self {
            Self::Node { rel_offset, .. }
            | Self::Token { rel_offset, .. }
            | Self::Empty { rel_offset } => *rel_offset,
        }
    }
    #[inline]
    pub(crate) fn rel_range(&self) -> TextRange {
        let text_len = match self.as_ref() {
            None => TextSize::from(0),
            Some(element) => element.text_len(),
        };

        TextRange::at(self.rel_offset(), text_len)
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Slots<'a> {
    pub(crate) raw: slice::Iter<'a, Slot>,
}

// NB: forward everything stable that iter::Slice specializes as of Rust 1.39.0
impl ExactSizeIterator for Slots<'_> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.raw.len()
    }
}

impl<'a> Iterator for Slots<'a> {
    type Item = &'a Slot;

    #[inline]
    fn next(&mut self) -> Option<&'a Slot> {
        self.raw.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.raw.size_hint()
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.raw.count()
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
        self.raw.nth(n)
    }

    #[inline]
    fn fold<Acc, Fold>(self, init: Acc, mut f: Fold) -> Acc
    where
        Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        let mut accum = init;
        for x in self {
            accum = f(accum, x);
        }
        accum
    }
}

impl DoubleEndedIterator for Slots<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.raw.next_back()
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.raw.nth_back(n)
    }

    #[inline]
    fn rfold<Acc, Fold>(mut self, init: Acc, mut f: Fold) -> Acc
    where
        Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        let mut accum = init;
        while let Some(x) = self.next_back() {
            accum = f(accum, x);
        }
        accum
    }
}

impl FusedIterator for Slots<'_> {}

#[derive(Debug, Clone)]
pub(crate) struct Child<'a> {
    slot: u32,
    rel_offset: TextSize,
    element: GreenElementRef<'a>,
}

impl<'a> Child<'a> {
    pub fn slot(&self) -> u32 {
        self.slot
    }
    pub fn rel_offset(&self) -> TextSize {
        self.rel_offset
    }
    pub fn element(&self) -> GreenElementRef<'a> {
        self.element
    }
}

impl<'a> TryFrom<(usize, &'a Slot)> for Child<'a> {
    type Error = ();

    fn try_from((index, slot): (usize, &'a Slot)) -> Result<Self, Self::Error> {
        match slot {
            Slot::Empty { .. } => Err(()),
            Slot::Node { node, rel_offset } => Ok(Child {
                element: NodeOrToken::Node(node.as_ref()),
                slot: index as u32,
                rel_offset: *rel_offset,
            }),
            Slot::Token { token, rel_offset } => Ok(Child {
                element: NodeOrToken::Token(token.as_ref()),
                slot: index as u32,
                rel_offset: *rel_offset,
            }),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Children<'a> {
    slots: Enumerate<Slots<'a>>,
}

impl<'a> Children<'a> {
    pub fn new(slots: Enumerate<Slots<'a>>) -> Self {
        Self { slots }
    }
}

impl<'a> Iterator for Children<'a> {
    type Item = Child<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.slots.find_map(|it| Child::try_from(it).ok())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.slots.size_hint()
    }
}

impl DoubleEndedIterator for Children<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.slots.next_back()?;

            if let Ok(child) = Child::try_from(next) {
                return Some(child);
            }
        }
    }
}

impl FusedIterator for Children<'_> {}

#[cfg(test)]
mod tests {
    use crate::TextSize;
    use crate::green::node::Slot;
    use crate::green::{GreenElement, GreenTrivia};
    use crate::raw_language::{RawLanguageKind, RawSyntaxTreeBuilder};
    use crate::{GreenNode, GreenToken, RawSyntaxKind, TextRange, TriviaPiece, TriviaPieceKind};

    fn build_test_list() -> GreenNode {
        let mut builder: RawSyntaxTreeBuilder = RawSyntaxTreeBuilder::new();

        // list
        builder.start_node(RawLanguageKind::SEPARATED_EXPRESSION_LIST);

        // element 1
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token(RawLanguageKind::STRING_TOKEN, "a");
        builder.finish_node();

        // Missing ,

        // element 2
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token(RawLanguageKind::STRING_TOKEN, "b");
        builder.finish_node();

        builder.finish_node();

        builder.finish_green()
    }

    #[test]
    fn children() {
        let root = build_test_list();
        let root = root.as_ref();

        // Test that children skips missing
        assert_eq!(root.children().count(), 2);
        assert_eq!(
            root.children()
                .map(|child| child.element.to_string())
                .collect::<Vec<_>>(),
            vec!["a", "b"]
        );

        // Slot 2 (index 1) is empty
        assert_eq!(
            root.children().map(|child| child.slot).collect::<Vec<_>>(),
            vec![0, 2]
        );

        // Same when reverse
        assert_eq!(
            root.children()
                .rev()
                .map(|child| child.slot)
                .collect::<Vec<_>>(),
            vec![2, 0]
        );
    }

    #[test]
    fn slots() {
        let root = build_test_list();
        let root = root.as_ref();

        // Has 3 slots, one is missing
        assert_eq!(root.slots().len(), 3);
    }

    /// Compares the offset-based search with the length-based one on every
    /// sequence of up to five slots drawn from missing slots, empty and
    /// non-empty tokens, and empty and non-empty nodes, for every range that
    /// starts and ends within one character past the node.
    // Skipped under Miri. Enumerating every slot sequence and range takes tens
    // of minutes there.
    #[cfg_attr(miri, ignore)]
    #[test]
    fn slot_at_range_matches_search_by_lengths() {
        let token = |text: &str| {
            Some(GreenElement::Token(GreenToken::new_raw(
                RawSyntaxKind(1),
                text,
            )))
        };
        let node = |text: &str| {
            Some(GreenElement::Node(GreenNode::new(
                RawSyntaxKind(2),
                [token(text)],
            )))
        };
        let alphabet = [
            None,
            token(""),
            token("a"),
            token("abc"),
            node(""),
            node("ab"),
        ];

        let mut sequences = vec![vec![]];
        let mut longest = vec![vec![]];
        for _ in 0..5 {
            longest = longest
                .iter()
                .flat_map(|sequence: &Vec<_>| {
                    alphabet.iter().map(move |slot| {
                        let mut sequence = sequence.clone();
                        sequence.push(slot.clone());
                        sequence
                    })
                })
                .collect();
            sequences.extend(longest.iter().cloned());
        }

        for sequence in sequences {
            let green = GreenNode::new(RawSyntaxKind(0), sequence);
            let green = green.as_ref();
            let len = u32::from(green.text_len()) + 1;
            for start in 0..=len {
                for end in start..=len {
                    let range = TextRange::new(start.into(), end.into());
                    let actual = green
                        .slot_at_range(range)
                        .map(|(index, offset, _)| (index, offset));
                    let expected = green
                        .slot_at_range_by_lengths(range)
                        .map(|(index, offset, _)| (index, offset));
                    assert_eq!(actual, expected, "{range:?} in {green:?}");
                }
            }
        }
    }

    #[test]
    fn slot_offsets_and_text_len() {
        let token = |text: &str| GreenElement::Token(GreenToken::new_raw(RawSyntaxKind(1), text));
        let node = GreenNode::new(
            RawSyntaxKind(0),
            [
                Some(token("ab")),
                None,
                Some(GreenElement::Node(GreenNode::new(
                    RawSyntaxKind(2),
                    [Some(token("c"))],
                ))),
                Some(token("")),
                Some(token("def")),
            ],
        );
        let node = node.as_ref();

        assert_eq!(node.kind(), RawSyntaxKind(0));
        assert_eq!(node.text_len(), TextSize::from(6));
        assert_eq!(
            node.slots().map(Slot::rel_offset).collect::<Vec<_>>(),
            [0, 2, 2, 3, 3].map(TextSize::from)
        );
        assert!(matches!(node.slice()[1], Slot::Empty { .. }));
        assert!(matches!(node.slice()[2], Slot::Node { .. }));
        let mut text = String::new();
        node.write_source_text(&mut text);
        assert_eq!(text, "abcdef");
    }

    #[test]
    fn propagates_trivia_flags() {
        let comment_token = GreenToken::with_trivia(
            RawSyntaxKind(0),
            "//value",
            GreenTrivia::new([TriviaPiece::single_line_comment(2)]),
            GreenTrivia::empty(),
        );
        let comment_node =
            GreenNode::new(RawSyntaxKind(0), [Some(GreenElement::Token(comment_token))]);
        assert!(comment_node.as_ref().has_comments());
        assert!(!comment_node.as_ref().has_skipped());

        let skipped_token = GreenToken::with_trivia(
            RawSyntaxKind(0),
            "?value",
            GreenTrivia::new([TriviaPiece::new(TriviaPieceKind::Skipped, 1)]),
            GreenTrivia::empty(),
        );
        let skipped_node =
            GreenNode::new(RawSyntaxKind(0), [Some(GreenElement::Token(skipped_token))]);
        assert!(!skipped_node.as_ref().has_comments());
        assert!(skipped_node.as_ref().has_skipped());

        let root = GreenNode::new(
            RawSyntaxKind(0),
            [
                Some(GreenElement::Node(comment_node)),
                Some(GreenElement::Node(skipped_node)),
            ],
        );
        assert!(root.as_ref().has_comments());
        assert!(root.as_ref().has_skipped());
    }

    #[test]
    fn splice_many_slots_ignores_out_of_order_and_out_of_range_edits() {
        let root = build_test_list();
        let token = |text| {
            Some(GreenElement::Token(GreenToken::new_raw(
                RawSyntaxKind(0),
                text,
            )))
        };

        let spliced = root.as_ref().splice_many_slots([
            (0, vec![token("x")]),
            (0, vec![token("y")]),
            (2, vec![]),
            (1, vec![token("z")]),
            (3, vec![token("w")]),
        ]);

        assert_eq!(spliced.as_ref().slots().len(), 2);
        assert_eq!(spliced.to_string(), "x∅");
    }
}
