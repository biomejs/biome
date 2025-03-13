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

    // Check if the next non-whitespace character is an unordered list marker
    p.nth_at(i, T![-]) || p.nth_at(i, STAR) || p.nth_at(i, PLUS)
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
    p.nth_at(digit_pos, PERIOD) || p.nth_at(digit_pos, T![')'])
}

/// Checks if the current position is at the start of any list
pub(crate) fn at_list_block(p: &mut MarkdownParser) -> bool {
    at_unordered_list_item(p) || at_ordered_list_item(p)
}

/// Parses an unordered list item
pub(crate) fn parse_unordered_list_item(p: &mut MarkdownParser) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let m = p.start();

    // Skip leading whitespace
    while p.at(WHITESPACE) || p.at(TAB) {
        p.bump_any();
    }

    // Make sure we're at an unordered list marker
    if !p.at(T![-]) && !p.at(STAR) && !p.at(PLUS) {
        p.rewind(checkpoint);
        return Absent;
    }

    // Consume the list marker
    p.bump_any();

    // Consume whitespace after the marker (required)
    if p.at(WHITESPACE) {
        p.bump(WHITESPACE);
    } else if p.at(TAB) {
        p.bump(TAB);
    } else {
        // Not a valid list item without space after marker
        p.rewind(checkpoint);
        return Absent;
    }

    // Parse the content of the list item
    parse_list_item_content(p);

    Present(m.complete(p, MD_UNORDERED_LIST_ITEM))
}

/// Parses an ordered list item
pub(crate) fn parse_ordered_list_item(p: &mut MarkdownParser) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let m = p.start();

    // Skip leading whitespace
    while p.at(WHITESPACE) || p.at(TAB) {
        p.bump_any();
    }

    // Make sure we're at a digit
    if !p.at(DIGIT) {
        p.rewind(checkpoint);
        return Absent;
    }

    // Parse digits - don't create a separate node for the number
    // Just consume the digits as part of the list item
    while p.at(DIGIT) {
        p.bump(DIGIT);
    }

    // Make sure we have a delimiter (period or parenthesis)
    if !(p.at(PERIOD) || p.at(T![')'])) {
        p.rewind(checkpoint);
        return Absent;
    }

    // Consume the delimiter
    p.bump_any();

    // Consume whitespace after the delimiter (required)
    if p.at(WHITESPACE) {
        p.bump(WHITESPACE);
    } else if p.at(TAB) {
        p.bump(TAB);
    } else {
        // Not a valid list item without space after delimiter
        p.rewind(checkpoint);
        return Absent;
    }

    // Parse the content of the list item
    parse_list_item_content(p);

    Present(m.complete(p, MD_ORDERED_LIST_ITEM))
}

/// Parses the content of a list item
fn parse_list_item_content(p: &mut MarkdownParser) {
    let content = p.start();

    // Parse content until the end of the line or EOF
    while !p.at(NEWLINE) && !p.at(EOF) {
        p.bump_any();
    }

    // Consume the newline if present
    if p.at(NEWLINE) {
        p.bump(NEWLINE);
    }

    // Parse continuation lines (indented by at least 2 spaces)
    while peek_continuation_line(p) {
        parse_continuation_line(p);
    }

    content.complete(p, MD_LIST_ITEM_CONTENT);
}

/// Check if the next line is a continuation of the current list item
fn peek_continuation_line(p: &mut MarkdownParser) -> bool {
    if p.at(EOF) {
        return false;
    }

    // If we're at the beginning of a new list item, it's not a continuation
    if at_list_block(p) {
        return false;
    }

    // Check if the line is indented by at least 2 spaces (or has a tab)
    let mut i = 0;
    let mut spaces = 0;

    while p.nth_at(i, WHITESPACE) {
        spaces += 1;
        i += 1;
        if spaces >= 2 {
            return true;
        }
    }

    // A tab counts as indentation
    if p.nth_at(i, TAB) {
        return true;
    }

    // If we're at a blank line, look ahead to see if there's an indented line after
    if p.at(NEWLINE) {
        let mut j = 1;
        while p.nth_at(j, NEWLINE) {
            j += 1;
        }

        // Check for indentation after newlines
        let mut indent_spaces = 0;
        while p.nth_at(j, WHITESPACE) {
            indent_spaces += 1;
            j += 1;
            if indent_spaces >= 2 {
                return true;
            }
        }

        // Tab after newlines also counts as indentation
        if p.nth_at(j, TAB) {
            return true;
        }
    }

    false
}

/// Parse a continuation line of a list item
fn parse_continuation_line(p: &mut MarkdownParser) {
    // Skip blank lines
    while p.at(NEWLINE) {
        p.bump(NEWLINE);
    }

    // Consume indentation (spaces or tabs)
    while p.at(WHITESPACE) || p.at(TAB) {
        p.bump_any();
    }

    // Parse content until the end of the line or EOF
    while !p.at(NEWLINE) && !p.at(EOF) {
        p.bump_any();
    }

    // Consume the newline if present
    if p.at(NEWLINE) {
        p.bump(NEWLINE);
    }
}

/// Parses an unordered list
pub(crate) fn parse_unordered_list(p: &mut MarkdownParser) -> ParsedSyntax {
    let m = p.start();

    while at_unordered_list_item(p) {
        let _ = parse_unordered_list_item(p);
    }

    Present(m.complete(p, MD_UNORDERED_LIST))
}

/// Parses an ordered list
pub(crate) fn parse_ordered_list(p: &mut MarkdownParser) -> ParsedSyntax {
    let m = p.start();

    while at_ordered_list_item(p) {
        let _ = parse_ordered_list_item(p);
    }

    Present(m.complete(p, MD_ORDERED_LIST))
}

/// Parses any type of list
pub(crate) fn parse_list_block(p: &mut MarkdownParser) -> ParsedSyntax {
    if at_unordered_list_item(p) {
        parse_unordered_list(p)
    } else if at_ordered_list_item(p) {
        parse_ordered_list(p)
    } else {
        Absent
    }
}
