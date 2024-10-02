use std::{
    iter::{FusedIterator, Peekable},
    str::Chars,
};

use biome_formatter::{
    format_args, prelude::*, write, Buffer, Format, FormatElement, FormatResult,
};
use biome_html_syntax::AnyHtmlElement;
use biome_rowan::{SyntaxResult, TextLen, TextRange, TextSize, TokenText};

use crate::{comments::HtmlComments, context::HtmlFormatContext, HtmlFormatter};

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
        HtmlWord {
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
}

impl HtmlChild {
    #[expect(dead_code)]
    pub(crate) const fn is_any_line(&self) -> bool {
        matches!(self, HtmlChild::EmptyLine | HtmlChild::Newline)
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
    _comments: &HtmlComments,
) -> SyntaxResult<Vec<HtmlChild>>
where
    I: IntoIterator<Item = AnyHtmlElement>,
{
    let mut builder = HtmlSplitChildrenBuilder::new();

    let mut prev_was_content = false;
    for child in children {
        match child {
            AnyHtmlElement::HtmlContent(text) => {
                // Split the text into words
                // Keep track if there's any leading/trailing empty line, new line or whitespace

                let value_token = text.value_token()?;
                let mut chunks = HtmlSplitChunksIterator::new(value_token.text()).peekable();

                // Text starting with a whitespace
                if let Some((_, HtmlTextChunk::Whitespace(_whitespace))) = chunks.peek() {
                    // SAFETY: We just checked this above.
                    match chunks.next().unwrap() {
                        (_, HtmlTextChunk::Whitespace(whitespace)) => {
                            if whitespace.contains('\n') && !prev_was_content {
                                if chunks.peek().is_none() {
                                    // A text only consisting of whitespace that also contains a new line isn't considered meaningful text.
                                    // It can be entirely removed from the content without changing the semantics.
                                    let newlines =
                                        whitespace.bytes().filter(|b| *b == b'\n').count();

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
                                builder.entry(HtmlChild::Whitespace)
                            }
                        }
                        _ => unreachable!(),
                    }
                }

                while let Some(chunk) = chunks.next() {
                    match chunk {
                        (_, HtmlTextChunk::Whitespace(whitespace)) => {
                            // Only handle trailing whitespace. Words must always be joined by new lines
                            if chunks.peek().is_none() {
                                if whitespace.contains('\n') {
                                    builder.entry(HtmlChild::Newline);
                                } else {
                                    builder.entry(HtmlChild::Whitespace)
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
                    }
                }
                prev_was_content = true;
            }
            child => {
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

                builder.entry(HtmlChild::NonText(child));
                prev_was_content = false;
            }
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
        HtmlSplitChildrenBuilder { buffer: vec![] }
    }

    fn entry(&mut self, child: HtmlChild) {
        match self.buffer.last_mut() {
            Some(last @ (HtmlChild::EmptyLine | HtmlChild::Newline | HtmlChild::Whitespace)) => {
                if matches!(child, HtmlChild::Whitespace) {
                    *last = child;
                } else if matches!(child, HtmlChild::NonText(_) | HtmlChild::Word(_)) {
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

        while let Some(next) = self.chars.peek() {
            let next_is_whitespace = matches!(next, ' ' | '\n' | '\t' | '\r');

            if is_whitespace != next_is_whitespace {
                break;
            }

            self.position += next.text_len();
            self.chars.next();
        }

        let range = TextRange::new(start, self.position);
        let slice = &self.text[range];

        let chunk = if is_whitespace {
            HtmlTextChunk::Whitespace(slice)
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
