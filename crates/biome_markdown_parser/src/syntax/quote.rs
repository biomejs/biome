//! Block quote parsing for Markdown.
//!
//! A block quote begins with `>` at the start of a line and can contain
//! nested block elements.

use biome_markdown_syntax::T;
use biome_markdown_syntax::kind::MarkdownSyntaxKind::*;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax::{self, *};

use crate::MarkdownParser;

/// Check if we're at the start of a block quote (`>`).
pub(crate) fn at_quote(p: &mut MarkdownParser) -> bool {
    p.at(T![>]) && (p.has_preceding_line_break() || p.at_start_of_input())
}

/// Parse a block quote.
///
/// Grammar: MdQuote = AnyMdBlock
///
/// A block quote starts with `>` and contains a block element.
/// We remap the `>` to textual content and include it as part of the paragraph.
pub(crate) fn parse_quote(p: &mut MarkdownParser) -> ParsedSyntax {
    if !at_quote(p) {
        return Absent;
    }

    let m = p.start();

    // Parse the quoted content as a paragraph (AnyMdBlock)
    // The grammar says MdQuote = AnyMdBlock, so we parse a paragraph
    let paragraph_m = p.start();
    let content_m = p.start();

    // Consume the `>` marker as part of the inline content
    // Remap it to MD_TEXTUAL_LITERAL so it becomes part of MdTextual
    let textual_m = p.start();
    p.bump_remap(MD_TEXTUAL_LITERAL);
    textual_m.complete(p, MD_TEXTUAL);

    // Parse the rest of the inline content
    while !p.at(T![EOF]) {
        if super::parse_any_inline(p).is_absent() {
            break;
        }
        // Check for blank line
        let start_trivia_pos = p.trivia_position();
        if p.has_blank_line_since(start_trivia_pos) {
            break;
        }
    }

    content_m.complete(p, MD_INLINE_ITEM_LIST);
    paragraph_m.complete(p, MD_PARAGRAPH);

    Present(m.complete(p, MD_QUOTE))
}
