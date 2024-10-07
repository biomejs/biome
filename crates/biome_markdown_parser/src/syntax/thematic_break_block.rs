use crate::parser::MarkdownParser;
use biome_markdown_syntax::MarkdownSyntaxKind::*;
use biome_parser::{
    prelude::ParsedSyntax::{self, *},
    Parser,
};

pub(crate) fn at_thematic_break_block(p: &mut MarkdownParser) -> bool {
    p.at(MD_THEMATIC_BREAK_LITERAL)
}

pub(crate) fn parse_thematic_break_block(p: &mut MarkdownParser) -> ParsedSyntax {
    if !at_thematic_break_block(p) {
        return Absent;
    }
    let m = p.start();

    p.expect(MD_THEMATIC_BREAK_LITERAL);

    Present(m.complete(p, MD_THEMATIC_BREAK_BLOCK))
}
