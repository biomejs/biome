use crate::MarkdownParser;
use biome_markdown_syntax::MarkdownSyntaxKind::*;
use biome_markdown_syntax::T;
use biome_parser::{
    Parser,
    prelude::ParsedSyntax::{self, *},
};

/// Checks if the current position is at the start of an ATX header (# Header)
pub(crate) fn at_atx_header(p: &mut MarkdownParser) -> bool {
    // Skip leading whitespace
    let mut i = 0;
    while p.nth_at(i, WHITESPACE) || p.nth_at(i, TAB) {
        i += 1;
    }

    // Check if the next non-whitespace character is a hash
    p.nth_at(i, HASH)
}

/// Parses an ATX header (# Header)
pub(crate) fn parse_atx_header(p: &mut MarkdownParser) -> ParsedSyntax {
    let checkpoint = p.checkpoint();

    // Skip leading whitespace
    while p.at(WHITESPACE) || p.at(TAB) {
        p.bump_any();
    }

    if !p.at(HASH) {
        p.rewind(checkpoint);
        return Absent;
    }

    let m = p.start();

    // Parse hash symbols to determine header level
    let hash_list = p.start();
    let mut hash_count = 0;

    while p.at(HASH) && hash_count < 6 {
        p.bump(HASH);
        hash_count += 1;
    }

    if hash_count == 0 {
        // Not a header
        hash_list.abandon(p);
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    // Complete the hash list node
    hash_list.complete(p, MD_HASH_LIST);

    // Require at least one whitespace after hashes for a valid ATX heading
    if !p.at(WHITESPACE) && !p.at(TAB) {
        // Not a valid header, needs whitespace after hashes
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    // Skip whitespace after hashes
    while p.at(WHITESPACE) || p.at(TAB) {
        p.bump_any();
    }

    // Parse header content until newline or EOF
    // Create paragraph item list for the content
    let content_list = p.start();

    // Parse all the text content until the end of the line
    while !p.at(NEWLINE) && !p.at(T![EOF]) {
        // Check for trailing hashes (optional closing sequence)
        if p.at(HASH) {
            let trailing_checkpoint = p.checkpoint();
            
            // Count trailing hashes
            while p.at(HASH) {
                p.bump(HASH);
            }
            
            // If we're at the end of the line after hashes, these are closing hashes
            if p.at(NEWLINE) || p.at(T![EOF]) || (p.at(WHITESPACE) && p.nth_at(1, NEWLINE)) {
                // Skip any whitespace between trailing hashes and newline
                while p.at(WHITESPACE) || p.at(TAB) {
                    p.bump_any();
                }
                break;
            }
            
            // Otherwise, rewind and treat them as normal content
            p.rewind(trailing_checkpoint);
        }
        
        // Parse text content
        let text_m = p.start();
        if p.at(MD_TEXTUAL_LITERAL) {
            p.bump(MD_TEXTUAL_LITERAL);
        } else {
            p.bump_any();
        }
        text_m.complete(p, MD_TEXTUAL);
    }

    // Complete the content list
    content_list.complete(p, MD_PARAGRAPH_ITEM_LIST);

    // Consume the newline if present
    if p.at(NEWLINE) {
        p.bump(NEWLINE);
    }

    // Complete the header node
    Present(m.complete(p, MD_HEADER))
}

/// Checks if the current position is at the start of a Setext header
/// (Header text followed by a line of === or ---)
pub(crate) fn at_setext_header(_p: &mut MarkdownParser) -> bool {
    // Setext headers are difficult to detect in advance
    // because they require looking ahead to the next line
    // This would typically be handled during paragraph parsing
    false
}
