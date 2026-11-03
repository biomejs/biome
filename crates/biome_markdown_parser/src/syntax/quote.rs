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
use biome_rowan::TextSize;
use std::rc::Rc;

use crate::MarkdownParser;
use crate::parser::OpenQuote;
use crate::syntax::parse_any_block_with_indent_code_policy;
use crate::syntax::parse_error::quote_nesting_too_deep;
use crate::syntax::{
    INDENT_CODE_BLOCK_SPACES, MAX_BLOCK_PREFIX_INDENT, TAB_STOP_SPACES,
    is_dash_only_thematic_break_text, is_paragraph_like, is_whitespace_only, parse_empty_paragraph,
};

/// Check if we're at the start of a block quote (`>`).
pub(crate) fn at_quote(p: &mut MarkdownParser) -> bool {
    let at_virtual_line_start = p.state().virtual_line_start == Some(p.cur_range().start());
    if !p.is_at_line_start() && !at_virtual_line_start {
        return false;
    }
    let mut leading_indent = p.line_start_leading_indent();
    if at_virtual_line_start && leading_indent > 0 {
        leading_indent = 0;
    }
    if leading_indent > MAX_BLOCK_PREFIX_INDENT {
        return false;
    }
    let indent = p.peek_line_indent(MAX_BLOCK_PREFIX_INDENT);
    p.nth_at(indent.token_count, T![>])
}

/// Parse a block quote.
///
/// Grammar: `MdQuote = prefix: MdQuotePrefix content: MdBlockList`
///
/// The first-line `>` is emitted as an `MdQuotePrefix` child of `MdQuote`.
/// Continuation `>` markers become `MdQuotePrefix` nodes interleaved in
/// the block list (between blocks) or inline item list (within paragraphs).
pub(crate) fn parse_quote(p: &mut MarkdownParser) -> ParsedSyntax {
    let list_indent = p.state().list_item_required_indent;
    parse_quote_in_container(p, list_indent)
}

/// Parse a block quote inside a list item whose content indent is
/// `list_indent` columns.
pub(crate) fn parse_quote_in_container(p: &mut MarkdownParser, list_indent: usize) -> ParsedSyntax {
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
        p.consume_line_indent_as_whitespace_trivia(MAX_BLOCK_PREFIX_INDENT);
        try_bump_quote_marker(p);
        let has_indented_code = at_quote_indented_code_start(p);
        emit_optional_marker_space(p, has_indented_code);
        return Present(bogus_m.complete(p, MD_BOGUS_BLOCK));
    }

    let m = p.start();
    p.state_mut().block_quote_depth += 1;
    let prev_open_quote = open_quote(p, list_indent);
    // A `>` marker interrupts a paragraph (CommonMark §5.1), so a pending
    // link-reference-definition continuation from a block before the quote
    // does not extend into the quote's content. Without this reset, an
    // ordered list starting at a number other than 1 (e.g. `> 2. q`) is
    // wrongly demoted to a paragraph by the §5.2 interrupt check. A
    // definition *inside* the quote sets the flag again after this point.
    p.state_mut().link_reference_definition_continuation = false;

    let marker_space = emit_quote_prefix_node(p);
    relex_after_quote_prefix_consumed(p);
    p.set_virtual_line_start();

    parse_quote_block_list(p);

    p.state_mut().open_quote = prev_open_quote;
    p.state_mut().block_quote_depth -= 1;

    let completed = m.complete(p, MD_QUOTE);
    let range = completed.range(p);
    let indent = 1 + if marker_space { 1 } else { 0 };
    p.record_quote_indent(range, indent);
    Present(completed)
}

/// Record the block quote that was just entered as the innermost open quote.
///
/// `list_indent` is the content indent of the list item the quote sits in,
/// which precedes the quote's `>` marker on continuation lines. It applies
/// only when that item opened inside the enclosing quote. `block_quote_depth`
/// must already count the new quote. Returns the previous innermost quote,
/// which the caller restores when the quote ends.
/// Quotes outside lists need no record because their container indent is zero.
pub(crate) fn open_quote(p: &mut MarkdownParser, list_indent: usize) -> Option<Rc<OpenQuote>> {
    if p.state().list_nesting_depth == 0 {
        return None;
    }
    let parent = p.state().open_quote.clone();
    let list_nesting_depth = p.state().list_nesting_depth;
    let in_list_inside_parent = parent
        .as_deref()
        .is_none_or(|parent| parent.list_nesting_depth < list_nesting_depth);
    let quote = OpenQuote {
        depth: p.state().block_quote_depth,
        container_indent: if in_list_inside_parent {
            list_indent
        } else {
            0
        },
        list_nesting_depth,
        parent: parent.clone(),
    };
    p.state_mut().open_quote = Some(Rc::new(quote));
    parent
}

/// The innermost open quote at or above nesting level `depth`.
pub(crate) fn open_quote_at_depth<'p>(
    p: &'p MarkdownParser,
    depth: usize,
) -> &'p Option<Rc<OpenQuote>> {
    let mut quote = &p.state().open_quote;
    while let Some(current) = quote.as_deref()
        && current.depth > depth
    {
        quote = &current.parent;
    }
    quote
}

/// Call `visit` with the container indent of each quote level from the
/// outermost through `depth`, stopping at the first `false`.
///
/// `quote` is the result of [`open_quote_at_depth`] for the same `depth`.
/// Levels without a recorded quote use no container indent.
pub(crate) fn visit_quote_container_indents(
    quote: Option<&OpenQuote>,
    depth: usize,
    mut visit: impl FnMut(usize) -> bool,
) -> bool {
    fn visit_from(quote: &OpenQuote, visit: &mut impl FnMut(usize) -> bool) -> bool {
        let outer_levels_match = match quote.parent.as_deref() {
            Some(parent) => visit_from(parent, visit),
            None => (1..quote.depth).all(|_| visit(0)),
        };
        outer_levels_match && visit(quote.container_indent)
    }

    match quote {
        Some(quote) => visit_from(quote, &mut visit) && (quote.depth..depth).all(|_| visit(0)),
        None => (0..depth).all(|_| visit(0)),
    }
}

/// Content indent of the innermost list item opened inside the innermost open
/// block quote, or 0 when the quote contains no open list item.
///
/// Columns after a quote prefix are measured from the quote content, while
/// `list_item_required_indent` may still describe a list item enclosing the
/// whole quote, whose indent the prefix already consumed.
pub(crate) fn list_indent_inside_quote(p: &MarkdownParser) -> usize {
    match p.state().open_quote.as_deref() {
        Some(quote) if quote.list_nesting_depth == p.state().list_nesting_depth => 0,
        _ => p.state().list_item_required_indent,
    }
}

/// Number of quote levels, from the outermost up to `depth`, whose `>` marker
/// needs no list item indentation before it.
///
/// A line can only continue those levels with bare `>` markers; a quote inside
/// a list item needs the item's indent before its marker.
pub(crate) fn unindented_quote_levels(p: &MarkdownParser, depth: usize) -> usize {
    let quote = open_quote_at_depth(p, depth);
    let mut levels = 0usize;
    visit_quote_container_indents(quote.as_deref(), depth, |container_indent| {
        if container_indent > 0 {
            return false;
        }
        levels += 1;
        true
    });
    levels
}

/// Emit an `MdQuotePrefix` node for a single `>` marker.
///
/// Consumes: [0-3 spaces as pre-marker indent] `>` [optional space as post-marker space]
/// Returns whether a post-marker space was consumed.
fn emit_quote_prefix_node(p: &mut MarkdownParser) -> bool {
    let prefix_m = p.start();
    let Some((marker_space, _)) = emit_quote_prefix_tokens(p, false, 0, None) else {
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
        if dispatched == WHS || matches!(b, b'\n' | b'\r') {
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
/// The indent must cover `container_indent` columns of list item indentation
/// and may add up to three more. It is measured from `content_column`, the
/// content column of the enclosing quote on this line, or from the cursor when
/// `None`. Returns whether a post-marker separator was consumed and the content
/// column of this quote, or `None` when no prefix matches.
pub(crate) fn emit_quote_prefix_tokens(
    p: &mut MarkdownParser,
    use_virtual_line_start: bool,
    container_indent: usize,
    content_column: Option<usize>,
) -> Option<(bool, usize)> {
    let saved_virtual = save_virtual_line_start_if_needed(p, use_virtual_line_start);

    let indent = emit_quote_pre_marker_indents(p, container_indent, content_column);

    restore_virtual_line_start_if_needed(p, saved_virtual);

    let marker_column = p.cached_absolute_column_at(p.cur_range().start().into());
    if indent < container_indent || !try_bump_quote_marker(p) {
        return None;
    }

    let has_indented_code = at_quote_indented_code_start(p);
    let marker_space = emit_post_marker_space(p, has_indented_code);
    // The separator occupies one column even when it is a wider tab (§5.1).
    Some((marker_space, marker_column + 1 + usize::from(marker_space)))
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

/// Emit pre-marker indentation as MD_QUOTE_INDENT nodes: `container_indent`
/// columns of list item indentation plus 0-3 columns of quote indentation.
///
/// Returns the number of columns consumed.
fn emit_quote_pre_marker_indents(
    p: &mut MarkdownParser,
    container_indent: usize,
    content_column: Option<usize>,
) -> usize {
    // Direct bounded scan (0-3 cols per CommonMark §5.1): simpler than ParseNodeList
    // here because we immediately validate `>` and keep this path no-recovery.
    let indent_list_m = p.start();
    let max_indent = container_indent + MAX_BLOCK_PREFIX_INDENT;
    let cursor_column = p.cached_absolute_column_at(p.cur_range().start().into());
    // Columns of a separator tab beyond its first column count as indentation,
    // whether the tab was consumed before the cursor or is still ahead of it.
    let start_column = content_column.unwrap_or(cursor_column);
    let mut consumed = cursor_column.saturating_sub(start_column);

    // Indentation arrives as one whitespace token per character; measure the
    // run on the source and consume it as a single MdQuoteIndent. Tabs advance
    // to the next absolute tab stop.
    if p.at(MD_TEXTUAL_LITERAL) && is_whitespace_only(p.cur_text()) {
        let mut column = cursor_column;
        let mut len = 0usize;
        for byte in p.source_after_current().bytes() {
            let next_column = match byte {
                b' ' => column + 1,
                b'\t' => column + TAB_STOP_SPACES - (column % TAB_STOP_SPACES),
                _ => break,
            };
            if next_column.saturating_sub(start_column) > max_indent {
                break;
            }
            column = next_column;
            len += 1;
        }
        consumed = column.saturating_sub(start_column);

        if len > 0 {
            let end = p.cur_range().start() + TextSize::from(len as u32);
            p.emit_span_as(end, MD_QUOTE_PRE_MARKER_INDENT, MD_QUOTE_INDENT);
        }
    }

    indent_list_m.complete(p, MD_QUOTE_INDENT_LIST);
    consumed
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

    // Inline lexing can fold the separator into following text (`> 1. q` on a
    // list item's first line). Re-lex at line start so it is its own token.
    if p.cur_text().len() > 1 && p.cur_text().starts_with([' ', '\t']) {
        p.force_relex_at_line_start();
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
            if !preserve_tab
                || !quote_tab_has_following_indent(p)
                || quote_tab_starts_nested_prefix(p)
            {
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

        if !self.first_line
            && !p.at(NEWLINE)
            && (p.is_at_line_start() || p.has_preceding_line_break())
        {
            if consume_quote_prefix(p, self.depth) {
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
        (p.is_at_blank_line() || has_empty_line_before(p) || self.last_block_was_paragraph)
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
        relex_after_quote_prefix_consumed(p);

        let parsed = parse_any_block_with_indent_code_policy(p, true);
        let parsed = if let Present(ref marker) = parsed
            && marker.kind(p) == MD_LINK_REFERENCE_DEFINITION
            && quote_link_reference_before_dash_thematic_break(p, self.depth)
        {
            Present(parse_empty_paragraph(p))
        } else {
            parsed
        };

        p.state_mut().virtual_line_start = prev_virtual;

        if let Present(ref marker) = parsed {
            self.last_block_was_paragraph = is_paragraph_like(marker.kind(p));
        } else {
            self.last_block_was_paragraph = false;
        }

        parsed
    }
}

fn quote_link_reference_before_dash_thematic_break(p: &mut MarkdownParser, depth: usize) -> bool {
    if !p.at(NEWLINE) || p.is_at_blank_line() {
        return false;
    }

    p.lookahead(|p| {
        p.bump(NEWLINE);
        if !consume_quote_prefix(p, depth) {
            return false;
        }
        relex_after_quote_prefix_consumed(p);
        p.at(MD_THEMATIC_BREAK_LITERAL) && is_dash_only_thematic_break_text(p.cur_text())
    })
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
    skip_quote_prefixes_in_line(p, line_start, start, depth).is_some()
}

/// Match the `>` prefixes of quote levels 1 through `depth` in the source line
/// starting at `line_start`, without looking at or past `limit`.
///
/// Each prefix may follow its quote's container indent plus up to three
/// columns. Returns the byte offset just past the last prefix, or `None` when
/// the line does not carry all of them.
pub(crate) fn skip_quote_prefixes_in_line(
    p: &MarkdownParser,
    line_start: usize,
    limit: usize,
    depth: usize,
) -> Option<usize> {
    let bytes = p.source().source_text().as_bytes();
    let mut idx = line_start;
    let mut column = 0usize;
    let mut content_column = 0usize;
    let quote = open_quote_at_depth(p, depth);
    visit_quote_container_indents(quote.as_deref(), depth, |container_indent| {
        skip_line_leading_indent(
            bytes,
            &mut idx,
            &mut column,
            content_column,
            limit,
            container_indent,
        ) && try_consume_quote_marker(bytes, &mut idx, &mut column, &mut content_column, limit)
    })
    .then_some(idx)
}

/// Advance `column` past one space or tab, with tabs reaching the next tab stop.
fn advance_column(column: usize, byte: u8) -> usize {
    if byte == b'\t' {
        column + TAB_STOP_SPACES - (column % TAB_STOP_SPACES)
    } else {
        column + 1
    }
}

/// Skip the indentation before a quote marker: `container_indent` columns of
/// list item indentation plus at most MAX_BLOCK_PREFIX_INDENT more.
///
/// `column` is the absolute column of `idx` and advances with it; the
/// indentation is measured from `content_column`. Returns `false` if the
/// indentation is shorter or longer than allowed.
fn skip_line_leading_indent(
    source: &[u8],
    idx: &mut usize,
    column: &mut usize,
    content_column: usize,
    limit: usize,
    container_indent: usize,
) -> bool {
    let start_column = content_column.min(*column);

    while *idx < limit && matches!(source[*idx], b' ' | b'\t') {
        *column = advance_column(*column, source[*idx]);
        *idx += 1;

        if *column - start_column > container_indent + MAX_BLOCK_PREFIX_INDENT {
            return false;
        }
    }

    *column - start_column >= container_indent
}

/// Try to consume a single quote marker (`>`) with optional trailing space/tab.
///
/// Returns `false` if no marker is found.
/// `column` is the absolute column of `idx` and advances with it.
/// `content_column` receives the column where this quote's content starts.
fn try_consume_quote_marker(
    source: &[u8],
    idx: &mut usize,
    column: &mut usize,
    content_column: &mut usize,
    limit: usize,
) -> bool {
    if *idx >= limit || source[*idx] != b'>' {
        return false;
    }

    *idx += 1;
    *column += 1;
    *content_column = *column;

    // Skip optional space or tab after `>`. A tab provides the separator with
    // its first column only (§5.1); the rest counts as indentation.
    if *idx < limit && (source[*idx] == b' ' || source[*idx] == b'\t') {
        *content_column += 1;
        *column = advance_column(*column, source[*idx]);
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
    // Tab expansion is relative to the absolute column position in the line:
    // a tab at column 2 (e.g. just after `> `) only advances 2 columns to the
    // next tab stop, not a full TAB_STOP_SPACES. Counting from column 0 here
    // overstates the indent and misclassifies `> \t- nested` as code.
    let rest = p.source_after_current();
    // Fast path: no tabs means column-relative math is unnecessary.
    if !rest
        .as_bytes()
        .iter()
        .take_while(|b| matches!(b, b' ' | b'\t'))
        .any(|b| *b == b'\t')
    {
        let spaces = rest.as_bytes().iter().take_while(|b| **b == b' ').count();
        return spaces >= INDENT_CODE_BLOCK_SPACES;
    }
    let cur: usize = p.cur_range().start().into();
    let start_col = p.cached_absolute_column_at(cur);
    let mut column = start_col;
    for c in rest.chars() {
        match c {
            ' ' => column += 1,
            '\t' => column += TAB_STOP_SPACES - (column % TAB_STOP_SPACES),
            _ => break,
        }
    }
    column - start_col >= INDENT_CODE_BLOCK_SPACES
}

fn quote_tab_starts_nested_prefix(p: &mut MarkdownParser) -> bool {
    p.lookahead(|p| {
        p.bump(MD_TEXTUAL_LITERAL);
        p.at(T![>]) || (p.at(MD_TEXTUAL_LITERAL) && p.cur_text() == ">")
    })
}

fn quote_tab_has_following_indent(p: &mut MarkdownParser) -> bool {
    p.lookahead(|p| {
        p.bump(MD_TEXTUAL_LITERAL);
        p.source_after_current()
            .chars()
            .next()
            .is_some_and(|c| c == ' ' || c == '\t')
    })
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

    let continues_code_block = p.lookahead(|p| {
        if !consume_quote_prefix(p, depth) {
            return false;
        }

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
        if !preserve_tab || !quote_tab_has_following_indent(p) || quote_tab_starts_nested_prefix(p)
        {
            p.bump_remap(MD_QUOTE_POST_MARKER_SPACE);
        }
        return true;
    }
    false
}

pub(crate) fn has_quote_prefix(p: &mut MarkdownParser, depth: usize) -> bool {
    if depth == 0 || (!p.is_at_line_start() && !p.has_preceding_line_break()) {
        return false;
    }

    if p.state().open_quote.is_none() && has_simple_quote_prefix(p, depth) {
        return true;
    }

    p.lookahead(|p| consume_quote_prefix_impl(p, depth, false))
}

/// Recognizes unindented quote markers separated by at most one space.
/// Other prefix shapes need the full token-consuming lookahead.
fn has_simple_quote_prefix(p: &mut MarkdownParser, depth: usize) -> bool {
    let mut offset = 0;
    for level in 0..depth {
        if !p.nth_at(offset, T![>]) {
            return false;
        }
        offset += 1;
        if level + 1 < depth && p.nth_text(offset) == Some(" ") {
            offset += 1;
        }
    }
    true
}

pub(crate) fn consume_quote_prefix(p: &mut MarkdownParser, depth: usize) -> bool {
    if depth == 0 {
        return false;
    }

    let checkpoint = p.checkpoint();
    if consume_quote_prefix_impl(p, depth, true) {
        true
    } else {
        p.rewind(checkpoint);
        false
    }
}

/// Consume quote prefix tokens without updating virtual line start.
pub(crate) fn consume_quote_prefix_without_virtual(p: &mut MarkdownParser, depth: usize) -> bool {
    if depth == 0 {
        return false;
    }

    let checkpoint = p.checkpoint();
    if consume_quote_prefix_impl(p, depth, false) {
        true
    } else {
        p.rewind(checkpoint);
        false
    }
}

fn consume_quote_prefix_impl(
    p: &mut MarkdownParser,
    depth: usize,
    set_virtual_line_start: bool,
) -> bool {
    if !p.is_at_line_start() && !p.has_preceding_line_break() {
        return false;
    }

    let quote = open_quote_at_depth(p, depth).clone();
    let mut content_column = None;
    let matched = visit_quote_container_indents(quote.as_deref(), depth, |container_indent| {
        let prefix_m = p.start();
        let Some((_, next_content_column)) =
            emit_quote_prefix_tokens(p, true, container_indent, content_column)
        else {
            prefix_m.abandon(p);
            return false;
        };
        content_column = Some(next_content_column);
        prefix_m.complete(p, MD_QUOTE_PREFIX);
        true
    });
    if !matched {
        return false;
    }

    if set_virtual_line_start {
        p.set_virtual_line_start();
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MarkdownParserOptions;

    #[test]
    fn quote_prefix_probe_preserves_cursor() {
        for (source, depth, container_indent, expected) in [
            ("> text", 1, 0, true),
            ("> > text", 2, 0, true),
            (">> text", 2, 0, true),
            ("> > >text", 3, 0, true),
            (">>", 3, 0, false),
            (">  > text", 2, 0, true),
            (">\t> text", 2, 0, true),
            ("> \t> text", 2, 0, true),
            ("   > text", 1, 0, true),
            ("    > text", 1, 0, false),
            ("\t> text", 1, 0, false),
            (">\t \t> text", 2, 0, false),
            ("> 1. item", 1, 0, true),
            (">\n> text", 2, 0, false),
            (">", 1, 0, true),
            ("> text", 1, 2, false),
            ("  > text", 1, 2, true),
            ("      > text", 1, 6, true),
            ("text", 1, 0, false),
            ("  ", 1, 0, false),
            ("", 1, 0, false),
        ] {
            let mut p = MarkdownParser::new(source, MarkdownParserOptions::default());
            p.state_mut().block_quote_depth = 1;
            p.state_mut().list_nesting_depth = usize::from(container_indent > 0);
            open_quote(&mut p, container_indent);
            let range = p.cur_range();
            let kind = p.cur();
            assert_eq!(has_quote_prefix(&mut p, depth), expected, "{source:?}");
            assert_eq!(p.cur_range(), range);
            assert_eq!(p.cur(), kind);
            assert_eq!(p.state().block_quote_depth, 1);
        }
    }

    #[test]
    fn quotes_outside_lists_need_no_container_records() {
        let mut p = MarkdownParser::new("", MarkdownParserOptions::default());
        for depth in 1..=4 {
            p.state_mut().block_quote_depth = depth;
            assert!(open_quote(&mut p, 0).is_none());
            assert!(p.state().open_quote.is_none());
            assert_eq!(unindented_quote_levels(&p, depth), depth);
        }

        p.state_mut().block_quote_depth = 5;
        p.state_mut().list_nesting_depth = 1;
        p.state_mut().list_item_required_indent = 6;
        assert!(open_quote(&mut p, 6).is_none());
        assert_eq!(unindented_quote_levels(&p, 5), 4);
        assert_eq!(list_indent_inside_quote(&p), 0);
        let mut indents = Vec::new();
        let quote = open_quote_at_depth(&p, 5);
        assert!(visit_quote_container_indents(
            quote.as_deref(),
            5,
            |indent| {
                indents.push(indent);
                true
            }
        ));
        assert_eq!(indents, [0, 0, 0, 0, 6]);
    }
}
