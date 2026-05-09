use biome_formatter::prelude::*;
use biome_formatter::{Format, FormatResult};
use biome_markdown_syntax::{AnyMdInline, MdHardLine, MdInlineItemList};
use biome_rowan::{AstNode, AstNodeList, SyntaxResult, TextRange, TextSize, TokenText};

use crate::{AsFormat, MarkdownFormatContext, MarkdownFormatter, format_removed};

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
        f.write_element(FormatElement::LocatedTokenText {
            source_position: self.source_position,
            slice: self.text.clone(),
        })
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
    WordGroup(Vec<ProseAtom>),
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
    /// Whether the original stream ended with a soft break (trailing \n in source).
    pub has_trailing_break: bool,
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
        stream.push(ProseItem::WordGroup(current_word_group));
    }

    // Check if stream ends with a soft break (trailing \n from source)
    let has_trailing_break = matches!(stream.last(), Some(ProseItem::SoftBreak));

    // Strip trailing breaks/spaces — they're not meaningful content
    while matches!(stream.last(), Some(ProseItem::SoftBreak | ProseItem::Space)) {
        stream.pop();
    }

    Ok(WordStreamResult {
        stream,
        has_trailing_break,
    })
}

fn flush_word_group(stream: &mut Vec<ProseItem>, current: &mut Vec<ProseAtom>) {
    if !current.is_empty() {
        stream.push(ProseItem::WordGroup(std::mem::take(current)));
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
/// - `MdSoftBreak` tokens are removed from the output (`format_removed`) and
///   replaced with a `SoftBreak` item. The fill infrastructure re-emits
///   the actual line break later.
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

                f.context()
                    .comments()
                    .mark_suppression_checked(text.syntax());

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

            AnyMdInline::MdSoftBreak(soft_break) => {
                f.context()
                    .comments()
                    .mark_suppression_checked(soft_break.syntax());
                let token = soft_break.value_token()?;
                // Mark the original token as removed so the formatter's
                // source-map accounts for it; the actual line break is
                // re-emitted later by the fill infrastructure.
                format_removed(&token).fmt(f).ok();
                flush_word_group(&mut stream, &mut current_word_group);
                stream.push(ProseItem::SoftBreak);
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
                f.context()
                    .comments()
                    .mark_suppression_checked(indent.syntax());
                let token = indent.md_indent_char_token()?;
                format_removed(&token).fmt(f).ok();
            }

            AnyMdInline::MdHtmlBlock(html_block) => {
                f.context()
                    .comments()
                    .mark_suppression_checked(html_block.syntax());
                flush_word_group(&mut stream, &mut current_word_group);
                current_word_group.push(ProseAtom::InlineElement(item));
                flush_word_group(&mut stream, &mut current_word_group);
            }
            AnyMdInline::MdQuotePrefix(_) => {
                // No need to do anything, it gets formatted as part of the formatting infra
            }
        }
    }
    Ok((stream, current_word_group))
}

/// Format a single word group (all atoms concatenated without separators).
pub(crate) struct FormatWordGroup<'a>(pub(crate) &'a [ProseAtom]);

impl Format<MarkdownFormatContext> for FormatWordGroup<'_> {
    fn fmt(&self, f: &mut Formatter<MarkdownFormatContext>) -> FormatResult<()> {
        for atom in self.0 {
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
