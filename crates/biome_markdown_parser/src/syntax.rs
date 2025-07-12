pub mod thematic_break_block;
pub mod atx_headings;

use biome_markdown_syntax::{T, kind::MarkdownSyntaxKind::*};
use biome_parser::{
    prelude::{ParsedSyntax::{self, *}, TokenSource},
    Parser,
};
use thematic_break_block::{at_thematic_break_block, parse_thematic_break_block};
use atx_headings::{at_atx_heading, parse_atx_heading};

use crate::MarkdownParser;

pub(crate) fn parse_document(p: &mut MarkdownParser) {
    let m = p.start();
    let _ = parse_block_list(p);
    m.complete(p, MD_DOCUMENT);
}

pub(crate) fn parse_block_list(p: &mut MarkdownParser) -> ParsedSyntax {
    let m = p.start();

    while !p.at(T![EOF]) {
        parse_any_block(p);
    }
    Present(m.complete(p, MD_BLOCK_LIST))
}

pub(crate) fn parse_any_block(p: &mut MarkdownParser) {
    if at_atx_heading(p) {
        let _ = parse_atx_heading(p);
    } else if at_indent_code_block(p) {
        parse_indent_code_block(p);
    } else if at_thematic_break_block(p) {
        let break_block = try_parse(p, |p| {
            let break_block = parse_thematic_break_block(p);
            if break_block.is_absent() {
                return Err(());
            }
            Ok(break_block)
        });
        if break_block.is_err() {
            let _ = parse_paragraph(p);
        }
    } else {
        let _ = parse_paragraph(p);
    }
}

pub(crate) fn at_indent_code_block(p: &mut MarkdownParser) -> bool {
    p.before_whitespace_count() > 4
}

pub(crate) fn parse_indent_code_block(_p: &mut MarkdownParser) {
    todo!()
}

pub(crate) fn parse_paragraph(p: &mut MarkdownParser) -> ParsedSyntax {
    let m = p.start();

    // Parse paragraph content until a blank line, EOF, or another block element
    parse_paragraph_line(p);

    // Additional lines in the paragraph
    while !p.at(T![EOF]) &&
          !is_blank_line(p) &&
          !at_atx_heading(p) &&
          !at_thematic_break_block(p) &&
          !at_indent_code_block(p) {
        parse_paragraph_line(p);
    }

    Present(m.complete(p, MD_PARAGRAPH))
}

// Helper to check if we're at a blank line
fn is_blank_line(p: &mut MarkdownParser) -> bool {
    // A simple check for a blank line - just newline or whitespace followed by newline
    p.at(NEWLINE) || (p.at(WHITESPACE) && p.nth(1) == NEWLINE)
}

// Renamed to be clearer that this parses a single line of paragraph content
pub(crate) fn parse_paragraph_line(p: &mut MarkdownParser) {
    // Parse until end of line or end of file
    while !p.at(T![EOF]) && !p.at(NEWLINE) {
        p.bump(p.source().current());
    }

    // Consume the newline if present
    if p.at(NEWLINE) {
        p.bump(NEWLINE);
    }
}

/// Attempt to parse some input with the given parsing function. If parsing
/// succeeds, `Ok` is returned with the result of the parse and the state is
/// preserved. If parsing fails, this function rewinds the parser back to
/// where it was before attempting the parse and the `Err` value is returned.
#[must_use = "The result of try_parse contains information about whether the parse succeeded and should not be ignored"]
pub(crate) fn try_parse<T, E>(
    p: &mut MarkdownParser,
    func: impl FnOnce(&mut MarkdownParser) -> Result<T, E>,
) -> Result<T, E> {
    let checkpoint = p.checkpoint();

    let res = func(p);

    if res.is_err() {
        p.rewind(checkpoint);
    }

    res
}
