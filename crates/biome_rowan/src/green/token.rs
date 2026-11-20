use std::{fmt, ptr};

use crate::green::trivia::GreenTrivia;
use crate::{
    TextSize,
    arc::{ThinArc, ThinArcBorrow},
    green::{GreenElementFlags, RawSyntaxKind},
};

#[derive(PartialEq, Eq, Hash)]
struct GreenTokenHead {
    kind: RawSyntaxKind,
    flags: GreenElementFlags,
    leading: GreenTrivia,
    trailing: GreenTrivia,
    #[cfg(feature = "countme")]
    _c: countme::Count<GreenToken>,
}

#[cfg(feature = "countme")]
pub(crate) fn has_live() -> bool {
    countme::get::<GreenToken>().live > 0
}

/// A [GreenToken] borrowed for `'a`.
///
/// Stands in for a `&'a GreenToken` and is `Copy` like one. A copy is another
/// borrow of the same token, not a new owner: it leaves the reference count
/// alone and can't outlive `'a`. [Self::to_owned] returns an owned
/// [GreenToken].
#[derive(Clone, Copy)]
pub(crate) struct GreenTokenRef<'a> {
    ptr: ThinArcBorrow<'a, GreenTokenHead, u8>,
}

impl PartialEq for GreenTokenRef<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.kind() == other.kind() && self.text() == other.text()
    }
}

/// Leaf node in the immutable tree.
#[derive(PartialEq, Eq, Hash, Clone)]
#[repr(transparent)]
pub(crate) struct GreenToken {
    ptr: ThinArc<GreenTokenHead, u8>,
}

impl fmt::Debug for GreenTokenRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GreenToken")
            .field("kind", &self.kind())
            .field("text", &self.text())
            .field("leading", &self.leading_trivia())
            .field("trailing", &self.trailing_trivia())
            .finish()
    }
}

impl fmt::Debug for GreenToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.as_ref(), f)
    }
}

impl fmt::Display for GreenToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.as_ref(), f)
    }
}

impl fmt::Display for GreenTokenRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text())
    }
}

impl<'a> GreenTokenRef<'a> {
    /// Returns an owned handle to the token, incrementing its reference count.
    #[inline]
    pub(crate) fn to_owned(self) -> GreenToken {
        GreenToken {
            ptr: self.ptr.to_owned(),
        }
    }

    /// Returns the address of the token, which identifies it for as long as
    /// it is alive. [GreenToken::into_raw] returns the same pointer.
    #[inline]
    pub(crate) fn as_ptr(self) -> ptr::NonNull<()> {
        self.ptr.as_ptr()
    }

    /// Creates a borrow from a pointer returned by [Self::as_ptr] or
    /// [GreenToken::into_raw].
    ///
    /// # Safety
    ///
    /// `ptr` must point to a green token that stays alive for `'a`.
    #[inline]
    pub(crate) unsafe fn from_ptr(ptr: ptr::NonNull<()>) -> Self {
        Self {
            // SAFETY: Guaranteed by the caller.
            ptr: unsafe { ThinArcBorrow::from_ptr(ptr) },
        }
    }

    #[inline]
    fn header(self) -> &'a GreenTokenHead {
        self.ptr.header()
    }

    /// Kind of this Token.
    #[inline]
    pub fn kind(self) -> RawSyntaxKind {
        self.header().kind
    }

    /// Whole text of this Token, including all trivia.
    #[inline]
    pub fn text(self) -> &'a str {
        unsafe { std::str::from_utf8_unchecked(self.ptr.slice()) }
    }

    pub(crate) fn leading_trailing_total_len(self) -> (TextSize, TextSize, TextSize) {
        let leading_len = self.header().leading.text_len();
        let trailing_len = self.header().trailing.text_len();
        let total_len = self.ptr.slice().len() as u32;
        (leading_len, trailing_len, total_len.into())
    }

    /// Text of this Token, excluding all trivia.
    #[inline]
    pub fn text_trimmed(self) -> &'a str {
        let (leading_len, trailing_len, total_len) = self.leading_trailing_total_len();

        let start: usize = leading_len.into();
        let end: usize = (total_len - trailing_len).into();
        &self.text()[start..end]
    }

    /// Returns the length of the text covered by this token.
    #[inline]
    pub fn text_len(self) -> TextSize {
        TextSize::of(self.text())
    }

    #[inline]
    pub fn leading_trivia(self) -> &'a GreenTrivia {
        &self.header().leading
    }

    #[inline]
    pub fn trailing_trivia(self) -> &'a GreenTrivia {
        &self.header().trailing
    }

    /// Whether the leading or trailing trivia of this token has a comment.
    #[inline]
    pub(crate) fn has_comments(self) -> bool {
        self.flags().contains(GreenElementFlags::HAS_COMMENTS)
    }

    #[inline]
    pub(crate) fn flags(self) -> GreenElementFlags {
        self.header().flags
    }
}

impl GreenToken {
    #[inline]
    pub fn new_raw(kind: RawSyntaxKind, text: &str) -> Self {
        let leading = GreenTrivia::empty();
        let trailing = leading.clone();

        Self::with_trivia(kind, text, leading, trailing)
    }

    #[inline]
    pub fn with_trivia(
        kind: RawSyntaxKind,
        text: &str,
        leading: GreenTrivia,
        trailing: GreenTrivia,
    ) -> Self {
        let flags = leading.flags().union(trailing.flags());
        Self::with_trivia_and_flags(kind, text, leading, trailing, flags)
    }

    #[inline]
    pub(crate) fn with_trivia_and_flags(
        kind: RawSyntaxKind,
        text: &str,
        leading: GreenTrivia,
        trailing: GreenTrivia,
        flags: GreenElementFlags,
    ) -> Self {
        let head = GreenTokenHead {
            kind,
            flags,
            leading,
            trailing,
            #[cfg(feature = "countme")]
            _c: countme::Count::new(),
        };
        let ptr = ThinArc::from_header_and_slice(head, text.as_bytes());
        Self { ptr }
    }

    /// Borrows the token without incrementing its reference count.
    #[inline]
    pub(crate) fn as_ref(&self) -> GreenTokenRef<'_> {
        GreenTokenRef {
            ptr: self.ptr.borrow_arc(),
        }
    }

    /// Converts the token into a pointer without decrementing its reference
    /// count. The reference leaks unless the pointer is passed to
    /// [Self::from_raw].
    #[inline]
    pub(crate) fn into_raw(self) -> ptr::NonNull<()> {
        self.ptr.into_raw()
    }

    /// Reconstructs the token from a pointer returned by [Self::into_raw].
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

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::*;

    #[test]
    fn green_token_text_and_len() {
        let t = GreenToken::with_trivia(
            RawSyntaxKind(0),
            "\n\t let \t\t",
            GreenTrivia::whitespace(3),
            GreenTrivia::whitespace(3),
        );

        assert_eq!("\n\t let \t\t", t.as_ref().text());
        assert_eq!(TextSize::from(9), t.as_ref().text_len());

        assert_eq!("let", t.as_ref().text_trimmed());

        assert_eq!("\n\t let \t\t", format!("{t}"));
    }

    #[test]
    fn empty_text_len() {
        assert_eq!(TextSize::from(0), GreenTrivia::empty().text_len());
    }

    #[quickcheck]
    fn whitespace_and_comments_text_len(len: u32) {
        let len = TextSize::from(len);
        assert_eq!(len, GreenTrivia::whitespace(len).text_len());
        assert_eq!(len, GreenTrivia::single_line_comment(len).text_len());
    }

    #[test]
    fn sizes() {
        assert_eq!(24, std::mem::size_of::<GreenTokenHead>());
        assert_eq!(8, std::mem::size_of::<GreenToken>());
    }
}
