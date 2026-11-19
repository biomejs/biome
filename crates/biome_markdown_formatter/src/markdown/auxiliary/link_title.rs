use crate::markdown::auxiliary::textual::FormatMdTextualOptions;
use crate::markdown::lists::inline_item_list::FormatMdFormatInlineItemListOptions;
use crate::prelude::*;
use crate::shared::{TextContext, TextPrintMode};
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{
    AnyMdInline, MarkdownLanguage, MarkdownSyntaxToken, MdLinkTitle, MdLinkTitleFields, MdTextual,
};
use biome_rowan::{AstNode, AstNodeListIterator, TextRange, TextSize};
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub(crate) struct FormatMdLinkTitle {
    leading_space: bool,
}

impl Default for FormatMdLinkTitle {
    fn default() -> Self {
        Self {
            leading_space: true,
        }
    }
}
impl FormatNodeRule<MdLinkTitle> for FormatMdLinkTitle {
    fn fmt_fields(&self, node: &MdLinkTitle, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdLinkTitleFields { content } = node.as_fields();

        let Some(normalization) = LinkTitleNormalization::from_node(node) else {
            let content = content
                .format()
                .with_options(FormatMdFormatInlineItemListOptions {
                    print_mode: TextPrintMode::trim_all(),
                    keep_fences_in_italics: false,
                    text_context: TextContext::Neutral,
                });
            return if self.leading_space {
                write!(f, [space(), content])
            } else {
                write!(f, [content])
            };
        };

        if normalization.is_empty() {
            write!(f, [normalization])
        } else if self.leading_space {
            write!(f, [space(), normalization])
        } else {
            write!(f, [normalization])
        }
    }
}

pub(crate) struct FormatMdLinkTitleOptions {
    pub(crate) leading_space: bool,
}

impl FormatRuleWithOptions<MdLinkTitle> for FormatMdLinkTitle {
    type Options = FormatMdLinkTitleOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.leading_space = options.leading_space;
        self
    }
}

/// Rewrites a link title to the delimiter form requiring the fewest escapes without changing its
/// CommonMark title string.
///
/// CommonMark removes one layer of backslash escaping from ASCII punctuation in title content. The
/// normalization uses those unescaped characters to select double quotes, single quotes, or
/// parentheses based on how many characters that delimiter form must escape. Ties prefer double
/// quotes, followed by single quotes and parentheses. Literal backslashes and the selected
/// delimiters are escaped when the title is serialized. Escapes of `&`, `#`, and `;` are retained
/// because removing them can form an entity or numeric character reference. Non-delimiter escapes
/// are also retained in multiline titles because unescaping a marker at the start of a physical
/// line can turn title content into block syntax when the formatted document is parsed again.
///
/// See <https://spec.commonmark.org/0.30/#link-title>.
struct LinkTitleNormalization<'a> {
    /// The source node traversed during analysis and formatting.
    node: &'a MdLinkTitle,
    /// The delimiter choice and source boundaries produced by analysis.
    analysis: LinkTitleAnalysisResult,
}

/// Iterates textual children in source order.
///
/// Iteration stops at the first non-textual child rather than skipping unsupported content.
struct LinkTitleTextualsIterator {
    /// The underlying iterator over all inline children of the title.
    inner: AstNodeListIterator<MarkdownLanguage, AnyMdInline>,
    /// Whether an unsupported child ended iteration.
    done: bool,
}

impl LinkTitleTextualsIterator {
    fn new(node: &MdLinkTitle) -> Self {
        Self {
            inner: node.content().iter(),
            done: false,
        }
    }
}

impl Iterator for LinkTitleTextualsIterator {
    type Item = MdTextual;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        match self.inner.next()? {
            AnyMdInline::MdTextual(textual) => Some(textual),
            _ => {
                self.done = true;
                None
            }
        }
    }
}

impl<'a> LinkTitleNormalization<'a> {
    fn from_node(node: &'a MdLinkTitle) -> Option<Self> {
        if !node
            .content()
            .iter()
            .all(|item| matches!(item, AnyMdInline::MdTextual(_)))
        {
            return None;
        }

        // Delimiter selection requires the complete title, but retaining its text would allocate.
        // The analysis keeps only counts and source boundaries for the formatting pass.
        let mut analysis = LinkTitleAnalysis::default();
        for textual in LinkTitleTextualsIterator::new(node) {
            let token = textual.value_token().ok()?;
            let token_start = token.text_range().start();
            for (offset, value) in token.text().char_indices() {
                let start = token_start + TextSize::from(offset as u32);
                analysis.record(LinkTitleSourceChar {
                    value,
                    start,
                    end: start + TextSize::of(value),
                });
            }
        }

        Some(Self {
            node,
            analysis: analysis.finish()?,
        })
    }

    fn is_empty(&self) -> bool {
        self.analysis.is_empty
    }
}

pub(crate) fn is_empty_link_title(title: &MdLinkTitle) -> bool {
    LinkTitleNormalization::from_node(title).is_some_and(|normalization| normalization.is_empty())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LinkTitleDelimiter {
    DoubleQuote,
    SingleQuote,
    Parentheses,
}

impl LinkTitleDelimiter {
    fn opening(self) -> &'static str {
        match self {
            Self::DoubleQuote => "\"",
            Self::SingleQuote => "'",
            Self::Parentheses => "(",
        }
    }

    fn closing(self) -> &'static str {
        match self {
            Self::DoubleQuote => "\"",
            Self::SingleQuote => "'",
            Self::Parentheses => ")",
        }
    }

    fn closing_char(self) -> char {
        match self {
            Self::DoubleQuote => '"',
            Self::SingleQuote => '\'',
            Self::Parentheses => ')',
        }
    }

    fn matches(opening: char, closing: char) -> bool {
        matches!((opening, closing), ('"', '"') | ('\'', '\'') | ('(', ')'))
    }
}

/// A character and its absolute UTF-8 source range.
#[derive(Clone, Copy)]
struct LinkTitleSourceChar {
    /// The Unicode scalar value read from the source.
    value: char,
    /// The start of the character in the source file.
    start: TextSize,
    /// The end of the character in the source file.
    end: TextSize,
}

/// Locates title boundaries while collecting the information needed to select a delimiter.
///
/// The latest non-whitespace character remains pending because it may be the closing delimiter.
#[derive(Default)]
struct LinkTitleAnalysis {
    /// The first non-whitespace character, which must be an opening delimiter.
    opening: Option<LinkTitleSourceChar>,
    /// The latest non-whitespace character after the opening delimiter.
    pending: Option<LinkTitleSourceChar>,
    /// Whether uncommitted whitespace follows the opening delimiter or `pending`.
    pending_whitespace: bool,
    /// Whether the uncommitted whitespace contains CR or LF.
    pending_line_break: bool,
    /// Counts for committed content, excluding the possible closing delimiter.
    decoded: LinkTitleDecodedAnalysis,
}

impl LinkTitleAnalysis {
    fn record(&mut self, source: LinkTitleSourceChar) {
        if self.opening.is_none() {
            if !source.value.is_whitespace() {
                self.opening = Some(source);
            }
            return;
        }

        if source.value.is_whitespace() {
            self.pending_whitespace = true;
            self.pending_line_break |= matches!(source.value, '\r' | '\n');
            return;
        }

        if let Some(pending) = self.pending.replace(source) {
            self.decoded.record(pending.value);
        }
        if self.pending_whitespace {
            self.decoded.record_whitespace(self.pending_line_break);
            self.pending_whitespace = false;
            self.pending_line_break = false;
        }
    }

    fn finish(mut self) -> Option<LinkTitleAnalysisResult> {
        let opening = self.opening?;
        let closing = self.pending?;
        if !LinkTitleDelimiter::matches(opening.value, closing.value) {
            return None;
        }

        self.decoded.finish_pending_backslash();
        let delimiter = self.decoded.preferred_delimiter();
        Some(LinkTitleAnalysisResult {
            delimiter,
            content_range: TextRange::new(opening.end, closing.start),
            opening_position: opening.start,
            closing_position: closing.start,
            is_multiline: self.decoded.is_multiline,
            is_empty: self.decoded.is_empty,
        })
    }
}

/// Counts characters after applying CommonMark's backslash-escape semantics.
struct LinkTitleDecodedAnalysis {
    /// Whether the last committed character was a backslash awaiting the next character.
    pending_backslash: bool,
    /// The number of decoded double quotes.
    double_quotes: usize,
    /// The number of decoded single quotes.
    single_quotes: usize,
    /// The combined number of decoded opening and closing parentheses.
    parentheses: usize,
    /// Whether the committed content contains CR or LF.
    is_multiline: bool,
    /// Whether the committed content has no characters, including whitespace.
    is_empty: bool,
}

impl Default for LinkTitleDecodedAnalysis {
    fn default() -> Self {
        Self {
            pending_backslash: false,
            double_quotes: 0,
            single_quotes: 0,
            parentheses: 0,
            is_multiline: false,
            is_empty: true,
        }
    }
}

impl LinkTitleDecodedAnalysis {
    fn record(&mut self, value: char) {
        if self.pending_backslash {
            self.pending_backslash = false;
            if value.is_ascii_punctuation() {
                self.record_decoded(value);
                return;
            }
            self.record_decoded('\\');
        }

        if value == '\\' {
            self.pending_backslash = true;
        } else {
            self.record_decoded(value);
        }
    }

    fn record_whitespace(&mut self, is_multiline: bool) {
        self.finish_pending_backslash();
        self.is_empty = false;
        self.is_multiline |= is_multiline;
    }

    fn finish_pending_backslash(&mut self) {
        if self.pending_backslash {
            self.pending_backslash = false;
            self.record_decoded('\\');
        }
    }

    fn record_decoded(&mut self, value: char) {
        self.is_empty = false;
        match value {
            '"' => self.double_quotes += 1,
            '\'' => self.single_quotes += 1,
            '(' | ')' => self.parentheses += 1,
            _ => {}
        }
    }

    fn preferred_delimiter(&self) -> LinkTitleDelimiter {
        if self.double_quotes <= self.single_quotes && self.double_quotes <= self.parentheses {
            LinkTitleDelimiter::DoubleQuote
        } else if self.single_quotes <= self.parentheses {
            LinkTitleDelimiter::SingleQuote
        } else {
            LinkTitleDelimiter::Parentheses
        }
    }
}

/// Source boundaries and content properties required to format a normalized title.
struct LinkTitleAnalysisResult {
    /// The delimiter form requiring the fewest inserted escapes.
    delimiter: LinkTitleDelimiter,
    /// The source range inside the original opening and closing delimiters.
    content_range: TextRange,
    /// The source position mapped to the normalized opening delimiter.
    opening_position: TextSize,
    /// The source position mapped to the normalized closing delimiter.
    closing_position: TextSize,
    /// Whether `content_range` contains CR or LF.
    is_multiline: bool,
    /// Whether `content_range` contains no characters.
    is_empty: bool,
}

/// A byte range backed by a syntax token, suitable for allocation-free formatter output.
#[derive(Clone)]
struct LinkTitleSourceSlice {
    /// The token that owns the source text.
    token: MarkdownSyntaxToken,
    /// The inclusive UTF-8 byte offset relative to `token`.
    start: usize,
    /// The exclusive UTF-8 byte offset relative to `token`.
    end: usize,
}

impl LinkTitleSourceSlice {
    fn new(token: &MarkdownSyntaxToken, start: usize, end: usize) -> Self {
        Self {
            token: token.clone(),
            start,
            end,
        }
    }

    fn position(&self) -> TextSize {
        self.token.text_range().start() + TextSize::from(self.start as u32)
    }

    fn fmt(&self, f: &mut MarkdownFormatter) -> FormatResult<()> {
        syntax_token_cow_slice(
            Cow::Borrowed(&self.token.text()[self.start..self.end]),
            &self.token,
            self.position(),
        )
        .fmt(f)
    }
}

/// Writes normalized title content while borrowing unchanged ranges from source tokens.
///
/// A trailing backslash remains pending so an escape may continue in the next token.
struct LinkTitleEncoder {
    /// The delimiter form used for the formatted title.
    delimiter: LinkTitleDelimiter,
    /// Whether non-delimiter escapes must be retained to protect multiline content.
    is_multiline: bool,
    /// A source backslash awaiting the next character, possibly from another token.
    pending_backslash: Option<LinkTitleSourceSlice>,
    /// Whether an LF immediately following a formatted CR should be omitted.
    skip_line_feed: bool,
}

impl LinkTitleEncoder {
    fn new(delimiter: LinkTitleDelimiter, is_multiline: bool) -> Self {
        Self {
            delimiter,
            is_multiline,
            pending_backslash: None,
            skip_line_feed: false,
        }
    }

    /// Formats `start..end`, where both offsets are UTF-8 byte offsets relative to `token`.
    fn record_token(
        &mut self,
        token: &MarkdownSyntaxToken,
        start: usize,
        end: usize,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        debug_assert!(
            start <= end
                && end <= token.text().len()
                && token.text().is_char_boundary(start)
                && token.text().is_char_boundary(end),
            "link title range must be within the token text",
        );

        let mut source_start = None;
        for (relative_start, value) in token.text()[start..end].char_indices() {
            let char_start = start + relative_start;
            let char_end = char_start + usize::from(TextSize::of(value));

            if self.skip_line_feed {
                self.skip_line_feed = false;
                if value == '\n' {
                    continue;
                }
            }

            if let Some(backslash) = self.pending_backslash.take() {
                Self::flush_source(token, source_start.take(), char_start, f)?;
                if value.is_ascii_punctuation() {
                    self.write_escaped(backslash, token, char_start, char_end, value, f)?;
                    continue;
                }
                Self::write_static_backslash(backslash.position(), f)?;
                backslash.fmt(f)?;
            }

            match value {
                '\\' => {
                    Self::flush_source(token, source_start.take(), char_start, f)?;
                    self.pending_backslash =
                        Some(LinkTitleSourceSlice::new(token, char_start, char_end));
                }
                '\r' => {
                    Self::flush_source(token, source_start.take(), char_start, f)?;
                    write!(
                        f,
                        [
                            source_position(
                                token.text_range().start() + TextSize::from(char_start as u32)
                            ),
                            literal_line_break_without_parent()
                        ]
                    )?;
                    self.skip_line_feed = true;
                }
                '\n' => {
                    Self::flush_source(token, source_start.take(), char_start, f)?;
                    write!(
                        f,
                        [
                            source_position(
                                token.text_range().start() + TextSize::from(char_start as u32)
                            ),
                            literal_line_break_without_parent()
                        ]
                    )?;
                }
                _ if self.needs_escape(value) => {
                    Self::flush_source(token, source_start.take(), char_start, f)?;
                    Self::write_static_backslash(
                        token.text_range().start() + TextSize::from(char_start as u32),
                        f,
                    )?;
                    LinkTitleSourceSlice::new(token, char_start, char_end).fmt(f)?;
                }
                _ => {
                    source_start.get_or_insert(char_start);
                }
            }
        }

        Self::flush_source(token, source_start, end, f)
    }

    /// Writes a pending backslash as literal content.
    fn finish(self, f: &mut MarkdownFormatter) -> FormatResult<()> {
        if let Some(backslash) = self.pending_backslash {
            Self::write_static_backslash(backslash.position(), f)?;
            backslash.fmt(f)?;
        }
        Ok(())
    }

    fn write_escaped(
        &self,
        backslash: LinkTitleSourceSlice,
        token: &MarkdownSyntaxToken,
        start: usize,
        end: usize,
        value: char,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        let preserve_escape = matches!(value, '&' | '#' | ';')
            || self.is_multiline && !matches!(value, '"' | '\'' | '(' | ')' | '\\');
        if preserve_escape {
            backslash.fmt(f)?;
        } else if self.needs_escape(value) {
            Self::write_static_backslash(
                token.text_range().start() + TextSize::from(start as u32),
                f,
            )?;
        }
        LinkTitleSourceSlice::new(token, start, end).fmt(f)
    }

    fn needs_escape(&self, value: char) -> bool {
        value == '\\'
            || value == self.delimiter.closing_char()
            || self.delimiter == LinkTitleDelimiter::Parentheses && value == '('
    }

    fn flush_source(
        token: &MarkdownSyntaxToken,
        start: Option<usize>,
        end: usize,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        if let Some(start) = start
            && start < end
        {
            LinkTitleSourceSlice::new(token, start, end).fmt(f)?;
        }
        Ok(())
    }

    fn write_static_backslash(position: TextSize, f: &mut MarkdownFormatter) -> FormatResult<()> {
        write!(f, [source_position(position), token("\\")])
    }
}

impl Format<MarkdownFormatContext> for LinkTitleNormalization<'_> {
    fn fmt(&self, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let Some(first) = LinkTitleTextualsIterator::new(self.node).next() else {
            return Ok(());
        };

        if self.analysis.is_empty {
            for textual in LinkTitleTextualsIterator::new(self.node) {
                textual
                    .format()
                    .with_options(FormatMdTextualOptions {
                        print_mode: TextPrintMode::Remove,
                        ..FormatMdTextualOptions::default()
                    })
                    .fmt(f)?;
            }
            return Ok(());
        }

        f.context()
            .comments()
            .mark_suppression_checked(first.syntax());
        let first_token = first.value_token()?;
        let replacement = format_with(|f| {
            write!(
                f,
                [
                    source_position(self.analysis.opening_position),
                    token(self.analysis.delimiter.opening())
                ]
            )?;

            let mut encoder =
                LinkTitleEncoder::new(self.analysis.delimiter, self.analysis.is_multiline);
            for textual in LinkTitleTextualsIterator::new(self.node) {
                let token = textual.value_token()?;
                let Some(range) = token.text_range().intersect(self.analysis.content_range) else {
                    continue;
                };
                let relative = range - token.text_range().start();
                encoder.record_token(
                    &token,
                    u32::from(relative.start()) as usize,
                    u32::from(relative.end()) as usize,
                    f,
                )?;
            }
            encoder.finish(f)?;

            write!(
                f,
                [
                    source_position(self.analysis.closing_position),
                    token(self.analysis.delimiter.closing())
                ]
            )
        });
        format_replaced(&first_token, &replacement).fmt(f)?;

        for textual in LinkTitleTextualsIterator::new(self.node).skip(1) {
            textual
                .format()
                .with_options(FormatMdTextualOptions {
                    print_mode: TextPrintMode::Remove,
                    ..FormatMdTextualOptions::default()
                })
                .fmt(f)?;
        }

        Ok(())
    }
}
