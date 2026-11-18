use crate::Parser;
use crate::event::Event;
use crate::event::Event::Token;
use crate::token_source::TokenSource;

use biome_rowan::{SyntaxKind, TextRange, TextSize};
#[cfg(debug_assertions)]
use drop_bomb::DebugDropBomb;
use std::fmt;
use std::num::{NonZeroU32, NonZeroU64};

// `Marker` and `CompletedMarker` each pack their four `u32` fields into two `u64`s.
// rustc passes and returns a struct of two scalars in two registers, but a struct of
// four `u32`s through memory. Parse functions return and receive these markers on
// every call, and reading a 16-byte value from memory right after it was written as
// four 4-byte stores stalls the CPU, because it cannot forward the narrow stores to
// the wide load.

/// Packs `low` into the low 32 bits and `high` into the high 32 bits.
#[inline]
const fn pack(low: u32, high: u32) -> u64 {
    (low as u64) | ((high as u64) << 32)
}

#[inline]
const fn low(packed: u64) -> u32 {
    packed as u32
}

#[inline]
const fn high(packed: u64) -> u32 {
    (packed >> 32) as u32
}

/// A structure signifying the start of parsing of a syntax tree node
#[must_use = "Marker must either be `completed` or `abandoned`"]
pub struct Marker {
    /// The index in the events list in the low half, and the byte index where
    /// the node starts in the high half.
    pos_and_start: u64,
    /// The old start in the low half. The high half is the distance from this
    /// marker's start event back to the start event of the completed node this
    /// marker precedes, or 0 if it precedes none.
    old_start_and_child: u64,
    #[cfg(debug_assertions)]
    bomb: DebugDropBomb,
}

impl Marker {
    #[inline]
    pub fn new(pos: u32, start: TextSize) -> Self {
        Self {
            pos_and_start: pack(pos, start.into()),
            old_start_and_child: pack(pos, 0),
            #[cfg(debug_assertions)]
            bomb: DebugDropBomb::new(
                "Marker must either be `completed` or `abandoned` to avoid that children are implicitly attached to a marker's parent.",
            ),
        }
    }

    #[inline]
    fn pos(&self) -> u32 {
        low(self.pos_and_start)
    }

    #[inline]
    fn old_start_pos(&self) -> u32 {
        low(self.old_start_and_child)
    }

    /// Returns the index of the start event of the completed node this marker precedes.
    #[inline]
    fn child_idx(&self) -> Option<usize> {
        match high(self.old_start_and_child) {
            0 => None,
            distance => Some((self.pos() - distance) as usize),
        }
    }

    #[inline]
    fn defuse(&mut self) {
        #[cfg(debug_assertions)]
        self.bomb.defuse();
    }

    #[inline]
    fn old_start(mut self, old: u32) -> Self {
        if self.old_start_pos() >= old {
            self.old_start_and_child = pack(old, high(self.old_start_and_child));
        };
        self
    }

    /// Finishes the syntax tree node and assigns `kind` to it,
    /// and mark the create a `CompletedMarker` for possible future
    /// operation like `.precede()` to deal with forward_parent.
    pub fn complete<P>(mut self, p: &mut P, kind: P::Kind) -> CompletedMarker
    where
        P: Parser,
    {
        self.defuse();
        let context = p.context_mut();

        let idx = self.pos() as usize;
        match context.events[idx] {
            Event::Start {
                kind: ref mut slot, ..
            } => {
                *slot = kind;
            }
            _ => unreachable!(),
        }
        let finish_pos = context.events.len() as u32;
        context.push_event(Event::Finish);

        let new = CompletedMarker::new(self.pos(), finish_pos, self.start());
        new.old_start(self.old_start_pos())
    }

    /// Abandons the syntax tree node. All its children
    /// are attached to its parent instead.
    pub fn abandon<P>(mut self, p: &mut P)
    where
        P: Parser,
    {
        self.defuse();
        let idx = self.pos() as usize;
        if idx == p.context().events.len() - 1
            && let Some(Event::Start {
                forward_parent: None,
                kind,
            }) = p.context_mut().events.pop()
        {
            assert_eq!(kind, P::Kind::TOMBSTONE);
        }
        if let Some(idx) = self.child_idx() {
            match p.context_mut().events[idx] {
                Event::Start {
                    ref mut forward_parent,
                    ..
                } => {
                    *forward_parent = None;
                }
                _ => unreachable!(),
            }
        }
    }

    #[inline]
    pub fn start(&self) -> TextSize {
        TextSize::from(high(self.pos_and_start))
    }
}

impl fmt::Debug for Marker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Marker")
            .field("pos", &self.pos())
            .field("start", &self.start())
            .field("old_start", &self.old_start_pos())
            .field("child_idx", &self.child_idx())
            .finish_non_exhaustive()
    }
}

/// A structure signifying a completed node
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CompletedMarker {
    /// The index of the start event in the low half, and the index of the
    /// finish event in the high half. The finish event always follows the
    /// start event, so this is never zero, which lets [crate::ParsedSyntax]
    /// and `Option<CompletedMarker>` use zero for their empty variant.
    start_and_finish_pos: NonZeroU64,
    /// The byte index where the node starts in the low half, and the old
    /// start in the high half.
    // The old start is a hack for parsing completed markers which have been preceded.
    // This should be redone completely in the future
    offset_and_old_start: u64,
}

impl CompletedMarker {
    /// # Panics
    ///
    /// Panics if both `start_pos` and `finish_pos` are 0. The finish event of
    /// a node always comes after its start event.
    #[inline]
    pub(crate) fn new(start_pos: u32, finish_pos: u32, offset: TextSize) -> Self {
        Self {
            start_and_finish_pos: NonZeroU64::new(pack(start_pos, finish_pos))
                .expect("The finish event of a node must come after its start event"),
            offset_and_old_start: pack(offset.into(), start_pos),
        }
    }

    #[inline]
    fn start_pos(&self) -> u32 {
        low(self.start_and_finish_pos.get())
    }

    #[inline]
    fn finish_pos(&self) -> u32 {
        high(self.start_and_finish_pos.get())
    }

    #[inline]
    fn offset(&self) -> TextSize {
        TextSize::from(low(self.offset_and_old_start))
    }

    #[inline]
    fn old_start_pos(&self) -> u32 {
        high(self.offset_and_old_start)
    }

    #[inline]
    pub(crate) fn old_start(mut self, old: u32) -> Self {
        // For multiple precedes we should not update the old start
        if self.old_start_pos() >= old {
            self.offset_and_old_start = pack(low(self.offset_and_old_start), old);
        };
        self
    }

    /// Change the kind of node this marker represents
    pub fn change_kind<P>(&mut self, p: &mut P, new_kind: P::Kind)
    where
        P: Parser,
    {
        match p
            .context_mut()
            .events
            .get_mut(self.start_pos() as usize)
            .expect("Finish position of marker is OOB")
        {
            Event::Start { kind, .. } => {
                *kind = new_kind;
            }
            _ => unreachable!(),
        }
    }

    pub fn change_to_bogus<P>(&mut self, p: &mut P)
    where
        P: Parser,
    {
        self.change_kind(p, self.kind(p).to_bogus());
    }

    /// Get the range of the marker
    pub fn range<P>(&self, p: &P) -> TextRange
    where
        P: Parser,
    {
        let end = p.context().events[self.old_start_pos() as usize..self.finish_pos() as usize]
            .iter()
            .rev()
            .find_map(|event| match event {
                Token { end, .. } => Some(*end),
                _ => None,
            })
            .unwrap_or(self.offset());

        TextRange::new(self.offset(), end)
    }

    /// Get the underlying text of a marker
    pub fn text<'a, P>(&self, p: &'a P) -> &'a str
    where
        P: Parser,
    {
        &p.source().text()[self.range(p)]
    }

    /// This method allows to create a new node which starts
    /// *before* the current one. That is, parser could start
    /// node `A`, then complete it, and then after parsing the
    /// whole `A`, decide that it should have started some node
    /// `B` before starting `A`. `precede` allows to do exactly
    /// that. See also docs about `forward_parent` in `Event::Start`.
    ///
    /// Given completed events `[START, FINISH]` and its corresponding
    /// `CompletedMarker(pos: 0, _)`.
    /// Append a new `START` events as `[START, FINISH, NEWSTART]`,
    /// then mark `NEWSTART` as `START`'s parent with saving its relative
    /// distance to `NEWSTART` into forward_parent(=2 in this case);
    pub fn precede<P>(self, p: &mut P) -> Marker
    where
        P: Parser,
    {
        let new_pos = p.start();
        let idx = self.start_pos() as usize;
        // Safety: The new marker is always inserted after the start marker of this node, thus
        // subtracting the two positions can never be 0.
        let distance = NonZeroU32::try_from(new_pos.pos() - self.start_pos()).unwrap();
        match p.context_mut().events[idx] {
            Event::Start {
                ref mut forward_parent,
                ..
            } => {
                *forward_parent = Some(distance);
            }
            _ => unreachable!(),
        }
        let new_pos = Marker {
            pos_and_start: pack(new_pos.pos(), self.offset().into()),
            old_start_and_child: pack(new_pos.old_start_pos(), distance.get()),
            #[cfg(debug_assertions)]
            bomb: new_pos.bomb,
        };
        new_pos.old_start(self.old_start_pos())
    }

    /// Undo this completion and turns into a `Marker`
    pub fn undo_completion<P>(self, p: &mut P) -> Marker
    where
        P: Parser,
    {
        let start_idx = self.start_pos() as usize;
        let finish_idx = self.finish_pos() as usize;

        let events = &mut p.context_mut().events;

        match events[start_idx] {
            Event::Start {
                ref mut kind,
                forward_parent: None,
            } => *kind = P::Kind::TOMBSTONE,
            _ => unreachable!(),
        }
        match events[finish_idx] {
            ref mut slot @ Event::Finish => *slot = Event::tombstone(),
            _ => unreachable!(),
        }
        Marker::new(self.start_pos(), self.offset())
    }

    pub fn kind<P>(&self, p: &P) -> P::Kind
    where
        P: Parser,
    {
        match p.context().events[self.start_pos() as usize] {
            Event::Start { kind, .. } => kind,
            _ => unreachable!(),
        }
    }
}

impl fmt::Debug for CompletedMarker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompletedMarker")
            .field("start_pos", &self.start_pos())
            .field("offset", &self.offset())
            .field("old_start", &self.old_start_pos())
            .field("finish_pos", &self.finish_pos())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ParsedSyntax;

    #[test]
    fn markers_fit_in_two_registers() {
        assert_eq!(size_of::<CompletedMarker>(), 16);
        assert_eq!(size_of::<ParsedSyntax>(), 16);
        assert_eq!(size_of::<Option<CompletedMarker>>(), 16);
        #[cfg(not(debug_assertions))]
        assert_eq!(size_of::<Marker>(), 16);
    }

    #[test]
    fn completed_marker_keeps_every_field() {
        let marker = CompletedMarker::new(u32::MAX - 1, u32::MAX, TextSize::from(u32::MAX));
        assert_eq!(marker.start_pos(), u32::MAX - 1);
        assert_eq!(marker.finish_pos(), u32::MAX);
        assert_eq!(marker.offset(), TextSize::from(u32::MAX));
        assert_eq!(marker.old_start_pos(), u32::MAX - 1);

        let marker = CompletedMarker::new(0, 1, TextSize::from(0));
        assert_eq!(marker.start_pos(), 0);
        assert_eq!(marker.finish_pos(), 1);
        assert_eq!(marker.offset(), TextSize::from(0));
        assert_eq!(marker.old_start_pos(), 0);
    }

    #[test]
    fn completed_marker_old_start_only_decreases() {
        let marker = CompletedMarker::new(10, 20, TextSize::from(7));
        let marker = marker.old_start(4);
        assert_eq!(marker.old_start_pos(), 4);
        let marker = marker.old_start(6);
        assert_eq!(marker.old_start_pos(), 4);
        assert_eq!(marker.start_pos(), 10);
        assert_eq!(marker.finish_pos(), 20);
        assert_eq!(marker.offset(), TextSize::from(7));
    }

    #[test]
    fn completed_marker_debug_lists_fields() {
        let marker = CompletedMarker::new(1, 5, TextSize::from(3)).old_start(0);
        assert_eq!(
            format!("{marker:?}"),
            "CompletedMarker { start_pos: 1, offset: 3, old_start: 0, finish_pos: 5 }"
        );
    }

    #[test]
    fn marker_keeps_every_field() {
        let mut marker = Marker::new(u32::MAX, TextSize::from(u32::MAX - 1));
        assert_eq!(marker.pos(), u32::MAX);
        assert_eq!(marker.start(), TextSize::from(u32::MAX - 1));
        assert_eq!(marker.old_start_pos(), u32::MAX);
        assert_eq!(marker.child_idx(), None);

        marker = marker.old_start(3);
        assert_eq!(marker.old_start_pos(), 3);
        marker = marker.old_start(5);
        assert_eq!(marker.old_start_pos(), 3);
        assert_eq!(marker.pos(), u32::MAX);
        assert_eq!(marker.child_idx(), None);
        marker.defuse();
    }

    #[test]
    fn marker_child_idx_is_relative_to_its_position() {
        let mut marker = Marker::new(9, TextSize::from(0));
        marker.old_start_and_child = pack(marker.old_start_pos(), 9);
        assert_eq!(marker.child_idx(), Some(0));
        marker.old_start_and_child = pack(marker.old_start_pos(), 2);
        assert_eq!(marker.child_idx(), Some(7));
        assert_eq!(marker.old_start_pos(), 9);
        marker.defuse();
    }
}
