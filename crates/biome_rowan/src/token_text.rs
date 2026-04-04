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

    /// Returns the range of this text relative to the token.
    pub fn relative_range(&self) -> TextRange {
        self.range
    }

    /// Given the range of the token in the source file, returns the range of this text in the source file.
    pub fn source_range(&self, whole_token_range: TextRange) -> TextRange {
        TextRange::new(
            whole_token_range.start() + self.range.start(),
            whole_token_range.start() + self.range.end(),
        )
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

    /// Returns a view into this `TokenText` with leading and trailing
    /// Unicode whitespace removed, without allocating.
    ///
    /// Whitespace is determined via `char::is_whitespace`.
    /// The returned `TokenText` references the same underlying token with
    /// an adjusted range.
    #[inline]
    pub fn trim_token(self) -> Self {
        let s = self.text();

        // Compute leading whitespace (in bytes)
        let mut start_bytes = 0;
        for (idx, ch) in s.char_indices() {
            if ch.is_whitespace() {
                start_bytes = idx + ch.len_utf8();
            } else {
                break;
            }
        }

        // Compute trailing whitespace (in bytes)
        let mut end_bytes = s.len();
        for (idx, ch) in s.char_indices().rev() {
            if ch.is_whitespace() {
                end_bytes = idx;
            } else {
                break;
            }
        }

        // Create a slice of the current view; range is relative to self.range
        // Clamp to avoid start > end when the entire slice is whitespace.
        let (start_u, end_u) = if end_bytes < start_bytes {
            (start_bytes, start_bytes)
        } else {
            (start_bytes, end_bytes)
        };
        let start = TextSize::from(start_u as u32);
        let end = TextSize::from(end_u as u32);
        let range = TextRange::new(start, end);
        self.slice(range)
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

impl TokenText {
    /// Returns an iterator over substrings of this `TokenText`, separated by
    /// occurrences of the given pattern.
    ///
    /// The returned items are `TokenText` values that reference the same underlying
    /// token, with adjusted ranges. This avoids copying or allocating new strings.
    ///
    /// Supported patterns:
    /// - `char`
    /// - `&str` (empty patterns are treated as "no-op", yielding the original text as a single item)
    pub fn split<P>(&self, pattern: P) -> TokenTextSplit<P>
    where
        P: TokenTextPattern,
    {
        TokenTextSplit {
            token: self.token.clone(),
            remaining: self.range,
            pattern,
            finished: false,
        }
    }
}

/// A trait representing a pattern that can be used to split a `TokenText`.
pub trait TokenTextPattern {
    /// Finds the next match of this pattern in `haystack`,
    /// returning the byte range `(start, end)` of the match.
    fn find(&self, haystack: &str) -> Option<(usize, usize)>;
}

impl TokenTextPattern for char {
    fn find(&self, haystack: &str) -> Option<(usize, usize)> {
        haystack
            .find(*self)
            .map(|start| (start, start + self.len_utf8()))
    }
}

impl TokenTextPattern for &str {
    fn find(&self, haystack: &str) -> Option<(usize, usize)> {
        // Treat empty pattern as "no-op" to avoid infinite splitting behavior.
        if self.is_empty() {
            return None;
        }
        haystack.find(self).map(|start| (start, start + self.len()))
    }
}

/// Iterator over the substrings of a `TokenText` separated by a pattern.
pub struct TokenTextSplit<P> {
    token: GreenToken,
    remaining: TextRange,
    pattern: P,
    finished: bool,
}

impl<P: TokenTextPattern> Iterator for TokenTextSplit<P> {
    type Item = TokenText;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let hay = &self.token.text()[self.remaining];

        if let Some((start, end)) = self.pattern.find(hay) {
            let piece_len = TextSize::from(start as u32);
            let piece_range = TextRange::at(self.remaining.start(), piece_len);

            // Advance remaining past the matched delimiter.
            let skip = TextSize::from(end as u32);
            let new_start = self.remaining.start() + skip;
            self.remaining = TextRange::new(new_start, self.remaining.end());

            Some(TokenText::with_range(self.token.clone(), piece_range))
        } else {
            // No more delimiters; yield the remaining part and finish.
            self.finished = true;
            let piece = TokenText::with_range(self.token.clone(), self.remaining);
            Some(piece)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RawSyntaxKind;

    fn tt(text: &str) -> TokenText {
        TokenText::new_raw(RawSyntaxKind(0), text)
    }

    #[test]
    fn split_char_basic() {
        let t = tt("a,b,c");
        let parts: Vec<String> = t.split(',').map(|p| p.text().to_string()).collect();
        assert_eq!(parts, vec!["a", "b", "c"]);
    }

    #[test]
    fn split_char_adjacent_delims() {
        let t = tt("a,,b,,,c,");
        let parts: Vec<String> = t.split(',').map(|p| p.text().to_string()).collect();
        assert_eq!(parts, vec!["a", "", "b", "", "", "c", ""]);
    }

    #[test]
    fn split_char_leading_trailing() {
        let t = tt(",a,b,");
        let parts: Vec<String> = t.split(',').map(|p| p.text().to_string()).collect();
        assert_eq!(parts, vec!["", "a", "b", ""]);
    }

    #[test]
    fn split_char_no_match() {
        let t = tt("abc");
        let parts: Vec<String> = t.split(',').map(|p| p.text().to_string()).collect();
        assert_eq!(parts, vec!["abc"]);
    }

    #[test]
    fn split_str_basic() {
        let t = tt("foo::bar::baz");
        let parts: Vec<String> = t.split("::").map(|p| p.text().to_string()).collect();
        assert_eq!(parts, vec!["foo", "bar", "baz"]);
    }

    #[test]
    fn split_str_adjacent_delims() {
        let t = tt("a--b----c-");
        let parts: Vec<String> = t.split("--").map(|p| p.text().to_string()).collect();
        // "a--b----c-" -> ["a", "b", "", "c-"]
        assert_eq!(parts, vec!["a", "b", "", "c-"]);
    }

    #[test]
    fn split_str_leading_trailing() {
        let t = tt("::a::b::");
        let parts: Vec<String> = t.split("::").map(|p| p.text().to_string()).collect();
        assert_eq!(parts, vec!["", "a", "b", ""]);
    }

    #[test]
    fn split_str_empty_pattern_is_noop() {
        let t = tt("abc");
        let parts: Vec<String> = t.split("").map(|p| p.text().to_string()).collect();
        // We chose to treat empty pattern as no-op: single item
        assert_eq!(parts, vec!["abc"]);
    }

    #[test]
    fn split_preserves_send_sync_semantics_by_referencing_same_token() {
        let t = tt("a,b");
        let mut it = t.split(',');
        let first = it.next().unwrap();
        let second = it.next().unwrap();
        assert_eq!(first.text(), "a");
        assert_eq!(second.text(), "b");
        // Ensure both items are views into the same underlying token by comparing against slices of the original text
        let original = t.text().to_string();
        assert_eq!(&original[0..1], first.text());
        assert_eq!(&original[2..3], second.text());
    }

    #[test]
    fn split_iter_exhaustion() {
        let t = tt("x|y");
        let mut it = t.split('|');
        assert!(it.next().is_some());
        assert!(it.next().is_some());
        assert!(it.next().is_none());
        assert!(it.next().is_none());
    }

    #[test]
    fn trim_basic_whitespace() {
        let t = tt("  \t\nhello world\n\t  ");
        let trimmed = t.trim_token();
        assert_eq!(trimmed.text(), "hello world");
    }

    #[test]
    fn trim_all_whitespace_becomes_empty() {
        let t = tt(" \t\n\r ");
        let trimmed = t.trim_token();
        assert_eq!(trimmed.text(), "");
        assert!(trimmed.is_empty());
        assert_eq!(trimmed.len(), TextSize::from(0));
    }

    #[test]
    fn trim_no_whitespace_unchanged() {
        let t = tt("nowhitespace");
        let trimmed = t.clone().trim_token();
        assert_eq!(trimmed.text(), "nowhitespace");
        assert_eq!(trimmed.len(), t.len());
    }

    #[test]
    fn trim_preserves_internal_whitespace() {
        let t = tt("  a  b   c  ");
        let trimmed = t.trim_token();
        assert_eq!(trimmed.text(), "a  b   c");
    }
}
