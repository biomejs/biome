use biome_markdown_syntax::{T, kind::MarkdownSyntaxKind::*};
use biome_parser::{
    Parser,
    prelude::ParsedSyntax::{self, *},
};

use crate::MarkdownParser;

/// Checks if the current position is at the start of a table
pub(crate) fn at_table(p: &mut MarkdownParser) -> bool {
    // A table must start with a line containing at least one pipe character
    // and must be followed by a delimiter row

    // Check if the current line contains a pipe
    let mut i = 0;
    let mut found_pipe = false;

    while !p.nth_at(i, NEWLINE) && !p.nth_at(i, T![EOF]) {
        if p.nth_at(i, PIPE) {
            found_pipe = true;
            break;
        }
        i += 1;
    }

    if !found_pipe {
        return false;
    }

    // Look ahead to check if the next line is a delimiter row
    // Skip to the beginning of the next line
    while !p.nth_at(i, NEWLINE) && !p.nth_at(i, T![EOF]) {
        i += 1;
    }

    if p.nth_at(i, NEWLINE) {
        i += 1; // Skip the newline
    } else {
        return false; // EOF, no next line
    }

    // Check if the next line contains at least one pipe and dashes
    let mut found_pipe_in_delimiter = false;
    let mut found_dash = false;

    while !p.nth_at(i, NEWLINE) && !p.nth_at(i, T![EOF]) {
        if p.nth_at(i, PIPE) {
            found_pipe_in_delimiter = true;
        } else if p.nth_at(i, T![-]) {
            found_dash = true;
        }
        i += 1;
    }

    found_pipe_in_delimiter && found_dash
}

/// Parses a table header row
fn parse_table_header(p: &mut MarkdownParser) -> ParsedSyntax {
    let m = p.start();

    // Parse cells until end of line
    while !p.at(NEWLINE) && !p.at(T![EOF]) {
        // Parse a cell
        if p.at(PIPE) {
            // Consume the pipe
            p.bump_any();
        }

        // Parse cell content
        let cell_marker = p.start();

        // Consume everything until the next pipe or end of line
        while !p.at(PIPE) && !p.at(NEWLINE) && !p.at(T![EOF]) {
            p.bump_any();
        }

        cell_marker.complete(p, MD_TABLE_CELL);
    }

    // Consume the newline if present
    if p.at(NEWLINE) {
        p.bump_any();
    }

    Present(m.complete(p, MD_TABLE_HEADER))
}

/// Parses a table delimiter row
fn parse_table_delimiter(p: &mut MarkdownParser) -> ParsedSyntax {
    let m = p.start();

    // Parse delimiter cells until end of line
    while !p.at(NEWLINE) && !p.at(T![EOF]) {
        // Parse a delimiter cell
        if p.at(PIPE) {
            // Consume the pipe
            p.bump_any();
        }

        // Parse delimiter cell content
        let cell_marker = p.start();

        // Consume everything until the next pipe or end of line
        while !p.at(PIPE) && !p.at(NEWLINE) && !p.at(T![EOF]) {
            p.bump_any();
        }

        cell_marker.complete(p, MD_TABLE_DELIMITER_CELL);
    }

    // Consume the newline if present
    if p.at(NEWLINE) {
        p.bump_any();
    }

    Present(m.complete(p, MD_TABLE_DELIMITER))
}

/// Parses a table row
fn parse_table_row(p: &mut MarkdownParser) -> ParsedSyntax {
    let m = p.start();

    // Parse cells until end of line
    while !p.at(NEWLINE) && !p.at(T![EOF]) {
        // Parse a cell
        if p.at(PIPE) {
            // Consume the pipe
            p.bump_any();
        }

        // Parse cell content
        let cell_marker = p.start();

        // Consume everything until the next pipe or end of line
        while !p.at(PIPE) && !p.at(NEWLINE) && !p.at(T![EOF]) {
            p.bump_any();
        }

        cell_marker.complete(p, MD_TABLE_CELL);
    }

    // Consume the newline if present
    if p.at(NEWLINE) {
        p.bump_any();
    }

    Present(m.complete(p, MD_TABLE_ROW))
}

/// Parses a table block
pub(crate) fn parse_table(p: &mut MarkdownParser) -> ParsedSyntax {
    let m = p.start();

    // Parse the header row
    let _ = parse_table_header(p);

    // Parse the delimiter row
    let _ = parse_table_delimiter(p);

    // Parse data rows
    while at_table_row(p) {
        let _ = parse_table_row(p);
    }

    Present(m.complete(p, MD_TABLE))
}

/// Checks if the current position is at the start of a table row
fn at_table_row(p: &mut MarkdownParser) -> bool {
    // A table row must contain at least one pipe character
    let mut i = 0;

    while !p.nth_at(i, NEWLINE) && !p.nth_at(i, T![EOF]) {
        if p.nth_at(i, PIPE) {
            return true;
        }
        i += 1;
    }

    false
}
