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
use biome_markdown_syntax::{T, kind::MarkdownSyntaxKind::*};
use biome_parser::{
    Parser,
    prelude::{
        ParsedSyntax::{self, *},
        TokenSource,
    },
};

use crate::syntax::parse_error::unterminated_fenced_code;
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

/// Find the start position of the current line in the source text.
///
/// Given a slice of text before the current position, finds the byte offset
/// where the current line begins (after the last newline, handling CRLF).
fn find_line_start(before: &str) -> usize {
    let last_newline_pos = before.rfind(['\n', '\r']);
    match last_newline_pos {
        Some(pos) => {
            let bytes = before.as_bytes();
            // Handle CRLF: if we found \r and next char is \n, skip both
            if bytes.get(pos) == Some(&b'\r') && bytes.get(pos + 1) == Some(&b'\n') {
                pos + 2
            } else {
                pos + 1
            }
        }
        None => 0,
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
///   l_fence: ('```' | '~~~')
///   code_list: MdCodeNameList
///   content: MdInlineItemList
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
    p.skip_line_indent(MAX_BLOCK_PREFIX_INDENT);

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
        if p.state().list_item_required_indent > 0 && p.at_line_start() {
            p.skip_line_indent(p.state().list_item_required_indent);
        }
        p.skip_line_indent(MAX_BLOCK_PREFIX_INDENT);
        bump_fence(p, is_tilde_fence);
    } else {
        // Emit diagnostic for unterminated code block
        p.error(unterminated_fenced_code(p, opening_range, fence_type));
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
        if at_line_start && quote_depth > 0 {
            let prev_virtual = p.state().virtual_line_start;
            p.state_mut().virtual_line_start = Some(p.cur_range().start());
            p.skip_line_indent(MAX_BLOCK_PREFIX_INDENT);
            p.state_mut().virtual_line_start = prev_virtual;

            let mut ok = true;
            for _ in 0..quote_depth {
                if p.at(MD_TEXTUAL_LITERAL) && p.cur_text().starts_with('>') {
                    p.force_relex_regular();
                }

                if p.at(T![>]) {
                    p.parse_as_skipped_trivia_tokens(|p| p.bump(T![>]));
                } else if p.at(MD_TEXTUAL_LITERAL) && p.cur_text() == ">" {
                    p.parse_as_skipped_trivia_tokens(|p| p.bump_remap(T![>]));
                } else {
                    ok = false;
                    break;
                }

                if p.at(MD_TEXTUAL_LITERAL) {
                    let text = p.cur_text();
                    if text == " " || text == "\t" {
                        p.parse_as_skipped_trivia_tokens(|p| p.bump(MD_TEXTUAL_LITERAL));
                    }
                }
            }

            if !ok {
                break;
            }
            at_line_start = false;
        }

        if p.at(NEWLINE) {
            // Preserve newlines as code content and reset virtual line start.
            let text_m = p.start();
            p.bump_remap(MD_TEXTUAL_LITERAL);
            text_m.complete(p, MD_TEXTUAL);
            p.set_virtual_line_start();
            at_line_start = true;
            continue;
        }

        if at_closing_fence(p, is_tilde_fence, fence_len) {
            break;
        }

        if at_line_start && fence_indent > 0 {
            skip_fenced_content_indent(p, fence_indent);
            if at_closing_fence(p, is_tilde_fence, fence_len) {
                break;
            }
        }

        // Consume the token as code content (including NEWLINE tokens)
        let text_m = p.start();
        p.bump_remap(MD_TEXTUAL_LITERAL);
        text_m.complete(p, MD_TEXTUAL);
        at_line_start = false;
    }

    m.complete(p, MD_INLINE_ITEM_LIST);
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
        p.parse_as_skipped_trivia_tokens(|p| p.bump(MD_TEXTUAL_LITERAL));
    }
}

fn line_has_closing_fence(p: &MarkdownParser, is_tilde_fence: bool, fence_len: usize) -> bool {
    let Some((start, source)) = get_source_context(p) else {
        return false;
    };

    let line_start = find_line_start(&source[..start]);

    if !is_whitespace_prefix(source, start, line_start) {
        return false;
    }

    let list_indent = p.state().list_item_required_indent;

    // Skip required list indent (must have enough whitespace)
    let Some(idx) = consume_indent(source.as_bytes(), line_start, list_indent, true) else {
        return false;
    };

    // Skip optional extra indent (up to 3 spaces per CommonMark)
    // This always succeeds since required=false
    let idx = consume_indent(source.as_bytes(), idx, MAX_BLOCK_PREFIX_INDENT, false).unwrap();

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
