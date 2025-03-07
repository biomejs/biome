use crate::parser::MarkdownParser;
use biome_markdown_syntax::{MarkdownSyntaxKind::*, T};
use biome_parser::{
    prelude::ParsedSyntax::{self, *},
    Parser,
};

pub fn at_thematic_break_block(p: &mut MarkdownParser) -> bool {
    // A thematic break starts with *, -, or _
    p.at(STAR) || p.at(MINUS) || p.at(UNDERSCORE)
}

pub fn parse_thematic_break_block(p: &mut MarkdownParser) -> ParsedSyntax {
    if !at_thematic_break_block(p) {
        return Absent;
    }

    // Save a checkpoint in case this is not actually a thematic break
    let checkpoint = p.checkpoint();

    let m = p.start();

    // Determine the marker type
    let marker = if p.at(STAR) {
        STAR
    } else if p.at(MINUS) {
        MINUS
    } else {
        UNDERSCORE
    };

    // Parse the thematic break
    let mut count = 0;

    // Try to parse a thematic break
    while !p.at(T![EOF]) && !p.at(NEWLINE) {
        if p.at(marker) {
            p.bump(marker);
            count += 1;
        } else if p.at(WHITESPACE) || p.at(TAB) {
            p.bump_any();
        } else {
            // Invalid character in thematic break
            p.rewind(checkpoint);
            return Absent;
        }
    }

    // Must have at least 3 markers
    if count < 3 {
        p.rewind(checkpoint);
        return Absent;
    }

    // Consume the newline if present
    if p.at(NEWLINE) {
        p.bump(NEWLINE);
    }

    Present(m.complete(p, MD_THEMATIC_BREAK_BLOCK))
}
