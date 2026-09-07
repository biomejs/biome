use crate::{
    Text, TextRange, TextSize, TokenAtOffset,
    cursor::{SyntaxNode, SyntaxToken},
};
use biome_text_size::TextLen;
use std::iter::FusedIterator;
use std::{cmp::Ordering, fmt};

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

    pub fn starts_with(&self, mut prefix: &str) -> bool {
        for (token, range) in self.tokens_with_ranges() {
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

    pub fn try_fold_chunks<T, F, E>(&self, init: T, mut f: F) -> Result<T, E>
    where
        F: FnMut(T, &str) -> Result<T, E>,
    {
        self.tokens_with_ranges()
            .try_fold(init, move |acc, (token, range)| {
                f(acc, &token.text()[range])
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
        let mut lhs = self.tokens_with_ranges();
        let mut rhs = other.tokens_with_ranges();
        zip_texts(&mut lhs, &mut rhs).is_none()
            && lhs.all(|it| it.1.is_empty())
            && rhs.all(|it| it.1.is_empty())
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
    use crate::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    use crate::{SyntaxNode, TextRange, TriviaPiece};
    use std::panic::{AssertUnwindSafe, catch_unwind};

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
}
