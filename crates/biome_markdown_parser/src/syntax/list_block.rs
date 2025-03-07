use biome_markdown_syntax::{kind::MarkdownSyntaxKind::*, T};
use biome_parser::{
    prelude::ParsedSyntax::{self, *},
    Parser,
};

use crate::MarkdownParser;

/// Checks if the current position is at the start of an unordered list item
pub(crate) fn at_unordered_list_item(p: &mut MarkdownParser) -> bool {
    p.at(T![-]) || p.at(STAR) || p.at(PLUS)
}

/// Checks if the current position is at the start of an ordered list item
pub(crate) fn at_ordered_list_item(p: &mut MarkdownParser) -> bool {
    // Check if the current token is a digit
    let mut i = 0;
    let mut has_digit = false;

    // Look for one or more digits
    while p.nth_at(i, DIGIT) {
        has_digit = true;
        i += 1;
    }

    if !has_digit {
        return false;
    }

    // Check for a period or closing parenthesis after digits
    p.nth_at(i, PERIOD) || p.nth_at(i, T![')'])
}

/// Checks if the current position is at the start of any list
pub(crate) fn at_list_block(p: &mut MarkdownParser) -> bool {
    at_unordered_list_item(p) || at_ordered_list_item(p)
}

/// Parses an unordered list item
pub(crate) fn parse_unordered_list_item(p: &mut MarkdownParser) -> ParsedSyntax {
    let m = p.start();

    // Consume the list marker
    p.bump_any();

    // Consume whitespace after the marker
    if p.at(WHITESPACE) {
        p.bump_any();
    }

    // Parse the content of the list item
    parse_list_item_content(p);

    Present(m.complete(p, MD_UNORDERED_LIST_ITEM))
}

/// Parses an ordered list item
pub(crate) fn parse_ordered_list_item(p: &mut MarkdownParser) -> ParsedSyntax {
    let m = p.start();

    // Consume the digits
    while p.at(DIGIT) {
        p.bump_any();
    }

    // Consume the delimiter (. or ))
    p.bump_any();

    // Consume whitespace after the marker
    if p.at(WHITESPACE) {
        p.bump_any();
    }

    // Parse the content of the list item
    parse_list_item_content(p);

    Present(m.complete(p, MD_ORDERED_LIST_ITEM))
}

/// Parses the content of a list item
fn parse_list_item_content(p: &mut MarkdownParser) {
    let m = p.start();

    // Parse text until end of line or end of file
    while !p.at(NEWLINE) && !p.at(T![EOF]) {
        p.bump_any();
    }

    // Consume the newline if present
    if p.at(NEWLINE) {
        p.bump_any();
    }

    m.complete(p, MD_LIST_ITEM_CONTENT);
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
