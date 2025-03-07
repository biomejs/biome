use crate::parser::MarkdownParser;
use biome_markdown_syntax::MarkdownSyntaxKind::*;
use biome_markdown_syntax::T;
use biome_parser::{
    diagnostic::ParseDiagnostic,
    prelude::ParsedSyntax::{self, *},
    Parser,
};

pub(crate) fn at_header_block(p: &mut MarkdownParser) -> bool {
    p.at(HASH)
}

pub(crate) fn parse_header_block(p: &mut MarkdownParser) -> ParsedSyntax {
    if !at_header_block(p) {
        return Absent;
    }

    let m = p.start();

    // Parse hash symbols (# to ######)
    let hash_list = p.start();
    let mut hash_count = 0;

    while p.at(HASH) && hash_count < 6 {
        p.bump(HASH);
        hash_count += 1;
    }

    // Validate header format - need whitespace after hash characters
    if !p.at(WHITESPACE) && !p.at(TAB) {
        // Not a valid header, just hash symbols
        // This creates a specific diagnostic for the invalid header
        let error_range = p.cur_range();
        let message = "Invalid header format: missing space after '#'";
        let diagnostic = ParseDiagnostic::new(message, error_range);
        p.error(diagnostic);

        // Complete the hash list
        hash_list.complete(p, MD_HASH_LIST);

        // Parse the rest of the line as a paragraph
        let paragraph = p.start();
        let item_list = p.start();

        while !p.at(NEWLINE) && !p.at(T![EOF]) {
            if p.at(MD_TEXTUAL_LITERAL) {
                let text_m = p.start();
                p.bump(MD_TEXTUAL_LITERAL);
                text_m.complete(p, MD_TEXTUAL);
            } else {
                p.bump_any();
            }
        }

        // Consume the newline if present
        p.eat(NEWLINE);

        item_list.complete(p, MD_PARAGRAPH_ITEM_LIST);
        paragraph.complete(p, MD_PARAGRAPH);

        // For test files that expect errors, we want to ensure the diagnostic is emitted
        // but we'll still parse this as a regular paragraph
        return Absent;
    }

    hash_list.complete(p, MD_HASH_LIST);

    // Consume whitespace
    p.eat(WHITESPACE);
    p.eat(TAB);

    // Parse header content as a paragraph
    let paragraph = p.start();
    let item_list = p.start();

    while !p.at(NEWLINE) && !p.at(T![EOF]) {
        if p.at(MD_TEXTUAL_LITERAL) {
            let text_m = p.start();
            p.bump(MD_TEXTUAL_LITERAL);
            text_m.complete(p, MD_TEXTUAL);
        } else {
            p.bump_any();
        }
    }

    item_list.complete(p, MD_PARAGRAPH_ITEM_LIST);
    paragraph.complete(p, MD_PARAGRAPH);

    // Consume the newline if present
    p.eat(NEWLINE);

    // Add an empty MD_HASH_LIST for the trailing hashes (which don't exist in ATX headers)
    let empty_hash_list = p.start();
    empty_hash_list.complete(p, MD_HASH_LIST);

    Present(m.complete(p, MD_HEADER))
}
