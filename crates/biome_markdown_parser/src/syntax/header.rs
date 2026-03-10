//! ATX and Setext heading parsing for Markdown (CommonMark §4.2-4.3).
//!
//! # ATX Headings (§4.2)
//!
//! An ATX heading consists of 1-6 `#` characters followed by a space and heading
//! text. The number of `#` characters determines the heading level.
//!
//! ```markdown
//! # Heading 1
//! ## Heading 2
//! ### Heading 3
//! ```
//!
//! # Setext Headings (§4.3)
//!
//! A setext heading consists of one or more lines of text followed by an
//! underline of `=` (level 1) or `-` (level 2) characters.
//!
//! ```markdown
//! Heading 1
//! =========
//!
//! Heading 2
//! ---------
//! ```

use crate::parser::MarkdownParser;
use crate::syntax::inline::EmphasisContext;
use biome_markdown_syntax::{T, kind::MarkdownSyntaxKind::*};
use biome_parser::{
    Parser,
    prelude::ParsedSyntax::{self, *},
};

use crate::syntax::MAX_BLOCK_PREFIX_INDENT;
use crate::syntax::parse_any_inline;

/// Maximum number of `#` characters allowed in an ATX heading (CommonMark §4.2).
const MAX_HEADER_HASHES: usize = 6;

/// Check if we might be at an ATX header.
/// We only check if the current token is a HASH - full validation happens in parse_header.
pub(crate) fn at_header(p: &mut MarkdownParser) -> bool {
    p.lookahead(|p| {
        if !p.at_line_start() && !p.at_start_of_input() {
            return false;
        }
        p.skip_line_indent(MAX_BLOCK_PREFIX_INDENT);
        p.at(T![#])
    })
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

    // Check hash count BEFORE starting to parse (via lookahead).
    // The lexer emits all consecutive `#` chars as a single HASH token,
    // so we need to verify the token length doesn't exceed 6 before consuming it.
    let hash_count = p.lookahead(|p| {
        p.skip_line_indent(MAX_BLOCK_PREFIX_INDENT);
        if p.at(T![#]) { p.cur_text().len() } else { 0 }
    });

    // Validate hash count (must be 1-6)
    // Diagnostic for >6 hashes is emitted in parse_any_block before try_parse
    if hash_count > MAX_HEADER_HASHES {
        // Not a valid header - let it be parsed as paragraph
        return Absent;
    }

    let m = p.start();

    p.skip_line_indent(MAX_BLOCK_PREFIX_INDENT);

    // Parse opening hashes (MdHashList containing MdHash nodes)
    parse_hash_list(p);

    // Per CommonMark §4.2: opening hashes must be followed by space, tab, or end of line.
    // `#foo` is NOT a valid header; `# foo`, `#\tfoo`, or `#\n` are valid.
    // Check if the next token has preceding whitespace or we're at EOL/EOF.
    if !p.at_inline_end() {
        // There's a token after hashes on the same line - check for whitespace
        // In Markdown, whitespace is significant and included in token text,
        // so we check if the token starts with a space or tab character.
        let text = p.cur_text();
        let token_starts_with_whitespace = text.starts_with(' ') || text.starts_with('\t');
        if !token_starts_with_whitespace {
            // No space/tab after hashes - not a valid header (e.g., "#foo")
            m.abandon(p);
            return Absent;
        }
    }

    // Parse content (optional paragraph) - content goes until end of line
    // The header ends at a single newline (not blank line)
    parse_header_content(p);

    // Parse trailing hashes (MdHashList)
    // Trailing hashes are valid if they're at the end of the line
    parse_trailing_hashes(p);

    Present(m.complete(p, MD_HEADER))
}

/// Parse the opening hash sequence for an ATX header.
///
/// The lexer emits all consecutive `#` characters as a single HASH token.
/// We determine the heading level from the token's text length.
///
/// Creates MdHashList containing a single MdHash node that wraps the token.
///
/// Returns the number of hashes (heading level).
fn parse_hash_list(p: &mut MarkdownParser) -> usize {
    let m = p.start();
    let count = if p.at(T![#]) {
        let len = p.cur_text().len();
        // Wrap the HASH token in an MdHash node to match grammar
        let hash_m = p.start();
        p.bump(T![#]);
        hash_m.complete(p, MD_HASH);
        len
    } else {
        0
    };
    m.complete(p, MD_HASH_LIST);
    count
}

/// Parse header content - inline content for the header.
///
/// This stops at end of line (NEWLINE or EOF) or when trailing hashes are detected.
/// Note: NEWLINE is an explicit token (not trivia), so we check `at_inline_end()`.
pub(crate) fn parse_header_content(p: &mut MarkdownParser) {
    // Check if there's any content (not at EOF or NEWLINE)
    if p.at_inline_end() {
        return;
    }

    // Set up emphasis context for header content (single line only)
    let prev_context = set_header_emphasis_context(p);

    // Parse content as a paragraph containing inline items
    let m = p.start();
    let inline_m = p.start();

    loop {
        if p.at(MD_HARD_LINE_LITERAL) {
            if p.cur_text().starts_with('\\') {
                // Backslash at end of heading is literal text, not a hard break.
                let _ = super::parse_textual(p);
            } else {
                // Trailing spaces before newline — skip as trivia.
                p.parse_as_skipped_trivia_tokens(|p| p.bump(MD_HARD_LINE_LITERAL));
            }
            break;
        }

        // Check for end of line (EOF, NEWLINE, or preceding line break)
        if p.at_inline_end() {
            break;
        }

        // Check if we're at trailing hashes (optional whitespace + hashes + end of line)
        if at_trailing_hashes_start(p) {
            // Stop content parsing - trailing hashes will be parsed separately
            break;
        }

        // Parse an inline element
        if parse_any_inline(p).is_absent() {
            break;
        }
    }

    inline_m.complete(p, MD_INLINE_ITEM_LIST);
    m.complete(p, MD_PARAGRAPH);

    // Restore previous emphasis context
    p.set_emphasis_context(prev_context);
}

/// Compute the byte length of header content (up to end of line or trailing hashes).
fn header_content_source_len(p: &mut MarkdownParser) -> usize {
    p.lookahead(|p| {
        let mut len = 0usize;

        loop {
            if p.at(T![EOF]) || p.at(NEWLINE) {
                break;
            }

            // Check for trailing hashes (whitespace + # + end of line)
            if p.at(MD_TEXTUAL_LITERAL) {
                let text = p.cur_text();
                if text.chars().all(|c| c == ' ' || c == '\t') {
                    // Might be whitespace before trailing hashes
                    let ws_len = text.len();
                    p.bump(MD_TEXTUAL_LITERAL);

                    // Check if followed by hash + end of line
                    if p.at(T![#]) {
                        let hash_len = p.cur_text().len();
                        p.bump(T![#]);

                        // Skip any whitespace after hashes
                        while p.at(MD_TEXTUAL_LITERAL)
                            && p.cur_text().chars().all(|c| c == ' ' || c == '\t')
                        {
                            p.bump(MD_TEXTUAL_LITERAL);
                        }

                        if p.at(T![EOF]) || p.at(NEWLINE) {
                            // This is trailing hashes - don't include in content
                            break;
                        }
                        // Not trailing hashes - include in content
                        len += ws_len + hash_len;
                    } else {
                        len += ws_len;
                    }
                    continue;
                }
            }

            // Check for MD_HARD_LINE_LITERAL (trailing spaces/backslash)
            if p.at(MD_HARD_LINE_LITERAL) {
                // Don't include hard line in emphasis context
                break;
            }

            len += p.cur_text().len();
            p.bump_any();
        }

        len
    })
}

/// Build an emphasis context for header content and install it on the parser.
/// Returns the previous context so it can be restored.
fn set_header_emphasis_context(p: &mut MarkdownParser) -> Option<EmphasisContext> {
    let source_len = header_content_source_len(p);
    let source = p.source_after_current();
    let inline_source = if source_len <= source.len() {
        &source[..source_len]
    } else {
        source
    };
    let base_offset = u32::from(p.cur_range().start()) as usize;
    let context = EmphasisContext::new(inline_source, base_offset, |label| {
        p.has_link_reference_definition(label)
    });
    p.set_emphasis_context(Some(context))
}

/// Check if the current position has a trailing hash sequence.
/// A trailing hash sequence is one or more `#` characters followed by end of line
/// (NEWLINE or EOF), and NOT preceded by a line break (which would
/// indicate a new block, not trailing hashes).
///
/// Note: The lexer emits all consecutive `#` characters as a single HASH token,
/// so we just need to consume that one token and check for end of line.
fn is_trailing_hash_sequence(p: &mut MarkdownParser) -> bool {
    if !p.at(T![#]) {
        return false;
    }

    let checkpoint = p.checkpoint();

    // Consume the single HASH token (contains all consecutive hashes)
    p.bump(T![#]);

    // Skip any trailing whitespace after hashes (may include newline in same token)
    // Also check MD_HARD_LINE_LITERAL (5+ trailing spaces before newline)
    while p.at(MD_TEXTUAL_LITERAL) || p.at(MD_HARD_LINE_LITERAL) {
        let text = p.cur_text();
        // Accept whitespace tokens that may include newline
        if text
            .chars()
            .all(|c| c == ' ' || c == '\t' || c == '\n' || c == '\r')
        {
            p.bump_any();
        } else {
            break;
        }
    }

    let at_end_of_line = p.at_inline_end();

    p.rewind(checkpoint);

    at_end_of_line
}

fn at_trailing_hashes_start(p: &mut MarkdownParser) -> bool {
    p.lookahead(|p| {
        let mut saw_ws = false;

        while p.at(MD_TEXTUAL_LITERAL) {
            let text = p.cur_text();
            if text.chars().all(|c| c == ' ' || c == '\t') {
                saw_ws = true;
                p.bump(MD_TEXTUAL_LITERAL);
            } else {
                break;
            }
        }

        saw_ws && is_trailing_hash_sequence(p)
    })
}

/// Parse trailing hashes for ATX headers.
///
/// Per CommonMark spec, a closing sequence of `#` characters is optional.
/// It must be at the end of the line, preceded by optional whitespace.
///
/// The lexer emits all consecutive `#` characters as a single HASH token.
/// We wrap it in an MdHash node to match the grammar.
pub(crate) fn parse_trailing_hashes(p: &mut MarkdownParser) {
    let m = p.start();

    if at_trailing_hashes_start(p) {
        // Skip whitespace before trailing hashes
        while p.at(MD_TEXTUAL_LITERAL) {
            let text = p.cur_text();
            if text.chars().all(|c| c == ' ' || c == '\t') {
                p.parse_as_skipped_trivia_tokens(|p| p.bump(MD_TEXTUAL_LITERAL));
            } else {
                break;
            }
        }

        // Consume the trailing hash token and wrap in MdHash node
        if p.at(T![#]) {
            let hash_m = p.start();
            p.bump(T![#]);
            hash_m.complete(p, MD_HASH);

            // Skip any trailing whitespace AFTER the closing hashes
            // Per CommonMark §4.2, trailing whitespace after closing hashes is ignored
            // The lexer may combine trailing whitespace with the newline into a single token
            // Check both MD_TEXTUAL_LITERAL and MD_HARD_LINE_LITERAL (5+ spaces before newline)
            while p.at(MD_TEXTUAL_LITERAL) || p.at(MD_HARD_LINE_LITERAL) {
                let text = p.cur_text();
                // Check if this is whitespace-only (spaces/tabs) or whitespace+newline
                let is_trailing_ws = text
                    .chars()
                    .all(|c| c == ' ' || c == '\t' || c == '\n' || c == '\r');
                if is_trailing_ws {
                    p.parse_as_skipped_trivia_tokens(|p| p.bump_any());
                } else {
                    break;
                }
            }
        }
    }

    m.complete(p, MD_HASH_LIST);
}
