mod blockquote_block;
mod code_block;
mod header_block;
mod list_block;
mod paragraph_block;
mod table_block;
mod thematic_break_block;

use biome_markdown_syntax::{T, kind::MarkdownSyntaxKind::*};
use biome_parser::Parser;

use crate::MarkdownParser;

/// Parse a markdown document
pub(crate) fn parse_document(p: &mut MarkdownParser) {
    // Start the document node
    let document = p.start();

    // Start the block list
    let block_list = p.start();

    // A document can have multiple blocks at the top level
    while !p.at(T![EOF]) {
        // Skip empty lines
        if p.at(NEWLINE) {
            p.bump(NEWLINE);
            continue;
        }

        // Create a checkpoint in case we need to fall back to paragraph parsing
        let checkpoint = p.checkpoint();

        // Try to parse different block elements
        if !try_parse_block_element(p) {
            // Reset to checkpoint and parse as paragraph
            p.rewind(checkpoint);

            // Try to parse a paragraph
            if paragraph_block::parse_paragraph(p).is_absent() {
                // Skip token if we couldn't parse it as a paragraph either
                p.bump_any();
            }
        }
    }

    // Complete the block list
    block_list.complete(p, MD_BLOCK_LIST);

    // Make sure we have an EOF token before completing the document
    if !p.at(T![EOF]) {
        p.expect(T![EOF]);
    }

    // Complete the document
    document.complete(p, MD_DOCUMENT);
}

/// Try to parse a block element
/// Returns true if a block element was successfully parsed
fn try_parse_block_element(p: &mut MarkdownParser) -> bool {
    // Try all block elements in order of precedence

    // Thematic break (horizontal rule)
    if thematic_break_block::at_thematic_break_block(p) {
        return thematic_break_block::parse_thematic_break_block(p).is_present();
    }

    // ATX Header
    if header_block::at_atx_header(p) {
        return header_block::parse_atx_header(p).is_present();
    }

    // Fenced code block
    if code_block::at_fenced_code_block(p) {
        return code_block::parse_fenced_code_block(p).is_present();
    }

    // Blockquote
    if blockquote_block::at_blockquote(p) {
        return blockquote_block::parse_blockquote(p).is_present();
    }

    // Table
    if table_block::at_table(p) {
        return table_block::parse_table(p).is_present();
    }

    // Ordered list
    if list_block::at_ordered_list_item(p) {
        eprintln!("try_parse_block_element: Found ordered list item, starting list");
        // The list node is created inside parse_ordered_list
        return list_block::parse_ordered_list(p).is_present();
    }

    // Unordered list
    eprintln!("Checking for unordered list item");
    let is_unordered = list_block::at_unordered_list_item(p);
    eprintln!("Is unordered list item: {}", is_unordered);

    if is_unordered {
        eprintln!("Starting to parse unordered list");
        // The list node is now created inside parse_unordered_list
        // directly, not here
        return list_block::parse_unordered_list(p).is_present();
    }

    // No block element recognized
    let current_token = if p.at(NEWLINE) {
        "NEWLINE".to_string()
    } else if p.at(T![EOF]) {
        "EOF".to_string()
    } else {
        // Try to get a more descriptive token name
        let mut token_desc = String::from("Unknown");

        // Check for common token types
        if p.at(WHITESPACE) {
            token_desc = "WHITESPACE".to_string();
        } else if p.at(TAB) {
            token_desc = "TAB".to_string();
        } else if p.at(HASH) {
            token_desc = "HASH".to_string();
        } else if p.at(STAR) {
            token_desc = "STAR".to_string();
        } else if p.at(MINUS) {
            token_desc = "MINUS".to_string();
        } else if p.at(PLUS) {
            token_desc = "PLUS".to_string();
        } else if p.at(BACKTICK) {
            token_desc = "BACKTICK".to_string();
        } else if p.at(DIGIT) {
            token_desc = "DIGIT".to_string();
        } else if p.at(MD_TEXTUAL_LITERAL) {
            token_desc = "MD_TEXTUAL_LITERAL".to_string();
        }

        token_desc
    };

    eprintln!(
        "Failed to recognize block element at token: {}",
        current_token
    );

    // Get some context - print the next few characters
    // We can't directly access token values, but we can check token types
    let mut token_types = Vec::new();
    for i in 0..5 {
        if p.nth_at(i, WHITESPACE) {
            token_types.push("WHITESPACE");
        } else if p.nth_at(i, TAB) {
            token_types.push("TAB");
        } else if p.nth_at(i, NEWLINE) {
            token_types.push("NEWLINE");
        } else if p.nth_at(i, HASH) {
            token_types.push("HASH");
        } else if p.nth_at(i, STAR) {
            token_types.push("STAR");
        } else if p.nth_at(i, MINUS) {
            token_types.push("MINUS");
        } else if p.nth_at(i, PLUS) {
            token_types.push("PLUS");
        } else if p.nth_at(i, BACKTICK) {
            token_types.push("BACKTICK");
        } else if p.nth_at(i, DIGIT) {
            token_types.push("DIGIT");
        } else if p.nth_at(i, T![EOF]) {
            token_types.push("EOF");
            break;
        } else if p.nth_at(i, MD_TEXTUAL_LITERAL) {
            token_types.push("MD_TEXTUAL_LITERAL");
        } else {
            token_types.push("OTHER");
        }
    }

    eprintln!("Next token types: {:?}", token_types);

    false
}
