//! List parsing for Markdown.
//!
//! Supports bullet lists (`-`, `*`, `+`) and ordered lists (`1.`, `2.`, etc.).

use biome_markdown_syntax::T;
use biome_markdown_syntax::kind::MarkdownSyntaxKind::{self, *};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::prelude::ParsedSyntax::{self, *};
use biome_parser::prelude::{ParseDiagnostic, TokenSet};
use biome_parser::{Parser, token_set};
use biome_rowan::TextRange;

use crate::MarkdownParser;

/// Tokens that start a new block (used for recovery)
const BLOCK_RECOVERY_SET: TokenSet<MarkdownSyntaxKind> = token_set![
    T![-],
    T![*],
    T![+],
    T![>],
    T![#],
    TRIPLE_BACKTICK,
    TRIPLE_TILDE
];

/// Check if we're at the start of a bullet list item (`-`, `*`, or `+`).
///
/// A bullet list marker at line start followed by content is a list item.
/// We check that it's at line start and not a thematic break.
pub(crate) fn at_bullet_list_item(p: &mut MarkdownParser) -> bool {
    // Check for -, *, or + at the start of a line
    // Thematic breaks (--- or ***) are lexed as MD_THEMATIC_BREAK_LITERAL,
    // so if we see MINUS, STAR, or PLUS, it's a single character marker
    if !p.at(T![-]) && !p.at(T![*]) && !p.at(T![+]) {
        return false;
    }

    // For a bullet list, the marker should be at line start.
    // This is true if:
    // 1. We have a preceding line break in trivia, OR
    // 2. We're at the very start of the document (no trivia collected yet)
    p.has_preceding_line_break() || p.at_start_of_input()
}

/// Struct implementing `ParseNodeList` for bullet lists.
struct BulletList;

impl ParseNodeList for BulletList {
    type Kind = MarkdownSyntaxKind;
    type Parser<'source> = MarkdownParser<'source>;

    const LIST_KIND: Self::Kind = MD_BULLET_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_bullet(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        // List ends when we're no longer at a bullet marker at line start
        !at_bullet_list_item(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(MD_BOGUS_BULLET, BLOCK_RECOVERY_SET)
                .enable_recovery_on_line_break(),
            expected_bullet,
        )
    }
}

/// Error builder for bullet list recovery
fn expected_bullet(p: &MarkdownParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Expected a list item", range)
        .with_hint("List items start with `-`, `*`, or `+` at the beginning of a line")
}

/// Parse a bullet list item.
///
/// Grammar:
/// MdBulletListItem = MdBulletList
/// MdBulletList = MdBullet*
/// MdBullet = bullet: ('-' | '*') content: MdInlineItemList
///
/// Parses consecutive bullet items into a single list. Each bullet includes
/// its marker and inline content. The list continues until we're no longer
/// at a bullet marker at line start.
pub(crate) fn parse_bullet_list_item(p: &mut MarkdownParser) -> ParsedSyntax {
    if !at_bullet_list_item(p) {
        return Absent;
    }

    let item_m = p.start();

    // Use ParseNodeList to parse the list with proper recovery
    BulletList.parse_list(p);

    Present(item_m.complete(p, MD_BULLET_LIST_ITEM))
}

/// Parse a single bullet (marker + content).
///
/// Returns `Present` if a bullet was successfully parsed, `Absent` otherwise.
fn parse_bullet(p: &mut MarkdownParser) -> ParsedSyntax {
    // Must be at a bullet marker at line start
    if !at_bullet_list_item(p) {
        return Absent;
    }

    let m = p.start();

    // Bump the bullet marker (-, *, or +)
    // The space after the marker is consumed as trailing trivia on this token
    if p.at(T![-]) {
        p.bump(T![-]);
    } else if p.at(T![*]) {
        p.bump(T![*]);
    } else {
        p.bump(T![+]);
    }

    // Parse inline content - stops at line break
    parse_bullet_content(p);

    Present(m.complete(p, MD_BULLET))
}

/// Parse the inline content of a bullet item.
/// Stops at line break (unlike paragraph which continues across lines).
fn parse_bullet_content(p: &mut MarkdownParser) {
    let m = p.start();

    // Parse inline items until we hit:
    // - EOF
    // - A newline (list items are single-line)
    loop {
        if p.at(T![EOF]) {
            break;
        }
        // Check for newline BEFORE parsing - if the next token has a preceding
        // line break, we're at the start of the next line
        if p.has_preceding_line_break() {
            break;
        }
        if super::parse_any_inline(p).is_absent() {
            break;
        }
    }

    m.complete(p, MD_INLINE_ITEM_LIST);
}

/// Check if we're at the start of an ordered list item (digit followed by `.` or `)`).
#[expect(dead_code)]
pub(crate) fn at_order_list_item(_p: &mut MarkdownParser) -> bool {
    // For now, ordered lists are not implemented
    // Would need to check for digit tokens followed by . or )
    false
}

/// Parse an ordered list item.
#[expect(dead_code)]
pub(crate) fn parse_order_list_item(_p: &mut MarkdownParser) -> ParsedSyntax {
    // For now, ordered lists are not implemented
    Absent
}
