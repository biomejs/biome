//! Fenced code block parsing for Markdown (CommonMark §4.5).
//!
//! A fenced code block begins with a code fence: at least three consecutive
//! backtick (`) or tilde (~) characters. It ends with a closing fence of at
//! least as many characters of the same type, or at the end of the document.
//!
//! # Examples
//!
//! ````markdown
//! ```rust
//! fn main() {}
//! ```
//!
//! ~~~python
//! def hello():
//!     pass
//! ~~~
//! ````
//!
//! # Info String
//!
//! The opening fence may be followed by an info string (language identifier)
//! that can be used for syntax highlighting.

use crate::parser::MarkdownParser;
use crate::token_source::find_line_start;
use biome_markdown_syntax::{T, kind::MarkdownSyntaxKind::*};
use biome_parser::{
    Parser,
    prelude::{
        ParsedSyntax::{self, *},
        TokenSource,
    },
};

use crate::syntax::parse_error::unterminated_fenced_code;
use crate::syntax::quote::try_bump_quote_marker;
use crate::syntax::{MAX_BLOCK_PREFIX_INDENT, TAB_STOP_SPACES};

/// Minimum number of fence characters required per CommonMark §4.5.
const MIN_FENCE_LENGTH: usize = 3;

/// Consume indent whitespace (spaces/tabs) from source bytes.
///
/// Tracks column width where tabs expand to next multiple of 4.
/// Returns `Some(new_idx)` on success, or `None` if required indent wasn't met.
fn consume_indent(source: &[u8], mut idx: usize, limit: usize, required: bool) -> Option<usize> {
    let mut column = 0usize;
    while column < limit {
        match source.get(idx).copied() {
            Some(b' ') => {
                column += 1;
                idx += 1;
            }
            Some(b'\t') => {
                let tab_width = TAB_STOP_SPACES - (column % TAB_STOP_SPACES);
                // For optional indent, don't exceed limit with tab
                if !required && column + tab_width > limit {
                    break;
                }
                column += tab_width;
                idx += 1;
            }
            _ => {
                if required {
                    return None;
                }
                break;
            }
        }
    }
    Some(idx)
}

/// Get parser source context with bounds checking.
///
/// Returns `Some((start_position, source_text))` if the current position is valid,
/// or `None` if the position is out of bounds.
fn get_source_context<'a>(p: &'a MarkdownParser) -> Option<(usize, &'a str)> {
    let start: usize = p.cur_range().start().into();
    let source = p.source().text();
    if start > source.len() {
        return None;
    }
    Some((start, source))
}

/// Check if the prefix from line_start to start is all whitespace.
///
/// Returns `true` if the prefix contains only spaces/tabs.
fn is_whitespace_prefix(source: &str, start: usize, line_start: usize) -> bool {
    source[line_start..start]
        .chars()
        .all(|c| c == ' ' || c == '\t')
}

/// Bump a fence token (``` or ~~~), remapping if necessary.
fn bump_fence(p: &mut MarkdownParser, is_tilde_fence: bool) {
    if is_tilde_fence {
        if p.at(TRIPLE_TILDE) {
            p.bump(TRIPLE_TILDE);
        } else {
            p.bump_remap(TRIPLE_TILDE);
        }
    } else if p.at(T!["```"]) {
        p.bump(T!["```"]);
    } else {
        p.bump_remap(T!["```"]);
    }
}

/// Detect a code fence at the start of a string.
///
/// Per CommonMark §4.5: "A code fence is a sequence of at least three
/// consecutive backtick characters (`) or tildes (~)."
///
/// Returns `Some((fence_char, length))` if a valid fence is found,
/// where `length` is the actual number of fence characters (3 or more).
/// Returns `None` if no valid fence is present.
pub(crate) fn detect_fence(s: &str) -> Option<(char, usize)> {
    let first_char = s.chars().next()?;

    if first_char != '`' && first_char != '~' {
        return None;
    }

    let len = s.chars().take_while(|&c| c == first_char).count();

    if len >= MIN_FENCE_LENGTH {
        Some((first_char, len))
    } else {
        None
    }
}

/// Check if we're at a fenced code block (``` or ~~~).
pub(crate) fn at_fenced_code_block(p: &mut MarkdownParser) -> bool {
    p.lookahead(|p| {
        if !p.at_start_of_input() && !is_line_start_within_indent(p, MAX_BLOCK_PREFIX_INDENT) {
            return false;
        }
        p.skip_line_indent(MAX_BLOCK_PREFIX_INDENT);

        let rest = p.source_after_current();
        let Some((fence_char, _)) = detect_fence(rest) else {
            return false;
        };
        if fence_char == '`' && info_string_has_backtick(p) {
            return false;
        }
        true
    })
}

/// Parse a fenced code block.
///
/// Grammar:
/// MdFencedCodeBlock =
///   indent: MdIndentTokenList
///   l_fence: ('```' | '~~~')
///   code_list: MdCodeNameList
///   content: MdInlineItemList
///   r_fence_indent: MdIndentTokenList
///   r_fence: ('```' | '~~~')
pub(crate) fn parse_fenced_code_block(p: &mut MarkdownParser) -> ParsedSyntax {
    parse_fenced_code_block_impl(p, false)
}

pub(crate) fn parse_fenced_code_block_force(p: &mut MarkdownParser) -> ParsedSyntax {
    parse_fenced_code_block_impl(p, true)
}

fn parse_fenced_code_block_impl(p: &mut MarkdownParser, force: bool) -> ParsedSyntax {
    if !force && !at_fenced_code_block(p) {
        return Absent;
    }

    let m = p.start();

    let mut fence_indent = p.line_start_leading_indent();
    if p.state().list_item_required_indent > 0
        && p.state().virtual_line_start == Some(p.cur_range().start())
    {
        fence_indent += p.state().list_item_required_indent;
    }
    p.emit_line_indent(MAX_BLOCK_PREFIX_INDENT);

    // Detect fence type and length (must close with same type and >= length per CommonMark §4.5)
    let text = p.cur_text();
    let (fence_char, fence_len) = detect_fence(text).unwrap_or(('`', MIN_FENCE_LENGTH));
    let is_tilde_fence = fence_char == '~';
    let fence_type = if is_tilde_fence { "~~~" } else { "```" };

    // Record opening fence range for diagnostic
    let opening_range = p.cur_range();

    // Opening fence (``` or ~~~)
    bump_fence(p, is_tilde_fence);

    // Optional language info string (MdCodeNameList)
    parse_code_name_list(p);

    // Content (everything until closing fence)
    parse_code_content(p, is_tilde_fence, fence_len, fence_indent);

    // Closing fence - emit specific diagnostic if missing
    // Must match the opening fence type
    let has_closing = at_closing_fence(p, is_tilde_fence, fence_len);

    if has_closing {
        // Closing fence indent: strip list continuation indent plus the
        // CommonMark-allowed 0-3 spaces before the fence itself.
        let max = if p.state().list_item_required_indent > 0 {
            p.state().list_item_required_indent + MAX_BLOCK_PREFIX_INDENT
        } else {
            MAX_BLOCK_PREFIX_INDENT
        };
        p.emit_line_indent(max);
        bump_fence(p, is_tilde_fence);
    } else {
        // Emit diagnostic for unterminated code block
        p.error(unterminated_fenced_code(p, opening_range, fence_type));
        // Emit empty r_fence_indent list to satisfy grammar slot
        let empty_m = p.start();
        empty_m.complete(p, MD_INDENT_TOKEN_LIST);
    }

    Present(m.complete(p, MD_FENCED_CODE_BLOCK))
}

/// Parse the code name list (language info string).
/// Grammar: MdCodeNameList = MdTextual*
///
/// The language name is on the same line as the opening fence.
/// If the current token has a preceding line break or is NEWLINE, the code block has no language.
fn parse_code_name_list(p: &mut MarkdownParser) {
    let m = p.start();

    // If the current token is already on a new line, there's no language name
    if p.at_inline_end() {
        m.complete(p, MD_CODE_NAME_LIST);
        return;
    }

    // Parse language identifiers until we hit end of line
    while !p.at_inline_end() {
        // Parse each token as textual content
        let text_m = p.start();
        p.bump_remap(MD_TEXTUAL_LITERAL);
        text_m.complete(p, MD_TEXTUAL);
    }

    m.complete(p, MD_CODE_NAME_LIST);
}

/// Parse the code content until we find a closing fence.
/// Grammar: content: MdInlineItemList
fn parse_code_content(
    p: &mut MarkdownParser,
    is_tilde_fence: bool,
    fence_len: usize,
    fence_indent: usize,
) {
    let m = p.start();
    let quote_depth = p.state().block_quote_depth;
    let mut at_line_start = false;

    // Consume all tokens until we see the matching closing fence or EOF
    while !p.at(T![EOF]) {
        match prepare_next_code_content_token(
            p,
            is_tilde_fence,
            fence_len,
            fence_indent,
            quote_depth,
            &mut at_line_start,
        ) {
            CodeContentTokenAction::Break => break,
            CodeContentTokenAction::Skip => {}
            CodeContentTokenAction::Consume => {
                bump_code_textual(p);
                at_line_start = false;
            }
        }
    }

    m.complete(p, MD_INLINE_ITEM_LIST);
}

enum CodeContentTokenAction {
    Break,
    Skip,
    Consume,
}

/// Prepare the next token inside fenced code block content.
///
/// Handles quote prefix consumption, newlines, closing fence detection,
/// and indent stripping. Returns an action telling the caller how to
/// proceed in the content loop.
fn prepare_next_code_content_token(
    p: &mut MarkdownParser,
    is_tilde_fence: bool,
    fence_len: usize,
    fence_indent: usize,
    quote_depth: usize,
    at_line_start: &mut bool,
) -> CodeContentTokenAction {
    if *at_line_start && quote_depth > 0 && !consume_quote_prefixes_in_code_content(p, quote_depth)
    {
        return CodeContentTokenAction::Break;
    }

    if consume_code_newline(p) {
        *at_line_start = true;
        return CodeContentTokenAction::Skip;
    }

    if at_closing_fence(p, is_tilde_fence, fence_len) {
        return CodeContentTokenAction::Break;
    }

    if *at_line_start && fence_indent > 0 {
        skip_fenced_content_indent(p, fence_indent);
        if at_closing_fence(p, is_tilde_fence, fence_len) {
            return CodeContentTokenAction::Break;
        }
    }

    // Prefix/indent consumption above can advance directly to EOF.
    if p.at(T![EOF]) {
        return CodeContentTokenAction::Break;
    }

    CodeContentTokenAction::Consume
}

/// Consume all expected `>` quote prefixes for the current line inside a
/// fenced code block.
///
/// Uses a lookahead preflight to verify all `quote_depth` prefixes are
/// present before consuming any. This prevents partial consumption from
/// stealing outer blockquote markers when an inner prefix is missing
/// (e.g., `> hello` inside a depth-2 blockquote would consume the outer
/// `>` but fail on the missing inner `>`, corrupting outer parsing).
///
/// Returns `true` if all prefixes were consumed successfully, `false` if
/// any prefix is missing — the caller should break out of the content loop.
fn consume_quote_prefixes_in_code_content(p: &mut MarkdownParser, quote_depth: usize) -> bool {
    // Preflight: verify all prefixes exist before consuming any.
    let all_present = p.lookahead(|p| {
        p.skip_line_indent(MAX_BLOCK_PREFIX_INDENT);
        for _ in 0..quote_depth {
            if p.at(MD_TEXTUAL_LITERAL) && p.cur_text().starts_with('>') {
                p.force_relex_regular();
            }
            if p.at(T![>]) {
                p.bump(T![>]);
            } else if p.at(MD_TEXTUAL_LITERAL) && p.cur_text() == ">" {
                p.bump(MD_TEXTUAL_LITERAL);
            } else {
                return false;
            }
            // Skip optional post-marker space
            if p.at(MD_TEXTUAL_LITERAL) {
                let text = p.cur_text();
                if text == " " || text == "\t" {
                    p.bump(MD_TEXTUAL_LITERAL);
                }
            }
        }
        true
    });

    if !all_present {
        return false;
    }

    let prev_virtual = p.state().virtual_line_start;
    p.state_mut().virtual_line_start = Some(p.cur_range().start());
    p.emit_indent_tokens(MAX_BLOCK_PREFIX_INDENT);
    p.state_mut().virtual_line_start = prev_virtual;

    for _ in 0..quote_depth {
        let consumed = consume_quote_prefix_in_code_content(p);
        debug_assert!(consumed, "preflight verified all prefixes present");
    }

    p.set_virtual_line_start();
    true
}

/// Consume a single `> ` quote prefix (marker + optional trailing space)
/// inside fenced code block content, emitting it as an `MdQuotePrefix` CST node.
///
/// Returns `true` if the prefix was consumed, `false` if the current token
/// is not a quote marker.
fn consume_quote_prefix_in_code_content(p: &mut MarkdownParser) -> bool {
    if p.at(MD_TEXTUAL_LITERAL) && p.cur_text().starts_with('>') {
        p.force_relex_regular();
    }

    if !(p.at(T![>]) || (p.at(MD_TEXTUAL_LITERAL) && p.cur_text() == ">")) {
        return false;
    }

    let prefix_m = p.start();

    // Empty pre-marker indent list (initial indent handled by skip_line_indent).
    let indent_list_m = p.start();
    indent_list_m.complete(p, MD_QUOTE_INDENT_LIST);

    let marker_bumped = try_bump_quote_marker(p);
    debug_assert!(
        marker_bumped,
        "consume_quote_prefix_in_code_content: quote marker not found after guard confirmed `>` \
         token — check that force_relex_regular and the guard condition are in sync"
    );
    if !marker_bumped {
        prefix_m.abandon(p);
        return false;
    }

    // Optional post-marker space
    if p.at(MD_TEXTUAL_LITERAL) {
        let text = p.cur_text();
        if text == " " || text == "\t" {
            p.bump_remap(MD_QUOTE_POST_MARKER_SPACE);
        }
    }

    prefix_m.complete(p, MD_QUOTE_PREFIX);
    true
}

fn consume_code_newline(p: &mut MarkdownParser) -> bool {
    if !p.at(NEWLINE) {
        return false;
    }

    // Preserve newlines as code content and reset virtual line start.
    let text_m = p.start();
    p.bump_remap(MD_TEXTUAL_LITERAL);
    text_m.complete(p, MD_TEXTUAL);
    p.set_virtual_line_start();
    true
}

/// Bump the current token as code textual content (`MdTextual` node).
fn bump_code_textual(p: &mut MarkdownParser) {
    let text_m = p.start();
    p.bump_remap(MD_TEXTUAL_LITERAL);
    text_m.complete(p, MD_TEXTUAL);
}

pub(crate) fn info_string_has_backtick(p: &mut MarkdownParser) -> bool {
    p.lookahead(|p| {
        if p.at(TRIPLE_TILDE) {
            return false;
        }

        if p.at(T!["```"]) {
            p.bump(T!["```"]);
        } else if p.at(BACKTICK) {
            p.bump(BACKTICK);
        } else {
            return false;
        }

        while !p.at_inline_end() {
            if p.at(BACKTICK) || p.at(T!["```"]) {
                return true;
            }
            p.bump(p.cur());
        }

        false
    })
}

fn at_closing_fence(p: &mut MarkdownParser, is_tilde_fence: bool, fence_len: usize) -> bool {
    p.lookahead(|p| line_has_closing_fence(p, is_tilde_fence, fence_len))
}

fn skip_fenced_content_indent(p: &mut MarkdownParser, indent: usize) {
    let mut consumed = 0usize;

    while consumed < indent && p.at(MD_TEXTUAL_LITERAL) {
        let text = p.cur_text();
        if text.is_empty() || !text.chars().all(|c| c == ' ' || c == '\t') {
            break;
        }

        let width = text
            .chars()
            .map(|c| if c == '\t' { TAB_STOP_SPACES } else { 1 })
            .sum::<usize>();

        if consumed + width > indent {
            break;
        }

        consumed += width;
        let char_m = p.start();
        p.bump_remap(MD_INDENT_CHAR);
        char_m.complete(p, MD_INDENT_TOKEN);
    }
}

fn line_has_closing_fence(p: &MarkdownParser, is_tilde_fence: bool, fence_len: usize) -> bool {
    let Some((start, source)) = get_source_context(p) else {
        return false;
    };

    // Use virtual_line_start only when it's on the same line as the
    // current position (e.g., after a blockquote prefix). A stale
    // virtual_line_start from the opening fence line would point to
    // a completely different line and break closing fence detection.
    let line_start: usize = match p.state().virtual_line_start {
        Some(virtual_start) => {
            let vs: usize = virtual_start.into();
            if vs <= start && !source[vs..start].contains(['\n', '\r']) {
                vs
            } else {
                find_line_start(&source[..start])
            }
        }
        None => find_line_start(&source[..start]),
    };

    if line_start != start && !is_whitespace_prefix(source, start, line_start) {
        return false;
    }

    let list_indent = p.state().list_item_required_indent;

    // Skip required list indent (must have enough whitespace)
    let Some(idx) = consume_indent(source.as_bytes(), line_start, list_indent, true) else {
        return false;
    };

    // Skip optional extra indent (up to 3 spaces per CommonMark)
    // This always succeeds since required=false
    let idx = consume_indent(source.as_bytes(), idx, MAX_BLOCK_PREFIX_INDENT, false).unwrap_or(idx);

    let fence_char = if is_tilde_fence { b'~' } else { b'`' };
    let mut fence_count = 0usize;
    while source.as_bytes().get(idx + fence_count) == Some(&fence_char) {
        fence_count += 1;
    }
    if fence_count < fence_len {
        return false;
    }

    let after_fence = &source[idx + fence_count..];
    let line_rest = after_fence
        .split_terminator(['\n', '\r'])
        .next()
        .unwrap_or("");

    line_rest.chars().all(|c| c == ' ' || c == '\t')
}

fn is_line_start_within_indent(p: &MarkdownParser, max_indent: usize) -> bool {
    if p.state().virtual_line_start == Some(p.cur_range().start()) {
        return true;
    }

    let Some((start, source)) = get_source_context(p) else {
        return false;
    };

    let virtual_start: usize = match p.state().virtual_line_start {
        Some(virtual_start) => virtual_start.into(),
        None => find_line_start(&source[..start]),
    };

    if !is_whitespace_prefix(source, start, virtual_start) {
        return false;
    }

    let mut indent = source[virtual_start..start]
        .chars()
        .fold(0usize, |count, c| {
            count + if c == '\t' { TAB_STOP_SPACES } else { 1 }
        });

    if p.state().virtual_line_start.is_none() && p.state().list_item_required_indent > 0 {
        indent = indent.saturating_sub(p.state().list_item_required_indent);
    }

    indent <= max_indent
}
