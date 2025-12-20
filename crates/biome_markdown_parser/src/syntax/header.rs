use crate::parser::MarkdownParser;
use biome_markdown_syntax::{T, kind::MarkdownSyntaxKind::*};
use biome_parser::{
    Parser,
    prelude::ParsedSyntax::{self, *},
};

use super::parse_any_inline;

/// Check if we might be at an ATX header.
/// We only check if the current token is a HASH - full validation happens in parse_header.
pub(crate) fn at_header(p: &mut MarkdownParser) -> bool {
    p.at(T![#])
}

/// Parse an ATX header.
///
/// Grammar: MdHeader = before: MdHashList content: MdParagraph? after: MdHashList
///
/// ATX headers start with 1-6 `#` characters followed by space or end of line.
/// More than 6 `#` characters is not a valid header.
///
/// Trailing hashes are optional and must be at the end of the line.
pub(crate) fn parse_header(p: &mut MarkdownParser) -> ParsedSyntax {
    if !at_header(p) {
        return Absent;
    }

    let m = p.start();

    // Parse opening hashes (MdHashList containing MdHash nodes)
    let hash_count = parse_hash_list(p);

    // Validate hash count (must be 1-6)
    if hash_count > 6 {
        // Too many hashes - not a valid header
        m.abandon(p);
        return Absent;
    }

    // Parse content (optional paragraph) - content goes until end of line
    // The header ends at a single newline (not blank line)
    parse_header_content(p);

    // Parse trailing hashes (MdHashList)
    // Trailing hashes are valid if they're at the end of the line
    parse_trailing_hashes(p);

    Present(m.complete(p, MD_HEADER))
}

/// Parse a list of hash tokens as MdHashList containing MdHash nodes.
/// Returns the number of hashes parsed.
fn parse_hash_list(p: &mut MarkdownParser) -> usize {
    let m = p.start();
    let mut count = 0;

    while p.at(T![#]) {
        let hash_m = p.start();
        p.bump(T![#]);
        hash_m.complete(p, MD_HASH);
        count += 1;
    }

    m.complete(p, MD_HASH_LIST);
    count
}

/// Parse header content - inline content for the header.
/// This stops at end of line (single newline in leading trivia of next token).
fn parse_header_content(p: &mut MarkdownParser) {
    // Check if there's any content (not at EOF)
    if p.at(T![EOF]) {
        return;
    }

    // Record starting trivia position
    let start_trivia_pos = p.trivia_position();

    // Check if we're immediately at a newline (empty header content)
    // This happens when header is just "# " followed by newline
    if has_newline_since(p, start_trivia_pos) {
        return;
    }

    // Parse content as a paragraph containing inline items
    let m = p.start();
    let inline_m = p.start();

    loop {
        // Check for EOF
        if p.at(T![EOF]) {
            break;
        }

        // Parse an inline element
        if parse_any_inline(p).is_absent() {
            break;
        }

        // Check if we've hit a newline (header ends at single newline)
        // We check AFTER parsing to ensure the newline is in the trivia
        if has_newline_since(p, start_trivia_pos) {
            break;
        }
    }

    inline_m.complete(p, MD_INLINE_ITEM_LIST);
    m.complete(p, MD_PARAGRAPH);
}

/// Check if there's any newline in the trivia since the given position.
/// Unlike has_blank_line_since which requires 2+ newlines, this triggers on 1 newline.
fn has_newline_since(p: &mut MarkdownParser, since_pos: usize) -> bool {
    p.has_newline_since(since_pos)
}

/// Parse trailing hashes for ATX headers.
///
/// Per CommonMark spec, a closing sequence of `#` characters is optional.
/// It must be at the end of the line, preceded by optional whitespace.
///
/// Note: Currently, trailing hashes are included as part of the header content
/// (in MdParagraph). Properly separating trailing hashes would require lookahead
/// to distinguish between hashes in the middle of content vs. at the end of line.
/// The CST still contains all the text; only the structural representation differs.
fn parse_trailing_hashes(p: &mut MarkdownParser) {
    let m = p.start();
    // Emit empty MdHashList - trailing hashes are currently part of content
    m.complete(p, MD_HASH_LIST);
}
