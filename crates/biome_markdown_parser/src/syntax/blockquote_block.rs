use biome_markdown_syntax::{T, kind::MarkdownSyntaxKind::*};
use biome_parser::{
    Parser,
    prelude::ParsedSyntax::{self, *},
};

use crate::MarkdownParser;

/// Checks if the current position is at the start of a blockquote
pub(crate) fn at_blockquote(p: &mut MarkdownParser) -> bool {
    // Skip leading whitespace
    let mut i = 0;
    while p.nth_at(i, WHITESPACE) || p.nth_at(i, TAB) {
        i += 1;
    }

    // Check if the next non-whitespace character is a blockquote marker
    p.nth_at(i, R_ANGLE)
}

/// Parses a blockquote block
pub(crate) fn parse_blockquote(p: &mut MarkdownParser) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let m = p.start();

    // Skip leading whitespace
    while p.at(WHITESPACE) || p.at(TAB) {
        p.bump_any();
    }

    // Check if we're at a blockquote marker
    if !p.at(R_ANGLE) {
        p.rewind(checkpoint);
        return Absent;
    }

    // Parse the first line which must start with '>'
    if parse_blockquote_line(p).is_absent() {
        p.rewind(checkpoint);
        return Absent;
    }

    // Parse additional lines that may be part of the blockquote
    // This handles consecutive blockquote lines
    while at_blockquote(p) {
        if parse_blockquote_line(p).is_absent() {
            break;
        }
    }

    Present(m.complete(p, MD_BLOCKQUOTE))
}

/// Parses a single line of a blockquote
fn parse_blockquote_line(p: &mut MarkdownParser) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let m = p.start();

    // Skip leading whitespace
    while p.at(WHITESPACE) || p.at(TAB) {
        p.bump_any();
    }

    // Make sure we're at a blockquote marker
    if !p.at(R_ANGLE) {
        p.rewind(checkpoint);
        return Absent;
    }

    // Consume the '>' character
    p.bump(R_ANGLE);

    // Optionally consume a space after '>'
    if p.at(WHITESPACE) {
        p.bump(WHITESPACE);
    }

    // Parse the content of the line
    let content_marker = p.start();

    // Consume everything until the end of line or end of file
    while !p.at(NEWLINE) && !p.at(T![EOF]) {
        // Special handling for nested blockquotes
        if p.at(R_ANGLE) {
            let nested_checkpoint = p.checkpoint();
            if parse_blockquote_line(p).is_present() {
                continue;
            }
            p.rewind(nested_checkpoint);
        }

        // Create proper textual nodes for the content
        let text_m = p.start();
        p.bump_any();
        text_m.complete(p, MD_TEXTUAL);
    }

    content_marker.complete(p, MD_BLOCKQUOTE_CONTENT);

    // Consume the newline if present
    if p.at(NEWLINE) {
        p.bump(NEWLINE);
    }

    Present(m.complete(p, MD_BLOCKQUOTE_LINE))
}
