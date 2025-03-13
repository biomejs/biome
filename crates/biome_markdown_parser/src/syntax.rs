mod blockquote_block;
mod code_block;
mod header_block;
mod list_block;
mod paragraph_block;
mod table_block;
mod thematic_break_block;

use biome_markdown_syntax::{T, kind::MarkdownSyntaxKind::*};
use biome_parser::{
    Parser,
    prelude::ParsedSyntax::{self},
};
use blockquote_block::{at_blockquote, parse_blockquote};
use code_block::{
    at_fenced_code_block, at_indent_code_block, parse_fenced_code_block, parse_indent_code_block,
};
use header_block::{at_header_block, parse_header_block};
use list_block::{at_list_block, parse_list_block};
use paragraph_block::parse_paragraph;
use table_block::{at_table, parse_table};
use thematic_break_block::{at_thematic_break_block, parse_thematic_break_block};

use crate::MarkdownParser;

pub(crate) fn parse_document(p: &mut MarkdownParser) {
    let m = p.start();

    // Unicode BOM is optional
    if p.at(UNICODE_BOM) {
        p.bump(UNICODE_BOM);
    }

    // Parse blocks until EOF
    let block_list = p.start();
    let mut has_content = false;

    while !p.at(T![EOF]) {
        // Skip any blank lines (just newlines)
        if p.at(NEWLINE) {
            p.bump(NEWLINE);
            continue;
        }

        // Try to parse any block
        if parse_any_block(p).is_present() {
            has_content = true;
            continue;
        }

        // If we can't parse a block, create a paragraph with textual nodes
        // This ensures we don't create bogus nodes for unparseable content
        let para_m = p.start();
        let item_list = p.start();

        // Parse tokens as textual until newline or EOF
        while !p.at(NEWLINE) && !p.at(T![EOF]) {
            let text_m = p.start();
            p.bump_any();
            text_m.complete(p, MD_TEXTUAL);
        }

        // Consume the newline if present
        if p.at(NEWLINE) {
            p.bump(NEWLINE);
        }

        item_list.complete(p, MD_PARAGRAPH_ITEM_LIST);
        para_m.complete(p, MD_PARAGRAPH);
        has_content = true;
    }

    // If we have no content, create an empty paragraph
    if !has_content {
        let para_m = p.start();
        let item_list = p.start();
        item_list.complete(p, MD_PARAGRAPH_ITEM_LIST);
        para_m.complete(p, MD_PARAGRAPH);
    }

    block_list.complete(p, MD_BLOCK_LIST);

    // Ensure we have an EOF token
    p.expect(T![EOF]);

    m.complete(p, MD_DOCUMENT);
}

pub(crate) fn parse_any_block(p: &mut MarkdownParser) -> ParsedSyntax {
    // Try each block type in sequence

    // Try thematic break block
    if at_thematic_break_block(p) {
        let checkpoint = p.checkpoint();
        let break_block = parse_thematic_break_block(p);
        if break_block.is_present() {
            return break_block;
        }
        p.rewind(checkpoint);
    }

    // Try header block
    if at_header_block(p) {
        let checkpoint = p.checkpoint();
        let header_block = parse_header_block(p);
        if header_block.is_present() {
            return header_block;
        }
        p.rewind(checkpoint);
    }

    // Try other block types
    if at_indent_code_block(p) {
        return parse_indent_code_block(p);
    }

    if at_fenced_code_block(p) {
        return parse_fenced_code_block(p);
    }

    if at_list_block(p) {
        let checkpoint = p.checkpoint();
        let list_block = parse_list_block(p);
        if list_block.is_present() {
            return list_block;
        }
        p.rewind(checkpoint);
    }

    if at_blockquote(p) {
        let checkpoint = p.checkpoint();
        let blockquote = parse_blockquote(p);
        if blockquote.is_present() {
            return blockquote;
        }
        p.rewind(checkpoint);
    }

    if at_table(p) {
        let checkpoint = p.checkpoint();
        let table = parse_table(p);
        if table.is_present() {
            return table;
        }
        p.rewind(checkpoint);
    }

    // If everything else fails, parse as paragraph
    parse_paragraph(p)
}
