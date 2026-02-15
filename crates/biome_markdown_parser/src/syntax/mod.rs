//! Block and inline syntax parsing for Markdown.
//!
//! # CommonMark Specification References
//!
//! This module implements CommonMark 0.31.2 block structure:
//!
//! ## Leaf Blocks (§4)
//! - **§4.1 Thematic breaks**: `---`, `***`, `___`
//! - **§4.2 ATX headings**: `# Heading`, `## Heading`, etc.
//! - **§4.3 Setext headings**: Underlined with `===` or `---`
//! - **§4.4 Indented code blocks**: 4+ spaces of indentation
//! - **§4.5 Fenced code blocks**: ``` or ~~~ delimited
//! - **§4.6 HTML blocks**: Raw HTML content
//! - **§4.7 Link reference definitions**: `[label]: url "title"`
//! - **§4.8 Paragraphs**: Default block content
//!
//! ## Container Blocks (§5)
//! - **§5.1 Block quotes**: `>` prefixed content
//! - **§5.2 List items**: `-`, `*`, `+` or `1.` prefixed
//! - **§5.3 Lists**: Sequences of list items
//!
//! ## Inline Content (§6)
//! See [`inline`] module for inline element parsing.

pub mod fenced_code_block;
pub mod header;
pub mod html_block;
pub mod inline;
pub mod link_block;
pub mod list;
pub mod parse_error;
pub mod quote;
pub mod reference;
pub mod thematic_break_block;

use biome_markdown_syntax::kind::MarkdownSyntaxKind;
use biome_markdown_syntax::{T, kind::MarkdownSyntaxKind::*};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::RecoveryResult;
use biome_parser::{
    Parser,
    prelude::ParsedSyntax::{self, *},
};
use biome_rowan::TextSize;
use fenced_code_block::{
    at_fenced_code_block, info_string_has_backtick, parse_fenced_code_block,
    parse_fenced_code_block_force,
};
use header::{at_header, parse_header};
use html_block::{at_html_block, at_html_block_interrupt, parse_html_block};
use inline::EmphasisContext;
use link_block::{at_link_block, parse_link_block};
use list::{
    at_bullet_list_item, at_order_list_item, marker_followed_by_whitespace_or_eol,
    parse_bullet_list_item, parse_order_list_item, textual_starts_with_ordered_marker,
};
use quote::{
    at_quote, consume_quote_prefix, consume_quote_prefix_without_virtual, has_quote_prefix,
    parse_quote,
};
use thematic_break_block::{at_thematic_break_block, parse_thematic_break_block};

use crate::MarkdownParser;

/// Maximum paren nesting allowed in link destinations per CommonMark.
pub(crate) const MAX_LINK_DESTINATION_PAREN_DEPTH: i32 = 32;

/// CommonMark requires 4 or more spaces for indented code blocks.
const INDENT_CODE_BLOCK_SPACES: usize = 4;
/// Tabs advance to the next 4-space tab stop in CommonMark parsing.
const TAB_STOP_SPACES: usize = 4;

pub(crate) fn parse_document(p: &mut MarkdownParser) {
    let m = p.start();
    let _ = parse_block_list(p);
    // Bump the EOF token - required by the grammar
    p.bump(T![EOF]);
    m.complete(p, MD_DOCUMENT);
}

/// Result of updating parenthesis depth when scanning link destinations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParenDepthResult {
    /// Depth updated successfully, contains new depth value
    Ok(i32),
    /// Depth would exceed the maximum (too many nested opening parens).
    /// Per cmark, this truncates the destination at this point.
    DepthExceeded,
    /// Unmatched closing paren (would go below 0).
    /// This typically means the `)` belongs to the enclosing construct.
    UnmatchedClose,
}

pub(crate) fn try_update_paren_depth(text: &str, depth: i32, max: i32) -> ParenDepthResult {
    let mut depth = depth;
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' && matches!(chars.peek(), Some('(' | ')')) {
            chars.next();
            continue;
        }

        if c == '(' {
            if depth == max {
                return ParenDepthResult::DepthExceeded;
            }
            depth += 1;
        } else if c == ')' {
            if depth == 0 {
                return ParenDepthResult::UnmatchedClose;
            }
            depth -= 1;
        }
    }

    ParenDepthResult::Ok(depth)
}

pub(crate) enum LinkDestinationKind {
    Enclosed,
    Raw,
}

pub(crate) fn validate_link_destination_text(
    text: &str,
    kind: LinkDestinationKind,
    pending_escape: &mut bool,
) -> bool {
    for c in text.chars() {
        if *pending_escape {
            if c.is_ascii_punctuation() {
                *pending_escape = false;
                continue;
            }
            *pending_escape = false;
        }

        if c == '\\' {
            *pending_escape = true;
            continue;
        }

        if c.is_ascii_control() {
            return false;
        }

        if matches!(kind, LinkDestinationKind::Enclosed) && c == '<' {
            return false;
        }
    }

    true
}

pub(crate) fn ends_with_unescaped_close(text: &str, close_char: char) -> bool {
    if !text.ends_with(close_char) {
        return false;
    }

    let mut backslashes = 0;
    for c in text.chars().rev().skip(1) {
        if c == '\\' {
            backslashes += 1;
        } else {
            break;
        }
    }

    backslashes % 2 == 0
}

pub(crate) fn parse_block_list(p: &mut MarkdownParser) -> ParsedSyntax {
    let mut list = DocumentBlockList;
    Present(list.parse_list(p))
}

/// Struct implementing `ParseNodeList` for document-level block content.
struct DocumentBlockList;

impl ParseNodeList for DocumentBlockList {
    type Kind = MarkdownSyntaxKind;
    type Parser<'source> = MarkdownParser<'source>;

    const LIST_KIND: Self::Kind = MD_BLOCK_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_any_block(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![EOF])
    }

    fn recover(
        &mut self,
        _p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        match parsed_element {
            Present(marker) => RecoveryResult::Ok(marker),
            Absent => RecoveryResult::Err(biome_parser::parse_recovery::RecoveryError::Eof),
        }
    }
}

pub(crate) fn parse_any_block(p: &mut MarkdownParser) -> ParsedSyntax {
    parse_any_block_with_indent_code_policy(p, true)
}

/// Check if a syntax kind represents a paragraph-like block.
///
/// This is used for lazy continuation logic in block quotes and lists.
/// Both paragraphs and setext headings are considered "paragraph-like"
/// because they share continuation behavior.
pub(crate) fn is_paragraph_like(kind: biome_markdown_syntax::MarkdownSyntaxKind) -> bool {
    matches!(kind, MD_PARAGRAPH | MD_SETEXT_HEADER)
}

pub(crate) fn parse_any_block_with_indent_code_policy(
    p: &mut MarkdownParser,
    allow_indent_code_block: bool,
) -> ParsedSyntax {
    let start = p.cur_range().start();
    // Handle standalone NEWLINE tokens as MdNewline nodes.
    // This prevents inter-block NEWLINEs from becoming "newline-only paragraphs".
    if p.at(NEWLINE) {
        let m = p.start();
        p.bump(NEWLINE);
        return Present(m.complete(p, MD_NEWLINE));
    }
    if at_blank_line_start(p) {
        while p.at(MD_TEXTUAL_LITERAL) {
            let text = p.cur_text();
            if text == " " || text == "\t" {
                p.parse_as_skipped_trivia_tokens(|p| p.bump(MD_TEXTUAL_LITERAL));
            } else {
                break;
            }
        }
        if p.at(NEWLINE) {
            let m = p.start();
            p.bump(NEWLINE);
            return Present(m.complete(p, MD_NEWLINE));
        }
        // Blank line with no trailing newline (e.g., end of file)
        return Absent;
    }

    let parsed = if allow_indent_code_block && at_indent_code_block(p) {
        parse_indent_code_block(p)
    } else if at_fenced_code_block(p) {
        parse_fenced_code_block(p)
    } else if line_starts_with_fence(p) {
        parse_fenced_code_block_force(p)
    } else if at_thematic_break_block(p) {
        let break_block = try_parse(p, |p| {
            let break_block = parse_thematic_break_block(p);
            if break_block.is_absent() {
                return Err(());
            }
            Ok(break_block)
        });
        if let Ok(parsed) = break_block {
            parsed
        } else {
            parse_paragraph(p)
        }
    } else if at_header(p) {
        // Check for too many hashes BEFORE try_parse (which would lose diagnostics on rewind)
        let too_many = check_too_many_hashes(p);
        let header_result = try_parse(p, |p| {
            let header = parse_header(p);
            if header.is_absent() {
                return Err(());
            }
            Ok(header)
        });
        if let Ok(parsed) = header_result {
            parsed
        } else {
            // Emit diagnostic for too many hashes (outside try_parse to persist)
            if let Some((range, count)) = too_many {
                p.error(parse_error::too_many_hashes(p, range, count));
            }
            // Not a valid header, parse as paragraph
            parse_paragraph(p)
        }
    } else if at_quote(p) {
        parse_quote(p)
    } else if at_bullet_list_item(p) {
        parse_bullet_list_item(p)
    } else if at_order_list_item(p) || at_order_list_item_textual(p) {
        let forced = if !at_order_list_item(p) && at_order_list_item_textual(p) {
            p.set_force_ordered_list_marker(true);
            p.force_relex_regular();
            true
        } else {
            false
        };
        let parsed = parse_order_list_item(p);
        if forced {
            p.set_force_ordered_list_marker(false);
        }
        if parsed.is_absent() {
            parse_paragraph(p)
        } else {
            parsed
        }
    } else if at_html_block(p) {
        parse_html_block(p)
    } else if at_link_block(p) {
        // Try to parse as link reference definition
        // Use try_parse to fall back to paragraph if not a valid definition
        let link_result = try_parse(p, |p| {
            let link = parse_link_block(p);
            if link.is_absent() {
                return Err(());
            }
            Ok(link)
        });
        if let Ok(parsed) = link_result {
            parsed
        } else {
            parse_paragraph(p)
        }
    } else if at_block_interrupt(p) {
        // We see a block interrupt but didn't match a concrete block above.
        // This can happen when list item indentation is still active.
        if at_bullet_list_item_at_any_indent(p) {
            let prev_required = p.state().list_item_required_indent;
            let prev_virtual = p.state().virtual_line_start;
            p.state_mut().list_item_required_indent = 0;
            p.state_mut().virtual_line_start = Some(p.cur_range().start());
            let parsed = parse_bullet_list_item(p);
            p.state_mut().list_item_required_indent = prev_required;
            p.state_mut().virtual_line_start = prev_virtual;
            if parsed.is_present() {
                parsed
            } else {
                parse_paragraph(p)
            }
        } else if at_order_list_item_at_any_indent(p) {
            let prev_required = p.state().list_item_required_indent;
            let prev_virtual = p.state().virtual_line_start;
            p.state_mut().list_item_required_indent = 0;
            p.state_mut().virtual_line_start = Some(p.cur_range().start());
            let parsed = parse_order_list_item(p);
            p.state_mut().list_item_required_indent = prev_required;
            p.state_mut().virtual_line_start = prev_virtual;
            if parsed.is_present() {
                parsed
            } else {
                parse_paragraph(p)
            }
        } else {
            parse_paragraph(p)
        }
    } else {
        // Default fallback: parse as paragraph
        parse_paragraph(p)
    };

    if start == p.cur_range().start() {
        let range = p.cur_range();
        if std::env::var("CMARK_HANG_DEBUG").is_ok() {
            eprintln!(
                "parse_any_block made no progress at {:?} {:?} => {:?}",
                p.cur(),
                p.cur_text(),
                parsed
            );
        }
        p.error(parse_error::parse_any_block_no_progress(p, range));
        if !p.at(T![EOF]) {
            p.bump_any();
        }
        return Absent;
    }

    parsed
}

pub(crate) fn with_virtual_line_start<F, R>(p: &mut MarkdownParser, start: TextSize, op: F) -> R
where
    F: FnOnce(&mut MarkdownParser) -> R,
{
    let prev_virtual = p.state().virtual_line_start;
    p.state_mut().virtual_line_start = Some(start);
    let result = op(p);
    p.state_mut().virtual_line_start = prev_virtual;
    result
}

enum QuoteBreakKind {
    None,
    SetextUnderline,
    Other,
}

/// Check if we're at an indented code block (4+ spaces of indentation).
///
/// Uses `line_start_leading_indent()` to correctly handle indentation when NEWLINE
/// tokens are explicit (not trivia).
pub(crate) fn at_indent_code_block(p: &mut MarkdownParser) -> bool {
    if !p.at_line_start() {
        return false;
    }

    if at_blank_line_start(p) {
        return false;
    }

    let indent = p.line_start_leading_indent();
    let required_indent = p.state().list_item_required_indent;

    // Inside a list item, we need 4 spaces BEYOND the list item's required indent.
    // e.g., if list item requires 2 spaces, code block needs 2 + 4 = 6 spaces.
    // Outside a list item (required_indent == 0), we need just 4 spaces.
    let effective_indent = indent.saturating_sub(required_indent);
    effective_indent >= INDENT_CODE_BLOCK_SPACES
}

/// Parse an indented code block.
///
/// Grammar: MdIndentCodeBlock = content: MdInlineItemList
///
/// An indented code block consists of one or more lines with 4+ spaces
/// of indentation. The indentation is tracked in trivia.
///
/// # NEWLINE Handling
///
/// NEWLINE is an explicit token. We consume tokens until NEWLINE,
/// then check if the next line has proper indentation.
pub(crate) fn parse_indent_code_block(p: &mut MarkdownParser) -> ParsedSyntax {
    if !at_indent_code_block(p) {
        return Absent;
    }

    let m = p.start();
    let content = p.start();

    // Parse content while we're at indented lines, including interior blank lines.
    loop {
        if p.at(T![EOF]) {
            break;
        }

        // Always include NEWLINE tokens in code content
        if p.at(NEWLINE) {
            if newline_is_blank_line(p) && !has_following_indented_code_line(p) {
                break;
            }
            let text_m = p.start();
            p.bump_remap(MD_TEXTUAL_LITERAL);
            text_m.complete(p, MD_TEXTUAL);
            p.set_virtual_line_start();
            p.set_virtual_line_start();
            continue;
        }

        if p.at_line_start() {
            if at_blank_line_start(p) {
                if has_following_indented_code_line(p) {
                    consume_blank_line(p);
                    continue;
                }
                break;
            } else if !at_indent_code_block(p) {
                break;
            }
            if p.state().list_item_required_indent == 0 {
                consume_indent_prefix(p, INDENT_CODE_BLOCK_SPACES);
            }
        }

        // Consume token as code content
        let text_m = p.start();
        p.bump_remap(MD_TEXTUAL_LITERAL);
        text_m.complete(p, MD_TEXTUAL);
    }

    content.complete(p, MD_INLINE_ITEM_LIST);
    Present(m.complete(p, MD_INDENT_CODE_BLOCK))
}

fn has_following_indented_code_line(p: &mut MarkdownParser) -> bool {
    p.lookahead(|p| {
        while p.at_line_start() && at_blank_line_start(p) {
            while p.at(MD_TEXTUAL_LITERAL) {
                let text = p.cur_text();
                if text == " " || text == "\t" {
                    p.bump(MD_TEXTUAL_LITERAL);
                } else {
                    break;
                }
            }

            if p.at(NEWLINE) {
                p.bump(NEWLINE);
            } else {
                break;
            }
        }

        at_indent_code_block(p)
    })
}

fn newline_is_blank_line(p: &MarkdownParser) -> bool {
    // Token stream doesn't expose the prior byte, so read source to detect CR/LF.
    let start: usize = p.cur_range().start().into();
    if start == 0 {
        return true;
    }

    let source = p.source().source_text();
    let prev = source.as_bytes()[start - 1];
    prev == b'\n' || prev == b'\r'
}

/// Check if the current line is blank (lookahead only).
fn consume_indent_prefix(p: &mut MarkdownParser, indent: usize) {
    if indent == 0 {
        return;
    }

    let mut consumed = 0usize;
    while consumed < indent && p.at(MD_TEXTUAL_LITERAL) {
        let text = p.cur_text();
        if text == " " {
            consumed += 1;
        } else if text == "\t" {
            consumed += 4;
        } else {
            break;
        }

        p.parse_as_skipped_trivia_tokens(|p| p.bump(MD_TEXTUAL_LITERAL));
    }
}

/// Consume exactly `indent` columns of leading whitespace at line start.
fn at_blank_line_start(p: &mut MarkdownParser) -> bool {
    if !p.at_line_start() {
        return false;
    }

    p.lookahead(|p| {
        while p.at(MD_TEXTUAL_LITERAL) {
            let text = p.cur_text();
            if text == " " || text == "\t" {
                p.bump(MD_TEXTUAL_LITERAL);
            } else {
                break;
            }
        }

        p.at(NEWLINE) || p.at(T![EOF])
    })
}

fn consume_blank_line(p: &mut MarkdownParser) {
    while p.at(MD_TEXTUAL_LITERAL) {
        let text = p.cur_text();
        if text == " " || text == "\t" {
            p.parse_as_skipped_trivia_tokens(|p| p.bump(MD_TEXTUAL_LITERAL));
        } else {
            break;
        }
    }

    if p.at(NEWLINE) {
        let text_m = p.start();
        p.bump_remap(MD_TEXTUAL_LITERAL);
        text_m.complete(p, MD_TEXTUAL);
    }
}

/// Parse a paragraph block, or a setext heading if followed by an underline.
///
/// A paragraph is a sequence of non-blank lines that cannot be interpreted as
/// other kinds of blocks. The paragraph ends at a blank line or EOF.
///
/// If the paragraph is followed by a setext heading underline (=== or ---),
/// it becomes a setext heading instead.
///
/// Grammar: MdParagraph = list: MdInlineItemList hard_line: MdHardLine?
/// Grammar: MdSetextHeader = content: MdInlineItemList underline: 'md_setext_underline_literal'
pub(crate) fn parse_paragraph(p: &mut MarkdownParser) -> ParsedSyntax {
    let m = p.start();

    let inline_start: usize = p.cur_range().start().into();
    parse_inline_item_list(p);
    let inline_end: usize = p.cur_range().start().into();

    let has_inline_content = inline_has_non_whitespace(p, inline_start, inline_end);
    let allow_setext = has_inline_content && allow_setext_heading(p);

    // Check if this paragraph is followed by a setext heading underline
    // MD_SETEXT_UNDERLINE_LITERAL is for `=` underlines
    // MD_THEMATIC_BREAK_LITERAL with only `-` is also a setext underline (H2)
    let completed = if allow_setext && p.at(MD_SETEXT_UNDERLINE_LITERAL) {
        let indent = real_line_indent_from_source(p);
        if indent < 4 {
            // This is a setext heading (H1 with `=`) - consume the underline
            p.bump(MD_SETEXT_UNDERLINE_LITERAL);
            m.complete(p, MD_SETEXT_HEADER)
        } else {
            // 4+ spaces of indent: not a setext underline (CommonMark §4.3)
            m.complete(p, MD_PARAGRAPH)
        }
    } else if allow_setext && p.at(MD_THEMATIC_BREAK_LITERAL) && is_dash_only_thematic_break(p) {
        let indent = real_line_indent_from_source(p);
        if indent < 4 {
            // This is a setext heading (H2 with `-`) - remap token and consume
            p.bump_remap(MD_SETEXT_UNDERLINE_LITERAL);
            m.complete(p, MD_SETEXT_HEADER)
        } else {
            // 4+ spaces of indent: not a setext underline (CommonMark §4.3)
            m.complete(p, MD_PARAGRAPH)
        }
    } else {
        m.complete(p, MD_PARAGRAPH)
    };
    Present(completed)
}

fn inline_has_non_whitespace(p: &MarkdownParser, start: usize, end: usize) -> bool {
    if end <= start {
        return false;
    }

    let source = p.source().source_text();
    if end > source.len() {
        return false;
    }

    !source[start..end]
        .trim_matches(|c: char| matches!(c, ' ' | '\t' | '\r' | '\n'))
        .is_empty()
}

/// Check if a thematic break text contains only dashes (used for setext H2 detection).
pub(crate) fn is_dash_only_thematic_break_text(text: &str) -> bool {
    !text.is_empty() && text.trim().chars().all(|c| c == '-')
}

/// Token-based check: is the current line a setext underline?
///
/// Call after consuming a NEWLINE token. Skips 0–3 columns of leading whitespace
/// (tabs expand to the next tab stop per CommonMark §2.2), then checks for
/// `MD_SETEXT_UNDERLINE_LITERAL` or a dash-only `MD_THEMATIC_BREAK_LITERAL`.
///
/// Returns `Some(bytes_consumed)` if the line is a setext underline, `None` otherwise.
/// The byte count includes only the whitespace tokens consumed during the indent skip,
/// NOT the underline token itself. Callers that track byte budgets must subtract this.
///
/// This is the shared helper for setext detection in inline contexts.
/// Used by `has_matching_code_span_closer`, `parse_inline_html`, and `parse_inline_item_list`.
///
/// Context safety: this function does NOT call `allow_setext_heading` because the token
/// stream itself encodes context. In blockquotes, `R_ANGLE` tokens appear after NEWLINE
/// before content, so the whitespace-only skip naturally rejects those lines. In list
/// items, the indent reflected in the token stream is the raw line indent, and the
/// `columns < 4` check correctly rejects lines with 4+ columns of leading whitespace.
pub(crate) fn at_setext_underline_after_newline(p: &mut MarkdownParser) -> Option<usize> {
    let mut columns = 0;
    let mut bytes_consumed = 0;
    while columns < INDENT_CODE_BLOCK_SPACES
        && p.at(MD_TEXTUAL_LITERAL)
        && p.cur_text().chars().all(|c| c == ' ' || c == '\t')
    {
        for c in p.cur_text().chars() {
            match c {
                ' ' => columns += 1,
                '\t' => columns += 4 - (columns % 4),
                _ => {}
            }
        }
        bytes_consumed += p.cur_text().len();
        p.bump(MD_TEXTUAL_LITERAL);
    }
    if columns >= INDENT_CODE_BLOCK_SPACES {
        return None;
    }
    let is_setext = p.at(MD_SETEXT_UNDERLINE_LITERAL)
        || (p.at(MD_THEMATIC_BREAK_LITERAL) && is_dash_only_thematic_break_text(p.cur_text()));
    if is_setext {
        Some(bytes_consumed)
    } else {
        None
    }
}

/// Token-based check: does an inline span of `byte_len` bytes cross a setext underline?
///
/// Walks tokens via lookahead. At each NEWLINE, delegates to
/// [`at_setext_underline_after_newline`] — the same detection used by
/// `has_matching_code_span_closer` and `parse_inline_item_list`.
pub(crate) fn inline_span_crosses_setext(p: &mut MarkdownParser, byte_len: usize) -> bool {
    p.lookahead(|p| {
        let mut remaining = byte_len;
        loop {
            if remaining == 0 || p.at(T![EOF]) {
                return false;
            }
            if p.at(NEWLINE) {
                let nl_len = p.cur_text().len();
                if nl_len > remaining {
                    return false;
                }
                remaining -= nl_len;
                p.bump(NEWLINE);
                if let Some(ws_bytes) = at_setext_underline_after_newline(p) {
                    // Only flag if the whitespace consumed is still within our span
                    return ws_bytes <= remaining;
                }
                continue;
            }
            let tok_len = p.cur_text().len();
            if tok_len > remaining {
                return false;
            }
            remaining -= tok_len;
            p.bump_any();
        }
    })
}

/// Check if the current thematic break token contains only dashes.
/// This is used to detect H2 setext underlines.
fn is_dash_only_thematic_break(p: &MarkdownParser) -> bool {
    is_dash_only_thematic_break_text(p.cur_text())
}

fn allow_setext_heading(p: &MarkdownParser) -> bool {
    let required_indent = p.state().list_item_required_indent;
    if required_indent > 0 {
        // Compute real indent from source text, since leading whitespace
        // may have been consumed as trivia in list item context.
        let indent = real_line_indent_from_source(p);
        if indent < required_indent {
            return false;
        }
    }

    if p.state().list_item_required_indent > 0 && p.at(MD_SETEXT_UNDERLINE_LITERAL) {
        let text = p.cur_text().trim_matches(|c| c == ' ' || c == '\t');
        if text == "-" {
            return false;
        }
    }

    let depth = p.state().block_quote_depth;
    if depth == 0 {
        return true;
    }

    line_has_quote_prefix(p, depth)
}

/// Compute the real leading indent of the current line from source text.
/// This is needed because leading whitespace may have been consumed as trivia
/// in list item context, making `line_start_leading_indent()` return 0.
/// Token-based lookahead cannot recover the original column once trivia is skipped.
fn real_line_indent_from_source(p: &MarkdownParser) -> usize {
    let source = p.source().source_text();
    let pos: usize = p.cur_range().start().into();

    // Find the start of the current line
    let line_start = source[..pos].rfind('\n').map_or(0, |i| i + 1);

    // Count leading whitespace columns on this line
    let mut column = 0;
    for c in source[line_start..].chars() {
        match c {
            ' ' => column += 1,
            '\t' => column += 4 - (column % 4),
            _ => break,
        }
    }
    column
}

fn line_has_quote_prefix(p: &MarkdownParser, depth: usize) -> bool {
    // Tokens may have consumed whitespace as trivia; scan source to recover columns.
    if depth == 0 {
        return false;
    }

    let source = p.source().source_text();
    let start: usize = p.cur_range().start().into();
    let line_start = source[..start].rfind('\n').map_or(0, |idx| idx + 1);

    let mut idx = line_start;
    let mut indent = 0usize;
    while idx < start {
        match source.as_bytes()[idx] {
            b' ' => {
                indent += 1;
                idx += 1;
            }
            b'\t' => {
                indent += 4;
                idx += 1;
            }
            _ => break,
        }
        if indent > 3 {
            return false;
        }
    }

    for _ in 0..depth {
        if idx >= start || source.as_bytes()[idx] != b'>' {
            return false;
        }
        idx += 1;
        if idx < start {
            let c = source.as_bytes()[idx];
            if c == b' ' || c == b'\t' {
                idx += 1;
            }
        }
    }

    true
}

fn classify_quote_break_after_newline(
    p: &mut MarkdownParser,
    quote_depth: usize,
    include_textual_markers: bool,
) -> QuoteBreakKind {
    p.lookahead(|p| {
        consume_quote_prefix_without_virtual(p, quote_depth);
        with_virtual_line_start(p, p.cur_range().start(), |p| {
            if p.at(MD_SETEXT_UNDERLINE_LITERAL)
                || (p.at(MD_THEMATIC_BREAK_LITERAL) && is_dash_only_thematic_break(p))
            {
                QuoteBreakKind::SetextUnderline
            } else if at_block_interrupt(p)
                || (include_textual_markers && textual_looks_like_list_marker(p))
            {
                QuoteBreakKind::Other
            } else {
                QuoteBreakKind::None
            }
        })
    })
}

enum InlineNewlineAction {
    Break,
    Continue,
}

fn handle_inline_newline(p: &mut MarkdownParser, has_content: bool) -> InlineNewlineAction {
    if p.at_blank_line() {
        let text_m = p.start();
        p.bump_remap(MD_TEXTUAL_LITERAL);
        text_m.complete(p, MD_TEXTUAL);
        return InlineNewlineAction::Break;
    }

    let quote_depth = p.state().block_quote_depth;
    if quote_depth > 0 {
        let is_quote_blank_line = p.lookahead(|p| {
            p.bump(NEWLINE);
            if is_quote_only_blank_line_from_source(p, quote_depth) {
                return true;
            }
            if !has_quote_prefix(p, quote_depth) {
                return false;
            }
            consume_quote_prefix_without_virtual(p, quote_depth);
            while p.at(MD_TEXTUAL_LITERAL) && p.cur_text().chars().all(|c| c == ' ' || c == '\t') {
                p.bump(MD_TEXTUAL_LITERAL);
            }
            p.at(NEWLINE) || p.at(T![EOF])
        });
        if is_quote_blank_line {
            let text_m = p.start();
            p.bump_remap(MD_TEXTUAL_LITERAL);
            text_m.complete(p, MD_TEXTUAL);
            return InlineNewlineAction::Break;
        }
    }

    // Not a blank line - this is a soft line break within paragraph
    // Consume the NEWLINE as textual content (remap to MD_TEXTUAL_LITERAL)
    let text_m = p.start();
    p.bump_remap(MD_TEXTUAL_LITERAL);
    text_m.complete(p, MD_TEXTUAL);

    // If we're inside a block quote, only consume the quote prefix
    // when it doesn't start a new block (e.g., a nested quote).
    if quote_depth > 0 && has_quote_prefix(p, quote_depth) {
        let break_kind = classify_quote_break_after_newline(p, quote_depth, true);
        match break_kind {
            QuoteBreakKind::SetextUnderline => {
                // Consume the quote prefix so the setext underline is visible
                // to the paragraph parser.
                consume_quote_prefix(p, quote_depth);
                return InlineNewlineAction::Break;
            }
            QuoteBreakKind::Other => {
                return InlineNewlineAction::Break;
            }
            QuoteBreakKind::None => {
                consume_quote_prefix(p, quote_depth);
            }
        }
    }
    if quote_depth > 0 && p.at(R_ANGLE) && !has_quote_prefix(p, quote_depth) {
        consume_partial_quote_prefix(p, quote_depth);
    }

    // After crossing a line, check for setext underlines.
    // For non-list paragraphs, we need to look past up to 3 spaces of indent
    // to detect setext underlines (CommonMark §4.3).
    // IMPORTANT: Only break if allow_setext_heading() is true - this ensures
    // setext underlines outside a blockquote (without >) don't incorrectly
    // terminate the paragraph (CommonMark example 093).
    if has_content && p.state().list_item_required_indent == 0 && allow_setext_heading(p) {
        let is_setext = p.lookahead(|p| at_setext_underline_after_newline(p).is_some());
        if is_setext {
            // Skip the indent so parse_paragraph sees the underline
            p.skip_line_indent(INDENT_CODE_BLOCK_SPACES);
            return InlineNewlineAction::Break;
        }
    }

    // Check if we're at a setext heading underline (already past indent)
    if has_content && p.at(MD_SETEXT_UNDERLINE_LITERAL) && allow_setext_heading(p) {
        return InlineNewlineAction::Break;
    }
    if has_content && p.at(MD_THEMATIC_BREAK_LITERAL) && is_dash_only_thematic_break(p) {
        return InlineNewlineAction::Break;
    }

    // If we're inside a list item and the next line meets the required indent,
    // check for block interrupts after skipping that indent. This allows
    // nested list markers like "\t - baz" to break out of the paragraph.
    let required_indent = p.state().list_item_required_indent;
    if required_indent > 0 {
        // Check for setext underline after indent stripping.
        // The `---` or `===` may be indented by the list item's required indent,
        // so we need to look past that indent.
        let real_indent = real_line_indent_from_source(p);
        if real_indent >= required_indent {
            let is_setext = p.lookahead(|p| {
                p.skip_line_indent(required_indent);
                p.at(MD_SETEXT_UNDERLINE_LITERAL)
                    || (p.at(MD_THEMATIC_BREAK_LITERAL) && is_dash_only_thematic_break(p))
            });
            if is_setext && has_content {
                // Skip the indent so parse_paragraph sees the underline
                p.skip_line_indent(required_indent);
                return InlineNewlineAction::Break;
            }
        }

        let indent = p.line_start_leading_indent();
        if indent >= required_indent {
            let interrupts = p.lookahead(|p| {
                p.skip_line_indent(required_indent);
                let prev_required = p.state().list_item_required_indent;
                with_virtual_line_start(p, p.cur_range().start(), |p| {
                    p.state_mut().list_item_required_indent = 0;
                    let breaks = at_block_interrupt(p) || textual_looks_like_list_marker(p);
                    p.state_mut().list_item_required_indent = prev_required;
                    breaks
                })
            });
            if interrupts {
                return InlineNewlineAction::Break;
            }
        }
    }

    // Check for block-level constructs that can interrupt paragraphs
    if line_starts_with_fence(p) {
        return InlineNewlineAction::Break;
    }
    if p.at(MD_TEXTUAL_LITERAL) {
        let text = p.cur_text();
        if text.starts_with("```") || text.starts_with("~~~") {
            return InlineNewlineAction::Break;
        }
    }
    if at_block_interrupt(p) {
        return InlineNewlineAction::Break;
    }

    // Also check for list markers that appear as textual content.
    // Inside inline content, '-' is lexed as MD_TEXTUAL_LITERAL, not MINUS,
    // so at_block_interrupt won't detect them. Per CommonMark §5.1, list
    // items can interrupt paragraphs (bullet lists always, ordered lists
    // only if they start with 1).
    if textual_looks_like_list_marker(p) {
        return InlineNewlineAction::Break;
    }

    // Per CommonMark §5.2, when inside a list item, check indentation.
    // If sufficient indentation, skip it. If insufficient, this is
    // "lazy continuation" - the content continues without meeting the
    // indent requirement (at_block_interrupt already checked above).
    if required_indent > 0 {
        let indent = p.line_start_leading_indent();
        if indent >= required_indent {
            // Sufficient indentation - skip it
            p.skip_line_indent(required_indent);
        }
        // else: Lazy continuation - don't break, don't skip indent.
        // The at_block_interrupt check above handles real interruptions.
        // Content continues at its actual position.
    }

    // For plain paragraphs, strip up to 4 leading spaces on continuation lines.
    if required_indent == 0 {
        p.skip_line_indent(INDENT_CODE_BLOCK_SPACES);
    }

    InlineNewlineAction::Continue
}

/// Parse the inline item list within a block.
///
/// Grammar: MdInlineItemList = AnyMdInline*
///
/// Inline content continues until we hit EOF, a blank line (paragraph boundary),
/// a setext heading underline, or a block-level construct that can interrupt
/// paragraphs per CommonMark.
///
/// # NEWLINE Handling
///
/// NEWLINE is an explicit token (not trivia). When we hit NEWLINE:
/// - If it's a blank line (NEWLINE + optional whitespace + NEWLINE/EOF) → stop
/// - Otherwise it's a soft line break → consume and continue to next line
pub(crate) fn parse_inline_item_list(p: &mut MarkdownParser) {
    let m = p.start();
    let prev_emphasis_context = set_inline_emphasis_context(p);
    let quote_depth = p.state().block_quote_depth;
    if quote_depth > 0 && p.at_line_start() && has_quote_prefix(p, quote_depth) {
        consume_quote_prefix(p, quote_depth);
    }
    let inline_start: usize = p.cur_range().start().into();
    let mut has_content = false;

    loop {
        // EOF ends inline content
        if p.at(T![EOF]) {
            break;
        }

        // NEWLINE handling: check for blank line (paragraph boundary)
        if p.at(NEWLINE) {
            if matches!(
                handle_inline_newline(p, has_content),
                InlineNewlineAction::Break
            ) {
                break;
            }
            continue;
        }

        // Check if we're at a setext heading underline (stop for paragraph to handle)
        // Per CommonMark §4.3, setext underlines can be indented 0-3 spaces only.
        if has_content
            && p.at(MD_SETEXT_UNDERLINE_LITERAL)
            && real_line_indent_from_source(p) < INDENT_CODE_BLOCK_SPACES
            && allow_setext_heading(p)
        {
            break;
        }

        // Check if we're at a thematic break that could be a setext underline
        // (dash-only thematic breaks following paragraph content are setext H2)
        if has_content
            && p.at(MD_THEMATIC_BREAK_LITERAL)
            && real_line_indent_from_source(p) < INDENT_CODE_BLOCK_SPACES
            && is_dash_only_thematic_break(p)
        {
            break;
        }

        // Per CommonMark, certain block-level constructs can interrupt paragraphs
        // without requiring a blank line. Check for these at line start.
        if p.has_preceding_line_break() && at_block_interrupt(p) {
            break;
        }

        // Also check for list markers in textual content (see comment above)
        if p.has_preceding_line_break() && textual_looks_like_list_marker(p) {
            break;
        }

        // Parse inline content (stops at NEWLINE via at_inline_end)
        let parsed = parse_any_inline(p);
        if parsed.is_absent() {
            break;
        }
        let after_hard_break = matches!(&parsed, Present(cm) if cm.kind(p) == MD_HARD_LINE);

        // Per CommonMark §6.7: after a hard line break, leading spaces on the
        // next line are ignored. Skip whitespace-only textual tokens as trivia.
        if after_hard_break
            && p.at(MD_TEXTUAL_LITERAL)
            && p.cur_text().chars().all(|c| c == ' ' || c == '\t')
        {
            while p.at(MD_TEXTUAL_LITERAL) && p.cur_text().chars().all(|c| c == ' ' || c == '\t') {
                p.parse_as_skipped_trivia_tokens(|p| p.bump(MD_TEXTUAL_LITERAL));
            }
        }

        let inline_end: usize = p.cur_range().start().into();
        has_content = inline_has_non_whitespace(p, inline_start, inline_end);
    }

    m.complete(p, MD_INLINE_ITEM_LIST);
    p.set_emphasis_context(prev_emphasis_context);
}

fn is_quote_only_blank_line_from_source(p: &MarkdownParser, depth: usize) -> bool {
    if depth == 0 {
        return false;
    }

    let source = p.source().source_text();
    let start: usize = p.cur_range().start().into();
    if start >= source.len() {
        return true;
    }

    // Scan up to 3 spaces/tabs before the first '>'
    let mut idx = start;
    let mut indent = 0usize;
    while idx < source.len() {
        match source.as_bytes()[idx] {
            b' ' => {
                indent += 1;
                idx += 1;
            }
            b'\t' => {
                indent += 4 - (indent % 4);
                idx += 1;
            }
            _ => break,
        }
        if indent > 3 {
            return false;
        }
    }

    // Consume quote markers and optional single space after each
    for _ in 0..depth {
        if idx >= source.len() || source.as_bytes()[idx] != b'>' {
            return false;
        }
        idx += 1;
        if idx < source.len() {
            let c = source.as_bytes()[idx];
            if c == b' ' || c == b'\t' {
                idx += 1;
            }
        }
    }

    // Skip trailing whitespace
    while idx < source.len() {
        match source.as_bytes()[idx] {
            b' ' | b'\t' => idx += 1,
            _ => break,
        }
    }

    // Blank if line ends here or at newline
    if idx >= source.len() {
        return true;
    }

    matches!(source.as_bytes()[idx], b'\n' | b'\r')
}

/// Build an emphasis context for the current inline list and install it on the parser.
/// Returns the previous context so it can be restored.
fn set_inline_emphasis_context(p: &mut MarkdownParser) -> Option<EmphasisContext> {
    let source_len = inline_list_source_len(p);
    let source = p.source_after_current();
    let inline_source = if source_len <= source.len() {
        &source[..source_len]
    } else {
        source
    };
    let base_offset = u32::from(p.cur_range().start()) as usize;
    // Create a reference checker closure that uses the parser's link reference definitions
    let context = EmphasisContext::new(inline_source, base_offset, |label| {
        p.has_link_reference_definition(label)
    });
    p.set_emphasis_context(Some(context))
}

/// Compute the byte length of the inline list starting at the current token.
fn inline_list_source_len(p: &mut MarkdownParser) -> usize {
    p.lookahead(|p| {
        let mut len = 0usize;

        loop {
            if p.at(T![EOF]) {
                break;
            }

            if p.at(NEWLINE) {
                if p.at_blank_line() {
                    len += p.cur_text().len();
                    p.bump(NEWLINE);
                    break;
                }

                len += p.cur_text().len();
                p.bump(NEWLINE);

                let quote_depth = p.state().block_quote_depth;
                if quote_depth > 0 && has_quote_prefix(p, quote_depth) {
                    let break_kind = classify_quote_break_after_newline(p, quote_depth, false);
                    if !matches!(break_kind, QuoteBreakKind::None) {
                        break;
                    }
                    consume_quote_prefix_without_virtual(p, quote_depth);
                }

                if p.at(MD_SETEXT_UNDERLINE_LITERAL) && allow_setext_heading(p) {
                    break;
                }

                if p.at(MD_THEMATIC_BREAK_LITERAL) && is_dash_only_thematic_break(p) {
                    break;
                }

                if quote_depth > 0 && p.at(R_ANGLE) && !has_quote_prefix(p, quote_depth) {
                    consume_partial_quote_prefix_lookahead(p, quote_depth, &mut len);
                }

                if line_starts_with_fence(p) {
                    break;
                }

                if at_block_interrupt(p) {
                    break;
                }

                let required_indent = p.state().list_item_required_indent;
                if required_indent > 0 {
                    let indent = p.line_start_leading_indent();
                    if indent < required_indent {
                        break;
                    }

                    let mut consumed = 0usize;
                    while consumed < required_indent && p.at(MD_TEXTUAL_LITERAL) {
                        let text = p.cur_text();
                        if text.is_empty() || !text.chars().all(|c| c == ' ' || c == '\t') {
                            break;
                        }

                        let indent = text
                            .chars()
                            .map(|c| if c == '\t' { 4 } else { 1 })
                            .sum::<usize>();

                        if consumed + indent > required_indent {
                            break;
                        }

                        consumed += indent;
                        len += text.len();
                        p.bump(MD_TEXTUAL_LITERAL);
                    }
                }

                continue;
            }

            if p.at(MD_SETEXT_UNDERLINE_LITERAL) && allow_setext_heading(p) {
                break;
            }

            if p.at(MD_THEMATIC_BREAK_LITERAL) && is_dash_only_thematic_break(p) {
                break;
            }

            if p.has_preceding_line_break() && at_block_interrupt(p) {
                break;
            }

            len += p.cur_text().len();
            p.bump(p.cur());
        }

        len
    })
}

fn line_starts_with_fence(p: &mut MarkdownParser) -> bool {
    if !p.at_line_start() {
        return false;
    }

    p.lookahead(|p| {
        if p.line_start_leading_indent() > 3 {
            return false;
        }
        p.skip_line_indent(3);
        let rest = p.source_after_current();
        let Some((fence_char, _len)) = fenced_code_block::detect_fence(rest) else {
            return false;
        };
        if fence_char == '`' {
            return !info_string_has_backtick(p);
        }
        true
    })
}

fn consume_partial_quote_prefix(p: &mut MarkdownParser, depth: usize) -> bool {
    let mut consumed = 0usize;
    while consumed < depth && p.at(R_ANGLE) {
        p.parse_as_skipped_trivia_tokens(|p| p.bump(R_ANGLE));
        if p.at(MD_TEXTUAL_LITERAL) {
            let text = p.cur_text();
            if text == " " || text == "\t" {
                p.parse_as_skipped_trivia_tokens(|p| p.bump(MD_TEXTUAL_LITERAL));
            }
        }
        consumed += 1;
    }
    consumed > 0
}

fn consume_partial_quote_prefix_lookahead(
    p: &mut MarkdownParser,
    depth: usize,
    len: &mut usize,
) -> bool {
    let mut consumed = 0usize;
    while consumed < depth && p.at(R_ANGLE) {
        *len += p.cur_text().len();
        p.bump(R_ANGLE);
        if p.at(MD_TEXTUAL_LITERAL) {
            let text = p.cur_text();
            if text == " " || text == "\t" {
                *len += text.len();
                p.bump(MD_TEXTUAL_LITERAL);
            }
        }
        consumed += 1;
    }
    consumed > 0
}

/// Check if we're at a block-level construct that can interrupt a paragraph.
///
/// Per CommonMark, these constructs can interrupt paragraphs:
/// - ATX headings (# followed by space or EOL)
/// - Fenced code blocks (``` or ~~~)
/// - Block quotes (>)
/// - Thematic breaks (---, ***, ___)
/// - List items (-, *, +, or ordered markers) - with restrictions
/// - HTML blocks (certain types)
///
/// Note: Setext headings and indented code blocks do NOT interrupt paragraphs.
/// Also, lines with 4+ spaces of indentation cannot start these constructs
/// (they would be indented code blocks instead).
///
/// ## List Interruption Rules (CommonMark §5.2)
///
/// - Bullet lists can interrupt paragraphs if item has content OR marker is followed by blank line
/// - Ordered lists can interrupt paragraphs ONLY if starting with `1` AND not empty
pub(crate) fn at_block_interrupt(p: &mut MarkdownParser) -> bool {
    // Per CommonMark, lines indented 4+ spaces cannot start block constructs
    // that interrupt paragraphs - they would be indented code blocks.
    // Tabs count as 4 spaces per CommonMark §2.2.
    if p.line_start_leading_indent() >= 4 {
        // Inside list items, allow list markers at the current indent to
        // interrupt paragraphs (nested lists).
        if (p.state().list_nesting_depth > 0 || p.state().list_item_required_indent > 0)
            && (at_bullet_list_item(p) || at_order_list_item(p))
        {
            return true;
        }
        return false;
    }

    // ATX heading: # at line start (must have space/tab after per CommonMark §4.2)
    // Use checkpoint to look ahead and verify it's a valid heading
    if at_header(p) {
        return is_valid_atx_heading_start(p);
    }

    // Fenced code block (``` or ~~~) - lexer already ensures line start
    if at_fenced_code_block(p) {
        return true;
    }

    // Block quote (>)
    if at_quote(p) {
        return true;
    }

    // Thematic break (---, ***, ___) - lexer already ensures line start
    if at_thematic_break_block(p) {
        return true;
    }

    // Bullet list item (-, *, +)
    // Per CommonMark §5.2: bullet lists can interrupt paragraphs only if the
    // item has content (non-empty). Empty markers cannot interrupt paragraphs.
    // When inside a list, we also need to check for list items at ANY indent
    // (not just at the current context's indent) because a less-indented list
    // marker would end the current list item and start a sibling/parent item.
    if at_bullet_list_item(p) || at_bullet_list_item_at_any_indent(p) {
        let in_list = p.state().list_nesting_depth > 0;
        if in_list || can_bullet_interrupt_paragraph(p) {
            return true;
        }
    }

    // Ordered list item (1., 2), etc.)
    // Per CommonMark §5.2: ordered lists can interrupt TOP-LEVEL paragraphs only if:
    //   - Starting with 1 (not 2, 3, etc.), AND
    //   - The item has content (empty ordered items cannot interrupt)
    // Inside a list context, any ordered marker can start a new sibling item.
    if at_order_list_item(p) || at_order_list_item_at_any_indent(p) {
        let in_list = p.state().list_nesting_depth > 0;
        if in_list || (is_ordered_list_starts_with_one(p) && !is_empty_list_item(p)) {
            return true;
        }
    }

    // HTML block (type 7 does not interrupt paragraphs)
    if at_html_block_interrupt(p) {
        return true;
    }

    false
}

/// Check if the current token looks like a list marker when lexed as textual content.
///
/// Inside inline content, list markers like `- ` are lexed as MD_TEXTUAL_LITERAL.
/// This function checks if a textual token at line start looks like it would be
/// a list marker in block context.
fn textual_looks_like_list_marker(p: &mut MarkdownParser) -> bool {
    if !p.at(MD_TEXTUAL_LITERAL) {
        return false;
    }

    let text = p.cur_text();

    // Bullet marker: single -, *, or + followed by space/tab or EOF
    if text == "-" || text == "*" || text == "+" {
        // Check if followed by space, tab, or at end of line
        return p.lookahead(|p| {
            p.bump(MD_TEXTUAL_LITERAL);
            if p.at(T![EOF]) || p.at(NEWLINE) {
                return true;
            }
            if p.at(MD_TEXTUAL_LITERAL) {
                let next = p.cur_text();
                return next.starts_with(' ') || next.starts_with('\t');
            }
            false
        });
    }

    // Ordered marker: per CommonMark §5.1, only ordered lists starting with 1
    // can interrupt paragraphs. Check if text is "1." or "1)" pattern.
    if let Some(rest) = text.strip_prefix('1') {
        // "1." or "1)" followed by space (the space might be in next token)
        if rest == "." || rest == ")" {
            return p.lookahead(|p| {
                p.bump(MD_TEXTUAL_LITERAL);
                if p.at(T![EOF]) || p.at(NEWLINE) {
                    return true;
                }
                if p.at(MD_TEXTUAL_LITERAL) {
                    let next = p.cur_text();
                    return next.starts_with(' ') || next.starts_with('\t');
                }
                false
            });
        }
        // "1. " or "1) " all in one token
        if rest.starts_with(". ")
            || rest.starts_with(".\t")
            || rest.starts_with(") ")
            || rest.starts_with(")\t")
        {
            return true;
        }
    }

    false
}

/// Check if an ordered list marker starts with the number 1.
///
/// Per CommonMark §5.2: "In order to solve of an ambiguity in the spec,
/// only ordered lists starting with 1 can interrupt paragraphs."
///
/// This prevents accidental list creation from wrapped lines like:
/// "The number of windows is 14. The number of doors is 6."
fn is_ordered_list_starts_with_one(p: &mut MarkdownParser) -> bool {
    if !p.at(MD_ORDERED_LIST_MARKER) {
        return false;
    }

    // The marker text includes digits + delimiter (e.g., "1.", "2)", "10.")
    // We want exactly "1." or "1)" - not "10.", "11.", etc.
    let text = p.cur_text();
    text == "1." || text == "1)"
}

/// Check for a bullet list item at any valid top-level indent (0-3 spaces).
///
/// This is used for detecting paragraph interruption when we're inside a nested
/// context (like a list item) but need to detect list markers at any level,
/// not just at the current context's indent level.
fn at_bullet_list_item_at_any_indent(p: &mut MarkdownParser) -> bool {
    p.lookahead(|p| {
        if !p.at_line_start() {
            return false;
        }

        // Top-level list items can have 0-3 spaces of leading indent
        let indent = p.line_start_leading_indent();
        if indent > 3 {
            return false;
        }

        // Skip leading whitespace tokens
        while p.at(MD_TEXTUAL_LITERAL) && p.cur_text().chars().all(|c| c == ' ' || c == '\t') {
            p.bump(MD_TEXTUAL_LITERAL);
        }

        // Check for -, *, or + marker
        if p.at(MD_SETEXT_UNDERLINE_LITERAL) {
            let trimmed = p.cur_text().trim_matches(|c| c == ' ' || c == '\t');
            if trimmed != "-" {
                return false;
            }
        } else if !p.at(T![-]) && !p.at(T![*]) && !p.at(T![+]) {
            return false;
        }

        if p.at(MD_SETEXT_UNDERLINE_LITERAL) {
            p.bump_remap(T![-]);
        } else {
            p.bump(p.cur());
        }
        marker_followed_by_whitespace_or_eol(p)
    })
}

/// Check for an ordered list item at any valid top-level indent (0-3 spaces).
///
/// This is used for detecting paragraph interruption when we're inside a nested
/// context (like a list item) but need to detect list markers at any level.
fn at_order_list_item_at_any_indent(p: &mut MarkdownParser) -> bool {
    p.lookahead(|p| {
        if !p.at_line_start() {
            return false;
        }

        // Top-level list items can have 0-3 spaces of leading indent
        let indent = p.line_start_leading_indent();
        if indent > 3 {
            return false;
        }

        // Skip leading whitespace tokens
        while p.at(MD_TEXTUAL_LITERAL) && p.cur_text().chars().all(|c| c == ' ' || c == '\t') {
            p.bump(MD_TEXTUAL_LITERAL);
        }

        // Check for ordered list marker (lexer produces MD_ORDERED_LIST_MARKER)
        p.at(MD_ORDERED_LIST_MARKER)
    })
}

fn at_order_list_item_textual(p: &mut MarkdownParser) -> bool {
    p.lookahead(|p| {
        if !p.at_line_start() {
            return false;
        }

        let indent = p.line_start_leading_indent();
        let base_indent = if p.state().virtual_line_start == Some(p.cur_range().start())
            && p.state().list_item_required_indent > 0
        {
            0
        } else {
            p.state().list_item_required_indent
        };

        if base_indent == 0 {
            if indent > 3 {
                return false;
            }
        } else if indent < base_indent || indent > base_indent + 3 {
            return false;
        }

        while p.at(MD_TEXTUAL_LITERAL) && p.cur_text().chars().all(|c| c == ' ' || c == '\t') {
            p.bump(MD_TEXTUAL_LITERAL);
        }

        p.at(MD_TEXTUAL_LITERAL) && textual_starts_with_ordered_marker(p.cur_text())
    })
}

/// Check if a bullet list item can interrupt a top-level paragraph.
///
/// Per CommonMark §5.2: "A bullet list can interrupt a paragraph only if
/// it starts with a non-empty item (that is, a list item that contains
/// some non-blank character)."
///
/// This means empty markers (marker followed by only whitespace/newline)
/// cannot interrupt paragraphs, regardless of what follows.
fn can_bullet_interrupt_paragraph(p: &mut MarkdownParser) -> bool {
    let checkpoint = p.checkpoint();

    // Bump the bullet marker (-, *, or +)
    if p.at(T![-]) {
        p.bump(T![-]);
    } else if p.at(T![*]) {
        p.bump(T![*]);
    } else if p.at(T![+]) {
        p.bump(T![+]);
    } else {
        p.rewind(checkpoint);
        return false;
    }

    if !marker_followed_by_whitespace_or_eol(p) {
        p.rewind(checkpoint);
        return false;
    }

    // Check what follows the marker
    // Per CommonMark §5.2: "A bullet list can interrupt a paragraph only if
    // it starts with a non-empty item (that is, a list item that contains
    // some non-blank character)."
    let result = if p.at(T![EOF]) {
        // Empty item at EOF - cannot interrupt
        false
    } else if p.at(NEWLINE) {
        // Empty item (marker + newline) - cannot interrupt paragraphs
        false
    } else if p.at(MD_TEXTUAL_LITERAL) && is_whitespace_only(p.cur_text()) {
        // Skip all whitespace tokens after marker
        while p.at(MD_TEXTUAL_LITERAL) && is_whitespace_only(p.cur_text()) {
            p.bump(MD_TEXTUAL_LITERAL);
        }
        // If only whitespace followed by newline/EOF, item is empty and cannot interrupt
        !(p.at(NEWLINE) || p.at(T![EOF]))
    } else {
        // Has content after marker - can interrupt
        true
    };

    p.rewind(checkpoint);
    result
}

/// Check if the current list item is empty (no content after marker).
///
/// Per CommonMark §5.2: "an empty list item cannot interrupt a paragraph."
///
/// Uses lookahead to check if only whitespace/newline follows the marker.
fn is_empty_list_item(p: &mut MarkdownParser) -> bool {
    let checkpoint = p.checkpoint();

    // Bump the list marker
    if p.at(MD_ORDERED_LIST_MARKER) {
        p.bump(MD_ORDERED_LIST_MARKER);
    } else if p.at(T![-]) {
        p.bump(T![-]);
    } else if p.at(T![*]) {
        p.bump(T![*]);
    } else if p.at(T![+]) {
        p.bump(T![+]);
    } else {
        p.rewind(checkpoint);
        return false;
    }

    if !marker_followed_by_whitespace_or_eol(p) {
        p.rewind(checkpoint);
        return false;
    }

    // Check what follows the marker
    if p.at(MD_TEXTUAL_LITERAL) && is_whitespace_only(p.cur_text()) {
        p.bump(MD_TEXTUAL_LITERAL);
    }

    // Empty if: EOF or NEWLINE after optional whitespace
    let is_empty = p.at(T![EOF]) || p.at(NEWLINE);

    p.rewind(checkpoint);
    is_empty
}

fn is_whitespace_only(text: &str) -> bool {
    !text.is_empty() && text.chars().all(|c| c == ' ' || c == '\t')
}

/// Check if the current position has too many hashes for an ATX heading (>6).
///
/// Returns `Some((range, count))` if there are >6 hashes, `None` otherwise.
/// This is used to emit a diagnostic BEFORE `try_parse` which would lose it on rewind.
fn check_too_many_hashes(p: &mut MarkdownParser) -> Option<(biome_rowan::TextRange, usize)> {
    p.lookahead(|p| {
        p.skip_line_indent(3);

        if !p.at(T![#]) {
            return None;
        }

        // The lexer emits all consecutive `#` as a single HASH token.
        // Get the count from token text length.
        let range = p.cur_range();
        let count = p.cur_text().len();

        if count > 6 {
            Some((range, count))
        } else {
            None
        }
    })
}

/// Check if we're at a valid ATX heading start (1-6 `#` followed by space or EOL).
/// Uses lookahead to verify without consuming tokens.
fn is_valid_atx_heading_start(p: &mut MarkdownParser) -> bool {
    p.lookahead(|p| {
        p.skip_line_indent(3);

        // The lexer emits all consecutive `#` as a single HASH token.
        // Count hash characters from the token's text length.
        if !p.at(T![#]) {
            return false;
        }

        let hash_count = p.cur_text().len();

        // Too many hashes - not a valid heading (must be 1-6)
        if hash_count > 6 {
            return false;
        }

        p.bump(T![#]);

        // Check if followed by space, tab, or EOL/EOF per CommonMark §4.2
        // In Markdown, whitespace is significant and included in token text.
        let text = p.cur_text();
        p.at(T![EOF])
            || p.has_preceding_line_break()
            || text.starts_with(' ')
            || text.starts_with('\t')
    })
}

/// Parse any inline element.
///
/// Dispatches to the appropriate inline parser based on the current token.
pub(crate) fn parse_any_inline(p: &mut MarkdownParser) -> ParsedSyntax {
    inline::parse_any_inline(p)
}

/// Parse a textual inline element.
///
/// Grammar: MdTextual = value: 'md_textual_literal'
///
/// For now, we treat any non-EOF token as textual content to ensure
/// the paragraph parser makes progress. In later prompts, we'll add
/// proper handling for inline elements like emphasis, links, etc.
pub(crate) fn parse_textual(p: &mut MarkdownParser) -> ParsedSyntax {
    if p.at(T![EOF]) {
        return Absent;
    }
    let m = p.start();
    // Remap any token to MD_TEXTUAL_LITERAL so the syntax factory accepts it.
    // This is necessary because tokens like L_PAREN, R_PAREN, etc. are lexed
    // as their specific token kinds, but MdTextual expects MD_TEXTUAL_LITERAL.
    p.bump_remap(MD_TEXTUAL_LITERAL);
    Present(m.complete(p, MD_TEXTUAL))
}

/// Attempt to parse some input with the given parsing function. If parsing
/// succeeds, `Ok` is returned with the result of the parse and the state is
/// preserved. If parsing fails, this function rewinds the parser back to
/// where it was before attempting the parse and the `Err` value is returned.
#[must_use = "The result of try_parse contains information about whether the parse succeeded and should not be ignored"]
pub(crate) fn try_parse<T, E>(
    p: &mut MarkdownParser,
    func: impl FnOnce(&mut MarkdownParser) -> Result<T, E>,
) -> Result<T, E> {
    let checkpoint = p.checkpoint();

    let res = func(p);

    if res.is_err() {
        p.rewind(checkpoint);
    }

    res
}
