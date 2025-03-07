mod blockquote_block;
mod code_block;
mod header_block;
mod list_block;
mod paragraph_block;
mod table_block;
mod thematic_break_block;

use biome_markdown_syntax::{kind::MarkdownSyntaxKind::*, T};
use biome_parser::{prelude::ParsedSyntax, Parser};
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
        if parse_any_block(p).is_present() {
            has_content = true;
            continue;
        }

        // If we can't parse a block, create a paragraph with a textual node
        let para_m = p.start();
        let item_list = p.start();
        let text_m = p.start();
        p.bump_any();
        text_m.complete(p, MD_TEXTUAL);
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
    if at_indent_code_block(p) {
        parse_indent_code_block(p)
    } else if at_fenced_code_block(p) {
        parse_fenced_code_block(p)
    } else if at_thematic_break_block(p) {
        let break_block = try_parse(p, |p| {
            let break_block = parse_thematic_break_block(p);
            if break_block.is_absent() {
                return Err(());
            }
            Ok(break_block)
        });
        if break_block.is_err() {
            // Try to parse as paragraph
            parse_paragraph(p)
        } else {
            break_block.unwrap()
        }
    } else if at_header_block(p) {
        // Special handling for invalid headers to ensure diagnostics are emitted
        let checkpoint = p.checkpoint();
        let header_block = parse_header_block(p);

        if header_block.is_present() {
            header_block
        } else {
            // Instead of using try_parse which would discard diagnostics,
            // we rewind manually and fallback to paragraph parsing
            p.rewind(checkpoint);
            parse_paragraph(p)
        }
    } else if at_list_block(p) {
        let list_block = try_parse(p, |p| {
            let list_block = parse_list_block(p);
            if list_block.is_absent() {
                return Err(());
            }
            Ok(list_block)
        });
        if list_block.is_err() {
            // Try to parse as paragraph
            parse_paragraph(p)
        } else {
            list_block.unwrap()
        }
    } else if at_blockquote(p) {
        let blockquote = try_parse(p, |p| {
            let blockquote = parse_blockquote(p);
            if blockquote.is_absent() {
                return Err(());
            }
            Ok(blockquote)
        });
        if blockquote.is_err() {
            // Try to parse as paragraph
            parse_paragraph(p)
        } else {
            blockquote.unwrap()
        }
    } else if at_table(p) {
        let table = try_parse(p, |p| {
            let table = parse_table(p);
            if table.is_absent() {
                return Err(());
            }
            Ok(table)
        });
        if table.is_err() {
            // Try to parse as paragraph
            parse_paragraph(p)
        } else {
            table.unwrap()
        }
    } else {
        // Try to parse as paragraph
        parse_paragraph(p)
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
