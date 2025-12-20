use crate::parser::MarkdownParser;
use biome_markdown_syntax::{T, kind::MarkdownSyntaxKind::*};
use biome_parser::{
    Parser,
    prelude::ParsedSyntax::{self, *},
};

/// Check if we're at a fenced code block (``` or ~~~).
pub(crate) fn at_fenced_code_block(p: &mut MarkdownParser) -> bool {
    p.at(T!["```"]) || p.at(TRIPLE_TILDE)
}

/// Parse a fenced code block.
///
/// Grammar:
/// MdFencedCodeBlock =
///   l_fence: ('```' | '~~~')
///   code_list: MdCodeNameList
///   content: MdInlineItemList
///   r_fence: ('```' | '~~~')
pub(crate) fn parse_fenced_code_block(p: &mut MarkdownParser) -> ParsedSyntax {
    if !at_fenced_code_block(p) {
        return Absent;
    }

    let m = p.start();

    // Track which fence type we opened with (must close with same type per CommonMark)
    let is_tilde_fence = p.at(TRIPLE_TILDE);

    // Opening fence (``` or ~~~)
    if is_tilde_fence {
        p.bump(TRIPLE_TILDE);
    } else {
        p.bump(T!["```"]);
    }

    // Optional language info string (MdCodeNameList)
    parse_code_name_list(p);

    // Content (everything until closing fence)
    parse_code_content(p, is_tilde_fence);

    // Closing fence - emits diagnostic if missing
    // Must match the opening fence type
    if is_tilde_fence {
        p.expect(TRIPLE_TILDE);
    } else {
        p.expect(T!["```"]);
    }

    Present(m.complete(p, MD_FENCED_CODE_BLOCK))
}

/// Parse the code name list (language info string).
/// Grammar: MdCodeNameList = MdTextual*
///
/// The language name is on the same line as the opening fence.
/// If the current token has a preceding line break, the code block has no language.
fn parse_code_name_list(p: &mut MarkdownParser) {
    let m = p.start();

    // If the current token is already on a new line, there's no language name
    if p.has_preceding_line_break() {
        m.complete(p, MD_CODE_NAME_LIST);
        return;
    }

    // Parse language identifiers until we hit a newline
    let start_pos = p.trivia_position();

    while !p.at(T![EOF]) && !p.has_newline_since(start_pos) {
        // Parse each token as textual content
        let text_m = p.start();
        p.bump_any();
        text_m.complete(p, MD_TEXTUAL);
    }

    m.complete(p, MD_CODE_NAME_LIST);
}

/// Parse the code content until we find a closing fence.
/// Grammar: content: MdInlineItemList
fn parse_code_content(p: &mut MarkdownParser, is_tilde_fence: bool) {
    let m = p.start();

    // Consume all tokens until we see the matching closing fence or EOF
    while !p.at(T![EOF]) {
        // Check for closing fence (must match opening fence type)
        if is_tilde_fence && p.at(TRIPLE_TILDE) {
            break;
        }
        if !is_tilde_fence && p.at(T!["```"]) {
            break;
        }

        let text_m = p.start();
        p.bump_any();
        text_m.complete(p, MD_TEXTUAL);
    }

    m.complete(p, MD_INLINE_ITEM_LIST);
}
