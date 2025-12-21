pub mod fenced_code_block;
pub mod header;
pub mod inline;
pub mod list;
pub mod quote;
pub mod thematic_break_block;

use biome_markdown_syntax::{T, kind::MarkdownSyntaxKind::*};
use biome_parser::{
    Parser,
    prelude::ParsedSyntax::{self, *},
};
use fenced_code_block::{at_fenced_code_block, parse_fenced_code_block};
use header::{at_header, parse_header};
use list::{at_bullet_list_item, parse_bullet_list_item};
use quote::{at_quote, parse_quote};
use thematic_break_block::{at_thematic_break_block, parse_thematic_break_block};

use crate::MarkdownParser;

/// CommonMark requires 4 or more spaces for indented code blocks.
const INDENT_CODE_BLOCK_SPACES: usize = 4;

pub(crate) fn parse_document(p: &mut MarkdownParser) {
    let m = p.start();
    let _ = parse_block_list(p);
    // Bump the EOF token - required by the grammar
    p.bump(T![EOF]);
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
    if at_indent_code_block(p) {
        parse_indent_code_block(p);
    } else if at_fenced_code_block(p) {
        let _ = parse_fenced_code_block(p);
    } else if at_thematic_break_block(p) {
        let break_block = try_parse(p, |p| {
            let break_block = parse_thematic_break_block(p);
            if break_block.is_absent() {
                return Err(());
            }
            Ok(break_block)
        });
        if break_block.is_err() {
            parse_paragraph(p);
        }
    } else if at_header(p) {
        let header_result = try_parse(p, |p| {
            let header = parse_header(p);
            if header.is_absent() {
                return Err(());
            }
            Ok(header)
        });
        if header_result.is_err() {
            // Not a valid header (e.g., > 6 hashes), parse as paragraph
            parse_paragraph(p);
        }
    } else if at_quote(p) {
        let _ = parse_quote(p);
    } else if at_bullet_list_item(p) {
        let _ = parse_bullet_list_item(p);
    } else {
        // Default fallback: parse as paragraph
        parse_paragraph(p);
    }
}

/// Check if we're at an indented code block (4+ spaces of indentation).
pub(crate) fn at_indent_code_block(p: &mut MarkdownParser) -> bool {
    p.before_whitespace_count() >= INDENT_CODE_BLOCK_SPACES
}

/// Parse an indented code block.
///
/// Grammar: MdIndentCodeBlock = content: MdInlineItemList
///
/// An indented code block consists of one or more lines with 4+ spaces
/// of indentation. The indentation is tracked in trivia.
pub(crate) fn parse_indent_code_block(p: &mut MarkdownParser) {
    if !at_indent_code_block(p) {
        return;
    }

    let m = p.start();
    let content = p.start();

    // Parse content while we're at indented lines
    // Continue until we hit a blank line or a line without 4+ space indent
    while !p.at(T![EOF]) && at_indent_code_block(p) {
        let text_m = p.start();
        // Remap any token to MD_TEXTUAL_LITERAL so the syntax factory accepts it.
        p.bump_remap(MD_TEXTUAL_LITERAL);
        text_m.complete(p, MD_TEXTUAL);
    }

    content.complete(p, MD_INLINE_ITEM_LIST);
    m.complete(p, MD_INDENT_CODE_BLOCK);
}

/// Parse a paragraph block.
///
/// A paragraph is a sequence of non-blank lines that cannot be interpreted as
/// other kinds of blocks. The paragraph ends at a blank line or EOF.
///
/// Grammar: MdParagraph = list: MdInlineItemList hard_line: MdHardLine?
pub(crate) fn parse_paragraph(p: &mut MarkdownParser) {
    let m = p.start();

    parse_inline_item_list(p);

    m.complete(p, MD_PARAGRAPH);
}

/// Parse the inline item list within a block.
///
/// Grammar: MdInlineItemList = AnyMdInline*
///
/// Inline content continues until we hit EOF or a blank line (paragraph boundary).
/// A blank line is detected by checking if the current token has two or more
/// newlines in its leading trivia.
pub(crate) fn parse_inline_item_list(p: &mut MarkdownParser) {
    let m = p.start();

    // Track trivia position to detect blank lines (paragraph boundaries)
    let start_trivia_pos = p.trivia_position();

    while !p.at(T![EOF]) {
        // Check if we've crossed a blank line (paragraph boundary)
        // This happens when there are 2+ consecutive newlines in the trivia
        if p.has_blank_line_since(start_trivia_pos) {
            break;
        }

        if parse_any_inline(p).is_absent() {
            break;
        }
    }

    m.complete(p, MD_INLINE_ITEM_LIST);
}

/// Parse any inline element.
///
/// Dispatches to the appropriate inline parser based on the current token.
pub(crate) fn parse_any_inline(p: &mut MarkdownParser) -> ParsedSyntax {
    inline::parse_any_inline(p)
}

/// Parse a textual inline element.
///
/// Grammar: MdTextual = value: 'md_textual_literal'
///
/// For now, we treat any non-EOF token as textual content to ensure
/// the paragraph parser makes progress. In later prompts, we'll add
/// proper handling for inline elements like emphasis, links, etc.
pub(crate) fn parse_textual(p: &mut MarkdownParser) -> ParsedSyntax {
    if p.at(T![EOF]) {
        return Absent;
    }
    let m = p.start();
    // Remap any token to MD_TEXTUAL_LITERAL so the syntax factory accepts it.
    // This is necessary because tokens like L_PAREN, R_PAREN, etc. are lexed
    // as their specific token kinds, but MdTextual expects MD_TEXTUAL_LITERAL.
    p.bump_remap(MD_TEXTUAL_LITERAL);
    Present(m.complete(p, MD_TEXTUAL))
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
