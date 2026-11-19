use crate::TriviaPiece;
use crate::arc::{ThinArc, ThinArcBorrow};
use crate::green::GreenElementFlags;
use biome_text_size::TextSize;
use std::fmt::Formatter;
use std::{fmt, ptr};

#[derive(PartialEq, Eq, Hash)]
pub(crate) struct GreenTriviaHead {
    #[cfg(feature = "countme")]
    _c: countme::Count<GreenTrivia>,
}

#[cfg(feature = "countme")]
pub(crate) fn has_live() -> bool {
    countme::get::<GreenTrivia>().live > 0
}

/// A [GreenTrivia] borrowed for `'a`.
///
/// It is a copy of the pointer of the trivia that doesn't own a reference.
#[derive(Clone, Copy)]
pub(crate) struct GreenTriviaRef<'a> {
    ptr: Option<ThinArcBorrow<'a, GreenTriviaHead, TriviaPiece>>,
}

impl<'a> GreenTriviaRef<'a> {
    #[inline]
    pub fn pieces(self) -> &'a [TriviaPiece] {
        match self.ptr {
            None => &[],
            Some(ptr) => ptr.slice(),
        }
    }

    /// Returns an owned handle to the trivia, incrementing its reference count.
    #[inline]
    pub(crate) fn to_owned(self) -> GreenTrivia {
        GreenTrivia {
            ptr: self.ptr.map(ThinArcBorrow::to_owned),
        }
    }

    /// Creates a borrow from a pointer returned by [GreenTrivia::into_raw].
    ///
    /// # Safety
    ///
    /// `ptr` must be null or point to a green trivia that stays alive for `'a`.
    #[inline]
    pub(crate) unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self {
            // SAFETY: Guaranteed by the caller.
            ptr: ptr::NonNull::new(ptr).map(|ptr| unsafe { ThinArcBorrow::from_ptr(ptr) }),
        }
    }
}

impl fmt::Debug for GreenTriviaRef<'_> {
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
        let data = ThinArc::from_header_and_iter(
            GreenTriviaHead {
                #[cfg(feature = "countme")]
                _c: countme::Count::new(),
            },
            pieces.into_iter(),
        );

        Self { ptr: Some(data) }
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

    /// Converts the trivia into a pointer without decrementing its reference
    /// count. The pointer is null for an empty trivia. The reference leaks
    /// unless the pointer is passed to [Self::from_raw].
    pub(crate) fn into_raw(self) -> *mut () {
        self.ptr
            .map_or_else(ptr::null_mut, |ptr| ptr.into_raw().as_ptr())
    }

    /// Reconstructs the trivia from a pointer returned by [Self::into_raw].
    ///
    /// # Safety
    ///
    /// `ptr` must come from [Self::into_raw], and each call to
    /// [Self::into_raw] allows a single call to this function.
    pub(crate) unsafe fn from_raw(ptr: *mut ()) -> Self {
        Self {
            // SAFETY: Guaranteed by the caller.
            ptr: ptr::NonNull::new(ptr).map(|ptr| unsafe { ThinArc::from_raw(ptr) }),
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
