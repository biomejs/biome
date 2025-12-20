use crate::parser::MarkdownParser;
use biome_markdown_syntax::{T, kind::MarkdownSyntaxKind::*};
use biome_parser::{
    Parser,
    prelude::ParsedSyntax::{self, *},
};

/// Check if we're at a fenced code block (``` or ~~~).
pub(crate) fn at_fenced_code_block(p: &mut MarkdownParser) -> bool {
    p.at(T!["```"]) || at_triple_tilde(p)
}

/// Check if we're at triple tilde (~~~) for fenced code blocks.
/// The lexer produces individual TILDE tokens, so we need to check for 3 consecutive.
fn at_triple_tilde(_p: &mut MarkdownParser) -> bool {
    // For now, we only support ``` since the lexer produces TRIPLE_BACKTICK as a single token.
    // Tilde support would require either a lexer change or lookahead.
    // TODO: Add tilde fence support
    false
}

/// Parse a fenced code block.
///
/// Grammar:
/// MdFencedCodeBlock =
///   l_fence: '```'
///   code_list: MdCodeNameList
///   content: MdInlineItemList
///   r_fence: '```'?
pub(crate) fn parse_fenced_code_block(p: &mut MarkdownParser) -> ParsedSyntax {
    if !at_fenced_code_block(p) {
        return Absent;
    }

    let m = p.start();

    // Opening fence (```)
    p.bump(T!["```"]);

    // Optional language info string (MdCodeNameList)
    parse_code_name_list(p);

    // Content (everything until closing fence) - optional
    parse_code_content(p);

    // Closing fence - optional (unclosed code blocks are valid but incomplete)
    if p.at(T!["```"]) {
        p.bump(T!["```"]);
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
fn parse_code_content(p: &mut MarkdownParser) {
    let m = p.start();

    // Consume all tokens until we see a closing fence (```) or EOF
    while !p.at(T![EOF]) && !p.at(T!["```"]) {
        let text_m = p.start();
        p.bump_any();
        text_m.complete(p, MD_TEXTUAL);
    }

    m.complete(p, MD_INLINE_ITEM_LIST);
}
