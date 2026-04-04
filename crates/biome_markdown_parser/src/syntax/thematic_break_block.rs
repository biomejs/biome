//! Thematic break parsing for Markdown (CommonMark §4.1).
//!
//! A thematic break (horizontal rule) is a line consisting of three or more
//! matching `-`, `_`, or `*` characters, optionally with spaces between them.
//!
//! # Examples
//!
//! ```markdown
//! ---
//! ***
//! ___
//! - - -
//! *  *  *
//! ```

use crate::parser::MarkdownParser;
use crate::syntax::MAX_BLOCK_PREFIX_INDENT;
use biome_markdown_syntax::MarkdownSyntaxKind::*;
use biome_markdown_syntax::T;
use biome_parser::{
    Parser,
    prelude::ParsedSyntax::{self, *},
};

/// CommonMark requires 3 or more matching characters for thematic breaks.
const THEMATIC_BREAK_MIN_CHARS: usize = 3;

pub(crate) fn at_thematic_break_block(p: &mut MarkdownParser) -> bool {
    p.lookahead(|p| {
        if p.at_line_start() || p.at_start_of_input() {
            if p.line_start_leading_indent() > MAX_BLOCK_PREFIX_INDENT {
                return false;
            }
            p.skip_line_indent(MAX_BLOCK_PREFIX_INDENT);
            return p.at(MD_THEMATIC_BREAK_LITERAL);
        }

        // Special case: we may not be at line start if a list marker was consumed
        // (e.g., `- * * *` where `-` was consumed as a list marker).
        // Check if the remaining content is a thematic break pattern.
        is_thematic_break_pattern(p)
    })
}

/// Check if the remaining content forms a thematic break pattern.
///
/// Per CommonMark §4.1, a thematic break is 3 or more matching characters
/// (`*`, `-`, or `_`) on a line by itself, optionally with spaces between them.
fn is_thematic_break_pattern(p: &mut MarkdownParser) -> bool {
    // Skip leading whitespace
    while p.at(MD_TEXTUAL_LITERAL) && p.cur_text().chars().all(|c| c == ' ' || c == '\t') {
        p.bump(MD_TEXTUAL_LITERAL);
    }

    // Check for lexer-produced thematic break token
    if p.at(MD_THEMATIC_BREAK_LITERAL) {
        return true;
    }

    // If the entire line segment is a single textual literal, validate it directly.
    if p.at(MD_TEXTUAL_LITERAL)
        && p.cur_text()
            .chars()
            .all(|c| c == ' ' || c == '\t' || c == '*' || c == '-' || c == '_')
    {
        let mut break_char = None;
        let mut break_count = 0usize;

        for c in p.cur_text().chars() {
            if c == ' ' || c == '\t' {
                continue;
            }
            if let Some(existing) = break_char {
                if existing != c {
                    return false;
                }
            } else {
                break_char = Some(c);
            }
            break_count += 1;
        }

        let has_eol = p.lookahead(|p| {
            p.bump(MD_TEXTUAL_LITERAL);
            while p.at(MD_TEXTUAL_LITERAL) && p.cur_text().chars().all(|c| c == ' ' || c == '\t') {
                p.bump(MD_TEXTUAL_LITERAL);
            }
            p.at(NEWLINE) || p.at(T![EOF])
        });

        return break_count >= THEMATIC_BREAK_MIN_CHARS && has_eol;
    }

    // Get the break character from the first non-whitespace token.
    // DOUBLE_STAR / DOUBLE_UNDERSCORE count as 2 of the underlying char.
    let break_char = if p.at(T![*]) || p.at(T![**]) {
        '*'
    } else if p.at(T![-]) {
        '-'
    } else if p.at(UNDERSCORE) || p.at(DOUBLE_UNDERSCORE) {
        '_'
    } else if p.at(MD_TEXTUAL_LITERAL) {
        let text = p.cur_text();
        if text.len() == 1 {
            match text.chars().next() {
                Some('*') => '*',
                Some('-') => '-',
                Some('_') => '_',
                _ => return false,
            }
        } else {
            return false;
        }
    } else {
        return false;
    };

    // Count matching characters.
    // DOUBLE_STAR / DOUBLE_UNDERSCORE contribute 2 to the count.
    let mut count = 0usize;

    loop {
        let (is_break, char_count) = match break_char {
            '*' if p.at(T![**]) => (true, 2),
            '*' if p.at(T![*]) || (p.at(MD_TEXTUAL_LITERAL) && p.cur_text() == "*") => (true, 1),
            '-' if p.at(T![-]) || (p.at(MD_TEXTUAL_LITERAL) && p.cur_text() == "-") => (true, 1),
            '_' if p.at(DOUBLE_UNDERSCORE) => (true, 2),
            '_' if p.at(UNDERSCORE) || (p.at(MD_TEXTUAL_LITERAL) && p.cur_text() == "_") => {
                (true, 1)
            }
            _ => (false, 0),
        };

        if is_break {
            count += char_count;
            p.bump_any();
            continue;
        }

        // Skip whitespace between break characters
        if p.at(MD_TEXTUAL_LITERAL) && p.cur_text().chars().all(|c| c == ' ' || c == '\t') {
            p.bump(MD_TEXTUAL_LITERAL);
            continue;
        }

        // End of line or other content
        break;
    }

    // Valid thematic break if 3+ characters followed by end of line
    count >= THEMATIC_BREAK_MIN_CHARS && (p.at(NEWLINE) || p.at(T![EOF]))
}

pub(crate) fn parse_thematic_break_block(p: &mut MarkdownParser) -> ParsedSyntax {
    if !at_thematic_break_block(p) {
        return Absent;
    }
    let m = p.start();

    parse_thematic_break_parts(p);

    Present(m.complete(p, MD_THEMATIC_BREAK_BLOCK))
}

// #region parse_thematic_break_parts

/// Parse thematic break content into a MdThematicBreakPartList.
///
/// Handles both paths:
/// - Happy path: MD_THEMATIC_BREAK_LITERAL present -> re-lex into parts
/// - Fallback path: individual tokens already available (e.g., after list marker)
fn parse_thematic_break_parts(p: &mut MarkdownParser) {
    let list_m = p.start();

    // Emit block prefix indent (0-3 spaces) as MdIndentToken nodes inside the
    // parts list. MdIndentToken is a valid AnyMdThematicBreakPart variant.
    p.emit_indent_tokens(MAX_BLOCK_PREFIX_INDENT);

    // If lexer produced a single literal, decompose it via re-lex.
    // Track this so all subsequent bumps use parts-mode context.
    // Mutable: fallback MD_TEXTUAL_LITERAL tokens also trigger re-lex (see below).
    let mut relex_active = if p.at(MD_THEMATIC_BREAK_LITERAL) {
        p.force_relex_thematic_break_parts();
        true
    } else {
        false
    };

    // Shared emission loop for both paths.
    // In relex_active mode: tokens are STAR/MINUS/UNDERSCORE/MD_INDENT_CHAR
    //   from the ThematicBreakParts context — use bump_thematic_break_parts().
    // In fallback mode: tokens may be individual punctuation (STAR etc.) or
    //   multi-char MD_TEXTUAL_LITERAL — the latter triggers re-lex on demand.
    loop {
        if p.at(NEWLINE) || p.at(T![EOF]) {
            break;
        }

        // Break character (STAR/MINUS/UNDERSCORE) — from re-lex or regular context
        if p.at(T![*]) || p.at(T![-]) || p.at(UNDERSCORE) {
            let char_m = p.start();
            if relex_active {
                p.bump_thematic_break_parts();
            } else {
                p.bump_any();
            }
            char_m.complete(p, MD_THEMATIC_BREAK_CHAR);
            continue;
        }

        // Whitespace (MD_INDENT_CHAR) — from re-lex or regular context
        if p.at(MD_INDENT_CHAR) {
            let char_m = p.start();
            if relex_active {
                p.bump_thematic_break_parts();
            } else {
                p.bump(MD_INDENT_CHAR);
            }
            char_m.complete(p, MD_INDENT_TOKEN);
            continue;
        }

        // Grouped tokens (DOUBLE_STAR, DOUBLE_UNDERSCORE) or multi-char
        // MD_TEXTUAL_LITERAL — force re-lex to decompose into single-char tokens.
        if p.at(T![**]) || p.at(DOUBLE_UNDERSCORE) {
            p.force_relex_thematic_break_parts();
            relex_active = true;
            continue;
        }

        if p.at(MD_TEXTUAL_LITERAL) {
            let first_char = p.cur_text().as_bytes().first().copied();
            match first_char {
                Some(b'*' | b'-' | b'_' | b' ' | b'\t') => {
                    p.force_relex_thematic_break_parts();
                    relex_active = true;
                    continue;
                }
                _ => break,
            }
        }

        // Unexpected token — shouldn't happen if detection was correct
        break;
    }

    list_m.complete(p, MD_THEMATIC_BREAK_PART_LIST);
}

// #endregion
