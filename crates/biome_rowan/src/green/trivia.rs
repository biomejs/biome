use crate::arc::{Arc, HeaderSlice, ThinArc};
use crate::TriviaPiece;
use biome_text_size::TextSize;
use countme::Count;
use std::fmt::Formatter;
use std::mem::ManuallyDrop;
use std::{fmt, mem, ptr};

#[derive(PartialEq, Eq, Hash)]
pub(crate) struct GreenTriviaHead {
    _c: Count<GreenTrivia>,
}

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

    #[inline]
    pub(crate) fn to_owned(&self) -> GreenTrivia {
        unsafe {
            let green = GreenTrivia::from_raw(self as *const _ as *mut _);
            let green = ManuallyDrop::new(green);
            GreenTrivia::clone(&green)
        }
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
    /// Creates a new trivia containing the passed in pieces
    pub fn new<I>(pieces: I) -> Self
    where
        I: IntoIterator<Item = TriviaPiece>,
        I::IntoIter: ExactSizeIterator,
    {
        let data =
            ThinArc::from_header_and_iter(GreenTriviaHead { _c: Count::new() }, pieces.into_iter());

        GreenTrivia { ptr: Some(data) }
    }

    /// Creates an empty trivia
    pub fn empty() -> Self {
        GreenTrivia { ptr: None }
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
        if let Some(ptr) = ptr.as_ref() {
            let arc = Arc::from_raw(&ptr.data as *const ReprThin);
            let arc = mem::transmute::<Arc<ReprThin>, ThinArc<GreenTriviaHead, TriviaPiece>>(arc);
            Self { ptr: Some(arc) }
        } else {
            Self { ptr: None }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::green::trivia::{GreenTrivia, GreenTriviaHead};
    use crate::syntax::TriviaPieceKind;
    use crate::TriviaPiece;
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
