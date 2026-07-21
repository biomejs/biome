use crate::markdown::auxiliary::hard_line::FormatMdFormatHardLineOptions;
use crate::markdown::auxiliary::inline_italic::FormatMdInlineItalicOptions;
use crate::markdown::auxiliary::textual::FormatMdTextualOptions;
use crate::prelude::*;
use crate::shared::{TextContext, TextPrintMode, TrimMode};
use crate::words::{FormatWordGroup, ProseItem, WordStreamResult, build_word_stream_flat};
use biome_formatter::Format;
use biome_markdown_syntax::MdBullet;

/// Formats a sequence of prose items as a single line — word groups joined by spaces.
/// SoftBreak and HardBreak are treated as space separators if encountered.
struct FormatSourceLine<'a>(&'a [ProseItem]);

impl Format<MarkdownFormatContext> for FormatSourceLine<'_> {
    fn fmt(&self, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let mut needs_space = false;
        for item in self.0 {
            match item {
                ProseItem::WordGroup { atoms, escape } => {
                    if needs_space {
                        write!(f, [space()])?;
                    }
                    FormatWordGroup {
                        atoms,
                        escape: *escape,
                    }
                    .fmt(f)?;
                    needs_space = true;
                }
                ProseItem::Space | ProseItem::SoftBreak | ProseItem::HardBreak(_) => {
                    needs_space = true;
                }
            }
        }
        Ok(())
    }
}
/// Removes leading `Space` items after every `SoftBreak` in the word stream.
///
/// When the parser doesn't recognize continuation-line whitespace as
/// `MdIndentToken` (e.g. because the source list marker had leading spaces
/// that shift the expected indent), those spaces end up as `MdTextual " "`
/// tokens → `Space` items in the stream. The structural `align()` in the IR
/// already provides the correct indentation, so these spaces are artifacts.
fn strip_spaces_after_soft_breaks(stream: &mut Vec<ProseItem>) {
    let mut i = 0;
    while i < stream.len() {
        if matches!(stream[i], ProseItem::SoftBreak) {
            let start = i + 1;
            let mut end = start;
            while end < stream.len() && matches!(stream[end], ProseItem::Space) {
                end += 1;
            }
            if end > start {
                stream.drain(start..end);
            }
        }
        i += 1;
    }
}

fn outdented_list_marker_lines(
    node: &MdInlineItemList,
    content_indent: usize,
) -> FormatResult<Vec<usize>> {
    let mut lines = Vec::new();
    let mut line_index = 0;
    let mut at_line_start = true;
    let mut leading_spaces = 0;

    for item in node.iter() {
        let AnyMdInline::MdTextual(textual) = item else {
            at_line_start = false;
            continue;
        };

        let token = textual.value_token()?;
        let mut text = token.text();

        loop {
            if at_line_start {
                let trimmed = text.trim_start_matches(' ');
                leading_spaces += text.len() - trimmed.len();
                text = trimmed;

                if text.is_empty() {
                    break;
                }

                if line_index > 0
                    && leading_spaces < content_indent
                    && starts_with_list_marker(text)
                {
                    lines.push(line_index);
                }

                at_line_start = false;
            }

            let Some(newline_index) = text.find('\n') else {
                break;
            };

            line_index += 1;
            at_line_start = true;
            leading_spaces = 0;
            text = &text[newline_index + 1..];
        }
    }

    Ok(lines)
}

fn starts_with_list_marker(text: &str) -> bool {
    let bytes = text.as_bytes();

    match bytes {
        [b'-' | b'*' | b'+'] => true,
        [b'-' | b'*' | b'+', next, ..] => next.is_ascii_whitespace(),
        [first, ..] if first.is_ascii_digit() => {
            let marker_end = bytes
                .iter()
                .position(|byte| !byte.is_ascii_digit())
                .unwrap_or(bytes.len());

            matches!(bytes.get(marker_end), Some(b'.' | b')'))
                && bytes
                    .get(marker_end + 1)
                    .is_none_or(|next| next.is_ascii_whitespace())
        }
        _ => false,
    }
}

fn enclosing_list_content_indent(node: &MdInlineItemList) -> Option<usize> {
    let bullet = node.syntax().ancestors().find_map(MdBullet::cast)?;
    let prefix = bullet.prefix().ok()?;
    let marker = prefix.marker().ok()?;
    let pre_marker_width = prefix
        .pre_marker_indent()
        .iter()
        .map(|indent| {
            indent
                .md_indent_char_token()
                .ok()
                .map_or(0, |token| token.text().len())
        })
        .sum::<usize>();

    Some(pre_marker_width + marker.text_trimmed().len() + prefix.post_marker_len().unwrap_or(2))
}

fn format_source_line(
    line_items: &[ProseItem],
    line_index: usize,
    needs_leading_break: bool,
    trailing_soft_break: bool,
    outdented_lines: &[usize],
    f: &mut MarkdownFormatter,
) -> FormatResult<()> {
    if line_items.is_empty() {
        return Ok(());
    }

    if needs_leading_break && outdented_lines.contains(&line_index) {
        return write!(
            f,
            [dedent_to_root(&format_with(|f| {
                write!(f, [hard_line_break()])?;
                FormatSourceLine(line_items).fmt(f)
            }))]
        );
    }

    if needs_leading_break {
        write!(f, [hard_line_break()])?;
    }

    FormatSourceLine(line_items).fmt(f)?;

    if trailing_soft_break {
        write!(f, [soft_line_break()])?;
    }

    Ok(())
}

use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{
    AnyMdInline, MarkdownLanguage, MdFencedCodeBlock, MdIndentCodeBlock, MdIndentToken,
    MdInlineItemList, MdTextual,
};
use biome_rowan::{AstNode, AstNodeListIterator};
use std::iter::{FusedIterator, Peekable};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineItemList {
    print_mode: TextPrintMode,
    /// When true, and there's a [MdInlineItalic], it instrustructs the formatter to keep the fences
    keep_fences_in_italics: bool,
    text_context: TextContext,
}

impl FormatRule<MdInlineItemList> for FormatMdInlineItemList {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &MdInlineItemList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let inside_fenced_code_block = node
            .syntax()
            .parent()
            .is_some_and(|p| MdFencedCodeBlock::can_cast(p.kind()));
        if self.print_mode.is_fill() && inside_fenced_code_block && self.text_context.is_list() {
            return self.fmt_fenced_code_block(node, f);
        } else if self.print_mode.is_fill() {
            return self.fmt_fill(node, f, self.text_context);
        } else if self.print_mode.is_remove() {
            return self.fmt_remove(node, f);
        } else if self.text_context.is_list() {
            return self.fmt_inside_list(node, f);
        } else if self.print_mode.is_auto_link_like() {
            return self.fmt_auto_link_like(node, f);
        } else if self.print_mode.is_normalize_words() {
            return self.fmt_normalize_words(node, f);
        } else if self.print_mode.is_trim_all() {
            return self.fmt_trim_all(node, f);
        } else if self.print_mode.is_pristine() {
            return self.fmt_pristine(node, f);
        } else if self.print_mode.is_clean() {
            return self.fmt_clean(node, f);
        }

        let mut joiner = f.join();

        let mut seen_new_line = false;
        for (index, item) in node.iter().enumerate() {
            match item {
                AnyMdInline::MdTextual(text) => {
                    let inside_indent_code_block = text
                        .syntax()
                        .grand_parent()
                        .is_some_and(|n| MdIndentCodeBlock::can_cast(n.kind()));

                    if text.is_empty_and_not_newline()? && seen_new_line {
                        if inside_indent_code_block || self.print_mode.is_fill() {
                            joiner.entry(&text.format().with_options(FormatMdTextualOptions {
                                print_mode: TextPrintMode::fill(),
                                ..FormatMdTextualOptions::default()
                            }));
                        } else {
                            let entry = format_with(|f| {
                                write!(
                                    f,
                                    [text.format().with_options(FormatMdTextualOptions {
                                        print_mode: TextPrintMode::Remove,
                                        ..FormatMdTextualOptions::default()
                                    })]
                                )
                            });
                            joiner.entry(&entry);
                        }
                    } else if text.is_newline()? {
                        let entry = format_with(|f| {
                            write!(
                                f,
                                [
                                    text.format().with_options(FormatMdTextualOptions {
                                        print_mode: TextPrintMode::Remove,
                                        ..FormatMdTextualOptions::default()
                                    }),
                                    hard_line_break()
                                ]
                            )
                        });
                        seen_new_line = true;
                        joiner.entry(&entry);
                    } else {
                        joiner.entry(&text.format().with_options(FormatMdTextualOptions {
                            print_mode: if self.print_mode.is_trim_start() && index == 0 {
                                self.print_mode
                            } else {
                                TextPrintMode::default()
                            },
                            ..FormatMdTextualOptions::default()
                        }));
                    }
                }

                AnyMdInline::MdHardLine(hard_line) => {
                    seen_new_line = true;
                    joiner.entry(&format_with(|f| {
                        write!(
                            f,
                            [hard_line
                                .format()
                                .with_options(FormatMdFormatHardLineOptions {
                                    print_mode: self.print_mode,
                                })]
                        )
                    }));
                }
                AnyMdInline::MdInlineItalic(italic) => {
                    joiner.entry(&italic.format().with_options(FormatMdInlineItalicOptions {
                        should_keep_fences: self.keep_fences_in_italics,
                    }));
                    seen_new_line = false;
                }
                AnyMdInline::MdIndentToken(indent) => {
                    joiner.entry(&indent.format());
                    seen_new_line = false;
                }
                _ => {
                    joiner.entry(&item.format());
                    seen_new_line = false;
                }
            }
        }

        joiner.finish()
    }
}

impl FormatMdInlineItemList {
    fn fmt_remove(&self, node: &MdInlineItemList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        for item in node.iter() {
            match item {
                AnyMdInline::MdTextual(textual) => write!(
                    f,
                    [textual.format().with_options(FormatMdTextualOptions {
                        print_mode: TextPrintMode::Remove,
                        ..FormatMdTextualOptions::default()
                    })]
                )?,
                item => write!(f, [item.format()])?,
            }
        }

        Ok(())
    }

    /// Formats inline content inside a list item, normalizing the indentation
    /// of continuation lines.
    ///
    /// In Markdown, each continuation line of a list item must be indented to
    /// align with the first character of the item's content. For example, with
    /// `1. item`, the content starts at column 3, so continuation lines need
    /// exactly 3 spaces. The parser represents those required spaces as
    /// `MdIndentToken` nodes in the CST (or, for blank-line-separated "loose"
    /// paragraphs, as a preceding `MdContinuationIndent` block). Any extra
    /// leading spaces beyond the required indent end up as `MdTextual " "`
    /// tokens at the start of the paragraph's `MdInlineItemList`.
    ///
    /// This function normalizes those extras:
    ///
    /// - At the very start of the paragraph (loose, blank-line-separated
    ///   context), ALL leading `" "` textuals are removed — they carry no
    ///   meaning past the continuation indent.
    /// - On a soft-wrap continuation within the same paragraph (after a `\n`
    ///   textual), a single excess space is stripped (typo) but two or more
    ///   are preserved (intentional alignment).
    /// - Spaces following a hard-line-break (`  \n`) are always preserved
    ///   because those reflect an explicit choice by the author.
    fn fmt_inside_list(
        &self,
        node: &MdInlineItemList,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        let items: Vec<AnyMdInline> = node.iter().collect();
        let mut joiner = f.join();
        // True until we have emitted the first non-space content of the
        // paragraph. While true, every leading `" "` textual is excess and is
        // removed unconditionally.
        let mut at_paragraph_start = true;
        let mut seen_new_line = false;
        let mut after_hard_line = false;

        for (i, item) in items.iter().enumerate() {
            match item {
                AnyMdInline::MdTextual(md_text) => {
                    if md_text.is_newline()? {
                        seen_new_line = true;
                        after_hard_line = false;
                        // Paragraph content has started — any spaces that come
                        // after this newline are soft-wrap, not paragraph-start.
                        at_paragraph_start = false;
                        joiner.entry(&format_with(|f| {
                            write!(
                                f,
                                [
                                    md_text.format().with_options(FormatMdTextualOptions {
                                        print_mode: TextPrintMode::Remove,
                                        ..FormatMdTextualOptions::default()
                                    }),
                                    hard_line_break()
                                ]
                            )
                        }));
                    } else if at_paragraph_start && md_text.value_token()?.text() == " " {
                        // Leading space at the start of a loose paragraph —
                        // strip unconditionally (these are pure excess past the
                        // continuation indent).
                        joiner.entry(&md_text.format().with_options(FormatMdTextualOptions {
                            print_mode: TextPrintMode::Remove,
                            ..FormatMdTextualOptions::default()
                        }));
                    } else if seen_new_line
                        && !after_hard_line
                        && md_text.value_token()?.text() == " "
                    {
                        // Soft-wrap continuation excess. Single extra space is
                        // a typo (strip); two or more mean intentional
                        // alignment (keep).
                        let next_is_space = matches!(
                            items.get(i + 1),
                            Some(AnyMdInline::MdTextual(next))
                            if next.value_token().is_ok_and(|t| t.text() == " ")
                        );
                        if next_is_space {
                            let was_after_newline = seen_new_line;
                            seen_new_line = false;
                            after_hard_line = false;
                            joiner.entry(&md_text.format().with_options(FormatMdTextualOptions {
                                print_mode: if was_after_newline {
                                    self.print_mode
                                } else {
                                    TextPrintMode::default()
                                },
                                ..FormatMdTextualOptions::default()
                            }));
                        } else {
                            // Single excess space before content — remove it.
                            seen_new_line = false;
                            joiner.entry(&md_text.format().with_options(FormatMdTextualOptions {
                                print_mode: TextPrintMode::Remove,
                                ..FormatMdTextualOptions::default()
                            }));
                        }
                    } else {
                        let was_after_newline = seen_new_line;
                        at_paragraph_start = false;
                        seen_new_line = false;
                        after_hard_line = false;

                        let next_is_newline_or_end = items.get(i + 1).is_none_or(|n| {
                            matches!(n, AnyMdInline::MdTextual(t) if t.is_newline().unwrap_or_default())
                        });
                        let print_mode = if was_after_newline {
                            self.print_mode
                        } else {
                            TextPrintMode::default()
                        };

                        if next_is_newline_or_end {
                            let token = md_text.value_token()?;
                            let trimmed = token.text().trim_end();
                            if trimmed.len() < token.text().len() {
                                joiner.entry(&format_with(|f: &mut MarkdownFormatter| {
                                    f.context()
                                        .comments()
                                        .mark_suppression_checked(md_text.syntax());

                                    write!(
                                        f,
                                        [format_replaced(
                                            &token,
                                            &text(
                                                trimmed,
                                                Some(token.text_trimmed_range().start())
                                            )
                                        )]
                                    )
                                }));
                            } else {
                                joiner.entry(&md_text.format().with_options(
                                    FormatMdTextualOptions {
                                        print_mode,
                                        ..FormatMdTextualOptions::default()
                                    },
                                ));
                            }
                        } else {
                            joiner.entry(&md_text.format().with_options(FormatMdTextualOptions {
                                print_mode,
                                ..FormatMdTextualOptions::default()
                            }));
                        }
                    }
                }
                AnyMdInline::MdHardLine(hard_line) => {
                    seen_new_line = true;
                    after_hard_line = true;
                    at_paragraph_start = false;
                    joiner.entry(&format_with(|f| {
                        write!(
                            f,
                            [hard_line
                                .format()
                                .with_options(FormatMdFormatHardLineOptions {
                                    print_mode: self.print_mode,
                                })]
                        )
                    }));
                }
                AnyMdInline::MdInlineItalic(italic) => {
                    seen_new_line = false;
                    after_hard_line = false;
                    at_paragraph_start = false;
                    joiner.entry(&italic.format().with_options(FormatMdInlineItalicOptions {
                        should_keep_fences: self.keep_fences_in_italics,
                    }));
                }
                AnyMdInline::MdIndentToken(indent) => {
                    // Continuation indent tokens are not content; keep seen_new_line
                    // so the next space-only token is still detected as excess.
                    joiner.entry(&indent.format());
                }
                _ => {
                    seen_new_line = false;
                    at_paragraph_start = false;
                    joiner.entry(&item.format());
                }
            }
        }

        joiner.finish()
    }

    /// If the first and last [MdTextual] are `<` and `>` respectively,
    /// they are removed. Otherwise falls back to [TrimMode::All].
    fn fmt_auto_link_like(
        &self,
        node: &MdInlineItemList,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        let items: Vec<_> = node.iter().collect();

        let starts_with_lt = matches!(items.first(), Some(AnyMdInline::MdTextual(t)) if t.value_token().is_ok_and(|tok| tok.text() == "<"));
        let ends_with_gt = matches!(items.last(), Some(AnyMdInline::MdTextual(t)) if t.value_token().is_ok_and(|tok| tok.text() == ">"));

        let is_auto_link = starts_with_lt && ends_with_gt && items.len() > 2;

        if !is_auto_link {
            return self.fmt_trim_all(node, f);
        }

        let mut joiner = f.join();
        for (index, item) in items.iter().enumerate() {
            if (index == 0 || index == items.len() - 1)
                && let AnyMdInline::MdTextual(text) = item
            {
                joiner.entry(&text.format().with_options(FormatMdTextualOptions {
                    print_mode: TextPrintMode::Remove,
                    ..FormatMdTextualOptions::default()
                }));
                continue;
            }
            match item {
                AnyMdInline::MdInlineItalic(italic) => {
                    joiner.entry(&italic.format().with_options(FormatMdInlineItalicOptions {
                        should_keep_fences: self.keep_fences_in_italics,
                    }));
                }
                _ => {
                    joiner.entry(&item.format());
                }
            }
        }
        joiner.finish()
    }

    /// Strips leading and trailing whitespace/hard-lines around the content.
    /// Items between the first and last non-empty nodes are kept as-is;
    /// items outside those boundaries are removed.
    fn fmt_trim_all(&self, node: &MdInlineItemList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let items: Vec<_> = node.iter().collect();
        let mut joiner = f.join();

        let is_content = |item: &AnyMdInline| match item {
            AnyMdInline::MdTextual(text) => !text.is_empty().unwrap_or_default(),
            AnyMdInline::MdHardLine(_) | AnyMdInline::MdIndentToken(_) => false,
            _ => true,
        };

        // Find the first non-empty item from the left.
        let first_content = items.iter().position(&is_content);

        // Find the first non-empty item from the right.
        let last_content = items
            .iter()
            .rev()
            .position(is_content)
            .map(|pos| items.len() - 1 - pos);

        for (index, item) in items.iter().enumerate() {
            let is_before_content = first_content.is_none_or(|first| index < first);
            let is_after_content = last_content.is_none_or(|last| index > last);

            if is_before_content || is_after_content {
                // Outside content boundaries: remove empty nodes.
                match item {
                    AnyMdInline::MdTextual(text) => {
                        joiner.entry(&text.format().with_options(FormatMdTextualOptions {
                            print_mode: TextPrintMode::Remove,
                            ..FormatMdTextualOptions::default()
                        }));
                    }
                    AnyMdInline::MdHardLine(hard_line) => {
                        joiner.entry(&hard_line.format().with_options(
                            FormatMdFormatHardLineOptions {
                                print_mode: TextPrintMode::trim_all(),
                            },
                        ));
                    }
                    AnyMdInline::MdInlineItalic(italic) => {
                        joiner.entry(&italic.format().with_options(FormatMdInlineItalicOptions {
                            should_keep_fences: self.keep_fences_in_italics,
                        }));
                    }
                    AnyMdInline::MdIndentToken(indent) => {
                        let token = indent.md_indent_char_token()?;
                        joiner.entry(&format_with(|f: &mut MarkdownFormatter| {
                            f.context()
                                .comments()
                                .mark_suppression_checked(indent.syntax());
                            format_removed(&token).fmt(f)
                        }));
                    }
                    _ => {
                        joiner.entry(&item.format());
                    }
                }
            } else {
                // Inside content boundaries: keep as-is.
                match item {
                    AnyMdInline::MdInlineItalic(italic) => {
                        joiner.entry(&italic.format().with_options(FormatMdInlineItalicOptions {
                            should_keep_fences: self.keep_fences_in_italics,
                        }));
                    }
                    _ => {
                        joiner.entry(&item.format());
                    }
                }
            }
        }

        joiner.finish()
    }

    /// Normalizes all whitespace in textual nodes to `hard_space`.
    ///
    /// For example, `[  Foo   Bar  ]` becomes `[ Foo Bar ]`.
    fn fmt_normalize_words(
        &self,
        node: &MdInlineItemList,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        let mut joiner = f.join();

        for item in node.iter() {
            match item {
                AnyMdInline::MdTextual(text) => {
                    joiner.entry(&text.format().with_options(FormatMdTextualOptions {
                        print_mode: TextPrintMode::Trim(TrimMode::NormalizeWords),
                        ..FormatMdTextualOptions::default()
                    }));
                }
                AnyMdInline::MdInlineItalic(italic) => {
                    joiner.entry(&italic.format().with_options(FormatMdInlineItalicOptions {
                        should_keep_fences: self.keep_fences_in_italics,
                    }));
                }
                _ => {
                    joiner.entry(&item.format());
                }
            }
        }

        joiner.finish()
    }

    /// Clean mode: formats content verbatim, but removes the first
    /// whitespace-only textual token.
    fn fmt_clean(&self, node: &MdInlineItemList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let mut joiner = f.join();
        let mut handled_first = false;
        for item in node.iter() {
            match item {
                AnyMdInline::MdTextual(text) if !handled_first => {
                    handled_first = true;
                    if text.is_empty_and_not_newline().unwrap_or_default()
                        || text.is_newline().unwrap_or_default()
                    {
                        // First token is trailing whitespace/newline from the
                        // info string line — remove it entirely.
                        joiner.entry(&text.format().with_options(FormatMdTextualOptions {
                            print_mode: TextPrintMode::Remove,
                            ..FormatMdTextualOptions::default()
                        }));
                    } else {
                        let token = text.value_token()?;
                        let token_text = token.text();
                        if token_text.trim().is_empty() {
                            // Mixed whitespace + newline (e.g. "    \n") — remove.
                            joiner.entry(&text.format().with_options(FormatMdTextualOptions {
                                print_mode: TextPrintMode::Remove,
                                ..FormatMdTextualOptions::default()
                            }));
                        } else {
                            // First token has content — keep as-is.
                            joiner.entry(&text.format());
                        }
                    }
                }
                AnyMdInline::MdTextual(text) if text.is_newline().unwrap_or_default() => {
                    // After a newline, reset the skip counter for the next line.
                    joiner.entry(&text.format());
                }
                AnyMdInline::MdHardLine(hd) => {
                    joiner.entry(&hd.format().with_options(FormatMdFormatHardLineOptions {
                        print_mode: TextPrintMode::Pristine,
                    }));
                }
                AnyMdInline::MdInlineItalic(italic) => {
                    joiner.entry(&italic.format().with_options(FormatMdInlineItalicOptions {
                        should_keep_fences: self.keep_fences_in_italics,
                    }));
                }
                node => {
                    joiner.entry(&node.format());
                }
            }
        }

        joiner.finish()
    }

    /// Formats all items verbatim, preserving the original text exactly.
    /// Hard lines are explicitly set to pristine mode to prevent
    /// normalization (e.g. collapsing multiple trailing spaces to two).
    fn fmt_pristine(&self, node: &MdInlineItemList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let mut joiner = f.join();

        for item in node.iter() {
            match item {
                AnyMdInline::MdHardLine(hd) => {
                    joiner.entry(&hd.format().with_options(FormatMdFormatHardLineOptions {
                        print_mode: TextPrintMode::Pristine,
                    }));
                }
                AnyMdInline::MdInlineItalic(italic) => {
                    joiner.entry(&italic.format().with_options(FormatMdInlineItalicOptions {
                        should_keep_fences: self.keep_fences_in_italics,
                    }));
                }
                node => {
                    joiner.entry(&node.format());
                }
            }
        }

        joiner.finish()
    }

    /// Formats fenced code block content: each source line becomes a separate
    /// IR entry joined by `hard_line_break`. Continuation-indent tokens are
    /// removed (the enclosing `align()` handles indentation). Spaces within
    /// lines are preserved verbatim. Trailing spaces before a newline are
    /// stripped.
    fn fmt_fenced_code_block(
        &self,
        node: &MdInlineItemList,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        for item in FencedCodeContentIterator::new(node.iter()) {
            match item {
                FencedCodeContentItem::NewlineRun {
                    newlines,
                    is_leading,
                } => {
                    let newline_count = newlines.len();

                    for newline in &newlines {
                        write!(
                            f,
                            [newline.format().with_options(FormatMdTextualOptions {
                                print_mode: TextPrintMode::Remove,
                                ..FormatMdTextualOptions::default()
                            })]
                        )?;
                    }

                    if is_leading {
                        if newline_count > 1 {
                            write!(f, [empty_line()])?;
                        }
                    } else if newline_count == 1 {
                        write!(f, [hard_line_break()])?;
                    } else {
                        for _ in 0..newline_count {
                            write!(f, [text("\n", None)])?;
                        }
                        write!(f, [hard_line_break()])?;
                    }
                }
                FencedCodeContentItem::Text {
                    text: md_text,
                    next_is_newline_or_end,
                } => {
                    let token = md_text.value_token()?;
                    if next_is_newline_or_end {
                        let trimmed = token.text().trim_end();
                        if trimmed.len() < token.text().len() {
                            f.context()
                                .comments()
                                .mark_suppression_checked(md_text.syntax());
                            write!(
                                f,
                                [format_replaced(
                                    &token,
                                    &text(trimmed, Some(token.text_trimmed_range().start()))
                                )]
                            )?;
                        } else {
                            write!(f, [md_text.format()])?;
                        }
                    } else {
                        write!(f, [md_text.format()])?;
                    }
                }
                FencedCodeContentItem::Indent(indent) => {
                    f.context()
                        .comments()
                        .mark_suppression_checked(indent.syntax());
                    let char_token = indent.md_indent_char_token()?;
                    write!(f, [format_removed(&char_token)])?;
                }
                FencedCodeContentItem::Other(item) => {
                    write!(f, [item.format()])?;
                }
            }
        }

        write!(f, [hard_line_break()])
    }

    /// Formats prose with `proseWrap: "preserve"` semantics.
    ///
    /// Each source line is emitted as-is with `hard_line_break` between them.
    /// Hard breaks (`  \n` or `\\\n`) are formatted using their original node.
    ///
    /// TODO: for `proseWrap: "always"`, replace sequential writes with `f.fill()`
    /// using `soft_line_break_or_space()` separators between word-level entries.
    /// The word stream from `build_word_stream_flat` already provides the
    /// granularity needed — just change the emission strategy.
    fn fmt_fill(
        &self,
        node: &MdInlineItemList,
        f: &mut MarkdownFormatter,
        text_context: TextContext,
    ) -> FormatResult<()> {
        let WordStreamResult { mut stream } = build_word_stream_flat(node, f)?;
        let inside_list = text_context.is_list();
        let outdented_lines = if inside_list {
            enclosing_list_content_indent(node)
                .map(|content_indent| outdented_list_marker_lines(node, content_indent))
                .transpose()?
                .unwrap_or_default()
        } else {
            Vec::new()
        };

        if inside_list {
            strip_spaces_after_soft_breaks(&mut stream);
        }

        let mut is_first_line = true;
        let mut line_index = 0;
        let mut line_start = 0;

        for (i, item) in stream.iter().enumerate() {
            match item {
                ProseItem::HardBreak(hard_break) => {
                    let line_items = &stream[line_start..i];
                    if !line_items.is_empty() {
                        format_source_line(
                            line_items,
                            line_index,
                            !is_first_line,
                            false,
                            &outdented_lines,
                            f,
                        )?;
                    }
                    write!(
                        f,
                        [hard_break
                            .format()
                            .with_options(FormatMdFormatHardLineOptions {
                                print_mode: TextPrintMode::fill(),
                            })]
                    )?;
                    is_first_line = true;
                    line_start = i + 1;
                    line_index += 1;
                }
                ProseItem::SoftBreak => {
                    let line_items = &stream[line_start..i];
                    if !line_items.is_empty() {
                        format_source_line(
                            line_items,
                            line_index,
                            !is_first_line,
                            !inside_list,
                            &outdented_lines,
                            f,
                        )?;
                        is_first_line = false;
                    }
                    line_start = i + 1;
                    line_index += 1;
                }
                _ => {}
            }
        }
        let remaining = &stream[line_start..];
        if !remaining.is_empty() {
            format_source_line(
                remaining,
                line_index,
                !is_first_line,
                false,
                &outdented_lines,
                f,
            )?;
        }

        if !inside_list {
            write!(f, [hard_line_break()])?;
        }

        Ok(())
    }
}

/// Business-level items for fenced code content inside lists.
///
/// The CST stores fenced code content as inline nodes, including source newlines
/// and list continuation indentation. This enum groups those nodes by the
/// formatting decision they require before `fmt_fenced_code_block` emits IR.
enum FencedCodeContentItem {
    /// One or more consecutive newline textual nodes.
    ///
    /// A leading run is the newline sequence immediately after the opening
    /// fence. Non-leading runs represent literal line breaks inside the code
    /// content and must preserve their exact blank-line count.
    NewlineRun {
        newlines: Vec<MdTextual>,
        is_leading: bool,
    },
    /// A non-newline textual node from the fenced code content.
    ///
    /// `next_is_newline_or_end` marks line-final text so trailing spaces can be
    /// stripped without changing spaces in the middle of a code line.
    Text {
        text: MdTextual,
        next_is_newline_or_end: bool,
    },
    /// Source indentation that belongs to list continuation, not code content.
    ///
    /// The enclosing list formatter provides indentation structurally with
    /// `align()`, so these tokens are removed from the printed code content.
    Indent(MdIndentToken),
    /// Any other inline node that should keep its own formatter behavior.
    Other(AnyMdInline),
}

struct FencedCodeContentIterator {
    items: Peekable<AstNodeListIterator<MarkdownLanguage, AnyMdInline>>,
    is_leading: bool,
}

impl FencedCodeContentIterator {
    fn new(items: AstNodeListIterator<MarkdownLanguage, AnyMdInline>) -> Self {
        Self {
            items: items.peekable(),
            is_leading: true,
        }
    }

    fn next_text(&mut self, text: MdTextual) -> Option<FencedCodeContentItem> {
        if text.is_newline().ok()? {
            let is_leading = self.is_leading;
            self.is_leading = false;
            let mut newlines = vec![text];

            loop {
                let next_is_newline = match self.items.peek() {
                    Some(AnyMdInline::MdTextual(next)) => next.is_newline().ok()?,
                    _ => false,
                };

                if !next_is_newline {
                    break;
                }

                if let Some(AnyMdInline::MdTextual(next)) = self.items.next() {
                    newlines.push(next);
                }
            }

            Some(FencedCodeContentItem::NewlineRun {
                newlines,
                is_leading,
            })
        } else {
            self.is_leading = false;
            let next_is_newline_or_end = match self.items.peek() {
                Some(AnyMdInline::MdTextual(next)) => next.is_newline().ok()?,
                None => true,
                _ => false,
            };

            Some(FencedCodeContentItem::Text {
                text,
                next_is_newline_or_end,
            })
        }
    }
}

impl Iterator for FencedCodeContentIterator {
    type Item = FencedCodeContentItem;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.items.next()?;

        match item {
            AnyMdInline::MdTextual(text) => self.next_text(text),
            AnyMdInline::MdIndentToken(indent) => Some(FencedCodeContentItem::Indent(indent)),
            item => {
                self.is_leading = false;
                Some(FencedCodeContentItem::Other(item))
            }
        }
    }
}

impl FusedIterator for FencedCodeContentIterator {}

#[derive(Debug, Default)]
pub(crate) struct FormatMdFormatInlineItemListOptions {
    /// When `true`, and there's a [MdInlineItalic], it instructions the formatter to keep the fences.
    /// When `false`, it lets the node figure it out.
    pub(crate) keep_fences_in_italics: bool,
    pub(crate) print_mode: TextPrintMode,
    pub(crate) text_context: TextContext,
}

impl FormatRuleWithOptions<MdInlineItemList> for FormatMdInlineItemList {
    type Options = FormatMdFormatInlineItemListOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.print_mode = options.print_mode;
        self.keep_fences_in_italics = options.keep_fences_in_italics;
        self.text_context = options.text_context;
        self
    }
}
