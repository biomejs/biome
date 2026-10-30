use crate::{
    Text, TextRange, TextSize, TokenAtOffset,
    cursor::{SyntaxNode, SyntaxToken},
    green::{GreenNodeData, Slot},
};
use biome_text_size::TextLen;
use std::iter::FusedIterator;
use std::{cmp::Ordering, fmt, slice};

#[derive(Clone)]
pub struct SyntaxNodeText {
    node: SyntaxNode,
    range: TextRange,
}

impl SyntaxNodeText {
    pub(crate) fn new(node: SyntaxNode) -> Self {
        let range = node.text_range();
        Self { node, range }
    }

    pub(crate) fn with_range(node: SyntaxNode, range: TextRange) -> Self {
        Self { node, range }
    }

    pub fn len(&self) -> TextSize {
        self.range.len()
    }

    pub fn is_empty(&self) -> bool {
        self.range.is_empty()
    }

    pub fn contains_char(&self, c: char) -> bool {
        self.try_for_each_chunk(|chunk| if chunk.contains(c) { Err(()) } else { Ok(()) })
            .is_err()
    }

    pub fn find_char(&self, c: char) -> Option<TextSize> {
        let mut acc: TextSize = 0.into();
        self.try_for_each_chunk(|chunk| {
            if let Some(pos) = chunk.find(c) {
                let pos: TextSize = (pos as u32).into();
                return Err(acc + pos);
            }
            acc += TextSize::of(chunk);
            Ok(())
        })
        .err()
    }

    pub fn char_at(&self, offset: TextSize) -> Option<char> {
        let mut start: TextSize = 0.into();
        self.try_for_each_chunk(|chunk| {
            let end = start + TextSize::of(chunk);
            if start <= offset && offset < end {
                let off: usize = u32::from(offset - start) as usize;
                return Err(chunk[off..].chars().next().unwrap());
            }
            start = end;
            Ok(())
        })
        .err()
    }

    pub fn slice<R: private::SyntaxTextRange>(&self, range: R) -> Self {
        let start = range.start().unwrap_or_default();
        let end = range.end().unwrap_or_else(|| self.len());
        assert!(start <= end);
        let len = end - start;
        let start = self.range.start() + start;
        let end = start + len;
        assert!(
            start <= end,
            "invalid slice, range: {:?}, slice: {:?}",
            self.range,
            (range.start(), range.end()),
        );
        let range = TextRange::new(start, end);
        assert!(
            self.range.contains_range(range),
            "invalid slice, range: {:?}, slice: {:?}",
            self.range,
            range,
        );
        Self {
            node: self.node.clone(),
            range,
        }
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        if prefix.is_empty() {
            return true;
        }

        let result =
            self.try_fold_chunks(prefix, |prefix, text| match text.len().cmp(&prefix.len()) {
                Ordering::Equal => Err(text == prefix),
                Ordering::Greater => Err(text.starts_with(prefix)),
                Ordering::Less => {
                    if text == &prefix[..text.len()] {
                        Ok(&prefix[text.len()..])
                    } else {
                        Err(false)
                    }
                }
            });

        match result {
            Ok(_) => false,
            Err(starts_with) => starts_with,
        }
    }

    /// Folds `f` over the text in source order, one chunk per token that the
    /// text's range overlaps. Chunks are never empty.
    ///
    /// # Panics
    ///
    /// When the range starts or ends inside a character, once the fold reaches
    /// that character's token.
    pub fn try_fold_chunks<T, F, E>(&self, init: T, f: F) -> Result<T, E>
    where
        F: FnMut(T, &str) -> Result<T, E>,
    {
        match self.green_chunks() {
            Some(mut chunks) => chunks.try_fold(init, f),
            None => self.try_fold_chunks_by_tokens(init, f),
        }
    }

    /// [Self::try_fold_chunks] for a range that extends past the node, which
    /// only red tokens can reach. A trimmed range does that when every token
    /// of the node is empty without its trivia.
    #[cold]
    fn try_fold_chunks_by_tokens<T, F, E>(&self, init: T, mut f: F) -> Result<T, E>
    where
        F: FnMut(T, &str) -> Result<T, E>,
    {
        self.tokens_with_ranges()
            .try_fold(init, move |acc, (token, range)| {
                let text = &token.text()[range];
                if text.is_empty() {
                    Ok(acc)
                } else {
                    f(acc, text)
                }
            })
    }

    pub fn try_for_each_chunk<F: FnMut(&str) -> Result<(), E>, E>(
        &self,
        mut f: F,
    ) -> Result<(), E> {
        self.try_fold_chunks((), move |(), chunk| f(chunk))
    }

    pub fn for_each_chunk<F: FnMut(&str)>(&self, mut f: F) {
        enum Void {}
        let out = self.try_for_each_chunk(|chunk| {
            f(chunk);
            Ok::<(), Void>(())
        });
        match out {
            Ok(()) => (),
            Err(void) => match void {},
        }
    }

    /// Returns the chunks of the text, read from the green tree, or `None` if
    /// the range extends past the node.
    fn green_chunks(&self) -> Option<GreenChunks<'_>> {
        let node_range = self.node.text_range();
        node_range
            .contains_range(self.range)
            .then(|| GreenChunks::new(self.node.green(), node_range.start(), self.range))
    }

    fn tokens_with_ranges(&self) -> impl FusedIterator<Item = (SyntaxToken, TextRange)> + use<> {
        SyntaxNodeTokenWithRanges::new(self)
    }

    pub fn chars(&self) -> impl FusedIterator<Item = char> + use<> {
        SyntaxNodeTextChars::new(self)
    }

    /// Converts the node text into `Text`, attempting to avoid a string allocation
    /// when the selected range is contained in the first token.
    pub fn into_text(self) -> Text {
        if let Some(token) = self.node.first_token() {
            let token_range = token.text_range();
            if token_range == self.range {
                return token.token_text().into();
            }

            if token_range.contains_range(self.range) && !self.range.is_empty() {
                let range = self.range - token_range.start();
                let text = token.text();
                // TokenText slices defer UTF-8 checks; invalid ranges use the string fallback.
                if text.is_char_boundary(usize::from(range.start()))
                    && text.is_char_boundary(usize::from(range.end()))
                {
                    return token.token_text().slice(range).into();
                }
            }
        }

        self.to_string().into()
    }
}

/// Iterator over the chunks of a range of a green node's text: the non-empty
/// part of the range that each token covers, in source order.
///
/// Unlike [SyntaxNodeTokenWithRanges], it allocates no red node or token per
/// token, and it skips subtrees that end before the range starts.
struct GreenChunks<'a> {
    range: TextRange,
    /// For each node the walk has entered, outermost first: its children that
    /// haven't been visited yet, and the offset where the node starts.
    pending_children: Vec<(slice::Iter<'a, Slot>, TextSize)>,
}

impl<'a> GreenChunks<'a> {
    /// Creates the iterator for `range` of `node`, whose text starts at
    /// `offset`. `range` must lie inside the node's text.
    fn new(node: &'a GreenNodeData, offset: TextSize, range: TextRange) -> Self {
        Self {
            range,
            pending_children: vec![(node.slice().iter(), offset)],
        }
    }
}

impl<'a> Iterator for GreenChunks<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (slots, node_offset) = self.pending_children.last_mut()?;
            let Some(slot) = slots.next() else {
                self.pending_children.pop();
                continue;
            };

            let start = *node_offset + slot.rel_offset();
            if start >= self.range.end() {
                self.pending_children.clear();
                return None;
            }

            match slot {
                Slot::Token { token, .. } => {
                    let token_range = TextRange::at(start, token.text_len());
                    if let Some(range) = token_range.intersect(self.range) {
                        // Slicing before the emptiness check panics on an
                        // empty range inside a character, as the red token
                        // walk does.
                        let text = &token.text()[range - start];
                        if !text.is_empty() {
                            return Some(text);
                        }
                    }
                }
                Slot::Node { node, .. } => {
                    if start + node.text_len() > self.range.start() {
                        self.pending_children.push((node.slice().iter(), start));
                    }
                }
                Slot::Empty { .. } => {}
            }
        }
    }
}

impl FusedIterator for GreenChunks<'_> {}

#[derive(Clone)]
struct SyntaxNodeTokenWithRanges {
    text_range: TextRange,
    next_token: Option<(SyntaxToken, TextRange)>,
}

impl SyntaxNodeTokenWithRanges {
    fn new(text: &SyntaxNodeText) -> Self {
        let text_range = text.range;

        let token = match text.node.token_at_offset(text_range.start()) {
            TokenAtOffset::None => None,
            TokenAtOffset::Single(token) => Some(token),
            TokenAtOffset::Between(_, next) => Some(next),
        };

        Self {
            next_token: token.and_then(|token| Self::with_intersecting_range(token, text_range)),
            text_range,
        }
    }

    fn with_intersecting_range(
        token: SyntaxToken,
        text_range: TextRange,
    ) -> Option<(SyntaxToken, TextRange)> {
        let token_range = token.text_range();

        let range = text_range.intersect(token_range)?;
        Some((token, range - token_range.start()))
    }
}

impl Iterator for SyntaxNodeTokenWithRanges {
    type Item = (SyntaxToken, TextRange);

    fn next(&mut self) -> Option<Self::Item> {
        let (token, range) = self.next_token.take()?;

        self.next_token = token
            .next_token()
            .and_then(|token| Self::with_intersecting_range(token, self.text_range));

        Some((token, range))
    }
}

impl FusedIterator for SyntaxNodeTokenWithRanges {}

/// Iterator over the characters of a [SyntaxNodeText].
///
/// It walks red tokens: unlike [GreenChunks], it owns its state, so it can't
/// borrow the green tree.
#[derive(Clone)]
struct SyntaxNodeTextChars {
    head: Option<(SyntaxToken, TextRange)>,
    tail: SyntaxNodeTokenWithRanges,
    index: TextSize,
}

impl SyntaxNodeTextChars {
    fn new(text: &SyntaxNodeText) -> Self {
        let mut chunks = SyntaxNodeTokenWithRanges::new(text);

        Self {
            head: chunks.next(),
            tail: chunks,
            index: TextSize::default(),
        }
    }
}

impl Iterator for SyntaxNodeTextChars {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (token, range) = self.head.as_ref()?;

            if self.index >= range.end() {
                self.head = self.tail.next();
                self.index = TextSize::default();
                continue;
            }

            let text = token.text();

            // SAFETY: Index check above guarantees that there's at least some text left
            let next_char = text[TextRange::new(self.index, range.end())]
                .chars()
                .next()
                .unwrap();

            self.index += next_char.text_len();
            break Some(next_char);
        }
    }
}

impl FusedIterator for SyntaxNodeTextChars {}

impl fmt::Debug for SyntaxNodeText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.to_string(), f)
    }
}

impl fmt::Display for SyntaxNodeText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.try_for_each_chunk(|chunk| fmt::Display::fmt(chunk, f))
    }
}

impl From<SyntaxNodeText> for String {
    fn from(text: SyntaxNodeText) -> Self {
        text.to_string()
    }
}

impl PartialEq<str> for SyntaxNodeText {
    fn eq(&self, mut rhs: &str) -> bool {
        self.try_for_each_chunk(|chunk| {
            if !rhs.starts_with(chunk) {
                return Err(());
            }
            rhs = &rhs[chunk.len()..];
            Ok(())
        })
        .is_ok()
            && rhs.is_empty()
    }
}

impl PartialEq<SyntaxNodeText> for str {
    fn eq(&self, rhs: &SyntaxNodeText) -> bool {
        rhs == self
    }
}

impl PartialEq<&'_ str> for SyntaxNodeText {
    fn eq(&self, rhs: &&str) -> bool {
        self == *rhs
    }
}

impl PartialEq<SyntaxNodeText> for &'_ str {
    fn eq(&self, rhs: &SyntaxNodeText) -> bool {
        rhs == self
    }
}

impl PartialEq for SyntaxNodeText {
    fn eq(&self, other: &Self) -> bool {
        if self.range.len() != other.range.len() {
            return false;
        }
        // Equal without reading the tokens, which the chunk walks would slice
        // and which panic on an empty range inside a character.
        if self.range.is_empty() {
            return true;
        }

        match (self.green_chunks(), other.green_chunks()) {
            (Some(lhs), Some(rhs)) => chunks_eq(lhs, rhs),
            _ => {
                let mut lhs = self.tokens_with_ranges();
                let mut rhs = other.tokens_with_ranges();
                zip_texts(&mut lhs, &mut rhs).is_none()
                    && lhs.all(|it| it.1.is_empty())
                    && rhs.all(|it| it.1.is_empty())
            }
        }
    }
}

/// Returns `true` if the concatenated chunks of `lhs` and `rhs` are equal.
fn chunks_eq<'a, 'b>(
    mut lhs: impl Iterator<Item = &'a str>,
    mut rhs: impl Iterator<Item = &'b str>,
) -> bool {
    let mut x: &[u8] = &[];
    let mut y: &[u8] = &[];
    loop {
        if x.is_empty() {
            match lhs.next() {
                Some(chunk) => x = chunk.as_bytes(),
                None => return y.is_empty() && rhs.next().is_none(),
            }
        }
        if y.is_empty() {
            match rhs.next() {
                Some(chunk) => y = chunk.as_bytes(),
                None => return false,
            }
        }
        let len = x.len().min(y.len());
        if x[..len] != y[..len] {
            return false;
        }
        x = &x[len..];
        y = &y[len..];
    }
}

fn zip_texts<I: Iterator<Item = (SyntaxToken, TextRange)>>(xs: &mut I, ys: &mut I) -> Option<()> {
    let mut x = xs.next()?;
    let mut y = ys.next()?;
    loop {
        while x.1.is_empty() {
            x = xs.next()?;
        }
        while y.1.is_empty() {
            y = ys.next()?;
        }
        let x_text = &x.0.text()[x.1];
        let y_text = &y.0.text()[y.1];
        if !(x_text.starts_with(y_text) || y_text.starts_with(x_text)) {
            return Some(());
        }
        let advance = std::cmp::min(x.1.len(), y.1.len());
        x.1 = TextRange::new(x.1.start() + advance, x.1.end());
        y.1 = TextRange::new(y.1.start() + advance, y.1.end());
    }
}

impl Eq for SyntaxNodeText {}

mod private {
    use std::ops;

    use crate::{TextRange, TextSize};

    pub trait SyntaxTextRange {
        fn start(&self) -> Option<TextSize>;
        fn end(&self) -> Option<TextSize>;
    }

    impl SyntaxTextRange for TextRange {
        fn start(&self) -> Option<TextSize> {
            Some(Self::start(*self))
        }
        fn end(&self) -> Option<TextSize> {
            Some(Self::end(*self))
        }
    }

    impl SyntaxTextRange for ops::Range<TextSize> {
        fn start(&self) -> Option<TextSize> {
            Some(self.start)
        }
        fn end(&self) -> Option<TextSize> {
            Some(self.end)
        }
    }

    impl SyntaxTextRange for ops::RangeFrom<TextSize> {
        fn start(&self) -> Option<TextSize> {
            Some(self.start)
        }
        fn end(&self) -> Option<TextSize> {
            None
        }
    }

    impl SyntaxTextRange for ops::RangeTo<TextSize> {
        fn start(&self) -> Option<TextSize> {
            None
        }
        fn end(&self) -> Option<TextSize> {
            Some(self.end)
        }
    }

    impl SyntaxTextRange for ops::RangeFull {
        fn start(&self) -> Option<TextSize> {
            None
        }
        fn end(&self) -> Option<TextSize> {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SyntaxNodeText, zip_texts};
    use crate::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    use crate::{SyntaxNode, TextRange, TextSize, TriviaPiece};
    use std::cell::Cell;
    use std::cmp::Ordering;
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::sync::Once;

    fn build_tree(chunks: &[&str]) -> SyntaxNode<RawLanguage> {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        for &chunk in chunks.iter() {
            builder.token(RawLanguageKind::STRING_TOKEN, chunk);
        }
        builder.finish_node();
        builder.finish()
    }

    #[test]
    fn test_text_equality() {
        fn do_check(t1: &[&str], t2: &[&str]) {
            let t1 = build_tree(t1).text_with_trivia();
            let t2 = build_tree(t2).text_with_trivia();
            let expected = t1.to_string() == t2.to_string();
            let actual = t1 == t2;
            assert_eq!(expected, actual, "`{t1}` (SyntaxText) `{t2}` (SyntaxText)");
            let actual = t1 == *t2.to_string();
            assert_eq!(expected, actual, "`{t1}` (SyntaxText) `{t2}` (&str)");
        }
        fn check(t1: &[&str], t2: &[&str]) {
            do_check(t1, t2);
            do_check(t2, t1)
        }

        check(&[""], &[""]);
        check(&["a"], &[""]);
        check(&["a"], &["a"]);
        check(&["abc"], &["def"]);
        check(&["hello", "world"], &["hello", "world"]);
        check(&["hellowo", "rld"], &["hell", "oworld"]);
        check(&["hel", "lowo", "rld"], &["helloworld"]);
        check(&["{", "abc", "}"], &["{", "123", "}"]);
        check(&["{", "abc", "}", "{"], &["{", "123", "}"]);
        check(&["{", "abc", "}"], &["{", "123", "}", "{"]);
        check(&["{", "abc", "}ab"], &["{", "abc", "}", "ab"]);
    }

    #[test]
    fn test_chars() {
        fn check(t1: &[&str], expected: &str) {
            let t1 = build_tree(t1).text_with_trivia();
            let actual = t1.chars().collect::<String>();

            assert_eq!(
                expected, &actual,
                "`{actual}` (SyntaxText) `{expected}` (SyntaxText)"
            );
        }

        check(&[""], "");
        check(&["a"], "a");
        check(&["hello", "world"], "helloworld");
        check(&["hellowo", "rld"], "helloworld");
        check(&["hel", "lowo", "rld"], "helloworld");
        check(&["{", "abc", "}"], "{abc}");
        check(&["{", "abc", "}", "{"], "{abc}{");
        check(&["{", "abc", "}ab"], "{abc}ab");
    }

    #[test]
    fn test_into_text_first_token_ranges() {
        let node = build_tree(&["hello", "world"]);
        for (start, end, expected) in [(0, 5, "hello"), (0, 2, "he"), (1, 4, "ell"), (3, 5, "lo")] {
            let text = node
                .text_with_trivia()
                .slice(TextRange::new(start.into(), end.into()))
                .into_text();
            assert_eq!(text, expected);
            assert!(!text.is_string());
        }
    }

    #[test]
    fn test_into_text_trivia_ranges() {
        let trivia = [
            TriviaPiece::whitespace(1),
            TriviaPiece::single_line_comment(5),
            TriviaPiece::whitespace(1),
        ];
        let node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
            builder.token_with_trivia(
                RawLanguageKind::STRING_TOKEN,
                " /*a*/ hello /*b*/ ",
                &trivia,
                &trivia,
            );
        });

        for (selection, expected) in [
            (node.text_with_trivia(), " /*a*/ hello /*b*/ "),
            (node.text_trimmed(), "hello"),
            (
                node.text_with_trivia()
                    .slice(TextRange::new(1.into(), 18.into())),
                "/*a*/ hello /*b*/",
            ),
            (
                node.text_with_trivia()
                    .slice(TextRange::new(6.into(), 13.into())),
                " hello ",
            ),
        ] {
            let text = selection.into_text();
            assert_eq!(text, expected);
            assert!(!text.is_string());
        }
    }

    #[test]
    fn test_into_text_nested_subtree() {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        builder.token(RawLanguageKind::STRING_TOKEN, "prefix");
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token_with_trivia(
            RawLanguageKind::STRING_TOKEN,
            " hello ",
            &[TriviaPiece::whitespace(1)],
            &[TriviaPiece::whitespace(1)],
        );
        builder.finish_node();
        builder.token(RawLanguageKind::STRING_TOKEN, "suffix");
        builder.finish_node();
        let node = builder.finish().first_child().unwrap();
        assert_eq!(node.text_range_with_trivia().start(), 6.into());

        let text = node.text_trimmed().into_text();
        assert_eq!(text, "hello");
        assert!(!text.is_string());
    }

    #[test]
    fn test_into_text_fallbacks() {
        let node = build_tree(&["hello", "world"]);
        for (start, end, expected) in [
            (0, 10, "helloworld"),
            (5, 10, "world"),
            (6, 9, "orl"),
            (3, 7, "lowo"),
            (0, 0, ""),
            (1, 1, ""),
            (5, 5, ""),
            (10, 10, ""),
        ] {
            let text = node
                .text_with_trivia()
                .slice(TextRange::new(start.into(), end.into()))
                .into_text();
            assert_eq!(text, expected);
            assert!(text.is_string());
        }

        let missing_token = build_tree(&[]).text_with_trivia().into_text();
        assert_eq!(missing_token, "");
        assert!(missing_token.is_string());

        let empty_token = build_tree(&[""]).text_with_trivia().into_text();
        assert_eq!(empty_token, "");
        assert!(!empty_token.is_string());
    }

    #[test]
    fn test_into_text_utf8_ranges() {
        let node = build_tree(&["aé界z"]);
        for (start, end) in [(1, 2), (2, 3), (2, 2)] {
            let selection = node
                .text_with_trivia()
                .slice(TextRange::new(start.into(), end.into()));
            assert!(catch_unwind(AssertUnwindSafe(|| selection.into_text())).is_err());
        }

        for (start, end, expected) in [(1, 3, "é"), (3, 6, "界"), (1, 6, "é界")] {
            let text = node
                .text_with_trivia()
                .slice(TextRange::new(start.into(), end.into()))
                .into_text();
            assert_eq!(text, expected);
            assert!(!text.is_string());
        }
    }

    /// Text of the tree that [build_edge_case_tree] returns.
    const EDGE_CASE_TEXT: &str = "  é界ab é界😀xy ";

    /// Builds a tree with zero-width tokens and nodes, trivia, characters of
    /// two to four bytes, a node whose only token is trivia, and the same
    /// subtree at two offsets, which the node cache stores once.
    fn build_edge_case_tree() -> SyntaxNode<RawLanguage> {
        let whitespace = [TriviaPiece::whitespace(1)];
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        builder.token(RawLanguageKind::STRING_TOKEN, "");

        builder.start_node(RawLanguageKind::BOGUS);
        builder.token_with_trivia(RawLanguageKind::STRING_TOKEN, " ", &whitespace, &[]);
        builder.finish_node();

        let shared_subtree = |builder: &mut RawSyntaxTreeBuilder| {
            builder.start_node(RawLanguageKind::BOGUS);
            builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
            builder.token_with_trivia(RawLanguageKind::STRING_TOKEN, " é界", &whitespace, &[]);
            builder.finish_node();
            builder.finish_node();
        };
        shared_subtree(&mut builder);

        builder.token(RawLanguageKind::STRING_TOKEN, "ab");
        builder.start_node(RawLanguageKind::BOGUS);
        builder.finish_node();
        shared_subtree(&mut builder);

        builder.start_node(RawLanguageKind::BOGUS);
        builder.token(RawLanguageKind::STRING_TOKEN, "😀x");
        builder.token(RawLanguageKind::STRING_TOKEN, "");
        builder.token_with_trivia(RawLanguageKind::STRING_TOKEN, "y ", &[], &whitespace);
        builder.finish_node();

        builder.token(RawLanguageKind::STRING_TOKEN, "");
        builder.finish_node();
        builder.finish()
    }

    thread_local! {
        static SILENCE_PANICS: Cell<bool> = const { Cell::new(false) };
    }

    /// Runs `f`, returning `None` if it panics, without printing the panic
    /// message.
    fn catch<T>(f: impl FnOnce() -> T) -> Option<T> {
        static INSTALL_HOOK: Once = Once::new();
        INSTALL_HOOK.call_once(|| {
            let default_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(move |info| {
                if !SILENCE_PANICS.get() {
                    default_hook(info);
                }
            }));
        });

        SILENCE_PANICS.set(true);
        let result = catch_unwind(AssertUnwindSafe(f));
        SILENCE_PANICS.set(false);
        result.ok()
    }

    /// Every slice of `text`, with the expected string, or `None` for a slice
    /// that splits a character.
    fn slices<'a>(
        text: &SyntaxNodeText,
        expected: &'a str,
    ) -> Vec<(SyntaxNodeText, Option<&'a str>)> {
        let mut slices = Vec::new();
        for start in 0..=expected.len() {
            for end in start..=expected.len() {
                let range = TextRange::new((start as u32).into(), (end as u32).into());
                slices.push((text.slice(range), expected.get(start..end)));
            }
        }
        slices
    }

    /// Collects the first `limit` non-empty chunks of `text`. Stops at the
    /// chunk after them, so the walk reaches one more chunk than it returns.
    fn first_chunks(text: &SyntaxNodeText, limit: usize) -> Vec<String> {
        let mut chunks = Vec::new();
        let _ = text.try_for_each_chunk(|chunk| {
            if chunk.is_empty() {
                return Ok(());
            }
            if chunks.len() == limit {
                return Err(());
            }
            chunks.push(chunk.to_string());
            Ok(())
        });
        chunks
    }

    /// [first_chunks] by walking red tokens, the reference for the chunks and
    /// the panics of every method that folds over chunks.
    fn first_chunks_by_tokens(text: &SyntaxNodeText, limit: usize) -> Vec<String> {
        let mut chunks = Vec::new();
        for (token, range) in text.tokens_with_ranges() {
            let chunk = &token.text()[range];
            if chunk.is_empty() {
                continue;
            }
            if chunks.len() == limit {
                break;
            }
            chunks.push(chunk.to_string());
        }
        chunks
    }

    /// [SyntaxNodeText::starts_with] by walking red tokens, the reference for
    /// its results and panics.
    fn starts_with_by_tokens(text: &SyntaxNodeText, mut prefix: &str) -> bool {
        for (token, range) in text.tokens_with_ranges() {
            if prefix.is_empty() {
                return true;
            }

            let text = &token.text()[range];
            match text.len().cmp(&prefix.len()) {
                Ordering::Equal => return text == prefix,
                Ordering::Greater => return text.starts_with(prefix),
                Ordering::Less => {
                    if text == &prefix[..text.len()] {
                        prefix = &prefix[text.len()..];
                    } else {
                        return false;
                    }
                }
            }
        }

        prefix.is_empty()
    }

    /// `lhs == rhs` by walking red tokens, the reference for its results and
    /// panics.
    fn eq_by_tokens(lhs: &SyntaxNodeText, rhs: &SyntaxNodeText) -> bool {
        if lhs.len() != rhs.len() {
            return false;
        }
        let mut lhs = lhs.tokens_with_ranges();
        let mut rhs = rhs.tokens_with_ranges();
        zip_texts(&mut lhs, &mut rhs).is_none()
            && lhs.all(|it| it.1.is_empty())
            && rhs.all(|it| it.1.is_empty())
    }

    /// Checks every method of `text` against `expected`, the same text as a
    /// string.
    fn check_methods(text: &SyntaxNodeText, expected: &str) {
        assert_eq!(text.to_string(), expected);
        assert_eq!(format!("{text:?}"), format!("{expected:?}"));
        assert_eq!(String::from(text.clone()), expected);
        assert_eq!(text.clone().into_text(), expected);
        assert_eq!(text.len(), TextSize::of(expected));
        assert_eq!(text.is_empty(), expected.is_empty());
        // `chars` isn't checked: on a range that starts inside a token, it
        // starts at the beginning of that token.

        let mut chunks = Vec::new();
        text.for_each_chunk(|chunk| chunks.push(chunk.to_string()));
        assert_eq!(chunks.concat(), expected);
        assert!(chunks.iter().all(|chunk| !chunk.is_empty()), "{chunks:?}");
        let folded = text.try_fold_chunks(String::new(), |mut acc, chunk| {
            acc.push_str(chunk);
            Ok::<_, ()>(acc)
        });
        assert_eq!(folded.as_deref(), Ok(expected));

        for c in ['a', 'b', 'é', '界', '😀', ' ', 'y', 'q'] {
            assert_eq!(text.contains_char(c), expected.contains(c), "{c:?}");
            assert_eq!(
                text.find_char(c),
                expected.find(c).map(|pos| TextSize::from(pos as u32)),
                "{c:?}"
            );
        }

        for offset in 0..=expected.len() + 1 {
            let actual = catch(|| text.char_at(TextSize::from(offset as u32)));
            let char_at = if offset >= expected.len() {
                Some(None)
            } else if expected.is_char_boundary(offset) {
                Some(expected[offset..].chars().next())
            } else {
                // Slicing a chunk in the middle of a character panics.
                None
            };
            assert_eq!(actual, char_at, "char_at({offset})");
        }

        for end in (0..=expected.len()).filter(|&end| expected.is_char_boundary(end)) {
            let prefix = &expected[..end];
            assert!(text.starts_with(prefix), "{prefix:?}");
            assert!(!text.starts_with(&format!("{prefix}q")), "{prefix:?}");
        }
        assert!(!text.starts_with(&format!("{expected} ")));

        assert!(*text == *expected);
        assert!(*expected == *text);
        assert!(*text == expected);
        assert!(expected == *text);
        assert!(*text != *format!("{expected}q"));
        if let Some(last) = expected.chars().last() {
            let changed = format!("{}q", &expected[..expected.len() - last.len_utf8()]);
            assert!(*text != *changed);
        }
    }

    /// Checks that `text`, whose range splits a character, panics where the
    /// red token walk panics and agrees with it elsewhere.
    fn check_split_methods(text: &SyntaxNodeText, prefixes: &[&str]) {
        assert_eq!(catch(|| text.to_string()), None);
        for limit in 0..=8 {
            assert_eq!(
                catch(|| first_chunks(text, limit)),
                catch(|| first_chunks_by_tokens(text, limit)),
                "first {limit} chunks of {:?}",
                text.range
            );
        }
        for prefix in prefixes {
            assert_eq!(
                catch(|| text.starts_with(prefix)),
                catch(|| starts_with_by_tokens(text, prefix)),
                "{:?} starts with {prefix:?}",
                text.range
            );
        }
    }

    #[test]
    fn edge_case_tree_shares_a_subtree() {
        let root = build_edge_case_tree();
        assert_eq!(root.text_with_trivia().to_string(), EDGE_CASE_TEXT);

        let shared: Vec<_> = root
            .children()
            .filter(|child| child.text_with_trivia() == " é界")
            .collect();
        assert_eq!(shared.len(), 2);
        assert_ne!(
            shared[0].text_range_with_trivia().start(),
            shared[1].text_range_with_trivia().start()
        );
        assert!(std::ptr::eq(
            shared[0].text_with_trivia().node.green(),
            shared[1].text_with_trivia().node.green()
        ));
    }

    #[test]
    fn trimmed_range_past_the_node() {
        let root = build_edge_case_tree();
        let trivia_only = root.children().next().unwrap();
        assert_eq!(trivia_only.text_with_trivia(), " ");

        let text = trivia_only.text_trimmed();
        assert!(!text.node.text_range().contains_range(text.range));
        check_methods(&text, "");
        check_methods(&text.slice(..), "");
    }

    #[test]
    fn methods_on_every_range() {
        let root = build_edge_case_tree();
        let root_text = root.text_with_trivia();
        let root_slices = slices(&root_text, EDGE_CASE_TEXT);
        let prefixes: Vec<_> = root_slices
            .iter()
            .filter_map(|(_, expected)| *expected)
            .chain(["q"])
            .collect();

        for node in root.descendants() {
            let range = node.text_range_with_trivia();
            let expected = &EDGE_CASE_TEXT[range];
            let text = node.text_with_trivia();
            check_methods(&text, expected);

            let trimmed_range = node.text_trimmed_range();
            check_methods(&node.text_trimmed(), &EDGE_CASE_TEXT[trimmed_range]);

            for (slice, expected) in slices(&text, expected) {
                match expected {
                    Some(expected) => check_methods(&slice, expected),
                    None => check_split_methods(&slice, &prefixes),
                }

                for (other, other_expected) in &root_slices {
                    if other.len() != slice.len() {
                        continue;
                    }
                    let eq = catch(|| slice == *other);
                    assert_eq!(eq, catch(|| eq_by_tokens(&slice, other)));
                    if let (Some(expected), Some(other_expected)) = (expected, other_expected) {
                        assert_eq!(eq, Some(expected == *other_expected));
                    }
                }
            }
        }
    }
}
