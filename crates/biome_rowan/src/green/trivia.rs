use crate::TriviaPiece;
use crate::arc::{Arc, HeaderSlice, ThinArc};
use crate::green::{GreenArena, GreenElementFlags};
use biome_text_size::TextSize;
use std::fmt::Formatter;
use std::{fmt, mem, ptr};

#[derive(PartialEq, Eq, Hash)]
pub(crate) struct GreenTriviaHead {
    #[cfg(feature = "countme")]
    _c: countme::Count<GreenTrivia>,
}

#[cfg(feature = "countme")]
pub(crate) fn has_live() -> bool {
    countme::get::<GreenTrivia>().live > 0
}

type ReprThin = HeaderSlice<GreenTriviaHead, [TriviaPiece; 0]>;

#[repr(transparent)]
pub(crate) struct GreenTriviaData {
    data: ReprThin,
}

impl GreenTriviaData {
    #[expect(unused)]
    #[inline]
    pub fn header(&self) -> &GreenTriviaHead {
        &self.data.header
    }

    #[inline]
    pub fn pieces(&self) -> &[TriviaPiece] {
        self.data.slice()
    }
}

impl PartialEq for GreenTriviaData {
    fn eq(&self, other: &Self) -> bool {
        self.pieces() == other.pieces()
    }
}

impl fmt::Debug for GreenTriviaData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.pieces().iter()).finish()
    }
}

/// List of trivia. Used to store either the leading or trailing trivia of a token.
/// The identity of a trivia is defined by the kinds and lengths of its items but not by
/// the texts of an individual piece. That means, that `\r` and `\n` can both be represented
/// by the same trivia, a trivia with a single `LINEBREAK` piece with the length 1.
/// This is safe because the text is stored on the token to which the trivia belongs and
/// `a\n` and `a\r` never resolve to the same tokens. Thus, they only share the trivia but are
/// otherwise two different tokens.
#[derive(Eq, PartialEq, Hash, Clone)]
#[repr(transparent)]
pub(crate) struct GreenTrivia {
    ptr: Option<ThinArc<GreenTriviaHead, TriviaPiece>>,
}

impl fmt::Debug for GreenTrivia {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.pieces(), f)
    }
}

impl GreenTrivia {
    #[inline]
    pub(crate) fn flags(&self) -> GreenElementFlags {
        let mut flags = GreenElementFlags::default();
        for piece in self.pieces() {
            flags.insert(GreenElementFlags::from_trivia_kind(piece.kind()));
            if flags.contains_all(GreenElementFlags::HAS_COMMENTS_AND_SKIPPED) {
                break;
            }
        }
        flags
    }

    /// Creates a new trivia containing the passed in pieces
    pub fn new<I>(pieces: I) -> Self
    where
        I: IntoIterator<Item = TriviaPiece>,
        I::IntoIter: ExactSizeIterator,
    {
        // SAFETY: there's no arena.
        unsafe { Self::new_in(pieces, None) }
    }

    /// Creates a trivia, in `arena` if there is one.
    ///
    /// # Safety
    /// The caller must be the thread that runs the build of `arena`, while it's open.
    pub(crate) unsafe fn new_in<I>(pieces: I, arena: Option<&GreenArena>) -> Self
    where
        I: IntoIterator<Item = TriviaPiece>,
        I::IntoIter: ExactSizeIterator,
    {
        // SAFETY: the caller upholds the requirements of the arena.
        let data = unsafe {
            ThinArc::from_header_and_iter_in(
                GreenTriviaHead {
                    #[cfg(feature = "countme")]
                    _c: countme::Count::new(),
                },
                pieces.into_iter(),
                arena,
            )
        };
        #[cfg(feature = "countme")]
        if let Some(arena) = arena {
            // SAFETY: the caller upholds the requirements of the arena, and the trivia never
            // drops its count, as it's in the arena.
            unsafe { arena.adopt_count(super::arena::duplicate(&data.header._c)) };
        }

        Self { ptr: Some(data) }
    }

    /// The count field of the trivia, see [ThinArc::count_field], unless it's empty.
    #[inline]
    pub(crate) fn count_field(&self) -> Option<usize> {
        self.ptr.as_ref().map(ThinArc::count_field)
    }

    /// Creates an empty trivia
    pub fn empty() -> Self {
        Self { ptr: None }
    }

    /// Returns the total length of all pieces
    pub fn text_len(&self) -> TextSize {
        let mut len = TextSize::default();

        for piece in self.pieces() {
            len += piece.length
        }

        len
    }

    /// Returns the pieces count
    pub fn len(&self) -> usize {
        match &self.ptr {
            None => 0,
            Some(ptr) => ptr.len(),
        }
    }

    /// Returns the pieces of the trivia
    pub fn pieces(&self) -> &[TriviaPiece] {
        match &self.ptr {
            None => &[],
            Some(ptr) => ptr.slice(),
        }
    }

    /// Returns the piece at the given index.
    pub fn get_piece(&self, index: usize) -> Option<&TriviaPiece> {
        self.pieces().get(index)
    }

    pub(crate) fn into_raw(self) -> *mut GreenTriviaData {
        self.ptr.map_or_else(ptr::null_mut, |ptr| {
            Arc::from_thin(ptr).into_raw().cast().as_ptr()
        })
    }

    pub(crate) unsafe fn from_raw(ptr: *mut GreenTriviaData) -> Self {
        unsafe {
            if let Some(ptr) = ptr.as_ref() {
                let arc = Arc::from_raw(&ptr.data as *const ReprThin);
                let arc =
                    mem::transmute::<Arc<ReprThin>, ThinArc<GreenTriviaHead, TriviaPiece>>(arc);
                Self { ptr: Some(arc) }
            } else {
                Self { ptr: None }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::TriviaPiece;
    use crate::green::trivia::{GreenTrivia, GreenTriviaHead};
    use crate::syntax::TriviaPieceKind;
    use biome_text_size::TextSize;

    impl GreenTrivia {
        /// Creates a trivia with a single whitespace piece
        pub fn whitespace<L: Into<TextSize>>(len: L) -> Self {
            Self::single(TriviaPieceKind::Whitespace, len.into())
        }

        /// Creates a trivia with one single line comment piece
        pub fn single_line_comment<L: Into<TextSize>>(len: L) -> Self {
            Self::single(TriviaPieceKind::SingleLineComment, len.into())
        }

        /// Creates a trivia containing a single piece
        pub fn single<L: Into<TextSize>>(kind: TriviaPieceKind, len: L) -> Self {
            Self::new([TriviaPiece::new(kind, len)])
        }
    }

    #[test]
    fn sizes() {
        assert_eq!(0, std::mem::size_of::<GreenTriviaHead>());
        assert_eq!(8, std::mem::size_of::<GreenTrivia>());
    }
}
