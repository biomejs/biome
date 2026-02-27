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

    // Get the break character from the first non-whitespace token
    let break_char = if p.at(T![*]) {
        '*'
    } else if p.at(T![-]) {
        '-'
    } else if p.at(UNDERSCORE) {
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

    // Count matching characters
    let mut count = 0usize;

    loop {
        // Check for the break character
        let is_break = match break_char {
            '*' => p.at(T![*]) || (p.at(MD_TEXTUAL_LITERAL) && p.cur_text() == "*"),
            '-' => p.at(T![-]) || (p.at(MD_TEXTUAL_LITERAL) && p.cur_text() == "-"),
            '_' => p.at(UNDERSCORE) || (p.at(MD_TEXTUAL_LITERAL) && p.cur_text() == "_"),
            _ => false,
        };

        if is_break {
            count += 1;
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

    p.skip_line_indent(MAX_BLOCK_PREFIX_INDENT);

    // If the lexer produced MD_THEMATIC_BREAK_LITERAL, use it directly.
    // Otherwise, parse the thematic break pattern from individual tokens and
    // ensure we emit a literal token (required by the grammar).
    if p.at(MD_THEMATIC_BREAK_LITERAL) {
        p.expect(MD_THEMATIC_BREAK_LITERAL);
    } else {
        parse_thematic_break_tokens(p);
    }

    Present(m.complete(p, MD_THEMATIC_BREAK_BLOCK))
}

/// Parse a thematic break from individual tokens when the lexer didn't produce
/// MD_THEMATIC_BREAK_LITERAL (e.g., after a list marker was consumed).
fn parse_thematic_break_tokens(p: &mut MarkdownParser) {
    // Skip leading whitespace
    while p.at(MD_TEXTUAL_LITERAL) && p.cur_text().chars().all(|c| c == ' ' || c == '\t') {
        p.parse_as_skipped_trivia_tokens(|p| p.bump(MD_TEXTUAL_LITERAL));
    }

    // If the entire thematic break is in a single textual token, remap it.
    if p.at(MD_TEXTUAL_LITERAL)
        && p.cur_text()
            .chars()
            .all(|c| c == ' ' || c == '\t' || c == '*' || c == '-' || c == '_')
    {
        let has_eol = p.lookahead(|p| {
            p.bump(MD_TEXTUAL_LITERAL);
            while p.at(MD_TEXTUAL_LITERAL) && p.cur_text().chars().all(|c| c == ' ' || c == '\t') {
                p.bump(MD_TEXTUAL_LITERAL);
            }
            p.at(NEWLINE) || p.at(T![EOF])
        });
        if !has_eol {
            return;
        }
        p.bump_remap(MD_THEMATIC_BREAK_LITERAL);
        return;
    }

    // Determine the break character for multi-token cases.
    let break_char = if p.at(T![*]) {
        Some('*')
    } else if p.at(T![-]) {
        Some('-')
    } else if p.at(UNDERSCORE) {
        Some('_')
    } else if p.at(MD_TEXTUAL_LITERAL) {
        let text = p.cur_text();
        match text.chars().next() {
            Some('*') => Some('*'),
            Some('-') => Some('-'),
            Some('_') => Some('_'),
            _ => None,
        }
    } else {
        None
    };

    // Emit the required literal token by remapping the first break marker token.
    if break_char.is_some()
        && (p.at(T![*]) || p.at(T![-]) || p.at(UNDERSCORE) || p.at(MD_TEXTUAL_LITERAL))
    {
        p.bump_remap(MD_THEMATIC_BREAK_LITERAL);
    }

    // Parse all break characters and whitespace until end of line
    loop {
        if p.at(NEWLINE) || p.at(T![EOF]) {
            break;
        }

        // Check for the break character
        let is_break = match break_char {
            Some('*') => p.at(T![*]) || (p.at(MD_TEXTUAL_LITERAL) && p.cur_text() == "*"),
            Some('-') => p.at(T![-]) || (p.at(MD_TEXTUAL_LITERAL) && p.cur_text() == "-"),
            Some('_') => p.at(UNDERSCORE) || (p.at(MD_TEXTUAL_LITERAL) && p.cur_text() == "_"),
            _ => false,
        };

        if is_break {
            p.parse_as_skipped_trivia_tokens(|p| p.bump_any());
            continue;
        }

        // Skip whitespace between break characters
        if p.at(MD_TEXTUAL_LITERAL) && p.cur_text().chars().all(|c| c == ' ' || c == '\t') {
            p.parse_as_skipped_trivia_tokens(|p| p.bump(MD_TEXTUAL_LITERAL));
            continue;
        }

        // Other content - shouldn't happen if at_thematic_break_block returned true
        break;
    }
}
