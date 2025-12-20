pub mod thematic_break_block;

use biome_markdown_syntax::{T, kind::MarkdownSyntaxKind::*};
use biome_parser::{
    Parser,
    prelude::ParsedSyntax::{self, *},
};
use thematic_break_block::{at_thematic_break_block, parse_thematic_break_block};

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
    if at_indent_code_block(p) {
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
            parse_paragraph(p);
        }
    } else {
        // Default fallback: parse as paragraph
        parse_paragraph(p);
    }
}

pub(crate) fn at_indent_code_block(p: &mut MarkdownParser) -> bool {
    p.before_whitespace_count() > 4
}

pub(crate) fn parse_indent_code_block(_p: &mut MarkdownParser) {
    todo!()
}

/// Parse a paragraph block.
///
/// A paragraph is a sequence of non-blank lines that cannot be interpreted as
/// other kinds of blocks. The paragraph ends at a blank line or EOF.
///
/// Grammar: MdParagraph = list: MdInlineItemList hard_line: MdHardLine?
pub(crate) fn parse_paragraph(p: &mut MarkdownParser) {
    let m = p.start();

    // Parse inline content list
    parse_inline_item_list(p);

    // Note: hard_line is optional in the grammar. We don't emit it here because
    // newlines are treated as trivia. When explicit hard breaks (like trailing
    // spaces or backslash) are supported, we'll emit MD_HARD_LINE_LITERAL.

    m.complete(p, MD_PARAGRAPH);
}

/// Parse the inline item list within a block.
///
/// Grammar: MdInlineItemList = AnyMdInline*
pub(crate) fn parse_inline_item_list(p: &mut MarkdownParser) {
    let m = p.start();

    // Record starting trivia position to detect blank lines during parsing
    let start_trivia_pos = p.trivia_position();

    // Continue parsing inline items until we hit:
    // - EOF
    // - A blank line (detected via 2+ newlines in trivia since we started)
    // - A block-level construct
    while !p.at(T![EOF]) {
        if parse_any_inline(p).is_absent() {
            // If we couldn't parse any inline, break to avoid infinite loop
            break;
        }
        // Check for blank line AFTER bumping a token (so trivia is collected)
        if p.has_blank_line_since(start_trivia_pos) {
            break;
        }
    }

    m.complete(p, MD_INLINE_ITEM_LIST);
}

/// Parse any inline element.
///
/// For now, this consumes any non-block token as textual content.
/// Other inline types (emphasis, links, etc.) will be added in later prompts.
pub(crate) fn parse_any_inline(p: &mut MarkdownParser) -> ParsedSyntax {
    // For now, consume any token as textual content
    // Later: add parse_inline_code, parse_inline_emphasis, etc.
    parse_textual(p)
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
    // Bump any token - treat everything as textual for now
    p.bump_any();
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
