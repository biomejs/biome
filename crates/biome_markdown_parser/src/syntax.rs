use biome_markdown_syntax::{kind::MarkdownSyntaxKind::*, T};
use biome_parser::Parser;

use crate::MarkdownParser;

pub(crate) fn parse_document(p: &mut MarkdownParser) {
    let m = p.start();
    parse_block_list(p);
    m.complete(p, MARKDOWN_DOCUMENT);
}

pub(crate) fn parse_block_list(p: &mut MarkdownParser) {
    let m = p.start();

    while !p.at(T![EOF]) {
        parse_any_block(p);
    }
    m.complete(p, MARKDOWN_BLOCK_LIST);
}

pub(crate) fn parse_any_block(p: &mut MarkdownParser) {
    if at_break_block(p) {
        parse_break_block(p);
    }
}

pub(crate) fn at_break_block(p: &mut MarkdownParser) -> bool {
    p.at(MARKDOWN_BREAK_BLOCK_LITERAL)
}

pub(crate) fn parse_break_block(_p: &mut MarkdownParser) {
    todo!()
}
