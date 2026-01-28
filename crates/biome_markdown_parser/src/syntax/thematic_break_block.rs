//! Thematic break parsing for Markdown (CommonMark §4.1).
//!
//! A thematic break (horizontal rule) is a line consisting of three or more
//! matching `-`, `_`, or `*` characters, optionally with spaces between them.
//!
//! # Examples
//!
//! ```markdown
//! ---
//! ***
//! ___
//! - - -
//! *  *  *
//! ```

use crate::parser::MarkdownParser;
use biome_markdown_syntax::MarkdownSyntaxKind::*;
use biome_parser::{
    Parser,
    prelude::ParsedSyntax::{self, *},
};

pub(crate) fn at_thematic_break_block(p: &mut MarkdownParser) -> bool {
    p.lookahead(|p| {
        if !p.at_line_start() && !p.at_start_of_input() {
            return false;
        }
        if p.line_start_leading_indent() > 3 {
            return false;
        }
        p.skip_line_indent(3);
        p.at(MD_THEMATIC_BREAK_LITERAL)
    })
}

pub(crate) fn parse_thematic_break_block(p: &mut MarkdownParser) -> ParsedSyntax {
    if !at_thematic_break_block(p) {
        return Absent;
    }
    let m = p.start();

    p.skip_line_indent(3);

    p.expect(MD_THEMATIC_BREAK_LITERAL);

    Present(m.complete(p, MD_THEMATIC_BREAK_BLOCK))
}
