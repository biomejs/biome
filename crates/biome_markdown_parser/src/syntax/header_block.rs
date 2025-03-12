use crate::parser::MarkdownParser;
use biome_markdown_syntax::MarkdownSyntaxKind::*;
use biome_markdown_syntax::T;
use biome_parser::{
    Parser,
    prelude::ParsedSyntax::{self, *},
};

/// Checks if the current position is at the start of a header
pub(crate) fn at_header_block(p: &mut MarkdownParser) -> bool {
    // Skip leading whitespace
    let mut i = 0;
    while p.nth_at(i, WHITESPACE) || p.nth_at(i, TAB) {
        i += 1;
    }

    // Check if the next non-whitespace character is a hash
    p.nth_at(i, HASH)
}

/// Parses an ATX header (# Header)
pub(crate) fn parse_header_block(p: &mut MarkdownParser) -> ParsedSyntax {
    // Save a checkpoint in case this is not actually a header
    let checkpoint = p.checkpoint();

    let m = p.start();

    // Skip leading whitespace
    while p.at(WHITESPACE) || p.at(TAB) {
        p.bump_any();
    }

    // Check if we're at a hash
    if !p.at(HASH) {
        p.rewind(checkpoint);
        return Absent;
    }

    // Parse hash symbols (# to ######)
    let hash_list = p.start();
    let mut hash_count = 0;

    while p.at(HASH) && hash_count < 6 {
        p.bump(HASH);
        hash_count += 1;
    }

    // Validate header format - need whitespace after hash characters
    let is_valid_header = p.at(WHITESPACE) || p.at(TAB);

    // Complete the hash list
    hash_list.complete(p, MD_HASH_LIST);

    if !is_valid_header {
        // Not a valid header, just hash symbols
        // Return Absent so it will be parsed as a paragraph
        p.rewind(checkpoint);
        return Absent;
    }

    // Consume whitespace
    while p.at(WHITESPACE) || p.at(TAB) {
        p.bump_any();
    }

    // Parse header content
    let paragraph = p.start();
    let item_list = p.start();

    // Parse the content until end of line or EOF
    while !p.at(NEWLINE) && !p.at(T![EOF]) {
        p.bump_any();
    }

    item_list.complete(p, MD_PARAGRAPH_ITEM_LIST);
    paragraph.complete(p, MD_PARAGRAPH);

    // Consume the newline if present
    if p.at(NEWLINE) {
        p.bump(NEWLINE);
    }

    // Add an empty MD_HASH_LIST for the trailing hashes (which don't exist in ATX headers)
    let empty_hash_list = p.start();
    empty_hash_list.complete(p, MD_HASH_LIST);

    Present(m.complete(p, MD_HEADER))
}
