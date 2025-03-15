use biome_markdown_syntax::{T, kind::MarkdownSyntaxKind::*};
use biome_parser::{
    Parser,
    prelude::ParsedSyntax::{self, *},
};

use crate::MarkdownParser;

/// Checks if the current position is at the start of an unordered list item
pub(crate) fn at_unordered_list_item(p: &mut MarkdownParser) -> bool {
    // Skip leading whitespace
    let mut i = 0;
    while p.nth_at(i, WHITESPACE) || p.nth_at(i, TAB) {
        i += 1;
    }

    // Log the whitespace count
    eprintln!("at_unordered_list_item: Skipped {} leading whitespace characters", i);

    // Check if the next non-whitespace character is an unordered list marker
    let is_minus = p.nth_at(i, MINUS);
    let is_star = p.nth_at(i, STAR);
    let is_plus = p.nth_at(i, PLUS);
    
    eprintln!("at_unordered_list_item: Marker check - MINUS: {}, STAR: {}, PLUS: {}", 
              is_minus, is_star, is_plus);
    
    // If we have a marker, check if it's followed by whitespace or tab
    if is_minus || is_star || is_plus {
        // For valid list items, there should be at least one whitespace after the marker
        let has_whitespace_after = p.nth_at(i + 1, WHITESPACE) || p.nth_at(i + 1, TAB);
        
        // There might also be a MD_TEXTUAL_LITERAL after the marker, so check that too
        let is_text_after = p.nth_at(i + 1, MD_TEXTUAL_LITERAL);
        
        eprintln!("at_unordered_list_item: Whitespace after marker: {}, Text after marker: {}", 
                 has_whitespace_after, is_text_after);
                 
        // Either whitespace or textual literal after the marker makes it a valid list item
        return has_whitespace_after || is_text_after;
    }

    eprintln!("at_unordered_list_item: No valid marker found");
    false
}

/// Checks if the current position is at the start of an ordered list item
pub(crate) fn at_ordered_list_item(p: &mut MarkdownParser) -> bool {
    // Skip leading whitespace
    let mut i = 0;
    while p.nth_at(i, WHITESPACE) || p.nth_at(i, TAB) {
        i += 1;
    }

    // Check if the current token is a digit
    let mut digit_pos = i;
    let mut has_digit = false;

    // Look for one or more digits
    while p.nth_at(digit_pos, DIGIT) {
        has_digit = true;
        digit_pos += 1;
    }

    if !has_digit {
        return false;
    }

    // Check for a period or closing parenthesis after digits
    let delimiter_pos = digit_pos;
    if !p.nth_at(delimiter_pos, PERIOD) && !p.nth_at(delimiter_pos, T![')']) {
        return false;
    }

    // Check for whitespace after the delimiter (required for a valid list item)
    p.nth_at(delimiter_pos + 1, WHITESPACE) || p.nth_at(delimiter_pos + 1, TAB)
}

/// Checks if the current position is at the start of any list
pub(crate) fn at_list_block(p: &mut MarkdownParser) -> bool {
    at_unordered_list_item(p) || at_ordered_list_item(p)
}

/// Parses an unordered list (-, *, +)
pub(crate) fn parse_unordered_list(p: &mut MarkdownParser) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let list_m = p.start();
    
    eprintln!("parse_unordered_list: Starting to parse unordered list");
    
    // Track the original indentation level
    let original_indent = count_leading_whitespace(p);
    eprintln!("parse_unordered_list: Original indentation level: {}", original_indent);
    
    // Parse the first list item
    eprintln!("parse_unordered_list: Attempting to parse first list item");
    let first_item = parse_unordered_list_item(p, original_indent);
    
    // If we couldn't parse the first item, the list is invalid
    if first_item.is_absent() {
        eprintln!("parse_unordered_list: Failed to parse first list item, abandoning list");
        list_m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }
    
    eprintln!("parse_unordered_list: Successfully parsed first list item");
    
    // Try to parse additional list items at the same indentation level
    let mut additional_items = 0;
    while !p.at(T![EOF]) {
        // Skip empty lines
        if p.at(NEWLINE) {
            p.bump(NEWLINE);
            eprintln!("parse_unordered_list: Skipped newline");
            continue;
        }
        
        // Check indentation of the next line
        let item_checkpoint = p.checkpoint();
        let current_indent = count_leading_whitespace(p);
        p.rewind(item_checkpoint);
        
        eprintln!("parse_unordered_list: Next line indentation: {} (original: {})", current_indent, original_indent);
        
        // If indentation is less than original, we're exiting the list
        if current_indent < original_indent {
            eprintln!("parse_unordered_list: Exiting list - indentation decreased");
            break;
        }
        
        // If this is another list item at the same indentation level
        if current_indent == original_indent && at_unordered_list_item(p) {
            eprintln!("parse_unordered_list: Found another list item at same level");
            if parse_unordered_list_item(p, original_indent).is_absent() {
                eprintln!("parse_unordered_list: Failed to parse additional list item");
                break;
            }
            // Successfully parsed another item
            additional_items += 1;
            eprintln!("parse_unordered_list: Successfully parsed additional item #{}", additional_items);
            continue;
        }
        
        // If we see a new block at a lesser indentation, we've exited the list
        if is_block_starter(p) {
            eprintln!("parse_unordered_list: Exiting list - new block starter found");
            break;
        }
        
        // Handle continuation lines or nested content
        let text_checkpoint = p.checkpoint();
        consume_leading_whitespace(p);
        
        // If no meaningful content, skip this line
        if p.at(NEWLINE) || p.at(T![EOF]) {
            eprintln!("parse_unordered_list: Skipping line with no content");
            p.bump_any();
            continue;
        }
        
        // Not a list item at this level, so we're done
        eprintln!("parse_unordered_list: Exiting list - not a list item at this level");
        p.rewind(text_checkpoint);
        break;
    }
    
    // Successfully parsed list
    eprintln!("parse_unordered_list: Completed list with {} additional items", additional_items);
    Present(list_m.complete(p, MD_UNORDERED_LIST))
}

/// Parses an ordered list (1., 2., etc.)
pub(crate) fn parse_ordered_list(p: &mut MarkdownParser) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let list_m = p.start();
    
    // Track the original indentation level
    let original_indent = count_leading_whitespace(p);
    
    // Parse the first list item
    let first_item = parse_ordered_list_item(p, original_indent);
    
    // If we couldn't parse the first item, the list is invalid
    if first_item.is_absent() {
        list_m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }
    
    // Try to parse additional list items at the same indentation level
    while !p.at(T![EOF]) {
        // Skip empty lines
        if p.at(NEWLINE) {
            p.bump(NEWLINE);
            continue;
        }
        
        // Check indentation of the next line
        let item_checkpoint = p.checkpoint();
        let current_indent = count_leading_whitespace(p);
        p.rewind(item_checkpoint);
        
        // If indentation is less than original, we're exiting the list
        if current_indent < original_indent {
            break;
        }
        
        // If this is another list item at the same indentation level
        if current_indent == original_indent && at_ordered_list_item(p) {
            if parse_ordered_list_item(p, original_indent).is_absent() {
                break;
            }
            // Successfully parsed another item
            continue;
        }
        
        // If we see a new block at a lesser indentation, we've exited the list
        if is_block_starter(p) {
            break;
        }
        
        // Handle continuation lines or nested content
        let text_checkpoint = p.checkpoint();
        consume_leading_whitespace(p);
        
        // If no meaningful content, skip this line
        if p.at(NEWLINE) || p.at(T![EOF]) {
            p.bump_any();
            continue;
        }
        
        // Not a list item at this level, so we're done
        p.rewind(text_checkpoint);
        break;
    }
    
    // Successfully parsed list
    Present(list_m.complete(p, MD_ORDERED_LIST))
}

/// Parses an unordered list item
fn parse_unordered_list_item(p: &mut MarkdownParser, parent_indent: usize) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let m = p.start();
    
    eprintln!("parse_unordered_list_item: Starting to parse unordered list item");

    // Skip leading whitespace
    let whitespace_count = count_leading_whitespace(p);
    eprintln!("parse_unordered_list_item: Skipped {} whitespace characters", whitespace_count);

    // Save marker type for consistency checking
    let marker_type = if p.at(MINUS) {
        p.bump(MINUS);
        eprintln!("parse_unordered_list_item: Found MINUS marker");
        Some(MINUS)
    } else if p.at(STAR) {
        p.bump(STAR);
        eprintln!("parse_unordered_list_item: Found STAR marker");
        Some(STAR)
    } else if p.at(PLUS) {
        p.bump(PLUS);
        eprintln!("parse_unordered_list_item: Found PLUS marker");
        Some(PLUS)
    } else {
        eprintln!("parse_unordered_list_item: No valid marker found");
        None
    };

    if marker_type.is_none() {
        // Not a valid list marker
        eprintln!("parse_unordered_list_item: Abandoning due to missing marker");
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    // Consume whitespace after the marker (required)
    let has_whitespace = p.at(WHITESPACE) || p.at(TAB);
    let has_text = p.at(MD_TEXTUAL_LITERAL);
    
    eprintln!("parse_unordered_list_item: After marker - whitespace: {}, text: {}", 
              has_whitespace, has_text);
    
    if has_whitespace {
        consume_leading_whitespace(p);
    } else if !has_text {
        // Not a valid list item without space or text after marker
        eprintln!("parse_unordered_list_item: Abandoning due to no whitespace or text after marker");
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    // Parse the content of the list item
    let content_indent = parent_indent + 2; // Standard indentation for list content
    let content_m = p.start();

    // Parse content until the end of the list item
    eprintln!("parse_unordered_list_item: Parsing content with indent {}", content_indent);
    parse_list_item_content(p, content_indent);

    // Complete the content node
    content_m.complete(p, MD_LIST_ITEM_CONTENT);

    // Complete the list item
    eprintln!("parse_unordered_list_item: Successfully completed list item");
    Present(m.complete(p, MD_UNORDERED_LIST_ITEM))
}

/// Parses an ordered list item
fn parse_ordered_list_item(p: &mut MarkdownParser, parent_indent: usize) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let m = p.start();

    // Skip leading whitespace
    consume_leading_whitespace(p);

    // Make sure we're at a digit
    if !p.at(DIGIT) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    // Parse the number
    let number_m = p.start();
    while p.at(DIGIT) {
        p.bump(DIGIT);
    }
    number_m.complete(p, MD_ORDERED_LIST_ITEM);

    // Make sure we have a delimiter (period or parenthesis)
    let has_delimiter = if p.at(PERIOD) {
        p.bump(PERIOD);
        true
    } else if p.at(T![')']) {
        p.bump(T![')']);
        true
    } else {
        false
    };

    if !has_delimiter {
        // Not a valid ordered list item without delimiter
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    // Consume whitespace after the delimiter (required)
    if !p.at(WHITESPACE) && !p.at(TAB) {
        // Not a valid list item without space after delimiter
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }
    consume_leading_whitespace(p);

    // Parse the content of the list item
    let content_indent = parent_indent + 2; // Standard indentation for list content
    let content_m = p.start();

    // Parse content until the end of the list item
    parse_list_item_content(p, content_indent);

    // Complete the content node
    content_m.complete(p, MD_LIST_ITEM_CONTENT);

    // Complete the list item
    Present(m.complete(p, MD_ORDERED_LIST_ITEM))
}

/// Parse the content of a list item, including nested content
fn parse_list_item_content(p: &mut MarkdownParser, content_indent: usize) {
    eprintln!("parse_list_item_content: Starting to parse content with indent {}", content_indent);
    
    // Parse first line
    let para_m = p.start();
    let item_list = p.start();
    
    // Parse inline content until newline or EOF
    let mut has_text = false;
    eprintln!("parse_list_item_content: Parsing first line content");
    
    while !p.at(NEWLINE) && !p.at(T![EOF]) {
        let text_m = p.start();
        if p.at(MD_TEXTUAL_LITERAL) {
            p.bump(MD_TEXTUAL_LITERAL);
            eprintln!("parse_list_item_content: Found textual literal");
            has_text = true;
        } else {
            let token_type = if p.at(WHITESPACE) {
                "WHITESPACE"
            } else if p.at(TAB) {
                "TAB"
            } else if p.at(STAR) {
                "STAR"
            } else if p.at(MINUS) {
                "MINUS"
            } else if p.at(PLUS) {
                "PLUS"
            } else {
                "OTHER"
            };
            eprintln!("parse_list_item_content: Bumping non-textual token: {}", token_type);
            p.bump_any();
        }
        text_m.complete(p, MD_TEXTUAL);
    }

    // Consume the newline if present
    if p.at(NEWLINE) {
        p.bump(NEWLINE);
        eprintln!("parse_list_item_content: Consumed newline after first line");
    }
    
    // Complete the paragraph only if we have content
    if has_text {
        item_list.complete(p, MD_PARAGRAPH_ITEM_LIST);
        para_m.complete(p, MD_PARAGRAPH);
        eprintln!("parse_list_item_content: Completed first line paragraph");
    } else {
        item_list.abandon(p);
        para_m.abandon(p);
        eprintln!("parse_list_item_content: Abandoned empty paragraph");
    }

    eprintln!("parse_list_item_content: First line content parsing completed, has_text: {}", has_text);

    // Process continuation lines
    while !p.at(T![EOF]) {
        // Skip blank lines
        if p.at(NEWLINE) {
            p.bump(NEWLINE);
            continue;
        }

        // Check indentation of the next line
        let line_checkpoint = p.checkpoint();
        let next_indent = count_leading_whitespace(p);
        
        // If indentation is less than content indent and it's a list item or other block element,
        // then we're done with this list item's content
        if next_indent < content_indent && (at_list_block(p) || is_block_starter(p)) {
            p.rewind(line_checkpoint);
            break;
        }
        
        // Skip the whitespace but keep track of whether this is indented enough
        p.rewind(line_checkpoint);
        consume_leading_whitespace(p);
        
        // If we're at a new line or EOF after consuming whitespace, just skip it
        if p.at(NEWLINE) || p.at(T![EOF]) {
            if p.at(NEWLINE) {
                p.bump(NEWLINE);
            }
            continue;
        }
        
        // Parse the content of the continuation line as a paragraph
        let para_m = p.start();
        let item_list = p.start();
        
        // Parse until end of line
        let mut has_continuation_text = false;
        while !p.at(NEWLINE) && !p.at(T![EOF]) {
            let text_m = p.start();
            if p.at(MD_TEXTUAL_LITERAL) {
                p.bump(MD_TEXTUAL_LITERAL);
            } else {
                p.bump_any();
            }
            text_m.complete(p, MD_TEXTUAL);
            has_continuation_text = true;
        }
        
        // Consume the newline if present
        if p.at(NEWLINE) {
            p.bump(NEWLINE);
        }
        
        // Complete the paragraph for this continuation line if it has content
        if has_continuation_text {
            item_list.complete(p, MD_PARAGRAPH_ITEM_LIST);
            para_m.complete(p, MD_PARAGRAPH);
        } else {
            item_list.abandon(p);
            para_m.abandon(p);
        }
    }
}

/// Helper function to count leading whitespace for indentation tracking
fn count_leading_whitespace(p: &mut MarkdownParser) -> usize {
    let mut count = 0;
    let mut i = 0;

    while p.nth_at(i, WHITESPACE) {
        count += 1;
        i += 1;
    }

    // Each tab counts as 4 spaces for indentation purposes
    while p.nth_at(i, TAB) {
        count += 4;
        i += 1;
    }

    count
}

/// Helper function to consume leading whitespace
fn consume_leading_whitespace(p: &mut MarkdownParser) {
    while p.at(WHITESPACE) || p.at(TAB) {
        p.bump_any();
    }
}

/// Check if we're at the start of a block element (other than list)
fn is_block_starter(p: &mut MarkdownParser) -> bool {
    let checkpoint = p.checkpoint();
    
    // Skip leading whitespace
    consume_leading_whitespace(p);
    
    // Check for various block starters
    let is_starter = p.at(HASH) || // Header
        p.at(BACKTICK) || p.at(TILDE) || // Fenced code block
        p.at(T!['>']) || // Blockquote
        (p.at(MINUS) && p.nth_at(1, MINUS)) || // Thematic break
        p.at(PIPE) && p.nth_at(1, PIPE); // Table
    
    // Restore position
    p.rewind(checkpoint);
    
    is_starter
}
