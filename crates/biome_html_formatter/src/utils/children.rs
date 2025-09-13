use std::{
    iter::{FusedIterator, Peekable},
    str::Chars,
};

use biome_formatter::{
    Buffer, Format, FormatElement, FormatResult, format_args, prelude::*, write,
};
use biome_html_syntax::{AnyHtmlContent, AnyHtmlElement};
use biome_rowan::{AstNode, SyntaxResult, TextLen, TextRange, TextSize, TokenText};

use crate::{HtmlFormatter, context::HtmlFormatContext};

pub(crate) static HTML_WHITESPACE_CHARS: [u8; 4] = [b' ', b'\n', b'\t', b'\r'];

/// Meaningful HTML text is defined to be text that has either non-whitespace
/// characters, or does not contain a newline. Whitespace is defined as ASCII
/// whitespace.
///
/// ```
/// use biome_html_formatter::utils::children::is_meaningful_html_text;
///
/// assert_eq!(is_meaningful_html_text("     \t\r   "), true);
/// assert_eq!(is_meaningful_html_text("     \n\r   "), false);
/// assert_eq!(is_meaningful_html_text("  Alien   "), true);
/// assert_eq!(is_meaningful_html_text("\n  Alien   "), true);
/// assert_eq!(is_meaningful_html_text("  Alien   \n"), true);
/// assert_eq!(is_meaningful_html_text(""), true);
/// ```
pub fn is_meaningful_html_text(text: &str) -> bool {
    let mut has_newline = false;
    for byte in text.bytes() {
        // If there is a non-whitespace character
        if !HTML_WHITESPACE_CHARS.contains(&byte) {
            return true;
        } else if byte == b'\n' {
            has_newline = true;
        }
    }

    !has_newline
}

/// A word in a Html Text. A word is string sequence that isn't separated by any HTML whitespace.
#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct HtmlWord {
    text: TokenText,
    source_position: TextSize,
}

impl HtmlWord {
    fn new(text: TokenText, source_position: TextSize) -> Self {
        Self {
            text,
            source_position,
        }
    }

    pub(crate) fn is_single_character(&self) -> bool {
        self.text.chars().count() == 1
    }
}

impl Format<HtmlFormatContext> for HtmlWord {
    fn fmt(&self, f: &mut Formatter<HtmlFormatContext>) -> FormatResult<()> {
        f.write_element(FormatElement::LocatedTokenText {
            source_position: self.source_position,
            slice: self.text.clone(),
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum HtmlChild {
    /// A Single word in a HTML text. For example, the words for `a b\nc` are `[a, b, c]`
    Word(HtmlWord),

    /// A comment in a HTML text.
    ///
    /// This is considered a separate kind of "word" here because we must preserve whitespace between text and comments.
    Comment(HtmlWord),

    /// A ` ` whitespace
    ///
    /// ```html
    /// <div> </div>
    /// <div>a </div>
    /// <div> a</div>
    /// <div>a
    ///  b</div>
    /// ```
    ///
    /// Whitespace between two words is not represented as whitespace
    /// ```javascript
    /// <div>a b</div>
    /// ```
    /// The space between `a` and `b` is not considered a whitespace.
    Whitespace,

    /// A new line at the start or end of a [HtmlText] with meaningful content. (that isn't all whitespace
    /// and contains a new line).
    ///
    /// ```html
    /// <div>
    ///     a
    /// </div>
    /// ```
    Newline,

    /// A [HtmlText] that only consists of whitespace and has at least two line breaks;
    ///
    /// ```html
    /// <div>
    ///
    ///   <test />
    /// </div>
    /// ```
    ///
    /// The text between `<div>` and `<test />` is an empty line text.
    EmptyLine,

    /// Any other content that isn't a text. Should be formatted as is.
    NonText(AnyHtmlElement),

    /// Any content that should be formatted verbatim, and not transformed in any way.
    ///
    /// Used for content that has formatting suppressed.
    Verbatim(AnyHtmlElement),
}

impl HtmlChild {
    #[expect(dead_code)]
    pub(crate) const fn is_any_line(&self) -> bool {
        matches!(self, Self::EmptyLine | Self::Newline)
    }
}

/// Creates either a space using an expression child and a string literal,
/// or a regular space, depending on whether the group breaks or not.
///
/// ```html
///  <div> Winter Light </div>;
///
///  <div>
///    Winter Light
///    Through A Glass Darkly
///    The Silence
///    Seventh Seal
///    Wild Strawberries
///  </div>
/// ```
#[derive(Default)]
pub(crate) struct HtmlSpace;

impl Format<HtmlFormatContext> for HtmlSpace {
    fn fmt(&self, formatter: &mut HtmlFormatter) -> FormatResult<()> {
        write![
            formatter,
            [
                if_group_breaks(&format_args![HtmlRawSpace, soft_line_break()]),
                if_group_fits_on_line(&space())
            ]
        ]
    }
}

pub(crate) struct HtmlRawSpace;

impl Format<HtmlFormatContext> for HtmlRawSpace {
    fn fmt(&self, f: &mut Formatter<HtmlFormatContext>) -> FormatResult<()> {
        write!(f, [text(" ")])
    }
}

pub(crate) fn html_split_children<I>(
    children: I,
    f: &mut HtmlFormatter,
) -> SyntaxResult<Vec<HtmlChild>>
where
    I: IntoIterator<Item = AnyHtmlElement>,
{
    let mut builder = HtmlSplitChildrenBuilder::new();

    let mut prev_child_was_content = false;
    for child in children {
        let element_has_content = matches!(
            &child,
            AnyHtmlElement::AnyHtmlContent(
                AnyHtmlContent::HtmlContent(_) | AnyHtmlContent::HtmlEmbeddedContent(_)
            )
        );

        if element_has_content {
            let (text_syntax, value_token) = match &child {
                AnyHtmlElement::AnyHtmlContent(AnyHtmlContent::HtmlContent(text)) => {
                    (text.syntax(), text.value_token()?)
                }
                AnyHtmlElement::AnyHtmlContent(AnyHtmlContent::HtmlEmbeddedContent(text)) => {
                    (text.syntax(), text.value_token()?)
                }
                _ => unreachable!(
                    "You should update the condition of `element_has_content` to handle this case."
                ),
            };
            // Split the text into words
            // Keep track if there's any leading/trailing empty line, new line or whitespace

            let is_suppressed = f.comments().is_suppressed(text_syntax);
            if is_suppressed {
                builder.entry(HtmlChild::Verbatim(child));
                continue;
            }
            f.state_mut().track_token(&value_token);
            // Manually mark these comments as formatted because they are. Because we override the formatting of text content in here, the formatter does not seem to recognize them as formatted.
            // We do have to manually check to make sure the comment's text range is actually inside this node's text range. Some comments may be included in this call to `leading_trailing_comments` that are not actually part of this node.

            let mut trailing_comments_to_format = vec![];
            for comment in f.comments().leading_dangling_trailing_comments(text_syntax) {
                let comment_range = comment.piece().text_range();
                // TODO: might be able to make this a debug assertion instead
                if comment_range.start() >= value_token.text_range().start()
                    && comment_range.end() <= value_token.text_range().end()
                {
                    comment.mark_formatted();
                }
            }
            for comment in f.comments().trailing_comments(text_syntax) {
                let comment_range = comment.piece().text_range();
                // TODO: might be able to make this a debug assertion instead
                if !(comment_range.start() >= value_token.text_range().start()
                    && comment_range.end() <= value_token.text_range().end())
                {
                    trailing_comments_to_format.push(comment);
                }
            }

            let mut chunks = HtmlSplitChunksIterator::new(value_token.text()).peekable();

            // Text starting with a whitespace
            if let Some((_, HtmlTextChunk::Whitespace(_whitespace))) = chunks.peek() {
                // SAFETY: We just checked this above.
                match chunks.next().unwrap() {
                    (_, HtmlTextChunk::Whitespace(whitespace)) => {
                        if whitespace.contains('\n') && !prev_child_was_content {
                            if chunks.peek().is_none() {
                                // A text only consisting of whitespace that also contains a new line isn't considered meaningful text.
                                // It can be entirely removed from the content without changing the semantics.
                                let newlines = whitespace.bytes().filter(|b| *b == b'\n').count();

                                // Keep up to one blank line between tags.
                                // ```html
                                // <div>
                                //
                                //   <MyElement />
                                // </div>
                                // ```
                                if newlines > 1 {
                                    builder.entry(HtmlChild::EmptyLine);
                                }

                                continue;
                            }

                            builder.entry(HtmlChild::Newline)
                        } else {
                            // if there's newlines before a comment, we need to preserve them
                            if whitespace.contains('\n')
                                && matches!(chunks.peek(), Some(&(_, HtmlTextChunk::Comment(_))))
                            {
                                builder.entry(HtmlChild::Newline)
                            } else {
                                builder.entry(HtmlChild::Whitespace)
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }

            let mut prev_chunk_was_comment = false;
            while let Some(chunk) = chunks.next() {
                match chunk {
                    (_, HtmlTextChunk::Whitespace(whitespace)) => {
                        // Only handle trailing whitespace. Words must always be joined by new lines
                        let newlines = whitespace.chars().filter(|b| *b == '\n').count();
                        match chunks.peek() {
                            Some(&(_, HtmlTextChunk::Comment(_))) => {
                                // if the next chunk is a comment, preserve the whitespace
                                if newlines >= 2 {
                                    builder.entry(HtmlChild::EmptyLine)
                                } else if newlines == 1 {
                                    builder.entry(HtmlChild::Newline)
                                } else {
                                    builder.entry(HtmlChild::Whitespace)
                                }
                            }
                            None => {
                                if newlines >= 1 {
                                    builder.entry(HtmlChild::Newline)
                                } else {
                                    builder.entry(HtmlChild::Whitespace)
                                }
                            }
                            _ => {
                                // if the previous chunk was a comment, we need to preserve the whitespace before the next chunk.
                                if prev_chunk_was_comment {
                                    if newlines >= 2 {
                                        builder.entry(HtmlChild::EmptyLine)
                                    } else if newlines == 1 {
                                        builder.entry(HtmlChild::Newline)
                                    } else {
                                        builder.entry(HtmlChild::Whitespace)
                                    }
                                }
                            }
                        }
                    }

                    (relative_start, HtmlTextChunk::Word(word)) => {
                        let text = value_token
                            .token_text()
                            .slice(TextRange::at(relative_start, word.text_len()));
                        let source_position = value_token.text_range().start() + relative_start;

                        builder.entry(HtmlChild::Word(HtmlWord::new(text, source_position)));
                    }
                    (relative_start, HtmlTextChunk::Comment(word)) => {
                        let text = value_token
                            .token_text()
                            .slice(TextRange::at(relative_start, word.text_len()));
                        let source_position = value_token.text_range().start() + relative_start;

                        builder.entry(HtmlChild::Comment(HtmlWord::new(text, source_position)));
                    }
                }
                prev_chunk_was_comment = matches!(chunk, (_, HtmlTextChunk::Comment(_)));
            }

            // There may be trailing comments that we attached to the content if this is the last child of an Element. They won't show up in the `value_token.text()` because they are actually attached to the leading token of the closing tag. This means we have to format them manually.
            for comment in trailing_comments_to_format {
                // This might not actually be the best way to handle the whitespace before the comment. If there are bugs here involving whitespace preceding the comment, try this:
                // Instead of the below match on `comment.lines_before()`, try to include the whitespace in the sliced range from the token text. Right now, that preceding whitespace is excluded, and we add it back in via the `lines_before` match below.
                match comment.lines_before() {
                    0 => {}
                    1 => builder.entry(HtmlChild::Newline),
                    _ => builder.entry(HtmlChild::EmptyLine),
                }
                let token = comment.piece().as_piece().token();
                let text = token.token_text();

                builder.entry(HtmlChild::Comment(HtmlWord::new(
                    text.slice(comment.piece().text_range() - token.text_range().start()),
                    comment.piece().text_range().start(),
                )));
                comment.mark_formatted();
            }

            prev_child_was_content = true;
        } else {
            let text = child.to_string();
            let mut chunks = HtmlSplitChunksIterator::new(&text).peekable();

            // Text starting with a whitespace
            if let Some((_, HtmlTextChunk::Whitespace(_whitespace))) = chunks.peek() {
                // SAFETY: We just checked this above.
                match chunks.next().unwrap() {
                    (_, HtmlTextChunk::Whitespace(whitespace)) => {
                        if whitespace.contains('\n') {
                            // A text only consisting of whitespace that also contains a new line isn't considered meaningful text.
                            // It can be entirely removed from the content without changing the semantics.
                            let newlines = whitespace.chars().filter(|c| *c == '\n').count();

                            // Keep up to one blank line between tags.
                            // ```html
                            // <div>
                            //
                            //   <MyElement />
                            // </div>
                            // ```
                            if newlines > 1 {
                                builder.entry(HtmlChild::EmptyLine);
                            } else {
                                builder.entry(HtmlChild::Newline);
                            }
                        } else {
                            builder.entry(HtmlChild::Whitespace)
                        }
                    }
                    _ => unreachable!(),
                }
            }

            let is_suppressed = f.comments().is_suppressed(child.syntax());
            if is_suppressed {
                builder.entry(HtmlChild::Verbatim(child.clone()));
            } else {
                builder.entry(HtmlChild::NonText(child.clone()));
            }
            prev_child_was_content = false;
        }
    }

    Ok(builder.finish())
}

/// The builder is used to:
/// 1. Remove [HtmlChild::EmptyLine], [HtmlChild::Newline], [HtmlChild::Whitespace] if a next element is [HtmlChild::Whitespace]
/// 2. Don't push a new element [HtmlChild::EmptyLine], [HtmlChild::Newline], [HtmlChild::Whitespace] if previous one is [HtmlChild::EmptyLine], [HtmlChild::Newline], [HtmlChild::Whitespace]
///
/// [Prettier applies]: https://github.com/prettier/prettier/blob/b0d9387b95cdd4e9d50f5999d3be53b0b5d03a97/src/language-js/print/jsx.js#L144-L180
#[derive(Debug)]
struct HtmlSplitChildrenBuilder {
    buffer: Vec<HtmlChild>,
}

impl HtmlSplitChildrenBuilder {
    fn new() -> Self {
        Self { buffer: vec![] }
    }

    fn entry(&mut self, child: HtmlChild) {
        match self.buffer.last_mut() {
            Some(last @ (HtmlChild::EmptyLine | HtmlChild::Newline | HtmlChild::Whitespace)) => {
                if matches!(child, HtmlChild::Whitespace) {
                    *last = child;
                } else if matches!(
                    child,
                    HtmlChild::NonText(_)
                        | HtmlChild::Word(_)
                        | HtmlChild::Comment(_)
                        | HtmlChild::Verbatim(_)
                ) {
                    self.buffer.push(child);
                }
            }
            _ => self.buffer.push(child),
        }
    }

    fn finish(self) -> Vec<HtmlChild> {
        self.buffer
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum HtmlTextChunk<'a> {
    Whitespace(&'a str),
    Word(&'a str),
    Comment(&'a str),
}

/// Splits a text into whitespace only and non-whitespace chunks.
///
/// See `jsx_split_chunks_iterator` test for examples
struct HtmlSplitChunksIterator<'a> {
    position: TextSize,
    text: &'a str,
    chars: Peekable<Chars<'a>>,
}

impl<'a> HtmlSplitChunksIterator<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            position: TextSize::default(),
            text,
            chars: text.chars().peekable(),
        }
    }
}

impl<'a> Iterator for HtmlSplitChunksIterator<'a> {
    type Item = (TextSize, HtmlTextChunk<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        let char = self.chars.next()?;

        let start = self.position;
        self.position += char.text_len();

        let is_whitespace = matches!(char, ' ' | '\n' | '\t' | '\r');
        let mut maybe_comment = char == '<';
        let mut definitely_comment = false;
        let mut seen_end_comment_chars = 0;

        while let Some(next) = self.chars.peek() {
            if maybe_comment && !definitely_comment {
                match (self.position - start, next) {
                    (idx, '!') if idx == 1.into() => {}
                    (idx, '-') if idx == 2.into() || idx == 3.into() => {}
                    (idx, _) if idx == 4.into() => {
                        definitely_comment = true;
                    }
                    _ => {
                        maybe_comment = false;
                    }
                }
            }

            if definitely_comment {
                match (seen_end_comment_chars, next) {
                    (0, '-') => seen_end_comment_chars += 1,
                    (1, '-') => seen_end_comment_chars += 1,
                    (2, '>') => seen_end_comment_chars += 1,
                    _ => seen_end_comment_chars = 0,
                }
            } else {
                let next_is_whitespace = matches!(next, ' ' | '\n' | '\t' | '\r');

                if is_whitespace != next_is_whitespace {
                    break;
                }
            }

            self.position += next.text_len();
            self.chars.next();

            if seen_end_comment_chars == 3 {
                break;
            }
            // we also need to stop progressing if we encounter a comment start
            let peek_end = self.position + TextSize::from(4);
            if !definitely_comment
                && peek_end <= TextSize::from(self.text.len() as u32)
                && self.text.is_char_boundary(peek_end.into())
                && &self.text[TextRange::new(self.position, peek_end)] == "<!--"
            {
                break;
            }
        }

        let range = TextRange::new(start, self.position);
        let slice = &self.text[range];

        let chunk = if is_whitespace {
            HtmlTextChunk::Whitespace(slice)
        } else if definitely_comment {
            HtmlTextChunk::Comment(slice)
        } else {
            HtmlTextChunk::Word(slice)
        };

        Some((start, chunk))
    }
}

impl FusedIterator for HtmlSplitChunksIterator<'_> {}

/// An iterator adaptor that allows a lookahead of three tokens
///
/// # Examples
/// ```
/// use biome_html_formatter::utils::children::HtmlChildrenIterator;
///
/// let buffer = vec![1, 2, 3, 4];
///
/// let mut iter = HtmlChildrenIterator::new(buffer.iter());
///
/// assert_eq!(iter.peek(), Some(&&1));
/// assert_eq!(iter.peek_next(), Some(&&2));
/// assert_eq!(iter.peek_next_next(), Some(&&3));
/// assert_eq!(iter.next(), Some(&1));
/// assert_eq!(iter.next(), Some(&2));
/// assert_eq!(iter.next(), Some(&3));
/// ```
#[derive(Clone, Debug)]
pub struct HtmlChildrenIterator<I: Iterator> {
    iter: I,

    peeked: Option<Option<I::Item>>,
    peeked_next: Option<Option<I::Item>>,
    peeked_next_next: Option<Option<I::Item>>,
}

impl<I: Iterator> HtmlChildrenIterator<I> {
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            peeked: None,
            peeked_next: None,
            peeked_next_next: None,
        }
    }

    pub fn peek(&mut self) -> Option<&I::Item> {
        let iter = &mut self.iter;
        self.peeked.get_or_insert_with(|| iter.next()).as_ref()
    }

    pub fn peek_next(&mut self) -> Option<&I::Item> {
        let iter = &mut self.iter;
        let peeked = &mut self.peeked;

        self.peeked_next
            .get_or_insert_with(|| {
                peeked.get_or_insert_with(|| iter.next());
                iter.next()
            })
            .as_ref()
    }

    pub fn peek_next_next(&mut self) -> Option<&I::Item> {
        let iter = &mut self.iter;
        let peeked = &mut self.peeked;
        let peeked_next = &mut self.peeked_next;

        self.peeked_next_next
            .get_or_insert_with(|| {
                peeked.get_or_insert_with(|| iter.next());
                peeked_next.get_or_insert_with(|| iter.next());
                iter.next()
            })
            .as_ref()
    }
}

impl<I: Iterator> Iterator for HtmlChildrenIterator<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.peeked.take() {
            Some(peeked) => {
                self.peeked = self.peeked_next.take();
                self.peeked_next = self.peeked_next_next.take();
                peeked
            }
            None => self.iter.next(),
        }
    }
}
