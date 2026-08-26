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
    if p.is_at_line_start() || p.is_at_start_of_input() {
        if p.line_start_leading_indent() > MAX_BLOCK_PREFIX_INDENT {
            return false;
        }
        let indent = p.peek_line_indent(MAX_BLOCK_PREFIX_INDENT);
        return p.nth_at(indent.token_count, MD_THEMATIC_BREAK_LITERAL)
            || is_thematic_break_pattern(p, indent.token_count);
    }

    // A list marker may already be consumed for input such as `- * * *`.
    is_thematic_break_pattern(p, 0)
}

/// Check if the remaining content forms a thematic break pattern.
///
/// Per CommonMark §4.1, a thematic break is 3 or more matching characters
/// (`*`, `-`, or `_`) on a line by itself, optionally with spaces between them.
fn is_thematic_break_pattern(p: &mut MarkdownParser, mut n: usize) -> bool {
    while p.nth_at(n, MD_TEXTUAL_LITERAL)
        && p.nth_text(n)
            .is_some_and(|text| !text.as_bytes().iter().any(|b| !matches!(b, b' ' | b'\t')))
    {
        n += 1;
    }

    if p.nth_at(n, MD_THEMATIC_BREAK_LITERAL) {
        return true;
    }

    let textual_break_count = p.nth_text(n).and_then(|text| {
        if text
            .as_bytes()
            .iter()
            .any(|b| !matches!(b, b' ' | b'\t' | b'\n' | b'\r' | b'*' | b'-' | b'_'))
        {
            return None;
        }
        let mut break_char = None;
        let mut break_count = 0usize;
        for &byte in text
            .as_bytes()
            .iter()
            .filter(|byte| !matches!(byte, b' ' | b'\t' | b'\n' | b'\r'))
        {
            if break_char.is_some_and(|existing| existing != byte) {
                return Some(0);
            }
            break_char = Some(byte);
            break_count += 1;
        }
        Some(break_count)
    });
    if p.nth_at(n, MD_TEXTUAL_LITERAL)
        && let Some(break_count) = textual_break_count
    {
        n += 1;
        while p.nth_at(n, MD_TEXTUAL_LITERAL)
            && p.nth_text(n)
                .is_some_and(|text| !text.as_bytes().iter().any(|b| !matches!(b, b' ' | b'\t')))
        {
            n += 1;
        }
        let has_eol = p.nth_at(n, NEWLINE) || p.nth_at(n, T![EOF]);
        return break_count >= THEMATIC_BREAK_MIN_CHARS && has_eol;
    }

    let break_char = if p.nth_at(n, T![*]) || p.nth_at(n, T![**]) {
        b'*'
    } else if p.nth_at(n, T![-]) {
        b'-'
    } else if p.nth_at(n, UNDERSCORE) || p.nth_at(n, DOUBLE_UNDERSCORE) {
        b'_'
    } else if p.nth_at(n, MD_TEXTUAL_LITERAL) {
        let Some(text) = p.nth_text(n) else {
            return false;
        };
        if text.len() == 1 {
            match text.as_bytes().first() {
                Some(b'*') => b'*',
                Some(b'-') => b'-',
                Some(b'_') => b'_',
                _ => return false,
            }
        } else {
            return false;
        }
    } else {
        return false;
    };

    let mut count = 0usize;

    loop {
        let (is_break, char_count) = match break_char {
            b'*' if p.nth_at(n, T![**]) => (true, 2),
            b'*' if p.nth_at(n, T![*])
                || (p.nth_at(n, MD_TEXTUAL_LITERAL) && p.nth_text(n) == Some("*")) =>
            {
                (true, 1)
            }
            b'-' if p.nth_at(n, T![-])
                || (p.nth_at(n, MD_TEXTUAL_LITERAL) && p.nth_text(n) == Some("-")) =>
            {
                (true, 1)
            }
            b'_' if p.nth_at(n, DOUBLE_UNDERSCORE) => (true, 2),
            b'_' if p.nth_at(n, UNDERSCORE)
                || (p.nth_at(n, MD_TEXTUAL_LITERAL) && p.nth_text(n) == Some("_")) =>
            {
                (true, 1)
            }
            _ => (false, 0),
        };

        if is_break {
            count += char_count;
            n += 1;
            continue;
        }

        if p.nth_at(n, MD_TEXTUAL_LITERAL)
            && p.nth_text(n)
                .is_some_and(|text| !text.as_bytes().iter().any(|b| !matches!(b, b' ' | b'\t')))
        {
            n += 1;
            continue;
        }
        break;
    }

    count >= THEMATIC_BREAK_MIN_CHARS
        && (p.nth_at(n, NEWLINE)
            || p.nth_at(n, T![EOF])
            || (p.nth_at(n, MD_TEXTUAL_LITERAL)
                && p.nth_text(n)
                    .is_some_and(|text| matches!(text, "\n" | "\r\n" | "\r"))))
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
