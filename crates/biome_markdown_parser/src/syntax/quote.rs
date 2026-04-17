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
//! nesting depth is limited by `MarkdownParserOptions::max_nesting_depth`
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
use crate::syntax::{
    INDENT_CODE_BLOCK_SPACES, MAX_BLOCK_PREFIX_INDENT, TAB_STOP_SPACES, is_paragraph_like,
    is_whitespace_only,
};

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
        if indent > MAX_BLOCK_PREFIX_INDENT {
            return false;
        }
        p.skip_line_indent(MAX_BLOCK_PREFIX_INDENT);
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
        // Wrap recovery tokens in MdBogusBlock (a valid AnyMdBlock child)
        // so they don't attach as Skipped trivia on normal content nodes.
        let bogus_m = p.start();
        p.skip_line_indent(MAX_BLOCK_PREFIX_INDENT);
        try_bump_quote_marker(p);
        let has_indented_code = at_quote_indented_code_start(p);
        emit_optional_marker_space(p, has_indented_code);
        return Present(bogus_m.complete(p, MD_BOGUS_BLOCK));
    }

    let m = p.start();
    p.state_mut().block_quote_depth += 1;

    let marker_space = emit_quote_prefix_node(p);
    relex_after_quote_prefix_consumed(p);
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

/// After consuming a quote prefix, selectively re-lex the current token as if
/// it were at line start when the remaining line could form a thematic break.
///
/// Re-lexing unconditionally perturbs ordinary quoted text tokenization by
/// splitting leading spaces into separate tokens. We only need line-start
/// semantics here for thematic-break candidates like `> ---`.
///
/// A candidate is any line whose non-whitespace bytes are all the **same**
/// thematic break character (`-`, `*`, or `_`). Per CommonMark §4.1, mixing
/// different break characters (e.g. `_*-`) does **not** form a thematic break.
fn force_relex_thematic_break_after_quote_prefix(p: &mut MarkdownParser) {
    let is_thematic_break_candidate = p.at(T![-])
        || p.at(T![*])
        || p.at(UNDERSCORE)
        || p.at(DOUBLE_UNDERSCORE)
        || (p.at(MD_TEXTUAL_LITERAL) && is_thematic_break_candidate_text(p.cur_text()));

    if is_thematic_break_candidate {
        p.force_relex_at_line_start();
    }
}

fn relex_after_quote_prefix_consumed(p: &mut MarkdownParser) {
    force_relex_thematic_break_after_quote_prefix(p);
}

/// Check if `text` could be a thematic break: all non-whitespace bytes must be
/// the **same** thematic break character (`-`, `*`, or `_`).
fn is_thematic_break_candidate_text(text: &str) -> bool {
    use biome_unicode_table::{
        Dispatch::{IDT, MIN, MUL, WHS},
        lookup_byte,
    };

    let mut break_char: Option<u8> = None;
    for &b in text.as_bytes() {
        let dispatched = lookup_byte(b);
        // Skip whitespace (space, tab, etc.) via the shared lookup table.
        if dispatched == WHS {
            continue;
        }
        // Match thematic break characters via dispatch variants:
        // MIN = `-`, MUL = `*`, IDT = `_` (IDT also covers letters, so
        // narrow to `b'_'` explicitly).
        let is_break_char = matches!(dispatched, MIN | MUL) || (dispatched == IDT && b == b'_');
        if is_break_char {
            if let Some(expected) = break_char {
                // Mixed break characters like `_*-` are not valid.
                if b != expected {
                    return false;
                }
            } else {
                break_char = Some(b);
            }
        } else {
            // Any other non-whitespace byte disqualifies the line.
            return false;
        }
    }
    break_char.is_some()
}

/// Emit one quote prefix token sequence: [indent?] `>` [optional space/tab].
///
/// Returns whether a post-marker separator was consumed.
pub(crate) fn emit_quote_prefix_tokens(
    p: &mut MarkdownParser,
    use_virtual_line_start: bool,
) -> Option<bool> {
    let saved_virtual = save_virtual_line_start_if_needed(p, use_virtual_line_start);

    emit_quote_pre_marker_indents(p);

    restore_virtual_line_start_if_needed(p, saved_virtual);

    if !try_bump_quote_marker(p) {
        return None;
    }

    let has_indented_code = at_quote_indented_code_start(p);
    let marker_space = emit_post_marker_space(p, has_indented_code);
    Some(marker_space)
}

/// Save virtual line start position if requested, returning the previous value.
fn save_virtual_line_start_if_needed(
    p: &mut MarkdownParser,
    use_virtual_line_start: bool,
) -> Option<Option<biome_rowan::TextSize>> {
    if use_virtual_line_start {
        let prev = p.state().virtual_line_start;
        p.state_mut().virtual_line_start = Some(p.cur_range().start());
        Some(prev)
    } else {
        None
    }
}

/// Restore virtual line start to saved value.
fn restore_virtual_line_start_if_needed(
    p: &mut MarkdownParser,
    saved_virtual: Option<Option<biome_rowan::TextSize>>,
) {
    if let Some(prev) = saved_virtual {
        p.state_mut().virtual_line_start = prev;
    }
}

/// Emit pre-marker indentation (0-3 spaces/tabs) as MD_QUOTE_INDENT nodes.
fn emit_quote_pre_marker_indents(p: &mut MarkdownParser) {
    // Direct bounded scan (0-3 cols per CommonMark §5.1): simpler than ParseNodeList
    // here because we immediately validate `>` and keep this path no-recovery.
    let indent_list_m = p.start();
    let mut consumed = 0usize;

    while p.at(MD_TEXTUAL_LITERAL) {
        let text = p.cur_text();
        if !is_whitespace_only(text) {
            break;
        }

        let indent = calculate_indent_width(text);
        if consumed + indent > MAX_BLOCK_PREFIX_INDENT {
            break;
        }

        consumed += indent;
        let indent_m = p.start();
        p.bump_remap(MD_QUOTE_PRE_MARKER_INDENT);
        indent_m.complete(p, MD_QUOTE_INDENT);
    }

    indent_list_m.complete(p, MD_QUOTE_INDENT_LIST);
}

/// Calculate indent width accounting for tab expansion (CommonMark §2.2).
fn calculate_indent_width(text: &str) -> usize {
    text.chars()
        .map(|c| if c == '\t' { TAB_STOP_SPACES } else { 1 })
        .sum()
}

/// Try to consume a `>` marker token.
///
/// Returns `true` if a marker was consumed, `false` otherwise.
pub(crate) fn try_bump_quote_marker(p: &mut MarkdownParser) -> bool {
    if p.at(T![>]) {
        p.bump(T![>]);
        true
    } else if p.at(MD_TEXTUAL_LITERAL) && p.cur_text() == ">" {
        p.bump_remap(T![>]);
        true
    } else {
        false
    }
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

    /// Handle line start and quote prefix detection.
    ///
    /// Returns `false` if parsing should stop (no prefix found when required).
    fn handle_line_start_prefix(&mut self, p: &mut MarkdownParser) -> bool {
        self.line_started_with_prefix = self.first_line;

        if !self.first_line && !p.at(NEWLINE) && (p.at_line_start() || p.has_preceding_line_break())
        {
            if has_quote_prefix(p, self.depth) {
                consume_quote_prefix(p, self.depth);
                relex_after_quote_prefix_consumed(p);
                self.line_started_with_prefix = true;
            } else {
                return false;
            }
        }

        self.first_line = false;
        true
    }

    /// Handle newline parsing with lazy continuation checks.
    fn handle_newline(&mut self, p: &mut MarkdownParser) -> ParsedSyntax {
        // Update prefix status if we can see a prefix at current position
        if !self.line_started_with_prefix && line_has_quote_prefix_at_current(p, self.depth) {
            self.line_started_with_prefix = true;
        }

        // Check if lazy continuation should stop
        if self.should_stop_lazy_continuation(p) {
            return Absent;
        }

        // If no prefix on this line, check if next line has one
        if !self.line_started_with_prefix && !self.next_line_has_prefix(p) {
            return Absent;
        }

        let text_m = p.start();
        p.bump(NEWLINE);
        Present(text_m.complete(p, MD_NEWLINE))
    }

    /// Check if lazy continuation should stop.
    fn should_stop_lazy_continuation(&self, p: &MarkdownParser) -> bool {
        (p.at_blank_line() || has_empty_line_before(p) || self.last_block_was_paragraph)
            && !self.line_started_with_prefix
    }

    /// Check if the next line (after newline) has a quote prefix.
    fn next_line_has_prefix(&self, p: &mut MarkdownParser) -> bool {
        p.lookahead(|p| {
            p.bump(NEWLINE);
            has_quote_prefix(p, self.depth)
        })
    }

    /// Parse a block element with virtual line start set.
    fn parse_block_with_virtual_line_start(&mut self, p: &mut MarkdownParser) -> ParsedSyntax {
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

        if !self.handle_line_start_prefix(p) {
            return Absent;
        }

        if p.at(NEWLINE) {
            return self.handle_newline(p);
        }

        if at_quote_indented_code_start(p) {
            let parsed = parse_quote_indented_code_block(p, self.depth);
            self.last_block_was_paragraph = false;
            return parsed;
        }

        self.parse_block_with_virtual_line_start(p)
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

    // Skip leading indentation (0-3 spaces/tabs)
    if !skip_line_leading_indent(source.as_bytes(), &mut idx, start) {
        return false;
    }

    // Check for required number of quote markers
    check_quote_markers_at_position(source.as_bytes(), &mut idx, start, depth)
}

/// Skip leading indentation on a line, respecting MAX_BLOCK_PREFIX_INDENT.
///
/// Returns `false` if indentation exceeds the limit.
fn skip_line_leading_indent(source: &[u8], idx: &mut usize, limit: usize) -> bool {
    let mut indent = 0usize;

    while *idx < limit {
        match source[*idx] {
            b' ' => {
                indent += 1;
                *idx += 1;
            }
            b'\t' => {
                indent += TAB_STOP_SPACES;
                *idx += 1;
            }
            _ => break,
        }

        if indent > MAX_BLOCK_PREFIX_INDENT {
            return false;
        }
    }

    true
}

/// Check if the required number of quote markers (`>`) exist at the current position.
fn check_quote_markers_at_position(
    source: &[u8],
    idx: &mut usize,
    limit: usize,
    depth: usize,
) -> bool {
    for _ in 0..depth {
        if !try_consume_quote_marker(source, idx, limit) {
            return false;
        }
    }
    true
}

/// Try to consume a single quote marker (`>`) with optional trailing space/tab.
///
/// Returns `false` if no marker is found.
fn try_consume_quote_marker(source: &[u8], idx: &mut usize, limit: usize) -> bool {
    if *idx >= limit || source[*idx] != b'>' {
        return false;
    }

    *idx += 1;

    // Skip optional space or tab after `>`
    if *idx < limit && (source[*idx] == b' ' || source[*idx] == b'\t') {
        *idx += 1;
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
            if !parse_code_block_newline(p, depth) {
                break;
            }
        } else {
            parse_code_block_textual(p);
        }
    }

    content.complete(p, MD_INLINE_ITEM_LIST);
    Present(m.complete(p, MD_INDENT_CODE_BLOCK))
}

/// Parse a newline in an indented code block within a quote.
///
/// Returns `false` if the code block should end (no more indented lines).
fn parse_code_block_newline(p: &mut MarkdownParser, depth: usize) -> bool {
    let text_m = p.start();
    p.bump_remap(MD_TEXTUAL_LITERAL);
    text_m.complete(p, MD_TEXTUAL);

    if p.at(T![EOF]) {
        return false;
    }

    if !has_quote_prefix(p, depth) {
        return false;
    }

    let continues_code_block = p.lookahead(|p| {
        consume_quote_prefix(p, depth);

        // Blank lines (consecutive newlines) are allowed in indented code.
        if p.at(NEWLINE) {
            return true;
        }

        at_quote_indented_code_start(p)
    });

    if !continues_code_block {
        return false;
    }

    consume_quote_prefix(p, depth);
    relex_after_quote_prefix_consumed(p);

    // Blank lines (consecutive newlines) are allowed in indented code.
    if p.at(NEWLINE) {
        return true;
    }

    true
}

/// Parse a single textual token in an indented code block.
fn parse_code_block_textual(p: &mut MarkdownParser) {
    let text_m = p.start();
    p.bump_remap(MD_TEXTUAL_LITERAL);
    text_m.complete(p, MD_TEXTUAL);
}

/// Emit the optional space/tab after a `>` quote marker as an explicit CST node.
pub(crate) fn emit_optional_marker_space(p: &mut MarkdownParser, preserve_tab: bool) -> bool {
    if !p.at(MD_TEXTUAL_LITERAL) {
        return false;
    }

    let text = p.cur_text();
    if text == " " {
        p.bump_remap(MD_QUOTE_POST_MARKER_SPACE);
        return true;
    }
    if text == "\t" {
        if !preserve_tab {
            p.bump_remap(MD_QUOTE_POST_MARKER_SPACE);
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
