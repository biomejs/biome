use biome_markdown_syntax::{T, kind::MarkdownSyntaxKind::*};
use biome_parser::{
    Parser,
    prelude::ParsedSyntax::{self, *},
};
use crate::syntax::header_block::at_atx_header;
use crate::MarkdownParser;

/// Parses a paragraph block
pub(crate) fn parse_paragraph(p: &mut MarkdownParser) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let m = p.start();

    // Skip leading whitespace
    while p.at(WHITESPACE) || p.at(TAB) {
        p.bump_any();
    }

    // Check if we're at a block element that would prevent paragraph parsing
    if at_atx_header(p) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    // Create a paragraph item list for the content
    let item_list = p.start();
    let mut has_content = false;

    // Parse the first line
    while !p.at(NEWLINE) && !p.at(T![EOF]) {
        let text_m = p.start();
        if p.at(MD_TEXTUAL_LITERAL) {
            p.bump(MD_TEXTUAL_LITERAL);
        } else {
            p.bump_any();
        }
        text_m.complete(p, MD_TEXTUAL);
        has_content = true;
    }

    // If we didn't parse any content, this isn't a paragraph
    if !has_content {
        item_list.abandon(p);
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    // Consume the newline if present
    if p.at(NEWLINE) {
        p.bump(NEWLINE);
    }

    // Complete the paragraph item list
    item_list.complete(p, MD_PARAGRAPH_ITEM_LIST);

    // Complete the paragraph
    Present(m.complete(p, MD_PARAGRAPH))
}
