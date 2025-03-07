use crate::parser::MarkdownParser;
use biome_markdown_syntax::MarkdownSyntaxKind::*;
use biome_markdown_syntax::T;
use biome_parser::{
    prelude::ParsedSyntax::{self, *},
    Parser,
};

pub(crate) fn parse_paragraph(p: &mut MarkdownParser) -> ParsedSyntax {
    let m = p.start();

    // Create a list for paragraph items
    let item_list = p.start();

    // Parse paragraph content until we hit a blank line or another block element
    let mut has_content = false;

    while !p.at(T![EOF]) {
        // Check for blank line (just a newline)
        if p.at(NEWLINE) {
            p.bump(NEWLINE);

            // If the next line is also a newline, it's a blank line that ends the paragraph
            if p.at(NEWLINE) || p.at(T![EOF]) {
                break;
            }

            // Otherwise, it's a soft break within the paragraph
            let soft_break = p.start();
            soft_break.complete(p, MD_SOFT_BREAK);
            continue;
        }

        // Check for hard line break (line ending with two or more spaces)
        if p.at(MD_HARD_LINE_LITERAL) {
            let hard_line = p.start();
            p.bump(MD_HARD_LINE_LITERAL);
            hard_line.complete(p, MD_HARD_LINE);
            has_content = true;
            continue;
        }

        // Parse inline elements
        if parse_inline_element(p).is_present() {
            has_content = true;
            continue;
        }

        // If we can't parse anything else and we have no content, abandon the paragraph
        if !has_content {
            m.abandon(p);
            return Absent;
        }

        // Otherwise, create a textual node for any remaining content
        let text_m = p.start();
        p.bump_any();
        text_m.complete(p, MD_TEXTUAL);
        has_content = true;
    }

    // Complete the paragraph item list
    item_list.complete(p, MD_PARAGRAPH_ITEM_LIST);

    // Complete the paragraph
    Present(m.complete(p, MD_PARAGRAPH))
}

fn parse_inline_element(p: &mut MarkdownParser) -> ParsedSyntax {
    if p.at(MD_TEXTUAL_LITERAL) {
        // Parse text
        let text_m = p.start();
        p.bump(MD_TEXTUAL_LITERAL);
        return Present(text_m.complete(p, MD_TEXTUAL));
    } else if p.at(BACKTICK) {
        // Parse inline code
        return parse_inline_code(p);
    } else if p.at(STAR) || p.at(UNDERSCORE) {
        // Parse emphasis
        return parse_emphasis(p);
    } else if p.at(L_BRACK) {
        // Parse link or image
        return parse_link_or_image(p);
    } else if !p.at(NEWLINE) && !p.at(T![EOF]) {
        // Parse any other character as text
        let text_m: biome_parser::Marker = p.start();
        p.bump_any();
        return Present(text_m.complete(p, MD_TEXTUAL));
    }

    Absent
}

fn parse_inline_code(p: &mut MarkdownParser) -> ParsedSyntax {
    if !p.at(BACKTICK) {
        return Absent;
    }

    let m = p.start();

    // Parse opening backticks
    let mut backtick_count = 0;
    while p.at(BACKTICK) {
        p.bump(BACKTICK);
        backtick_count += 1;
    }

    // Parse content until closing backticks
    while !p.at(T![EOF]) {
        if p.at(BACKTICK) {
            // Check if we have the right number of closing backticks
            let checkpoint = p.checkpoint();
            let mut count = 0;

            while p.at(BACKTICK) {
                p.bump(BACKTICK);
                count += 1;
            }

            if count == backtick_count {
                break;
            } else {
                // Not the right number, rewind and continue
                p.rewind(checkpoint);
                p.bump_any();
            }
        } else {
            // Parse content
            if p.at(MD_TEXTUAL_LITERAL) {
                let text_m = p.start();
                p.bump(MD_TEXTUAL_LITERAL);
                text_m.complete(p, MD_TEXTUAL);
            } else {
                p.bump_any();
            }
        }
    }

    Present(m.complete(p, MD_INLINE_CODE))
}

fn parse_emphasis(p: &mut MarkdownParser) -> ParsedSyntax {
    if !p.at(STAR) && !p.at(UNDERSCORE) {
        return Absent;
    }

    let m = p.start();

    // Determine emphasis marker
    let marker = if p.at(STAR) { STAR } else { UNDERSCORE };

    // Parse opening markers
    let mut marker_count = 0;
    while p.at(marker) {
        p.bump(marker);
        marker_count += 1;
    }

    while !p.at(T![EOF]) && !p.at(NEWLINE) {
        if p.at(marker) {
            // Check if we have the right number of closing markers
            let checkpoint = p.checkpoint();
            let mut count = 0;

            while p.at(marker) {
                p.bump(marker);
                count += 1;
            }

            if count == marker_count {
                break;
            } else {
                // Not the right number, rewind and continue
                p.rewind(checkpoint);
                p.bump_any();
            }
        } else {
            // Parse nested inline elements
            if parse_inline_element(p).is_absent() {
                p.bump_any();
            }
        }
    }

    Present(m.complete(p, MD_INLINE_EMPHASIS))
}

fn parse_link_or_image(p: &mut MarkdownParser) -> ParsedSyntax {
    if !p.at(L_BRACK) {
        return Absent;
    }

    // Check if it's an image (starts with ![)
    let is_image = p.at(BANG) && p.nth_at(1, L_BRACK);

    let m = p.start();

    if is_image {
        p.bump(BANG);
    }

    // Parse [text]
    p.bump(L_BRACK);

    // Parse link text
    while !p.at(R_BRACK) && !p.at(T![EOF]) && !p.at(NEWLINE) {
        if parse_inline_element(p).is_absent() {
            p.bump_any();
        }
    }

    if !p.at(R_BRACK) {
        // Incomplete link/image
        m.abandon(p);
        return Absent;
    }

    p.bump(R_BRACK);

    // Parse (url "title")
    if !p.at(L_PAREN) {
        // Reference-style link/image not implemented yet
        m.abandon(p);
        return Absent;
    }

    p.bump(L_PAREN);

    // Parse URL
    while !p.at(R_PAREN) && !p.at(T![EOF]) && !p.at(NEWLINE) {
        if p.at(MD_STRING_LITERAL) {
            // Parse title
            let string_m = p.start();
            p.bump(MD_STRING_LITERAL);
            string_m.complete(p, MD_STRING);
        } else if p.at(MD_TEXTUAL_LITERAL) {
            let text_m = p.start();
            p.bump(MD_TEXTUAL_LITERAL);
            text_m.complete(p, MD_TEXTUAL);
        } else {
            p.bump_any();
        }
    }

    if !p.at(R_PAREN) {
        // Incomplete link/image
        m.abandon(p);
        return Absent;
    }

    p.bump(R_PAREN);

    if is_image {
        Present(m.complete(p, MD_INLINE_IMAGE))
    } else {
        Present(m.complete(p, MD_INLINE_LINK))
    }
}
