use crate::parser::MarkdownParser;
use biome_markdown_syntax::{MarkdownSyntaxKind::*, T};
use biome_parser::{
    Parser,
    prelude::ParsedSyntax::{self, *},
};

pub(crate) fn at_thematic_break_block(p: &mut MarkdownParser) -> bool {
    // Skip any amount of whitespace
    let mut i = 0;
    while p.nth_at(i, WHITESPACE) || p.nth_at(i, TAB) {
        i += 1;
    }

    // Check if the next non-whitespace character is a thematic break marker
    let has_marker = p.nth_at(i, STAR) || p.nth_at(i, T![-]) || p.nth_at(i, UNDERSCORE);

    if !has_marker {
        return false;
    }

    // Determine if we have at least 3 of the same marker (with optional spaces)
    let marker_type = if p.nth_at(i, STAR) {
        STAR
    } else if p.nth_at(i, T![-]) {
        T![-]
    } else {
        UNDERSCORE
    };

    let mut count = 0;
    let mut j = i;

    // Count markers of the same type (allowing spaces between)
    while !p.nth_at(j, NEWLINE) && !p.nth_at(j, T![EOF]) {
        if p.nth_at(j, marker_type) {
            count += 1;
        } else if !p.nth_at(j, WHITESPACE) && !p.nth_at(j, TAB) {
            // Only spaces are allowed between markers
            return false;
        }
        j += 1;
    }

    // Need at least 3 markers to be a valid thematic break
    count >= 3
}

pub(crate) fn parse_thematic_break_block(p: &mut MarkdownParser) -> ParsedSyntax {
    // Save a checkpoint in case this is not actually a thematic break
    let checkpoint = p.checkpoint();

    let m = p.start();

    // Skip leading whitespace
    while p.at(WHITESPACE) || p.at(TAB) {
        p.bump_any();
    }

    // Check if we're at a potential marker
    if !p.at(STAR) && !p.at(T![-]) && !p.at(UNDERSCORE) {
        p.rewind(checkpoint);
        return Absent;
    }

    // Determine the marker type
    let marker = if p.at(STAR) {
        STAR
    } else if p.at(T![-]) {
        T![-]
    } else {
        UNDERSCORE
    };

    // Parse the thematic break
    let mut marker_count = 0;

    // Try to parse a thematic break (allowing spaces between markers)
    while !p.at(T![EOF]) && !p.at(NEWLINE) {
        if p.at(marker) {
            p.bump(marker);
            marker_count += 1;
        } else if p.at(WHITESPACE) || p.at(TAB) {
            // Spaces between markers are allowed
            p.bump_any();
        } else {
            // Invalid character in thematic break
            p.rewind(checkpoint);
            return Absent;
        }
    }

    // Must have at least 3 markers
    if marker_count < 3 {
        p.rewind(checkpoint);
        return Absent;
    }

    // Consume the newline if present
    if p.at(NEWLINE) {
        p.bump(NEWLINE);
    }

    Present(m.complete(p, MD_THEMATIC_BREAK_BLOCK))
}
