use biome_markdown_syntax::{kind::MarkdownSyntaxKind::*, T};
use biome_parser::{
    prelude::ParsedSyntax::{self, *},
    Parser,
};

use crate::MarkdownParser;

/// Checks if the current position is at the start of a blockquote
pub(crate) fn at_blockquote(p: &mut MarkdownParser) -> bool {
    p.at(R_ANGLE)
}

/// Parses a blockquote block
pub(crate) fn parse_blockquote(p: &mut MarkdownParser) -> ParsedSyntax {
    let m = p.start();

    // Parse the first line which must start with '>'
    let _ = parse_blockquote_line(p);

    // Parse additional lines that may be part of the blockquote
    while p.at(R_ANGLE) {
        let _ = parse_blockquote_line(p);
    }

    Present(m.complete(p, MD_BLOCKQUOTE))
}

/// Parses a single line of a blockquote
fn parse_blockquote_line(p: &mut MarkdownParser) -> ParsedSyntax {
    let m = p.start();

    // Consume the '>' character
    p.bump(R_ANGLE);

    // Optionally consume a space after '>'
    if p.at(WHITESPACE) {
        p.bump_any();
    }

    // Parse the content of the line
    let content_marker = p.start();

    // Consume everything until the end of line or end of file
    while !p.at(NEWLINE) && !p.at(T![EOF]) {
        p.bump_any();
    }

    content_marker.complete(p, MD_BLOCKQUOTE_CONTENT);

    // Consume the newline if present
    if p.at(NEWLINE) {
        p.bump_any();
    }

    Present(m.complete(p, MD_BLOCKQUOTE_LINE))
}
