use biome_formatter::prelude::*;
use biome_formatter::{Format, FormatOptions, FormatResult, write};
use biome_markdown_syntax::{
    AnyMdInline, MdHardLine, MdInlineEmphasis, MdInlineItalic, MdInlineItemList,
    emphasis_ext::{MdEmphasisFence, MdItalicFence},
};
use biome_rowan::{AstNode, AstNodeList, SyntaxResult, TextRange, TextSize, TokenText};

use crate::markdown::auxiliary::quote_prefix::FormatMdQuotePrefixOptions;
use crate::{AsFormat, MarkdownFormatContext, MarkdownFormatter, format_removed, format_replaced};

/// A word slice from a syntax token — stores the text and source position for source maps.
#[derive(Debug, Clone)]
pub(crate) struct MdWord {
    text: TokenText,
    source_position: TextSize,
}

impl MdWord {
    fn new(text: TokenText, source_position: TextSize) -> Self {
        Self {
            text,
            source_position,
        }
    }
}

impl Format<MarkdownFormatContext> for MdWord {
    fn fmt(&self, f: &mut Formatter<MarkdownFormatContext>) -> FormatResult<()> {
        if f.source_map_generation().is_enabled() {
            f.write_element(FormatElement::MappedLocatedTokenText {
                slice: self.text.clone(),
                source_position: self.source_position,
            })
        } else {
            f.write_element(FormatElement::LocatedTokenText {
                slice: self.text.clone(),
                text_width: TextWidth::from_text(&self.text, f.options().indent_width()),
            })
        }
    }
}

/// A single atom within a word group.
#[derive(Debug, Clone)]
pub(crate) enum ProseAtom {
    /// A plain text word slice from an MdTextual token.
    Word(MdWord),
    /// An atomic inline element: emphasis, code, link, image, autolink, etc.
    InlineElement(AnyMdInline),
}

/// An item in the flattened word stream.
#[derive(Debug, Clone)]
pub(crate) enum ProseItem {
    /// One or more adjacent atoms with no whitespace between them (a single "word").
    WordGroup {
        /// Adjacent text slices and inline elements printed without separators.
        atoms: Vec<ProseAtom>,
        /// Special printing mode for emphasis delimiter runs that must not be emitted raw.
        escape: WordGroupEscape,
    },
    /// Whitespace between words — becomes the fill separator.
    Space,
    /// Source line break (\n) — behavior depends on proseWrap mode.
    SoftBreak,
    /// Hard line break (  \n or \\\n) — always breaks, segments the fill.
    /// Carries the original node so it can be formatted with proper token tracking.
    HardBreak(MdHardLine),
}

/// Result of building the word stream.
pub(crate) struct WordStreamResult {
    /// Flat stream of prose items — may contain `HardBreak` items inline.
    pub stream: Vec<ProseItem>,
}

/// Build a flat word stream from an `MdInlineItemList`.
///
/// Returns the stream plus metadata about trailing breaks.
pub(crate) fn build_word_stream_flat(
    node: &MdInlineItemList,
    f: &mut MarkdownFormatter,
) -> SyntaxResult<WordStreamResult> {
    let (mut stream, current_word_group) = build_word_stream(node, f)?;

    // Flush any remaining word group
    if !current_word_group.is_empty() {
        stream.push(ProseItem::WordGroup {
            atoms: current_word_group,
            escape: WordGroupEscape::None,
        });
    }

    mark_words_that_need_escaping(&mut stream);

    Ok(WordStreamResult { stream })
}

fn flush_word_group(stream: &mut Vec<ProseItem>, current: &mut Vec<ProseAtom>) {
    if !current.is_empty() {
        stream.push(ProseItem::WordGroup {
            atoms: std::mem::take(current),
            escape: WordGroupEscape::None,
        });
    }
}

/// Walk an `MdInlineItemList` and flatten its tokens into a linear word stream.
///
/// The stream captures the paragraph's content as a sequence of word groups,
/// spaces, soft breaks, and hard breaks — stripping away whitespace-only
/// structural tokens (indents, quote prefixes) and performing token tracking
/// so the formatter's source-map stays accurate.
///
/// Word grouping rule: consecutive non-whitespace atoms (plain text slices and
/// inline elements like code spans or links) accumulate into the same
/// `WordGroup` until a whitespace boundary (Space, SoftBreak, HardBreak) is
/// encountered. This means an inline element adjacent to text without
/// intervening whitespace (e.g. `word[link](url)`) stays in one group and
/// won't be separated by a line break.
///
/// - Whitespace splits plain text slices from 'MdTextual' tokens into
///   individual `Word` atoms.
/// - Inline elements (emphasis, code, links, images, etc.) become single
///   `InlineElement` atoms — kept opaque so their own formatters handle
///   internal layout and fence normalization.
/// - Any run of whitespace inside an `MdTextual` token collapses into a
///   single `Space`, ending the current group.
/// - A newline `MdTextual` token emits a `SoftBreak`, ending the current
///   group. (The token is tracked but not removed — it's already a no-op
///   in the output since it has no visible content.)
/// - `MdHardLine` emits a `HardBreak`, ending the current group.
/// - `MdIndentToken` tokens are removed from the output (`format_removed`)
///   and produce no stream items — indentation is handled structurally
///   by the formatter.
fn build_word_stream(
    node: &MdInlineItemList,
    f: &mut MarkdownFormatter,
) -> SyntaxResult<(Vec<ProseItem>, Vec<ProseAtom>)> {
    let mut stream = Vec::new();
    let mut current_word_group = Vec::new();

    for item in node.iter() {
        match &item {
            AnyMdInline::MdTextual(text) => {
                let token = text.value_token()?;
                let token_text_str = token.text();

                f.context().comments().is_suppressed(text.syntax());

                if text.is_newline()? {
                    flush_word_group(&mut stream, &mut current_word_group);
                    stream.push(ProseItem::SoftBreak);
                    f.state_mut().track_token(&token);
                    continue;
                }

                f.state_mut().track_token(&token);
                let token_start = token.text_range().start();

                let bytes = token_text_str.as_bytes();
                let len = bytes.len();
                let mut pos = 0usize;

                // Split token text into words at ASCII whitespace boundaries.
                while pos < len {
                    let white_space_start = pos;
                    while pos < len && bytes[pos].is_ascii_whitespace() {
                        pos += 1;
                    }
                    // Flush consecutive whitespaces into a single Space.
                    if pos > white_space_start {
                        flush_word_group(&mut stream, &mut current_word_group);
                        stream.push(ProseItem::Space);
                    }

                    let word_start = pos;
                    while pos < len && !bytes[pos].is_ascii_whitespace() {
                        pos += 1;
                    }
                    if pos > word_start {
                        let start = TextSize::from(word_start as u32);
                        let end = TextSize::from(pos as u32);
                        let text_slice = token.token_text().slice(TextRange::new(start, end));
                        let source_position = token_start + start;
                        current_word_group
                            .push(ProseAtom::Word(MdWord::new(text_slice, source_position)));
                    }
                }
            }

            AnyMdInline::MdHardLine(hard_line) => {
                // Don't format_removed here — the node will be formatted in Phase 2
                // using its own formatter which handles token tracking internally.
                flush_word_group(&mut stream, &mut current_word_group);
                stream.push(ProseItem::HardBreak(hard_line.clone()));
            }

            // Atomic inline elements — never broken internally.
            // Emphasis/italic are kept atomic to preserve their fence
            // normalization logic (e.g. _ → *) which lives in their formatters.
            AnyMdInline::MdInlineItalic(_)
            | AnyMdInline::MdInlineEmphasis(_)
            | AnyMdInline::MdInlineCode(_)
            | AnyMdInline::MdInlineLink(_)
            | AnyMdInline::MdInlineImage(_)
            | AnyMdInline::MdAutolink(_)
            | AnyMdInline::MdReferenceLink(_)
            | AnyMdInline::MdReferenceImage(_)
            | AnyMdInline::MdInlineHtml(_)
            | AnyMdInline::MdEntityReference(_) => {
                current_word_group.push(ProseAtom::InlineElement(item));
            }

            AnyMdInline::MdIndentToken(indent) => {
                f.context().comments().is_suppressed(indent.syntax());
                let token = indent.md_indent_char_token()?;
                format_removed(&token).fmt(f).ok();
            }

            AnyMdInline::MdHtmlBlock(html_block) => {
                f.context().comments().is_suppressed(html_block.syntax());
                flush_word_group(&mut stream, &mut current_word_group);
                current_word_group.push(ProseAtom::InlineElement(item));
                flush_word_group(&mut stream, &mut current_word_group);
            }

            // Document-level fenced code content never appears in prose;
            // handled defensively as an atomic element printed verbatim by
            // its own formatter.
            AnyMdInline::MdCodeContent(code) => {
                f.context().comments().is_suppressed(code.syntax());
                flush_word_group(&mut stream, &mut current_word_group);
                current_word_group.push(ProseAtom::InlineElement(item));
                flush_word_group(&mut stream, &mut current_word_group);
            }
            AnyMdInline::MdQuotePrefix(prefix) => {
                prefix
                    .format()
                    .with_options(FormatMdQuotePrefixOptions {
                        should_remove: true,
                    })
                    .fmt(f)
                    .ok();
            }
        }
    }
    Ok((stream, current_word_group))
}

/// Format a single word group (all atoms concatenated without separators).
pub(crate) struct FormatWordGroup<'a> {
    pub(crate) atoms: &'a [ProseAtom],
    pub(crate) escape: WordGroupEscape,
}

impl Format<MarkdownFormatContext> for FormatWordGroup<'_> {
    fn fmt(&self, f: &mut Formatter<MarkdownFormatContext>) -> FormatResult<()> {
        if self.escape == WordGroupEscape::EscapeEachMarker {
            for atom in self.atoms {
                match atom {
                    ProseAtom::Word(word) => write!(f, [token("\\"), word])?,
                    ProseAtom::InlineElement(elem) => elem.format().fmt(f)?,
                }
            }
            return Ok(());
        }

        if self.escape == WordGroupEscape::EmptyStrongWithEscapedMarker {
            return fmt_empty_strong_delimiter_run(self.atoms, f);
        }

        if fmt_unmatched_underscore_delimiter_run(self.atoms, f)? {
            return Ok(());
        }

        for atom in self.atoms {
            match atom {
                ProseAtom::Word(word) => word.fmt(f)?,
                ProseAtom::InlineElement(elem) => {
                    elem.format().fmt(f)?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub(crate) enum WordGroupEscape {
    /// Print the word group normally.
    #[default]
    None,
    /// Prefix each marker atom with a backslash.
    EscapeEachMarker,
    /// Print a five-marker delimiter run as strong emphasis around an escaped marker.
    EmptyStrongWithEscapedMarker,
}

fn fmt_unmatched_underscore_delimiter_run(
    atoms: &[ProseAtom],
    f: &mut Formatter<MarkdownFormatContext>,
) -> FormatResult<bool> {
    match atoms {
        [
            ProseAtom::Word(leading),
            ProseAtom::InlineElement(AnyMdInline::MdInlineItalic(italic)),
        ] if is_single_underscore_word(leading) => {
            fmt_leading_underscore_with_italic(leading, italic, f)
        }
        [
            ProseAtom::InlineElement(AnyMdInline::MdInlineItalic(italic)),
            ProseAtom::Word(trailing),
        ] if is_single_underscore_word(trailing) => {
            fmt_italic_with_trailing_underscore(italic, trailing, f)
        }
        [
            ProseAtom::Word(first),
            ProseAtom::Word(second),
            ProseAtom::Word(third),
            ProseAtom::InlineElement(AnyMdInline::MdInlineItalic(italic)),
        ] if is_single_underscore_word(first)
            && is_single_underscore_word(second)
            && is_single_underscore_word(third) =>
        {
            fmt_leading_underscore_run_with_italic(first, second, third, italic, f)
        }
        [
            ProseAtom::Word(leading),
            ProseAtom::InlineElement(AnyMdInline::MdInlineEmphasis(emphasis)),
        ] if is_single_underscore_word(leading) => {
            fmt_leading_underscore_with_emphasis(leading, emphasis, f)
        }
        [
            ProseAtom::InlineElement(AnyMdInline::MdInlineEmphasis(emphasis)),
            ProseAtom::Word(trailing),
        ] if is_single_underscore_word(trailing) => {
            fmt_emphasis_with_trailing_underscore(emphasis, trailing, f)
        }
        _ => Ok(false),
    }
}

fn is_single_underscore_word(word: &MdWord) -> bool {
    word.text.text() == "_"
}

fn fmt_escaped_underscore_word(
    word: &MdWord,
    f: &mut Formatter<MarkdownFormatContext>,
) -> FormatResult<()> {
    write!(f, [token("\\"), word])
}

fn fmt_leading_underscore_with_italic(
    leading: &MdWord,
    italic: &MdInlineItalic,
    f: &mut Formatter<MarkdownFormatContext>,
) -> FormatResult<bool> {
    if italic.fence()? != MdItalicFence::Underscore {
        return Ok(false);
    }

    write!(
        f,
        [
            format_with(|f| fmt_escaped_underscore_word(leading, f)),
            italic.format()
        ]
    )?;

    Ok(true)
}

fn fmt_italic_with_trailing_underscore(
    italic: &MdInlineItalic,
    trailing: &MdWord,
    f: &mut Formatter<MarkdownFormatContext>,
) -> FormatResult<bool> {
    let l_fence = italic.l_fence()?;
    let r_fence = italic.r_fence()?;
    if italic.fence()? != MdItalicFence::Underscore {
        return Ok(false);
    }
    f.context().comments().is_suppressed(italic.syntax());

    write!(
        f,
        [
            format_replaced(&l_fence, &text("\\_", Some(l_fence.text_range().start()))),
            italic.content().format(),
            format_replaced(&r_fence, &text("\\_", Some(r_fence.text_range().start()))),
            format_with(|f| fmt_escaped_underscore_word(trailing, f)),
        ]
    )?;

    Ok(true)
}

fn fmt_leading_underscore_run_with_italic(
    first: &MdWord,
    second: &MdWord,
    third: &MdWord,
    italic: &MdInlineItalic,
    f: &mut Formatter<MarkdownFormatContext>,
) -> FormatResult<bool> {
    let l_fence = italic.l_fence()?;
    let r_fence = italic.r_fence()?;
    if italic.fence()? != MdItalicFence::Underscore {
        return Ok(false);
    }
    f.context().comments().is_suppressed(italic.syntax());

    write!(
        f,
        [
            format_with(|f| fmt_escaped_underscore_word(first, f)),
            second,
            format_with(|f| fmt_escaped_underscore_word(third, f)),
            format_replaced(&l_fence, &text("\\_", Some(l_fence.text_range().start()))),
            italic.content().format(),
            r_fence.format(),
        ]
    )?;

    Ok(true)
}

fn fmt_leading_underscore_with_emphasis(
    leading: &MdWord,
    emphasis: &MdInlineEmphasis,
    f: &mut Formatter<MarkdownFormatContext>,
) -> FormatResult<bool> {
    let l_fence = emphasis.l_fence()?;
    let r_fence = emphasis.r_fence()?;
    if emphasis.fence()? != MdEmphasisFence::DoubleUnderscore {
        return Ok(false);
    }
    f.context().comments().is_suppressed(emphasis.syntax());

    write!(
        f,
        [
            format_replaced(&l_fence, &text("**", Some(l_fence.text_range().start()))),
            format_with(|f| fmt_escaped_underscore_word(leading, f)),
            emphasis.content().format(),
            format_replaced(&r_fence, &text("**", Some(r_fence.text_range().start()))),
        ]
    )?;

    Ok(true)
}

fn fmt_emphasis_with_trailing_underscore(
    emphasis: &MdInlineEmphasis,
    trailing: &MdWord,
    f: &mut Formatter<MarkdownFormatContext>,
) -> FormatResult<bool> {
    let l_fence = emphasis.l_fence()?;
    let r_fence = emphasis.r_fence()?;
    if emphasis.fence()? != MdEmphasisFence::DoubleUnderscore {
        return Ok(false);
    }
    f.context().comments().is_suppressed(emphasis.syntax());

    write!(
        f,
        [
            format_replaced(&l_fence, &text("**", Some(l_fence.text_range().start()))),
            emphasis.content().format(),
            format_with(|f| fmt_escaped_underscore_word(trailing, f)),
            format_replaced(&r_fence, &text("**", Some(r_fence.text_range().start()))),
        ]
    )?;

    Ok(true)
}

#[derive(Debug, Clone, Copy)]
struct EmptyEmphasisDelimiterRun {
    delimiter: u8,
    len: usize,
}

impl EmptyEmphasisDelimiterRun {
    fn matches_start_of_word_group(self, atoms: &[ProseAtom]) -> bool {
        self.matches_word_group_boundary(atoms.iter().map(|atom| match atom {
            ProseAtom::Word(word) => Some(word.text.text().bytes()),
            ProseAtom::InlineElement(_) => None,
        }))
    }

    fn matches_end_of_word_group(self, atoms: &[ProseAtom]) -> bool {
        self.matches_word_group_boundary(atoms.iter().rev().map(|atom| match atom {
            ProseAtom::Word(word) => Some(word.text.text().bytes().rev()),
            ProseAtom::InlineElement(_) => None,
        }))
    }

    fn matches_word_group_boundary<I, B>(self, words: I) -> bool
    where
        I: IntoIterator<Item = Option<B>>,
        B: IntoIterator<Item = u8>,
    {
        let mut remaining = self.len;

        for word in words {
            let Some(bytes) = word else {
                return false;
            };

            for byte in bytes {
                if byte != self.delimiter {
                    return false;
                }

                remaining -= 1;
                if remaining == 0 {
                    return true;
                }
            }
        }

        false
    }
}

fn mark_words_that_need_escaping(stream: &mut [ProseItem]) {
    for index in 0..stream.len() {
        let escape = word_group_escape(stream, index);
        if let ProseItem::WordGroup {
            escape: word_group_escape,
            ..
        } = &mut stream[index]
        {
            *word_group_escape = escape;
        }
    }
}

/// Returns how a word group should print marker-only text.
///
/// A word group needs special escaping when every atom is plain text, the whole
/// group looks like an empty emphasis delimiter run, and the run is not paired
/// with a matching marker boundary elsewhere in the paragraph:
///
/// - `**` or `__`, which looks like empty emphasis.
/// - `***` or `___`, which looks like an empty combined emphasis/strong run.
/// - `****` or `____`, which looks like empty strong emphasis.
/// - `*****` or `_____`, which is printed as strong emphasis containing an
///   escaped literal marker.
///
/// The Markdown parser keeps these marker runs as plain `MdTextual` words.
/// Although they parse as literal text, printing them raw makes them look like
/// emphasis delimiters, so the formatter prints an escaped form.
///
/// If another word in the paragraph starts or ends with the same marker run,
/// the run is treated as part of that larger emphasis-like boundary and left
/// unchanged. This preserves cases such as `** foo bar**`, `**foo bar **`, and
/// their `_` equivalents.
///
/// Inline atoms are excluded because emphasis, links, code spans, and other
/// structured inline nodes already own their delimiters and escaping rules.
/// Single markers and runs longer than five are left unchanged because they are
/// outside this empty-emphasis compatibility case.
fn word_group_escape(stream: &[ProseItem], index: usize) -> WordGroupEscape {
    let Some(ProseItem::WordGroup { atoms, .. }) = stream.get(index) else {
        return WordGroupEscape::None;
    };

    let Some(delimiter_run) = empty_emphasis_delimiter_run(atoms) else {
        return WordGroupEscape::None;
    };

    if has_matching_marker_boundary_before(stream, index, delimiter_run)
        || has_matching_marker_boundary_after(stream, index, delimiter_run)
    {
        return WordGroupEscape::None;
    }

    match delimiter_run.len {
        2..=4 => WordGroupEscape::EscapeEachMarker,
        5 => WordGroupEscape::EmptyStrongWithEscapedMarker,
        _ => WordGroupEscape::None,
    }
}

fn fmt_empty_strong_delimiter_run(
    atoms: &[ProseAtom],
    f: &mut Formatter<MarkdownFormatContext>,
) -> FormatResult<()> {
    let Some(delimiter_run) = empty_emphasis_delimiter_run(atoms) else {
        return Ok(());
    };

    debug_assert_eq!(
        delimiter_run.len, 5,
        "empty strong delimiter formatting is selected only for five-marker runs"
    );

    let mut positions = Vec::with_capacity(delimiter_run.len);
    for atom in atoms {
        let ProseAtom::Word(word) = atom else {
            return Ok(());
        };

        for index in 0..word.text.text().len() {
            positions.push(word.source_position + TextSize::from(index as u32));
        }
    }

    let escaped_marker = if delimiter_run.delimiter == b'*' {
        "\\*"
    } else {
        "\\_"
    };

    if positions.len() != 5 {
        for atom in atoms {
            if let ProseAtom::Word(word) = atom {
                word.fmt(f)?;
            }
        }
        return Ok(());
    }

    // Five marker runs are printed as strong emphasis around an escaped marker:
    // the first two markers become the opening `**`, the middle marker is
    // escaped, and the last two markers become the closing `**`.
    let first = positions[0];
    let middle = positions[2];
    let last_start = positions[3];

    write!(
        f,
        [
            text("**", Some(first)),
            text(escaped_marker, Some(middle)),
            text("**", Some(last_start)),
        ]
    )
}

fn empty_emphasis_delimiter_run(atoms: &[ProseAtom]) -> Option<EmptyEmphasisDelimiterRun> {
    let mut delimiter = None;
    let mut len = 0usize;

    for atom in atoms {
        let ProseAtom::Word(word) = atom else {
            return None;
        };

        for byte in word.text.text().bytes() {
            if !matches!(byte, b'*' | b'_') {
                return None;
            }

            if let Some(delimiter) = delimiter {
                if delimiter != byte {
                    return None;
                }
            } else {
                delimiter = Some(byte);
            }

            len += 1;
        }
    }

    delimiter.map(|delimiter| EmptyEmphasisDelimiterRun { delimiter, len })
}

fn has_matching_marker_boundary_before(
    stream: &[ProseItem],
    index: usize,
    delimiter_run: EmptyEmphasisDelimiterRun,
) -> bool {
    stream[..index].iter().any(|item| match item {
        ProseItem::WordGroup { atoms, .. } => delimiter_run.matches_start_of_word_group(atoms),
        _ => false,
    })
}

fn has_matching_marker_boundary_after(
    stream: &[ProseItem],
    index: usize,
    delimiter_run: EmptyEmphasisDelimiterRun,
) -> bool {
    stream[index + 1..].iter().any(|item| match item {
        ProseItem::WordGroup { atoms, .. } => delimiter_run.matches_end_of_word_group(atoms),
        _ => false,
    })
}
