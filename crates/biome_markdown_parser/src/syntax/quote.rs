//! Block quote parsing for Markdown (CommonMark §5.1).
//!
//! A block quote begins with `>` at the start of a line and can contain
//! nested block elements. Multiple consecutive `>` lines form a single quote.
//! Nested quotes are created with `>>`, `>>>`, etc.
//!
//! # CommonMark §5.1 Block Quotes
//!
//! A block quote marker consists of 0-3 spaces of indentation, `>`, and an
//! optional space. The contents of the block quote are the result of parsing
//! the remainder of the line (after the `>` and optional space) as blocks.
//!
//! ## Depth Limits
//!
//! To prevent stack overflow from pathological input (e.g., hundreds of `>`),
//! nesting depth is limited by `MarkdownParseOptions::max_nesting_depth`
//! (default: 100). Deeper nesting emits a diagnostic and treats additional
//! `>` as content.
//!
//! ## Lazy Continuation (§5.1)
//!
//! A block quote can contain "lazy continuation lines" — paragraph content
//! that continues without requiring `>` on each line. For example:
//!
//! ```markdown
//! > This is a quote
//! that continues here without >
//! ```
//!
//! Both lines belong to the same block quote. Lazy continuation stops at:
//! - A blank line
//! - A line that starts another block-level construct (header, code, list, etc.)

use biome_markdown_syntax::T;
use biome_markdown_syntax::kind::MarkdownSyntaxKind::{self, *};
use biome_parser::Parser;
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::RecoveryResult;
use biome_parser::prelude::ParsedSyntax::{self, *};

use crate::MarkdownParser;
use crate::syntax::parse_any_block_with_indent_code_policy;
use crate::syntax::parse_error::quote_nesting_too_deep;
use crate::syntax::{INDENT_CODE_BLOCK_SPACES, TAB_STOP_SPACES, is_paragraph_like};

/// Check if we're at the start of a block quote (`>`).
pub(crate) fn at_quote(p: &mut MarkdownParser) -> bool {
    p.lookahead(|p| {
        let at_virtual_line_start = p.state().virtual_line_start == Some(p.cur_range().start());
        if !p.at_line_start() && !p.at_start_of_input() && !at_virtual_line_start {
            return false;
        }
        let mut indent = p.line_start_leading_indent();
        if at_virtual_line_start && indent > 0 {
            // Treat virtual line start as column 0.
            indent = 0;
        }
        if indent > 3 {
            return false;
        }
        p.skip_line_indent(3);
        p.at(T![>])
    })
}

/// Parse a block quote.
///
/// Grammar: `MdQuote = prefix: MdQuotePrefix content: MdBlockList`
///
/// The first-line `>` is emitted as an `MdQuotePrefix` child of `MdQuote`.
/// Continuation `>` markers become `MdQuotePrefix` nodes interleaved in
/// the block list (between blocks) or inline item list (within paragraphs).
pub(crate) fn parse_quote(p: &mut MarkdownParser) -> ParsedSyntax {
    if !at_quote(p) {
        return Absent;
    }

    let max_nesting_depth = p.options().max_nesting_depth;
    if p.state().block_quote_depth >= max_nesting_depth {
        let range = p.cur_range();
        p.error(quote_nesting_too_deep(p, range, max_nesting_depth));
        p.state_mut().quote_depth_exceeded = true;
        p.skip_line_indent(3);
        if p.at(T![>]) {
            p.parse_as_skipped_trivia_tokens(|p| p.bump(T![>]));
        } else if p.at(MD_TEXTUAL_LITERAL) && p.cur_text() == ">" {
            p.parse_as_skipped_trivia_tokens(|p| p.bump_remap(T![>]));
        }
        let has_indented_code = at_quote_indented_code_start(p);
        skip_optional_marker_space(p, has_indented_code);
        return Absent;
    }

    let m = p.start();
    p.state_mut().block_quote_depth += 1;

    let marker_space = emit_quote_prefix_node(p);
    p.set_virtual_line_start();

    parse_quote_block_list(p);

    p.state_mut().block_quote_depth -= 1;

    let completed = m.complete(p, MD_QUOTE);
    let range = completed.range(p);
    let indent = 1 + if marker_space { 1 } else { 0 };
    p.record_quote_indent(range, indent);
    Present(completed)
}

/// Emit an `MdQuotePrefix` node for a single `>` marker.
///
/// Consumes: [0-3 spaces as pre-marker indent] `>` [optional space as post-marker space]
/// Returns whether a post-marker space was consumed.
fn emit_quote_prefix_node(p: &mut MarkdownParser) -> bool {
    let prefix_m = p.start();
    let Some(marker_space) = emit_quote_prefix_tokens(p, false) else {
        prefix_m.abandon(p);
        return false;
    };

    prefix_m.complete(p, MD_QUOTE_PREFIX);
    marker_space
}

/// Emit one quote prefix token sequence: [indent?] `>` [optional space/tab].
///
/// Returns whether a post-marker separator was consumed.
fn emit_quote_prefix_tokens(p: &mut MarkdownParser, use_virtual_line_start: bool) -> Option<bool> {
    // TODO: Emit MD_QUOTE_PRE_MARKER_INDENT directly instead of skipping indent trivia.
    // This is intentionally deferred from Step 1b to keep parser migration scope narrow.
    let saved_virtual = if use_virtual_line_start {
        let prev = p.state().virtual_line_start;
        p.state_mut().virtual_line_start = Some(p.cur_range().start());
        Some(prev)
    } else {
        None
    };
    p.skip_line_indent(3);
    if let Some(prev) = saved_virtual {
        p.state_mut().virtual_line_start = prev;
    }

    if p.at(T![>]) {
        p.bump(T![>]);
    } else if p.at(MD_TEXTUAL_LITERAL) && p.cur_text() == ">" {
        p.bump_remap(T![>]);
    } else {
        return None;
    }

    let has_indented_code = at_quote_indented_code_start(p);
    let marker_space = emit_post_marker_space(p, has_indented_code);
    Some(marker_space)
}

/// Consume the optional space after `>` as an `MD_QUOTE_POST_MARKER_SPACE` token.
///
/// Returns `true` if a space was consumed.
fn emit_post_marker_space(p: &mut MarkdownParser, preserve_tab: bool) -> bool {
    if !p.at(MD_TEXTUAL_LITERAL) {
        return false;
    }

    match p.cur_text() {
        " " => {
            p.bump_remap(MD_QUOTE_POST_MARKER_SPACE);
            true
        }
        "\t" => {
            // When preserve_tab is true (e.g. indented code in quote), the tab still
            // semantically counts as the optional post-marker separator, but remains
            // in the stream so the child block can claim it as indentation.
            if !preserve_tab {
                p.bump_remap(MD_QUOTE_POST_MARKER_SPACE);
            }
            true
        }
        _ => false,
    }
}

struct QuoteBlockList {
    depth: usize,
    first_line: bool,
    last_block_was_paragraph: bool,
    line_started_with_prefix: bool,
}

impl QuoteBlockList {
    fn new(depth: usize) -> Self {
        Self {
            depth,
            first_line: true,
            last_block_was_paragraph: false,
            line_started_with_prefix: true, // First line implicitly has prefix
        }
    }
}

impl ParseNodeList for QuoteBlockList {
    type Kind = MarkdownSyntaxKind;
    type Parser<'source> = MarkdownParser<'source>;

    const LIST_KIND: Self::Kind = MD_BLOCK_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        if p.state().quote_depth_exceeded {
            p.state_mut().quote_depth_exceeded = false;
            return Absent;
        }

        self.line_started_with_prefix = self.first_line;
        if !self.first_line && !p.at(NEWLINE) && (p.at_line_start() || p.has_preceding_line_break())
        {
            if has_quote_prefix(p, self.depth) {
                consume_quote_prefix(p, self.depth);
                self.line_started_with_prefix = true;
            } else {
                return Absent;
            }
        }
        self.first_line = false;

        if p.at(NEWLINE) {
            if !self.line_started_with_prefix && line_has_quote_prefix_at_current(p, self.depth) {
                self.line_started_with_prefix = true;
            }
            if (p.at_blank_line() || has_empty_line_before(p) || self.last_block_was_paragraph)
                && !self.line_started_with_prefix
            {
                return Absent;
            }
            if !self.line_started_with_prefix {
                let has_next_prefix = p.lookahead(|p| {
                    p.bump(NEWLINE);
                    has_quote_prefix(p, self.depth)
                });
                if !has_next_prefix {
                    return Absent;
                }
            }
            let text_m = p.start();
            p.bump(NEWLINE);
            return Present(text_m.complete(p, MD_NEWLINE));
        }

        if at_quote_indented_code_start(p) {
            let parsed = parse_quote_indented_code_block(p, self.depth);
            self.last_block_was_paragraph = false;
            return parsed;
        }

        // Treat content after '>' as column 0 for block parsing (fence detection).
        let prev_virtual = p.state().virtual_line_start;
        p.state_mut().virtual_line_start = Some(p.cur_range().start());
        let parsed = parse_any_block_with_indent_code_policy(p, true);
        p.state_mut().virtual_line_start = prev_virtual;
        if let Present(ref marker) = parsed {
            self.last_block_was_paragraph = is_paragraph_like(marker.kind(p));
        } else {
            self.last_block_was_paragraph = false;
        }

        parsed
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![EOF]) || p.state().quote_depth_exceeded
    }

    fn recover(
        &mut self,
        _p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        // No recovery needed - Absent means we're done
        match parsed_element {
            Present(marker) => RecoveryResult::Ok(marker),
            Absent => RecoveryResult::Err(biome_parser::parse_recovery::RecoveryError::Eof),
        }
    }
}

pub(crate) fn parse_quote_block_list(p: &mut MarkdownParser) {
    let depth = p.state().block_quote_depth;
    let mut list = QuoteBlockList::new(depth);
    list.parse_list(p);
}

pub(crate) fn line_has_quote_prefix_at_current(p: &MarkdownParser, depth: usize) -> bool {
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
                indent += TAB_STOP_SPACES;
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

fn has_empty_line_before(p: &MarkdownParser) -> bool {
    let start: usize = p.cur_range().start().into();
    if start == 0 {
        return false;
    }
    let source = p.source().source_text().as_bytes();
    matches!(source.get(start - 1), Some(b'\n' | b'\r'))
}

pub(crate) fn at_quote_indented_code_start(p: &MarkdownParser) -> bool {
    let mut column = 0usize;

    for c in p.source_after_current().chars() {
        match c {
            ' ' => column += 1,
            '\t' => column += TAB_STOP_SPACES - (column % TAB_STOP_SPACES),
            _ => break,
        }
    }

    column >= INDENT_CODE_BLOCK_SPACES
}

fn parse_quote_indented_code_block(p: &mut MarkdownParser, depth: usize) -> ParsedSyntax {
    let m = p.start();
    let content = p.start();

    loop {
        if p.at(T![EOF]) {
            break;
        }

        if p.at(NEWLINE) {
            let text_m = p.start();
            p.bump_remap(MD_TEXTUAL_LITERAL);
            text_m.complete(p, MD_TEXTUAL);

            if p.at(T![EOF]) {
                break;
            }

            if !has_quote_prefix(p, depth) {
                break;
            }
            consume_quote_prefix(p, depth);

            if p.at(NEWLINE) {
                continue;
            }
            if !at_quote_indented_code_start(p) {
                break;
            }
            continue;
        }

        let text_m = p.start();
        p.bump_remap(MD_TEXTUAL_LITERAL);
        text_m.complete(p, MD_TEXTUAL);
    }

    content.complete(p, MD_INLINE_ITEM_LIST);
    Present(m.complete(p, MD_INDENT_CODE_BLOCK))
}

pub(crate) fn skip_optional_marker_space(p: &mut MarkdownParser, preserve_tab: bool) -> bool {
    if !p.at(MD_TEXTUAL_LITERAL) {
        return false;
    }

    let text = p.cur_text();
    if text == " " {
        p.parse_as_skipped_trivia_tokens(|p| p.bump(MD_TEXTUAL_LITERAL));
        return true;
    }
    if text == "\t" {
        if !preserve_tab {
            p.parse_as_skipped_trivia_tokens(|p| p.bump(MD_TEXTUAL_LITERAL));
        }
        return true;
    }
    false
}

pub(crate) fn has_quote_prefix(p: &mut MarkdownParser, depth: usize) -> bool {
    if depth == 0 {
        return false;
    }

    p.lookahead(|p| consume_quote_prefix_impl(p, depth, false))
}

pub(crate) fn consume_quote_prefix(p: &mut MarkdownParser, depth: usize) -> bool {
    if depth == 0 || !has_quote_prefix(p, depth) {
        return false;
    }

    consume_quote_prefix_impl(p, depth, true)
}

/// Consume quote prefix tokens without updating virtual line start.
pub(crate) fn consume_quote_prefix_without_virtual(p: &mut MarkdownParser, depth: usize) -> bool {
    if depth == 0 || !has_quote_prefix(p, depth) {
        return false;
    }

    consume_quote_prefix_impl(p, depth, false)
}

fn consume_quote_prefix_impl(
    p: &mut MarkdownParser,
    depth: usize,
    set_virtual_line_start: bool,
) -> bool {
    if !p.at_line_start() && !p.at_start_of_input() && !p.has_preceding_line_break() {
        return false;
    }

    for _ in 0..depth {
        let prefix_m = p.start();
        if emit_quote_prefix_tokens(p, true).is_none() {
            prefix_m.abandon(p);
            return false;
        }

        prefix_m.complete(p, MD_QUOTE_PREFIX);
    }

    if set_virtual_line_start {
        p.set_virtual_line_start();
    }

    true
}
