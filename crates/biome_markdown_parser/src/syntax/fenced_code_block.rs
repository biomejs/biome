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
//! The opening fence may be followed by an info string. Its first word can be
//! used for syntax highlighting.

use crate::lexer::{MarkdownLexContext, MarkdownReLexContext};
use crate::parser::MarkdownParser;
use crate::syntax::quote::{
    QuotePrefixesEnd, consume_quote_prefix, list_indent_inside_quote, quote_prefixes_end_in_line,
};
use crate::syntax::{MAX_BLOCK_PREFIX_INDENT, TAB_STOP_SPACES, is_whitespace_only};
use crate::token_source::find_line_start;
use biome_markdown_syntax::{T, kind::MarkdownSyntaxKind::*};
use biome_parser::{
    Parser,
    prelude::{
        ParsedSyntax::{self, *},
        TokenSource,
    },
};
use biome_rowan::TextSize;

/// Minimum number of fence characters required per CommonMark §4.5.
const MIN_FENCE_LENGTH: usize = 3;

/// Consume up to `limit` columns of indent whitespace (spaces/tabs) from
/// source bytes.
///
/// `column` is the absolute column of `idx`, which places the tab stops: a tab
/// expands to the next multiple of 4. Indentation is measured from
/// `base_column`, so `column - base_column` columns already count towards
/// `limit`. Returns the new index together with its absolute column, or `None`
/// if `required` is set and the line has less than `limit` columns of indent.
fn consume_indent_from_column(
    source: &[u8],
    mut idx: usize,
    mut column: usize,
    base_column: usize,
    limit: usize,
    required: bool,
) -> Option<(usize, usize)> {
    while column - base_column < limit {
        match source.get(idx).copied() {
            Some(b' ') => {
                column += 1;
                idx += 1;
            }
            Some(b'\t') => {
                let tab_width = TAB_STOP_SPACES - (column % TAB_STOP_SPACES);
                // For optional indent, don't exceed limit with tab
                if !required && column + tab_width - base_column > limit {
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
    Some((idx, column))
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
    if !p.is_at_start_of_input() && !is_line_start_within_indent(p, MAX_BLOCK_PREFIX_INDENT) {
        return false;
    }
    let indent = p.peek_line_indent(MAX_BLOCK_PREFIX_INDENT);
    let Some(rest) = p.source_after_current().get(indent.byte_count..) else {
        return false;
    };
    let Some((fence_char, fence_len)) = detect_fence(rest) else {
        return false;
    };
    fence_char != '`' || !info_string_has_backtick(rest, fence_len)
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
///   r_fence: ('```' | '~~~')?  // absent for an unterminated fence (§4.5)
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

    let list_indent = list_indent_inside_quote(p);
    let mut fence_indent = p.line_start_leading_indent();
    if list_indent > 0 && p.state().virtual_line_start == Some(p.cur_range().start()) {
        fence_indent += list_indent;
    }
    p.emit_line_indent(MAX_BLOCK_PREFIX_INDENT);

    // Detect fence type and length (must close with same type and >= length per CommonMark §4.5)
    let text = p.cur_text();
    let (fence_char, fence_len) = detect_fence(text).unwrap_or(('`', MIN_FENCE_LENGTH));
    let is_tilde_fence = fence_char == '~';
    // Opening fence (``` or ~~~)
    bump_fence(p, is_tilde_fence);

    // Optional language info string (MdCodeNameList)
    parse_code_name_list(p);

    // Content (everything until closing fence)
    parse_code_content(p, is_tilde_fence, fence_len, fence_indent);

    // The closing fence is optional at the end of its containing block.
    let has_closing = at_closing_fence(p, is_tilde_fence, fence_len);

    if has_closing {
        // Closing fence indent: the list continuation indent plus the
        // CommonMark-allowed 0-3 spaces before the fence itself. The
        // closing-fence probe measured it with column-dependent tab stops, so
        // all the whitespace before the fence belongs to it.
        let indent_len = p
            .source_after_current()
            .bytes()
            .take_while(|byte| matches!(byte, b' ' | b'\t'))
            .count();
        p.parse_indent_token_list_bytes(indent_len);
        bump_fence(p, is_tilde_fence);
        // Closing-fence whitespace is structural, even when the regular lexer
        // bundles trailing spaces and a newline into a hard line break.
        p.re_lex(MarkdownReLexContext::CodeInfoString);
        if p.at(MD_TEXTUAL_LITERAL) && is_whitespace_only(p.cur_text()) {
            p.consume_as_whitespace_trivia();
        }
    } else {
        p.emit_line_indent(0);
    }

    Present(m.complete(p, MD_FENCED_CODE_BLOCK))
}

/// Parse the code name list (opening-fence info string).
/// Grammar: MdCodeNameList = MdTextual*
///
/// The info string is limited to the opening fence's physical line.
fn parse_code_name_list(p: &mut MarkdownParser) {
    p.re_lex(MarkdownReLexContext::CodeInfoString);

    let m = p.start();

    skip_info_string_whitespace(p);

    if p.at(NEWLINE) || p.at(T![EOF]) {
        m.complete(p, MD_CODE_NAME_LIST);
        return;
    }

    while !p.at(NEWLINE) && !p.at(T![EOF]) {
        let text_m = p.start();
        bump_info_string_content(p);
        text_m.complete(p, MD_TEXTUAL);

        skip_info_string_whitespace(p);
    }

    m.complete(p, MD_CODE_NAME_LIST);
}

/// Move leading info-string spaces and tabs into whitespace trivia.
///
/// Boundary whitespace stays in trivia to preserve the source while remaining
/// outside `MdCodeNameList`. The next token is lexed in the code-info-string
/// context so trailing spaces cannot consume the physical-line newline as a
/// hard line break.
fn skip_info_string_whitespace(p: &mut MarkdownParser) {
    if !p.at(MD_TEXTUAL_LITERAL) {
        return;
    }

    let text = p.cur_text();
    let whitespace_len = text.len() - text.trim_start_matches([' ', '\t']).len();
    if whitespace_len == 0 {
        return;
    }

    if whitespace_len < text.len() {
        let end = p.cur_range().start() + TextSize::from(whitespace_len as u32);
        p.re_lex_span(end, MD_TEXTUAL_LITERAL);
    }

    p.consume_as_whitespace_trivia_with_context(MarkdownLexContext::CodeInfoString);
}

/// Bump the info string's content, leaving trailing spaces and tabs
/// behind for [`skip_info_string_whitespace`] to move into the trivia list.
///
/// The current token must start with non-whitespace content; leading boundary
/// whitespace is moved into trivia before this helper is called. Advancing in
/// the code-info-string context keeps the physical-line newline separate.
fn bump_info_string_content(p: &mut MarkdownParser) {
    let text = p.cur_text();
    let content_len = text.trim_end_matches([' ', '\t']).len();

    if content_len > 0 && content_len < text.len() {
        let end = p.cur_range().start() + TextSize::from(content_len as u32);
        p.re_lex_span(end, MD_TEXTUAL_LITERAL);
    }

    p.bump_remap_with_context(MD_TEXTUAL_LITERAL, MarkdownLexContext::CodeInfoString);
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

    // Document level: no container prefixes can interleave with the content,
    // so the whole region is stored verbatim in a single MdCodeContent node.
    if quote_depth == 0 && p.state().list_item_required_indent == 0 {
        parse_document_code_content(p, is_tilde_fence, fence_len);
        m.complete(p, MD_INLINE_ITEM_LIST);
        return;
    }

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
                // Code content is literal text: consume the whole rest of
                // the line as one token instead of one token per character
                // group. The closing fence always starts on its own line,
                // so it can never be swallowed.
                if !p.at(NEWLINE) {
                    p.re_lex(MarkdownReLexContext::CodeInfoString);
                }
                bump_code_textual(p);
                at_line_start = false;
            }
        }
    }

    m.complete(p, MD_INLINE_ITEM_LIST);
}

/// Parse document-level fenced code content as one verbatim token.
///
/// The content — everything between the opening-fence line and the closing
/// fence, leading and trailing newline included — becomes a single
/// `MD_CODE_LITERAL` token wrapped in `MdCodeContent`, mirroring how HTML
/// blocks store their content in `md_html_literal`.
fn parse_document_code_content(p: &mut MarkdownParser, is_tilde_fence: bool, fence_len: usize) {
    // We are on the newline right after "```lang"; the content starts here.
    // No newline means the file ends at the opening fence, so there is no
    // content and no literal to emit.
    if !p.at(NEWLINE) {
        return;
    }

    let end = measure_document_code_content_end(p, is_tilde_fence, fence_len);
    p.emit_span_as(end, MD_CODE_LITERAL, MD_CODE_CONTENT);

    // The literal ends right after a newline; mark the line start for the
    // closing-fence indent and fence parsing that follow.
    if !p.at(T![EOF]) {
        p.set_virtual_line_start();
    }
}

/// Scan raw source from the current NEWLINE token to the start of the
/// closing-fence line, or to the end of input when the block is unterminated.
fn measure_document_code_content_end(
    p: &MarkdownParser,
    is_tilde_fence: bool,
    fence_len: usize,
) -> TextSize {
    let Some((start, source)) = get_source_context(p) else {
        return p.cur_range().end();
    };

    let bytes = source.as_bytes();
    let mut idx = start;
    loop {
        // Advance to the end of the current line.
        while idx < bytes.len() && !matches!(bytes[idx], b'\n' | b'\r') {
            idx += 1;
        }
        if idx >= bytes.len() {
            break;
        }
        // Consume the line terminator (\n, \r\n, or \r).
        if bytes[idx] == b'\r' && bytes.get(idx + 1) == Some(&b'\n') {
            idx += 2;
        } else {
            idx += 1;
        }
        if closing_fence_from(source, idx, 0, 0, is_tilde_fence, fence_len, 0) {
            return TextSize::from(idx as u32);
        }
    }

    TextSize::from(bytes.len() as u32)
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
    // A fenced code block cannot be lazily continued (CommonMark §4.5). When a
    // content line de-indents below the enclosing list item's required indent
    // (measured after any blockquote prefixes), it no longer belongs to the
    // item, so the item — and thus this unterminated block — ends here. This is
    // decided *before* consuming the line's quote prefix, so on a de-indent the
    // prefix is handed back to the list/quote parser with the rest of the
    // sibling line instead of being absorbed into the code block.
    if *at_line_start && line_below_list_item_indent(p, quote_depth) {
        return CodeContentTokenAction::Break;
    }

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
        // On a whitespace-only line the indentation runs into the newline,
        // and the line after it starts with its own indentation.
        if consume_code_newline(p) {
            return CodeContentTokenAction::Skip;
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
/// Verifies all `quote_depth` prefixes in the source before consuming any.
/// This prevents partial consumption from stealing outer blockquote markers
/// when an inner prefix is missing
/// (e.g., `> hello` inside a depth-2 blockquote would consume the outer
/// `>` but fail on the missing inner `>`, corrupting outer parsing).
///
/// Returns `true` if all prefixes were consumed successfully, `false` if
/// any prefix is missing — the caller should break out of the content loop.
fn consume_quote_prefixes_in_code_content(p: &mut MarkdownParser, quote_depth: usize) -> bool {
    consume_quote_prefix(p, quote_depth)
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

pub(crate) fn info_string_has_backtick(line: &str, fence_len: usize) -> bool {
    line.get(fence_len..)
        .and_then(|rest| rest.split(['\n', '\r']).next())
        .is_some_and(|info| info.contains('`'))
}

/// Skip the `quote_depth` blockquote prefixes of the content line starting at
/// the current position, each after its quote's container indent plus up to
/// three columns.
///
/// Returns where the last prefix ends, or `None` if fewer than
/// `quote_depth` prefixes are present — in which case the missing-prefix path
/// in [`consume_quote_prefixes_in_code_content`] handles ending the block.
fn skip_code_content_quote_prefixes(
    p: &MarkdownParser,
    quote_depth: usize,
) -> Option<QuotePrefixesEnd> {
    let (start, source) = get_source_context(p)?;
    let line_end = source[start..]
        .find(['\n', '\r'])
        .map_or(source.len(), |offset| start + offset);
    quote_prefixes_end_in_line(p, start, line_end, quote_depth)
}

/// At a content line start, returns `true` when the line's leading indent —
/// measured after any blockquote prefixes — is below the enclosing list item's
/// required indent, i.e. the line belongs to an outer container, not this item.
///
/// The current position is the raw start of the line (set by
/// [`MarkdownParser::set_virtual_line_start`] after the preceding newline), so
/// indentation is measured from there with [`consume_indent_from_column`],
/// starting at the quote's content column so that the part of a tab after `>`
/// that is not the marker's separator counts. CommonMark block indentation is spaces and
/// tabs only (tabs to the next [`TAB_STOP_SPACES`] multiple) — deliberately not
/// `biome_unicode_table`'s `WHS`, which also matches `\n\r\f\v` and would
/// mis-measure indent. Blank lines (only whitespace before the end of line)
/// are exempt: they stay inside the code block.
fn line_below_list_item_indent(p: &MarkdownParser, quote_depth: usize) -> bool {
    let required = list_indent_inside_quote(p);
    if required == 0 {
        return false;
    }

    let Some((_, source)) = get_source_context(p) else {
        return false;
    };

    let bytes = source.as_bytes();
    let Some(prefixes) = skip_code_content_quote_prefixes(p, quote_depth) else {
        return false;
    };
    let QuotePrefixesEnd {
        offset: content_start,
        column,
        content_column,
    } = prefixes;

    // Enough indent to keep continuing the item?
    if consume_indent_from_column(bytes, content_start, column, content_column, required, true)
        .is_some()
    {
        return false;
    }

    // Below the required indent: break unless the line is blank, in which case
    // the remaining (under-required) indent runs straight into the line end.
    let after_indent = consume_indent_from_column(
        bytes,
        content_start,
        column,
        content_column,
        required,
        false,
    )
    .map_or(content_start, |(idx, _)| idx);
    !matches!(bytes.get(after_indent), None | Some(b'\n' | b'\r'))
}

fn at_closing_fence(p: &mut MarkdownParser, is_tilde_fence: bool, fence_len: usize) -> bool {
    p.lookahead(|p| line_has_closing_fence(p, is_tilde_fence, fence_len))
}

fn skip_fenced_content_indent(p: &mut MarkdownParser, indent: usize) {
    if !p.at(MD_TEXTUAL_LITERAL) || !is_whitespace_only(p.cur_text()) {
        return;
    }

    // Indentation arrives as one whitespace token per character; measure the
    // run on the source and consume it as a single MdIndentToken.
    let mut consumed = 0usize;
    let mut len = 0usize;
    for byte in p.source_after_current().bytes() {
        let width = match byte {
            b' ' => 1,
            b'\t' => TAB_STOP_SPACES,
            _ => break,
        };
        if consumed + width > indent {
            break;
        }
        consumed += width;
        len += 1;
    }

    if len > 0 {
        let end = p.cur_range().start() + TextSize::from(len as u32);
        p.emit_span_as(end, MD_INDENT_CHAR, MD_INDENT_TOKEN);
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

    // After the line's quote prefixes, measure the indentation from the
    // quote's content column, so that the part of a tab after `>` that is not
    // the marker's separator counts. The prefix tokens may or may not include
    // that separator.
    let quote_depth = p.state().block_quote_depth;
    let indent_start = if quote_depth > 0 {
        let line_end = source[line_start..]
            .find(['\n', '\r'])
            .map_or(source.len(), |offset| line_start + offset);
        quote_prefixes_end_in_line(
            p,
            find_line_start(&source[..line_start]),
            line_end,
            quote_depth,
        )
        .filter(|prefixes| {
            prefixes.offset == line_start
                || (prefixes.offset == line_start + 1
                    && matches!(source.as_bytes()[line_start], b' ' | b'\t'))
        })
    } else {
        None
    };
    let (line_start, column, base_column) = indent_start.map_or((line_start, 0, 0), |prefixes| {
        (prefixes.offset, prefixes.column, prefixes.content_column)
    });

    let list_indent = list_indent_inside_quote(p);
    closing_fence_from(
        source,
        line_start,
        column,
        base_column,
        is_tilde_fence,
        fence_len,
        list_indent,
    )
}

/// Whether the line starting at `line_start` is a closing fence: optional
/// list continuation indent, up to 3 extra spaces, at least `fence_len`
/// fence characters of the right type, and only whitespace after them.
///
/// `column` is the absolute column of `line_start`, and indentation is
/// measured from `base_column`, as in [`consume_indent_from_column`].
fn closing_fence_from(
    source: &str,
    line_start: usize,
    column: usize,
    base_column: usize,
    is_tilde_fence: bool,
    fence_len: usize,
    list_indent: usize,
) -> bool {
    // Skip required list indent (must have enough whitespace)
    let Some((idx, column)) = consume_indent_from_column(
        source.as_bytes(),
        line_start,
        column,
        base_column,
        list_indent,
        true,
    ) else {
        return false;
    };

    // Skip optional extra indent (up to 3 spaces per CommonMark), counted
    // from the list indent: a tab that ran past it already used some of them.
    // This always succeeds since required=false
    let idx = consume_indent_from_column(
        source.as_bytes(),
        idx,
        column,
        base_column + list_indent,
        MAX_BLOCK_PREFIX_INDENT,
        false,
    )
    .map_or(idx, |(idx, _)| idx);

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

    let list_indent = list_indent_inside_quote(p);
    if p.state().virtual_line_start.is_none() && list_indent > 0 {
        indent = indent.saturating_sub(list_indent);
    }

    indent <= max_indent
}
