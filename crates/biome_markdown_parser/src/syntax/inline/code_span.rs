use biome_markdown_syntax::T;
use biome_markdown_syntax::kind::MarkdownSyntaxKind::*;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax::{self, *};
use biome_rowan::TextSize;

use crate::MarkdownParser;
use crate::lexer::MarkdownLexContext;
use crate::syntax::{TAB_STOP_SPACES, at_block_interrupt, at_setext_underline_after_newline};

/// Parse a hard line break.
///
/// Grammar: MdHardLine = value: 'md_hard_line_literal'
///
/// A hard line break is created by either:
/// - Two or more trailing spaces followed by a newline
/// - A backslash followed by a newline
pub(crate) fn parse_hard_line(p: &mut MarkdownParser) -> ParsedSyntax {
    if !p.at(MD_HARD_LINE_LITERAL) {
        return Absent;
    }

    let m = p.start();
    p.bump(MD_HARD_LINE_LITERAL);
    Present(m.complete(p, MD_HARD_LINE))
}

/// Returns the position of a matching closing backtick sequence before EOF/blank line.
///
/// Per CommonMark §6.1, a code span opener must have a matching closer with the
/// same number of backticks. If no match exists, the opener should be treated
/// as literal text, not an unclosed code span.
///
/// Returns `None` if no match is found; the opener is literal text.
fn matching_code_span_closer(p: &mut MarkdownParser, opening_count: usize) -> Option<TextSize> {
    p.lookahead(|p| {
        // Backslashes are literal from the first content byte, including before a closing tick.
        p.bump_with_context(p.cur(), MarkdownLexContext::CodeSpan);

        loop {
            // EOF = no matching closer found
            if p.at(T![EOF]) {
                return None;
            }

            // Blank line = paragraph boundary, terminates search
            if p.at(NEWLINE) && p.is_at_blank_line() {
                return None;
            }

            // Per CommonMark §4.3, setext heading underlines take priority over
            // inline code spans. If crossing a newline would land on a setext
            // underline, the code span is invalid — the underline forms a heading.
            if p.at(NEWLINE) {
                p.bump_remap_with_context(MD_TEXTUAL_LITERAL, MarkdownLexContext::CodeSpan);
                if at_setext_underline_after_newline(p).is_some() {
                    return None;
                }
                // Per CommonMark, block interrupts (including list markers) can
                // terminate paragraphs. A code span cannot cross a block boundary.
                // The list check uses regular lexing, so its token changes must be rewound.
                if at_block_interrupt(p) || p.lookahead(is_at_list_marker_after_newline) {
                    return None;
                }
                continue;
            }

            // Found backticks - check if they match (handle both BACKTICK and TRIPLE_BACKTICK)
            if p.at(BACKTICK) || p.at(T!["```"]) {
                let closing_count = p.cur_text().len();
                if closing_count == opening_count {
                    return Some(p.cur_range().start());
                }
                // Not matching - continue searching
                p.bump_with_context(p.cur(), MarkdownLexContext::CodeSpan);
                continue;
            }

            // Consume token and continue (use CodeSpan context for proper backslash handling)
            p.bump_remap_with_context(MD_TEXTUAL_LITERAL, MarkdownLexContext::CodeSpan);
        }
    })
}

/// Consumes a complete code span during lookahead, leaving unmatched backticks untouched.
pub(super) fn skip_code_span_in_lookahead(p: &mut MarkdownParser) -> bool {
    if !p.at(BACKTICK) && !p.at(T!["```"]) {
        return false;
    }
    let Some(close) = matching_code_span_closer(p, p.cur_text().len()) else {
        return false;
    };
    p.bump_with_context(p.cur(), MarkdownLexContext::CodeSpan);
    while !p.at(T![EOF]) && p.cur_range().start() < close {
        p.bump_remap_with_context(MD_TEXTUAL_LITERAL, MarkdownLexContext::CodeSpan);
    }
    p.bump_any();
    true
}

/// Check if we're at a list marker after a newline.
/// This is used to detect when a code span would cross a list item boundary.
fn is_at_list_marker_after_newline(p: &mut MarkdownParser) -> bool {
    // List markers can be indented up to 3 spaces; 4+ means indented code block.
    const LIST_MARKER_MAX_INDENT: usize = 4;

    // Skip up to 3 spaces of indent (list markers can be indented 0-3 spaces)
    let mut columns = 0usize;
    while columns < LIST_MARKER_MAX_INDENT
        && p.at(MD_TEXTUAL_LITERAL)
        && p.cur_text().chars().all(|c| c == ' ' || c == '\t')
    {
        for c in p.cur_text().chars() {
            match c {
                ' ' => columns += 1,
                '\t' => columns += TAB_STOP_SPACES - (columns % TAB_STOP_SPACES),
                _ => {}
            }
        }
        if columns >= LIST_MARKER_MAX_INDENT {
            return false; // Indented code block, not a list marker
        }
        p.bump(MD_TEXTUAL_LITERAL);
    }

    // Check for bullet list markers: -, *, +
    if p.at(T![-]) || p.at(T![*]) || p.at(T![+]) {
        let marker_text = p.cur_text();
        // Only a single -, *, or + is a list marker; longer runs are not.
        if marker_text.len() == 1 {
            p.bump_any();
            return is_list_marker_followed_by_space_or_eol(p);
        }
        return false;
    }

    // Check for ordered list marker: digits followed by . or )
    if p.at(MD_ORDERED_LIST_MARKER) {
        p.bump(MD_ORDERED_LIST_MARKER);
        return is_list_marker_followed_by_space_or_eol(p);
    }

    // Check for textual bullet markers (lexed as MD_TEXTUAL_LITERAL in some contexts)
    if p.at(MD_TEXTUAL_LITERAL) {
        let text = p.cur_text();
        if text == "-" || text == "*" || text == "+" {
            p.bump(MD_TEXTUAL_LITERAL);
            return is_list_marker_followed_by_space_or_eol(p);
        }
    }

    false
}

/// A list marker must be followed by space, tab, or end of line/input.
fn is_list_marker_followed_by_space_or_eol(p: &MarkdownParser) -> bool {
    if p.at(NEWLINE) || p.at(T![EOF]) {
        return true;
    }
    if p.at(MD_TEXTUAL_LITERAL) {
        let text = p.cur_text();
        return text.starts_with(' ') || text.starts_with('\t');
    }
    false
}

/// Parse inline code span (`` `code` `` or ``` `` `code` `` ```).
///
/// Grammar: MdInlineCode = l_tick: '`' content: MdInlineItemList r_tick: '`'
///
/// Per CommonMark §6.1:
/// - Code spans can use multiple backticks to allow literal backticks inside
/// - The opening and closing backtick strings must be the same length
/// - Backslash escapes are NOT processed inside code spans (\` is literal `\``)
/// - If no matching closer exists, the opener is treated as literal text
pub(crate) fn parse_inline_code(p: &mut MarkdownParser) -> ParsedSyntax {
    // Handle both BACKTICK and TRIPLE_BACKTICK (T!["```"] ) as code span openers.
    // TRIPLE_BACKTICK can appear when backticks are at line start but info string
    // contains backticks, making it not a fenced code block (CommonMark examples 138, 145).
    let is_backtick = p.at(BACKTICK);
    let is_triple_backtick = p.at(T!["```"]);
    if !is_backtick && !is_triple_backtick {
        return Absent;
    }

    let opening_count = p.cur_text().len();

    // DESIGN PRINCIPLE #2 & #4: Check for matching closer BEFORE creating any nodes.
    // If no match exists, return Absent so backticks become literal text.
    // This avoids synthesizing MD_INLINE_CODE with missing r_tick_token.
    if matching_code_span_closer(p, opening_count).is_none() {
        return Absent; // Caller will treat backtick as literal MD_TEXTUAL
    }

    // We have a valid code span - now parse it
    let m = p.start();

    // Opening backtick(s) - remap TRIPLE_BACKTICK to BACKTICK for consistency
    p.bump_remap_with_context(BACKTICK, MarkdownLexContext::CodeSpan);

    // Content - parse until we find matching closing backticks
    // Per CommonMark, code spans can span multiple lines (newlines become spaces in output)
    // All content is lexed in CodeSpan context to keep backslash literal and avoid
    // hard-line-break detection.
    let content = p.start();
    loop {
        // EOF should not happen (lookahead guaranteed a closer), but handle defensively
        if p.at(T![EOF]) {
            break;
        }

        // DESIGN PRINCIPLE #3: Terminate on blank line (paragraph boundary)
        if p.at(NEWLINE) {
            if p.is_at_blank_line() {
                break; // Paragraph boundary - stop
            }
            // Soft line break - consume NEWLINE as content and continue
            // Use CodeSpan context so next token is also lexed without escape processing
            let text_m = p.start();
            p.bump_remap_with_context(MD_TEXTUAL_LITERAL, MarkdownLexContext::CodeSpan);
            text_m.complete(p, MD_TEXTUAL);
            continue;
        }

        // Found matching closing backticks (handle both BACKTICK and TRIPLE_BACKTICK)
        if (p.at(BACKTICK) || p.at(T!["```"])) && p.cur_text().len() == opening_count {
            break;
        }

        let end = code_span_text_end(p, opening_count);
        let text_m = p.start();
        p.re_lex_span(end, MD_TEXTUAL_LITERAL);
        p.bump_remap_with_context(MD_TEXTUAL_LITERAL, MarkdownLexContext::CodeSpan);
        text_m.complete(p, MD_TEXTUAL);
    }
    content.complete(p, MD_INLINE_ITEM_LIST);

    // Closing backticks (guaranteed to exist due to lookahead check)
    // Remap TRIPLE_BACKTICK to BACKTICK for consistency
    if p.at(T!["```"]) {
        p.bump_remap(BACKTICK);
    } else {
        p.bump(BACKTICK);
    }

    Present(m.complete(p, MD_INLINE_CODE))
}

fn code_span_text_end(p: &MarkdownParser, opening_count: usize) -> TextSize {
    let start = p.cur_range().start();
    let source = p.source_after_current();
    let bytes = source.as_bytes();
    let mut offset = 0;

    while offset < bytes.len() {
        match bytes[offset] {
            b'\n' | b'\r' => break,
            b'`' => {
                let run_start = offset;
                while offset < bytes.len() && bytes[offset] == b'`' {
                    offset += 1;
                }
                if offset - run_start == opening_count {
                    return start + TextSize::from(run_start as u32);
                }
            }
            _ => offset += 1,
        }
    }

    start + TextSize::from(offset as u32)
}
