use crate::GreenToken;
use biome_text_size::{TextRange, TextSize};
use std::ops::Deref;
use std::{borrow::Borrow, fmt::Formatter};

/// Reference to the text of a SyntaxToken without having to worry about the lifetime of `&str`.
#[derive(Eq, Clone)]
#[repr(C)]
pub struct TokenText {
    // Using a green token to ensure this type is Send + Sync.
    token: GreenToken,
    /// Relative range of the "selected" token text.
    range: TextRange,
}

impl std::hash::Hash for TokenText {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.text().hash(state);
    }
}

impl Ord for TokenText {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.text().cmp(other.text())
    }
}

impl PartialOrd for TokenText {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TokenText {
    #[inline]
    pub fn new_raw(kind: crate::RawSyntaxKind, text: &str) -> Self {
        Self::new(GreenToken::new_raw(kind, text))
    }

    pub(crate) fn new(token: GreenToken) -> Self {
        let range = TextRange::at(TextSize::default(), token.text_len());
        Self { token, range }
    }

    pub(crate) fn with_range(token: GreenToken, range: TextRange) -> Self {
        debug_assert!(range.end() <= token.text_len());
        Self { token, range }
    }

    /// Returns the length of the text
    pub fn len(&self) -> TextSize {
        self.range.len()
    }

    /// Returns `true` if the text is empty
    pub fn is_empty(&self) -> bool {
        self.range.is_empty()
    }

    /// Returns a subslice of the text.
    /// `range.end()` must be lower or equal to `self.len()`
    pub fn slice(mut self, range: TextRange) -> Self {
        assert!(
            range.end() <= self.len(),
            "Range {range:?} exceeds the text length {:?}",
            self.len()
        );
        self.range = range + self.range.start();
        self
    }

    pub fn text(&self) -> &str {
        &self.token.text()[self.range]
    }
}

impl Deref for TokenText {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.text()
    }
}

impl std::fmt::Display for TokenText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

impl std::fmt::Debug for TokenText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.text())
    }
}

impl PartialEq for TokenText {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl PartialEq<&'_ str> for TokenText {
    fn eq(&self, rhs: &&'_ str) -> bool {
        **self == **rhs
    }
}

impl PartialEq<TokenText> for &'_ str {
    fn eq(&self, other: &TokenText) -> bool {
        **self == **other
    }
}

impl AsRef<str> for TokenText {
    fn as_ref(&self) -> &str {
        self.text()
    }
}

impl Borrow<str> for TokenText {
    fn borrow(&self) -> &str {
        self.text()
    }
}
