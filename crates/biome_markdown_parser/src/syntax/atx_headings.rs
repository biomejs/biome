use crate::parser::MarkdownParser;
use biome_markdown_syntax::MarkdownSyntaxKind::*;
use biome_markdown_syntax::T;
use biome_parser::{
    prelude::ParsedSyntax::{self, *},
    Parser,
    prelude::TokenSource,
};

pub(crate) fn at_atx_heading(p: &mut MarkdownParser) -> bool {
    // ATX headings start with 1-6 hash characters
    if !p.at(T![#]) {
        return false;
    }

    // Count consecutive hash characters (max 6)
    let mut hash_count = 0;
    while p.nth(hash_count) == T![#] && hash_count < 6 {
        hash_count += 1;
    }

    // Must be followed by whitespace or EOL to be a valid heading
    let next = p.nth(hash_count);
    next == WHITESPACE || next == NEWLINE || next == T![EOF]
}

pub(crate) fn parse_atx_heading(p: &mut MarkdownParser) -> ParsedSyntax {
    let m = p.start();

    // Parse opening hash marks
    let hash_list_m = p.start();
    let mut hash_count = 0;
    while p.at(T![#]) && hash_count < 6 {
        p.bump(T![#]);
        hash_count += 1;
    }
    hash_list_m.complete(p, MD_HASH_LIST);

    // Skip whitespace after the hash marks
    if p.at(WHITESPACE) {
        p.bump(WHITESPACE);
    }

    // Parse heading content as a paragraph (optional)
    if !p.at(NEWLINE) && !p.at(T![EOF]) {
        let paragraph_m = p.start();

        // Parse until end of line, or until trailing hashes
        while !p.at(NEWLINE) && !p.at(T![EOF]) && !p.at(T![#]) {
            p.bump(p.source().current());
        }

        paragraph_m.complete(p, MD_PARAGRAPH);
    }

    // Parse trailing hash marks (optional)
    let trailing_hash_list_m = p.start();
    while p.at(T![#]) {
        p.bump(T![#]);
    }
    trailing_hash_list_m.complete(p, MD_HASH_LIST);

    // Skip trailing whitespace
    if p.at(WHITESPACE) {
        p.bump(WHITESPACE);
    }

    // Consume the newline if present
    if p.at(NEWLINE) {
        p.bump(NEWLINE);
    }

    // Complete with the appropriate heading node type based on hash count
    let node = match hash_count {
        1 => MD_HEADER1,
        2 => MD_HEADER2,
        3 => MD_HEADER3,
        4 => MD_HEADER4,
        5 => MD_HEADER5,
        6 => MD_HEADER6,
        _ => MD_HEADER // Fallback, should not happen
    };

    Present(m.complete(p, node))
}
