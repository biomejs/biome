use crate::parser::MarkdownParser;
use biome_markdown_syntax::MarkdownSyntaxKind::*;
use biome_markdown_syntax::T;
use biome_parser::{
    prelude::ParsedSyntax::{self, *},
    Parser,
};

// Indented code blocks
pub(crate) fn at_indent_code_block(p: &mut MarkdownParser) -> bool {
    p.before_whitespace_count() >= 4
}

pub(crate) fn parse_indent_code_block(p: &mut MarkdownParser) -> ParsedSyntax {
    if !at_indent_code_block(p) {
        return Absent;
    }

    let m = p.start();

    // Parse indented code content
    while at_indent_code_block(p) && !p.at(T![EOF]) {
        // Parse the indent
        let indent_m = p.start();

        // Consume the indentation (already counted in before_whitespace_count)
        if p.at(MD_INDENT_CHUNK_LITERAL) {
            p.bump(MD_INDENT_CHUNK_LITERAL);
        } else {
            // Consume whitespace manually if needed
            while (p.at(WHITESPACE) || p.at(TAB)) && p.before_whitespace_count() > 0 {
                p.bump_any();
            }
        }

        indent_m.complete(p, MD_INDENT);

        // Parse the code content until end of line
        while !p.at(NEWLINE) && !p.at(T![EOF]) {
            if p.at(MD_TEXTUAL_LITERAL) {
                let text_m = p.start();
                p.bump(MD_TEXTUAL_LITERAL);
                text_m.complete(p, MD_TEXTUAL);
            } else {
                p.bump_any();
            }
        }

        // Consume the newline
        p.eat(NEWLINE);
    }

    Present(m.complete(p, MD_INDENT_CODE_BLOCK))
}

// Fenced code blocks
pub(crate) fn at_fenced_code_block(p: &mut MarkdownParser) -> bool {
    // Check for ``` or ~~~
    (p.at(BACKTICK) && p.nth_at(1, BACKTICK) && p.nth_at(2, BACKTICK))
        || (p.at(TILDE) && p.nth_at(1, TILDE) && p.nth_at(2, TILDE))
}

pub(crate) fn parse_fenced_code_block(p: &mut MarkdownParser) -> ParsedSyntax {
    if !at_fenced_code_block(p) {
        return Absent;
    }

    let m = p.start();

    // Determine fence type (backticks or tildes)
    let fence_char = if p.at(BACKTICK) { BACKTICK } else { TILDE };

    // Parse opening fence (at least 3 backticks or tildes)
    let mut fence_length = 0;
    while p.at(fence_char) {
        p.bump(fence_char);
        fence_length += 1;
    }

    // Parse optional language identifier
    while !p.at(NEWLINE) && !p.at(T![EOF]) {
        if p.at(MD_TEXTUAL_LITERAL) {
            let text_m = p.start();
            p.bump(MD_TEXTUAL_LITERAL);
            text_m.complete(p, MD_TEXTUAL);
        } else {
            p.bump_any();
        }
    }

    // Consume the newline
    p.eat(NEWLINE);

    // Parse code block content until closing fence

    while !p.at(T![EOF]) {
        // Check for closing fence
        if p.at(fence_char) {
            let checkpoint = p.checkpoint();
            let mut count = 0;

            while p.at(fence_char) {
                p.bump(fence_char);
                count += 1;
            }

            // Valid closing fence must have at least the same length as opening fence
            // and be followed by only whitespace until end of line
            if count >= fence_length
                && (p.at(NEWLINE) || p.at(T![EOF]) || p.at(WHITESPACE) || p.at(TAB))
            {
                // Consume any trailing whitespace
                while p.at(WHITESPACE) || p.at(TAB) {
                    p.bump_any();
                }

                // Consume the final newline
                p.eat(NEWLINE);

                break;
            } else {
                // Not a valid closing fence, rewind and continue
                p.rewind(checkpoint);
            }
        }

        // Parse line content
        while !p.at(NEWLINE) && !p.at(T![EOF]) {
            if p.at(MD_TEXTUAL_LITERAL) {
                let text_m = p.start();
                p.bump(MD_TEXTUAL_LITERAL);
                text_m.complete(p, MD_TEXTUAL);
            } else {
                p.bump_any();
            }
        }

        // Consume the newline
        p.eat(NEWLINE);
    }

    Present(m.complete(p, MD_FENCED_CODE_BLOCK))
}
