//! List parsing for Markdown (CommonMark §5.2-5.3).
//!
//! Supports bullet lists (`-`, `*`, `+`) and ordered lists (`1.`, `2.`, etc.).
//! Also supports multi-line list items via continuation lines and nested lists.
//!
//! # CommonMark Specification References
//!
//! - **§5.2 List items**: A list item is a sequence of blocks that belong to a
//!   single list marker. Items can span multiple lines with proper indentation.
//! - **§5.3 Lists**: A list is a sequence of list items of the same type (bullet
//!   or ordered) that are not separated by blank lines (tight) or are (loose).
//!
//! ## Bullet List Markers (§5.2)
//! - `-` (hyphen-minus)
//! - `*` (asterisk)
//! - `+` (plus sign)
//!
//! ## Ordered List Markers (§5.2)
//! - `1.` through `999999999.` (1-9 digits followed by `.`)
//! - `1)` through `999999999)` (1-9 digits followed by `)`)
//!
//! ## Depth Limits
//!
//! To prevent stack overflow from pathological input (deeply nested lists),
//! nesting depth is limited by `MarkdownParseOptions::max_nesting_depth`
//! (default: 100). Deeper nesting emits a diagnostic and treats additional
//! list markers as content.
//!
//! ## Current Limitations
//!
//! - **Tight vs loose lists**: Not yet tracked; affects HTML output formatting.
//! - **List interruption rules**: Some constructs can interrupt lists; not all
//!   rules from CommonMark are implemented.

use biome_markdown_syntax::T;
use biome_markdown_syntax::kind::MarkdownSyntaxKind::{self, *};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::prelude::ParsedSyntax::{self, *};
use biome_parser::prelude::{CompletedMarker, Marker, ParseDiagnostic, TokenSet};
use biome_parser::{Parser, token_set};

use super::quote::{consume_quote_prefix, consume_quote_prefix_without_virtual, has_quote_prefix};
use biome_rowan::TextRange;

use super::fenced_code_block::parse_fenced_code_block;
use super::parse_error::list_nesting_too_deep;
use super::{ParsedBlockKind, at_block_interrupt, at_indent_code_block};
use crate::MarkdownParser;
use crate::syntax::parse_any_block_with_indent_code_policy;

/// Tokens that start a new block (used for recovery)
const BLOCK_RECOVERY_SET: TokenSet<MarkdownSyntaxKind> = token_set![
    T![-],
    T![*],
    T![+],
    T![>],
    T![#],
    TRIPLE_BACKTICK,
    TRIPLE_TILDE,
    MD_ORDERED_LIST_MARKER
];
/// CommonMark requires 4 or more spaces for indented code blocks.
const INDENT_CODE_BLOCK_SPACES: usize = 4;

/// Check if we're at the start of a bullet list item (`-`, `*`, or `+`).
///
/// A bullet list marker at line start followed by content is a list item.
/// We check that it's at line start and not a thematic break.
pub(crate) fn at_bullet_list_item(p: &mut MarkdownParser) -> bool {
    at_bullet_list_item_with_base_indent(p, list_marker_base_indent(p))
}

fn list_marker_base_indent(p: &MarkdownParser) -> usize {
    p.state().list_item_required_indent
}

fn list_item_within_indent(p: &mut MarkdownParser, base_indent: usize) -> bool {
    if !p.at_line_start() {
        return false;
    }

    let indent = p.line_start_leading_indent();
    let base_indent =
        if p.state().virtual_line_start == Some(p.cur_range().start()) && base_indent > 0 {
            0
        } else {
            base_indent
        };

    if base_indent == 0 {
        indent <= 3
    } else {
        indent >= base_indent && indent <= base_indent + 3
    }
}

fn skip_leading_whitespace_tokens(p: &mut MarkdownParser) {
    while p.at(MD_TEXTUAL_LITERAL) && is_whitespace_only(p.cur_text()) {
        p.bump(MD_TEXTUAL_LITERAL);
    }
}

fn skip_list_marker_indent(p: &mut MarkdownParser) {
    while p.at(MD_TEXTUAL_LITERAL) && is_whitespace_only(p.cur_text()) {
        p.parse_as_skipped_trivia_tokens(|p| p.bump(MD_TEXTUAL_LITERAL));
    }
}

fn is_whitespace_only(text: &str) -> bool {
    !text.is_empty() && text.chars().all(|c| c == ' ' || c == '\t')
}

fn at_bullet_list_item_with_base_indent(p: &mut MarkdownParser, base_indent: usize) -> bool {
    p.lookahead(|p| {
        if !list_item_within_indent(p, base_indent) {
            return false;
        }

        skip_leading_whitespace_tokens(p);

        // Check for -, *, or + at the start of a line
        // Thematic breaks (--- or ***) are lexed as MD_THEMATIC_BREAK_LITERAL,
        // so if we see MINUS, STAR, or PLUS, it's a single character marker.
        // A single-dash setext underline token can also represent an empty list item.
        if p.at(MD_SETEXT_UNDERLINE_LITERAL) {
            if !is_single_dash_setext_marker(p.cur_text()) {
                return false;
            }
        } else if p.at(MD_TEXTUAL_LITERAL) {
            if !is_textual_bullet_marker(p.cur_text()) {
                return false;
            }
        } else if !p.at(T![-]) && !p.at(T![*]) && !p.at(T![+]) {
            return false;
        }

        if p.at(MD_SETEXT_UNDERLINE_LITERAL) {
            p.bump_remap(T![-]);
        } else if p.at(MD_TEXTUAL_LITERAL) {
            let text = p.cur_text();
            if text == "-" {
                p.bump_remap(T![-]);
            } else if text == "*" {
                p.bump_remap(T![*]);
            } else if text == "+" {
                p.bump_remap(T![+]);
            } else {
                return false;
            }
        } else {
            p.bump(p.cur());
        }
        marker_followed_by_whitespace_or_eol(p)
    })
}

pub(crate) fn marker_followed_by_whitespace_or_eol(p: &mut MarkdownParser) -> bool {
    if p.at(NEWLINE) || p.at(T![EOF]) {
        return true;
    }

    if p.at(MD_TEXTUAL_LITERAL) {
        let text = p.cur_text();
        return text.starts_with(' ') || text.starts_with('\t');
    }

    false
}

/// Tracks blank-line information for a list item.
#[derive(Default)]
struct ListItemBlankInfo {
    /// True if a blank line occurred anywhere within the item content.
    has_blank_line: bool,
    /// True if the item ended with a blank line.
    ends_with_blank_line: bool,
}

fn skip_blank_lines_between_items(
    p: &mut MarkdownParser,
    has_item_after_blank_lines: fn(&mut MarkdownParser) -> bool,
    is_tight: &mut bool,
    last_item_ends_with_blank: &mut bool,
) {
    // Skip blank lines between list items.
    // Per CommonMark §5.3, blank lines between items make the list loose
    // but don't end the list.
    while p.at(NEWLINE) {
        // Only skip if there's another list item after the blank lines
        if !has_item_after_blank_lines(p) {
            break;
        }
        // Blank lines between items make the list loose
        *is_tight = false;
        *last_item_ends_with_blank = true;
        // Skip the blank line as trivia (no tree node created)
        p.parse_as_skipped_trivia_tokens(|p| p.bump(NEWLINE));
    }
}

fn update_list_tightness(
    blank_info: ListItemBlankInfo,
    is_tight: &mut bool,
    last_item_ends_with_blank: &mut bool,
) {
    // Blank line between items makes the list loose
    if *last_item_ends_with_blank {
        *is_tight = false;
    }

    // Blank line inside an item makes the list loose
    if blank_info.has_blank_line {
        *is_tight = false;
    }

    *last_item_ends_with_blank = blank_info.ends_with_blank_line;
}

fn parse_list_element_common<M, FMarker, FParse>(
    p: &mut MarkdownParser,
    marker_state: &mut Option<M>,
    current_marker: FMarker,
    parse_item: FParse,
    has_item_after_blank_lines: fn(&mut MarkdownParser) -> bool,
    is_tight: &mut bool,
    last_item_ends_with_blank: &mut bool,
) -> ParsedSyntax
where
    FMarker: Fn(&mut MarkdownParser) -> Option<M>,
    FParse: Fn(&mut MarkdownParser) -> (ParsedSyntax, ListItemBlankInfo),
{
    skip_blank_lines_between_items(
        p,
        has_item_after_blank_lines,
        is_tight,
        last_item_ends_with_blank,
    );

    if marker_state.is_none() {
        *marker_state = current_marker(p);
    }

    let (parsed, blank_info) = parse_item(p);
    update_list_tightness(blank_info, is_tight, last_item_ends_with_blank);
    parsed
}

fn is_at_list_end_common<M, FAt, FMarker, FNewline>(
    p: &mut MarkdownParser,
    marker_state: Option<M>,
    at_list_item: FAt,
    current_marker: FMarker,
    has_item_after_blank_lines: fn(&mut MarkdownParser) -> bool,
    handle_newline: FNewline,
) -> bool
where
    M: Copy + PartialEq,
    FAt: Fn(&mut MarkdownParser) -> bool,
    FMarker: Fn(&mut MarkdownParser) -> Option<M>,
    FNewline: Fn(&mut MarkdownParser, Option<M>) -> Option<bool>,
{
    let quote_depth = p.state().block_quote_depth;
    let at_virtual_line_start = p.state().virtual_line_start == Some(p.cur_range().start());
    if quote_depth > 0
        && !at_virtual_line_start
        && (p.at_line_start() || p.has_preceding_line_break())
        && !has_quote_prefix(p, quote_depth)
    {
        return true;
    }

    // Check if we're directly at a list marker
    if at_list_item(p) {
        if let (Some(current), Some(next)) = (marker_state, current_marker(p))
            && current != next
        {
            return true;
        }
        return false;
    }

    // If at a blank line, look ahead to see if there's another list item.
    // Per CommonMark §5.3, blank lines between items make the list loose,
    // but don't end the list.
    if p.at_line_start() && at_blank_line_start(p) {
        return !has_item_after_blank_lines(p);
    }

    // Also check if we're directly AT a NEWLINE token (blank line)
    // This handles the case where we're at the newline itself, not after it
    if p.at(NEWLINE) {
        if let Some(result) = handle_newline(p, marker_state) {
            return result;
        }
        return !has_item_after_blank_lines(p);
    }

    // Not at a marker and not at a blank line with continuation
    true
}

/// Struct implementing `ParseNodeList` for bullet lists.
struct BulletList {
    /// A list is tight if there are no blank lines between items or inside items.
    is_tight: bool,
    /// Whether the last parsed item ended with a blank line.
    last_item_ends_with_blank: bool,
    /// The marker kind for this list (`-`, `*`, or `+`).
    marker_kind: Option<MarkdownSyntaxKind>,
}

impl BulletList {
    fn new() -> Self {
        Self {
            is_tight: true,
            last_item_ends_with_blank: false,
            marker_kind: None,
        }
    }
}

impl ParseNodeList for BulletList {
    type Kind = MarkdownSyntaxKind;
    type Parser<'source> = MarkdownParser<'source>;

    const LIST_KIND: Self::Kind = MD_BULLET_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_list_element_common(
            p,
            &mut self.marker_kind,
            current_bullet_marker,
            parse_bullet,
            has_bullet_item_after_blank_lines,
            &mut self.is_tight,
            &mut self.last_item_ends_with_blank,
        )
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_list_end_common(
            p,
            self.marker_kind,
            at_bullet_list_item,
            current_bullet_marker,
            has_bullet_item_after_blank_lines,
            |p, _marker_kind| {
                let next_is_bullet = p.lookahead(|p| {
                    p.bump(NEWLINE);
                    skip_leading_whitespace_tokens(p);
                    if p.at(T![-]) || p.at(T![*]) || p.at(T![+]) {
                        p.bump(p.cur());
                        return marker_followed_by_whitespace_or_eol(p);
                    }
                    false
                });
                if next_is_bullet {
                    Some(false)
                } else {
                    Some(!has_bullet_item_after_blank_lines(p))
                }
            },
        )
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

    fn finish_list(&mut self, p: &mut Self::Parser<'_>, m: Marker) -> CompletedMarker {
        let completed = m.complete(p, Self::LIST_KIND);
        let range = completed.range(p);
        p.record_list_tightness(range, self.is_tight);
        completed
    }
}

fn current_bullet_marker(p: &mut MarkdownParser) -> Option<MarkdownSyntaxKind> {
    p.lookahead(|p| {
        if !p.at_line_start() {
            return None;
        }

        skip_leading_whitespace_tokens(p);

        if p.at(MD_SETEXT_UNDERLINE_LITERAL) {
            if is_single_dash_setext_marker(p.cur_text()) {
                return Some(T![-]);
            }
            return None;
        }

        if p.at(MD_TEXTUAL_LITERAL) {
            return match p.cur_text() {
                "-" => Some(T![-]),
                "*" => Some(T![*]),
                "+" => Some(T![+]),
                _ => None,
            };
        }

        if p.at(T![-]) {
            return Some(T![-]);
        }
        if p.at(T![*]) {
            return Some(T![*]);
        }
        if p.at(T![+]) {
            return Some(T![+]);
        }

        None
    })
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
/// MdBullet = bullet: ('-' | '*') content: MdBlockList
///
/// Parses consecutive bullet items into a single list.
///
/// Nesting is limited to `MarkdownParseOptions::max_nesting_depth` to prevent stack overflow.
pub(crate) fn parse_bullet_list_item(p: &mut MarkdownParser) -> ParsedSyntax {
    if !at_bullet_list_item(p) {
        return Absent;
    }

    // Check depth limit before parsing
    let max_nesting_depth = p.options().max_nesting_depth;
    if p.state().list_nesting_depth >= max_nesting_depth {
        // Emit diagnostic and treat as content
        let range = p.cur_range();
        p.error(list_nesting_too_deep(p, range, max_nesting_depth));
        skip_list_marker_indent(p);
        if p.at(MD_SETEXT_UNDERLINE_LITERAL) {
            p.parse_as_skipped_trivia_tokens(|p| p.bump_remap(T![-]));
        } else if p.at(MD_TEXTUAL_LITERAL) {
            let text = p.cur_text();
            if text == "-" {
                p.parse_as_skipped_trivia_tokens(|p| p.bump_remap(T![-]));
            } else if text == "*" {
                p.parse_as_skipped_trivia_tokens(|p| p.bump_remap(T![*]));
            } else if text == "+" {
                p.parse_as_skipped_trivia_tokens(|p| p.bump_remap(T![+]));
            }
        } else if p.at(T![-]) || p.at(T![*]) || p.at(T![+]) {
            p.parse_as_skipped_trivia_tokens(|p| p.bump(p.cur()));
        }
        if p.at(MD_TEXTUAL_LITERAL) {
            let text = p.cur_text();
            if text.starts_with(' ') || text.starts_with('\t') {
                p.parse_as_skipped_trivia_tokens(|p| p.bump(MD_TEXTUAL_LITERAL));
            }
        }
        return Absent;
    }

    let item_m = p.start();

    // Increment list depth
    p.state_mut().list_nesting_depth += 1;

    // Use ParseNodeList to parse the list with proper recovery
    let mut list_helper = BulletList::new();
    list_helper.parse_list(p);

    // Decrement list depth
    p.state_mut().list_nesting_depth -= 1;

    Present(item_m.complete(p, MD_BULLET_LIST_ITEM))
}

/// Parse a single bullet (marker + content).
///
/// Returns `Present` if a bullet was successfully parsed, `Absent` otherwise.
/// Also returns blank-line information for the list item.
fn parse_bullet(p: &mut MarkdownParser) -> (ParsedSyntax, ListItemBlankInfo) {
    // Must be at a bullet marker at line start
    if !at_bullet_list_item(p) {
        return (Absent, ListItemBlankInfo::default());
    }

    let m = p.start();

    let marker_indent = if p.state().virtual_line_start == Some(p.cur_range().start()) {
        0
    } else {
        p.source().line_start_leading_indent()
    };
    skip_list_marker_indent(p);

    // Bullet marker is 1 character (-, *, or +)
    let marker_width = 1;

    // Bump the bullet marker (-, *, or +)
    let mut marker_token_text = None;
    if p.at(MD_SETEXT_UNDERLINE_LITERAL) && is_single_dash_setext_marker(p.cur_text()) {
        marker_token_text = Some(p.cur_text().to_string());
        p.bump_remap(T![-]);
    } else if p.at(MD_TEXTUAL_LITERAL) && is_textual_bullet_marker(p.cur_text()) {
        let text = p.cur_text().to_string();
        marker_token_text = Some(text.clone());
        if text == "-" {
            p.bump_remap(T![-]);
        } else if text == "*" {
            p.bump_remap(T![*]);
        } else {
            p.bump_remap(T![+]);
        }
    } else if p.at(T![-]) {
        p.bump(T![-]);
    } else if p.at(T![*]) {
        p.bump(T![*]);
    } else {
        p.bump(T![+]);
    }

    // Count spaces after marker to determine required indentation.
    // Per CommonMark §5.2, content aligns to first non-space after marker.
    let spaces_after_marker = if let Some(text) = marker_token_text.as_deref() {
        count_spaces_after_dash_in_token(text, marker_indent + marker_width)
    } else {
        count_spaces_after_marker(p.source_after_current(), marker_indent + marker_width)
    };

    // Set required indent for continuation lines
    // Required indent = marker width + spaces after marker (minimum 1)
    let prev_required_indent = p.state().list_item_required_indent;
    let prev_marker_indent = p.state().list_item_marker_indent;
    p.state_mut().list_item_required_indent = if spaces_after_marker > INDENT_CODE_BLOCK_SPACES {
        marker_indent + marker_width + 1
    } else {
        marker_indent + marker_width + spaces_after_marker.max(1)
    };
    p.state_mut().list_item_marker_indent = marker_indent;

    // Parse block content (MD_BLOCK_LIST)
    let blank_info = parse_list_item_block_content(p, spaces_after_marker);

    // Restore previous required indent
    p.state_mut().list_item_required_indent = prev_required_indent;
    p.state_mut().list_item_marker_indent = prev_marker_indent;

    let completed = m.complete(p, MD_BULLET);
    let range = completed.range(p);
    let indent = marker_indent + marker_width + spaces_after_marker.max(1);
    p.record_list_item_indent(
        range,
        indent,
        marker_indent,
        marker_width,
        spaces_after_marker,
    );
    (Present(completed), blank_info)
}

/// Check if we're at the start of an ordered list item (e.g., "1.", "2)").
///
/// An ordered list marker is a sequence of 1-9 digits followed by `.` or `)`,
/// at the start of a line.
pub(crate) fn at_order_list_item(p: &mut MarkdownParser) -> bool {
    at_order_list_item_with_base_indent(p, list_marker_base_indent(p))
}

fn at_order_list_item_with_base_indent(p: &mut MarkdownParser, base_indent: usize) -> bool {
    p.lookahead(|p| {
        if !list_item_within_indent(p, base_indent) {
            return false;
        }

        skip_leading_whitespace_tokens(p);

        // Check for ordered list marker token at line start
        if !p.at(MD_ORDERED_LIST_MARKER) {
            return false;
        }

        p.bump(MD_ORDERED_LIST_MARKER);
        marker_followed_by_whitespace_or_eol(p)
    })
}

/// Struct implementing `ParseNodeList` for ordered lists.
struct OrderedList {
    /// A list is tight if there are no blank lines between items or inside items.
    is_tight: bool,
    /// Whether the last parsed item ended with a blank line.
    last_item_ends_with_blank: bool,
    /// The delimiter for this ordered list (`.` or `)`).
    marker_delim: Option<char>,
}

impl OrderedList {
    fn new() -> Self {
        Self {
            is_tight: true,
            last_item_ends_with_blank: false,
            marker_delim: None,
        }
    }
}

impl ParseNodeList for OrderedList {
    type Kind = MarkdownSyntaxKind;
    type Parser<'source> = MarkdownParser<'source>;

    const LIST_KIND: Self::Kind = MD_BULLET_LIST; // Reuse bullet list node structure

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_list_element_common(
            p,
            &mut self.marker_delim,
            current_ordered_delim,
            parse_ordered_bullet,
            has_ordered_item_after_blank_lines,
            &mut self.is_tight,
            &mut self.last_item_ends_with_blank,
        )
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_list_end_common(
            p,
            self.marker_delim,
            at_order_list_item,
            current_ordered_delim,
            has_ordered_item_after_blank_lines,
            |p, marker_delim| {
                let next_is_ordered = p.lookahead(|p| {
                    p.bump(NEWLINE);
                    skip_leading_whitespace_tokens(p);
                    if p.at(MD_ORDERED_LIST_MARKER) {
                        p.bump(MD_ORDERED_LIST_MARKER);
                        return marker_followed_by_whitespace_or_eol(p);
                    }
                    false
                });
                if next_is_ordered {
                    if let (Some(current_delim), Some(next_delim)) =
                        (marker_delim, current_ordered_delim(p))
                        && current_delim != next_delim
                    {
                        return Some(true);
                    }
                    return Some(false);
                }
                Some(!has_ordered_item_after_blank_lines(p))
            },
        )
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
            expected_ordered_item,
        )
    }

    fn finish_list(&mut self, p: &mut Self::Parser<'_>, m: Marker) -> CompletedMarker {
        let completed = m.complete(p, Self::LIST_KIND);
        let range = completed.range(p);
        p.record_list_tightness(range, self.is_tight);
        completed
    }
}

fn current_ordered_delim(p: &mut MarkdownParser) -> Option<char> {
    p.lookahead(|p| {
        if !p.at_line_start() {
            return None;
        }

        skip_leading_whitespace_tokens(p);

        if !p.at(MD_ORDERED_LIST_MARKER) {
            return None;
        }

        let text = p.cur_text();
        text.chars().last().filter(|c| *c == '.' || *c == ')')
    })
}

/// Error builder for ordered list recovery
fn expected_ordered_item(p: &MarkdownParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Expected an ordered list item", range)
        .with_hint("Ordered list items start with a number followed by `.` or `)` at the beginning of a line")
}

/// Parse an ordered list item.
///
/// Grammar:
/// MdOrderListItem = MdBulletList (reusing bullet list structure)
///
/// Parses consecutive ordered items into a single list.
///
/// Nesting is limited to `MarkdownParseOptions::max_nesting_depth` to prevent stack overflow.
pub(crate) fn parse_order_list_item(p: &mut MarkdownParser) -> ParsedSyntax {
    if !at_order_list_item(p) {
        return Absent;
    }

    // Check depth limit before parsing
    let max_nesting_depth = p.options().max_nesting_depth;
    if p.state().list_nesting_depth >= max_nesting_depth {
        // Emit diagnostic and treat as content
        let range = p.cur_range();
        p.error(list_nesting_too_deep(p, range, max_nesting_depth));
        skip_list_marker_indent(p);
        if p.at(MD_ORDERED_LIST_MARKER) {
            p.parse_as_skipped_trivia_tokens(|p| p.bump(MD_ORDERED_LIST_MARKER));
        }
        if p.at(MD_TEXTUAL_LITERAL) {
            let text = p.cur_text();
            if text.starts_with(' ') || text.starts_with('\t') {
                p.parse_as_skipped_trivia_tokens(|p| p.bump(MD_TEXTUAL_LITERAL));
            }
        }
        return Absent;
    }

    let item_m = p.start();

    // Increment list depth
    p.state_mut().list_nesting_depth += 1;

    // Use ParseNodeList to parse the list with proper recovery
    let mut list_helper = OrderedList::new();
    list_helper.parse_list(p);

    // Decrement list depth
    p.state_mut().list_nesting_depth -= 1;

    Present(item_m.complete(p, MD_ORDERED_LIST_ITEM))
}

/// Parse a single ordered item (marker + content).
fn parse_ordered_bullet(p: &mut MarkdownParser) -> (ParsedSyntax, ListItemBlankInfo) {
    if !at_order_list_item(p) {
        return (Absent, ListItemBlankInfo::default());
    }

    let m = p.start();

    let marker_indent = if p.state().virtual_line_start == Some(p.cur_range().start()) {
        0
    } else {
        p.source().line_start_leading_indent()
    };
    skip_list_marker_indent(p);

    // Get marker width from actual token text (e.g., "1." = 2, "10." = 3)
    let marker_width = p.cur_text().len();

    // Bump the ordered list marker
    p.bump(MD_ORDERED_LIST_MARKER);

    // Count spaces after marker to determine required indentation.
    // Per CommonMark §5.2, content aligns to first non-space after marker.
    let spaces_after_marker =
        count_spaces_after_marker(p.source_after_current(), marker_indent + marker_width);

    // Set required indent for continuation lines
    // Required indent = marker width + spaces after marker (minimum 1)
    let prev_required_indent = p.state().list_item_required_indent;
    let prev_marker_indent = p.state().list_item_marker_indent;
    p.state_mut().list_item_required_indent = if spaces_after_marker > INDENT_CODE_BLOCK_SPACES {
        marker_indent + marker_width + 1
    } else {
        marker_indent + marker_width + spaces_after_marker.max(1)
    };
    p.state_mut().list_item_marker_indent = marker_indent;

    // Parse block content
    let blank_info = parse_list_item_block_content(p, spaces_after_marker);

    // Restore previous required indent
    p.state_mut().list_item_required_indent = prev_required_indent;
    p.state_mut().list_item_marker_indent = prev_marker_indent;

    let completed = m.complete(p, MD_BULLET);
    let range = completed.range(p);
    let indent = marker_indent + marker_width + spaces_after_marker.max(1);
    p.record_list_item_indent(
        range,
        indent,
        marker_indent,
        marker_width,
        spaces_after_marker,
    );
    (Present(completed), blank_info)
}

/// Count the number of space/tab characters at the start of a string.
/// Used to determine actual spaces after list marker.
fn count_spaces_after_marker(s: &str, start_column: usize) -> usize {
    let mut column = start_column;

    for c in s.chars() {
        match c {
            ' ' => column += 1,
            '\t' => column += 4 - (column % 4),
            _ => break,
        }
    }

    column.saturating_sub(start_column)
}

fn is_single_dash_setext_marker(text: &str) -> bool {
    let trimmed = text.trim_matches(|c| c == ' ' || c == '\t');
    trimmed == "-"
}

fn is_textual_bullet_marker(text: &str) -> bool {
    text == "-" || text == "*" || text == "+"
}

pub(crate) fn textual_starts_with_ordered_marker(text: &str) -> bool {
    let trimmed = text.trim_start_matches([' ', '\t']);
    let mut chars = trimmed.chars().peekable();
    let mut digit_count = 0;

    while let Some(c) = chars.peek().copied() {
        if c.is_ascii_digit() {
            digit_count += 1;
            if digit_count > 9 {
                return false;
            }
            chars.next();
        } else {
            break;
        }
    }

    if digit_count == 0 {
        return false;
    }

    match chars.next() {
        Some('.' | ')') => {}
        _ => return false,
    }

    matches!(chars.peek(), None | Some(' ' | '\t' | '\n' | '\r'))
}

fn count_spaces_after_dash_in_token(text: &str, start_column: usize) -> usize {
    let mut column = start_column;
    let mut seen_dash = false;

    for c in text.chars() {
        if !seen_dash {
            if c == '-' {
                seen_dash = true;
            }
            continue;
        }

        match c {
            ' ' => column += 1,
            '\t' => column += 4 - (column % 4),
            _ => break,
        }
    }

    column.saturating_sub(start_column)
}

fn line_indent_from_current(p: &MarkdownParser) -> usize {
    let mut column = 0usize;
    for c in p.source_after_current().chars() {
        match c {
            ' ' => column += 1,
            '\t' => column += 4 - (column % 4),
            _ => break,
        }
    }
    column
}

fn quote_only_line_indent_at_current(p: &MarkdownParser, depth: usize) -> Option<usize> {
    if depth == 0 {
        return None;
    }

    let mut start: usize = p.cur_range().start().into();
    let source = p.source().source_text();
    let bytes = source.as_bytes();
    while start > 0 && bytes[start - 1] != b'\n' && bytes[start - 1] != b'\r' {
        start -= 1;
    }
    let line_end = source[start..]
        .find('\n')
        .map_or(source.len(), |offset| start + offset);

    let mut i = start;
    for _ in 0..depth {
        let mut column = 0usize;
        while i < line_end && column < 3 {
            match bytes[i] {
                b' ' => {
                    column += 1;
                    i += 1;
                }
                b'\t' => {
                    let advance = 4 - (column % 4);
                    column += advance;
                    i += 1;
                }
                _ => break,
            }
        }

        if i >= line_end || bytes[i] != b'>' {
            return None;
        }
        i += 1;

        if i < line_end && (bytes[i] == b' ' || bytes[i] == b'\t') {
            i += 1;
        }
    }

    let mut indent = 0usize;
    while i < line_end {
        match bytes[i] {
            b' ' => {
                indent += 1;
                i += 1;
            }
            b'\t' => {
                indent += 4 - (indent % 4);
                i += 1;
            }
            _ => return None,
        }
    }

    Some(indent)
}

fn next_quote_content_indent(p: &MarkdownParser, depth: usize) -> Option<usize> {
    if depth == 0 {
        return None;
    }

    let source = p.source().source_text();
    let bytes = source.as_bytes();
    let mut line_start: usize = p.cur_range().start().into();
    while line_start > 0 && bytes[line_start - 1] != b'\n' && bytes[line_start - 1] != b'\r' {
        line_start -= 1;
    }

    loop {
        let newline_index = source[line_start..]
            .find('\n')
            .map(|offset| line_start + offset);
        let mut line_end = newline_index.unwrap_or(source.len());
        if line_end > line_start && bytes[line_end - 1] == b'\r' {
            line_end -= 1;
        }

        let mut i = line_start;
        for _ in 0..depth {
            let mut column = 0usize;
            while i < line_end && column < 3 {
                match bytes[i] {
                    b' ' => {
                        column += 1;
                        i += 1;
                    }
                    b'\t' => {
                        let advance = 4 - (column % 4);
                        column += advance;
                        i += 1;
                    }
                    _ => break,
                }
            }

            if i >= line_end || bytes[i] != b'>' {
                return None;
            }
            i += 1;

            if i < line_end && (bytes[i] == b' ' || bytes[i] == b'\t') {
                i += 1;
            }
        }

        let mut indent = 0usize;
        while i < line_end {
            match bytes[i] {
                b' ' => {
                    indent += 1;
                    i += 1;
                }
                b'\t' => {
                    indent += 4 - (indent % 4);
                    i += 1;
                }
                _ => return Some(indent),
            }
        }

        let newline_index = newline_index?;
        line_start = newline_index + 1;
    }
}

/// Parse block content for a list item.
///
/// Handles the sequence of blocks belonging to a list item.
/// The first block usually starts on the same line as the marker.
/// Subsequent lines must be indented to at least `required_indent` columns.
///
/// Per CommonMark §5.2, continuation lines must align with the first non-space
/// character after the list marker.
///
/// Returns blank-line information for the list item content.
fn parse_list_item_block_content(
    p: &mut MarkdownParser,
    spaces_after_marker: usize,
) -> ListItemBlankInfo {
    let m = p.start();
    let mut has_blank_line = false;
    let mut last_was_blank = false;
    let mut last_block_was_paragraph = false;
    let required_indent = p.state().list_item_required_indent;
    let marker_indent = p.state().list_item_marker_indent;

    // Track whether we're on the first line (same line as marker)
    let mut first_line = true;

    loop {
        if p.at(T![EOF]) {
            break;
        }

        let quote_depth = p.state().block_quote_depth;
        if !first_line
            && quote_depth > 0
            && quote_only_line_indent_at_current(p, quote_depth).is_some()
            && let Some(next_indent) = next_quote_content_indent(p, quote_depth)
            && next_indent < required_indent
        {
            break;
        }
        let newline_has_quote_prefix = quote_depth > 0
            && p.at(NEWLINE)
            && (p.at_line_start() || p.has_preceding_line_break())
            && has_quote_prefix(p, quote_depth);

        if !first_line && p.at(NEWLINE) && !p.at_blank_line() && !newline_has_quote_prefix {
            let action = classify_blank_line(p, required_indent, marker_indent);
            match action {
                BlankLineAction::ContinueItem => {
                    consume_blank_line(p);
                    has_blank_line = true;
                    last_was_blank = true;
                    continue;
                }
                BlankLineAction::EndItemAfterBlank => {
                    consume_blank_line(p);
                    has_blank_line = true;
                    last_was_blank = true;
                    break;
                }
                BlankLineAction::EndItemBeforeBlank => {
                    break;
                }
            }
        }

        let line_has_quote_prefix = quote_depth > 0
            && (p.at_line_start() || p.has_preceding_line_break())
            && (has_quote_prefix(p, quote_depth)
                || quote_only_line_indent_at_current(p, quote_depth).is_some());

        let blank_line_after_prefix = if line_has_quote_prefix {
            p.lookahead(|p| {
                consume_quote_prefix_without_virtual(p, quote_depth);
                at_blank_line_after_prefix(p)
            })
        } else {
            at_blank_line_after_prefix(p)
        };

        if (p.at_line_start() || line_has_quote_prefix) && blank_line_after_prefix {
            if line_has_quote_prefix
                && quote_only_line_indent_at_current(p, quote_depth).is_some()
                && let Some(next_indent) = next_quote_content_indent(p, quote_depth)
            {
                if next_indent >= required_indent {
                    if line_has_quote_prefix {
                        consume_quote_prefix(p, quote_depth);
                    }
                    consume_blank_line(p);
                    if !first_line {
                        has_blank_line = true;
                    }
                    last_was_blank = true;
                    first_line = false;
                    continue;
                }
                if next_indent < required_indent {
                    break;
                }
            }
            let marker_line_break = first_line;
            let action = if quote_depth > 0 {
                classify_blank_line_in_quote(p, required_indent, marker_indent, quote_depth)
            } else {
                classify_blank_line(p, required_indent, marker_indent)
            };
            match action {
                BlankLineAction::ContinueItem => {
                    if line_has_quote_prefix {
                        consume_quote_prefix(p, quote_depth);
                    }
                    consume_blank_line(p);
                    if !marker_line_break {
                        has_blank_line = true;
                    }
                    last_was_blank = true;
                    first_line = false;
                    continue;
                }
                BlankLineAction::EndItemAfterBlank => {
                    if line_has_quote_prefix {
                        consume_quote_prefix(p, quote_depth);
                    }
                    consume_blank_line(p);
                    if !marker_line_break {
                        has_blank_line = true;
                    }
                    last_was_blank = true;
                    break;
                }
                BlankLineAction::EndItemBeforeBlank => {
                    break;
                }
            }
        }

        if line_has_quote_prefix {
            consume_quote_prefix(p, quote_depth);
        }
        let line_started_with_quote_prefix = line_has_quote_prefix;

        let prev_was_blank = last_was_blank;

        if first_line && p.at(NEWLINE) {
            let next_is_sibling = p.lookahead(|p| {
                p.bump(NEWLINE);
                if p.at_line_start() {
                    at_bullet_list_item_with_base_indent(p, marker_indent)
                        || at_order_list_item_with_base_indent(p, marker_indent)
                } else {
                    false
                }
            });

            // Marker-only line: consume the newline as trivia and continue.
            p.parse_as_skipped_trivia_tokens(|p| p.bump(NEWLINE));
            first_line = false;
            last_was_blank = false;

            if next_is_sibling {
                continue;
            }
        }

        if first_line {
            enum NestedListMarker {
                Bullet,
                Ordered,
            }

            let fenced_code_start = p.lookahead(|p| {
                while p.at(MD_TEXTUAL_LITERAL) && is_whitespace_only(p.cur_text()) {
                    p.bump(MD_TEXTUAL_LITERAL);
                }
                if p.at(TRIPLE_BACKTICK) || p.at(TRIPLE_TILDE) {
                    return true;
                }
                (p.at(BACKTICK) || p.at(TILDE)) && p.cur_text().len() >= 3
            });

            if fenced_code_start {
                while p.at(MD_TEXTUAL_LITERAL) && is_whitespace_only(p.cur_text()) {
                    p.parse_as_skipped_trivia_tokens(|p| p.bump(MD_TEXTUAL_LITERAL));
                }

                let parsed = super::with_virtual_line_start(p, p.cur_range().start(), |p| {
                    parse_fenced_code_block(p)
                });
                if parsed.is_present() {
                    last_block_was_paragraph = false;
                    last_was_blank = false;
                    first_line = false;
                    continue;
                }
            }

            let html_block_start = p.lookahead(|p| {
                super::with_virtual_line_start(p, p.cur_range().start(), |p| {
                    super::html_block::at_html_block(p)
                })
            });

            if html_block_start {
                let parsed = super::with_virtual_line_start(p, p.cur_range().start(), |p| {
                    super::html_block::parse_html_block(p)
                });
                if parsed.is_present() {
                    last_block_was_paragraph = false;
                    last_was_blank = false;
                    first_line = false;
                    continue;
                }
            }

            let nested_marker = p.lookahead(|p| {
                while p.at(MD_TEXTUAL_LITERAL) && is_whitespace_only(p.cur_text()) {
                    p.bump(MD_TEXTUAL_LITERAL);
                }

                if p.at(MD_ORDERED_LIST_MARKER) {
                    p.bump(MD_ORDERED_LIST_MARKER);
                    return marker_followed_by_whitespace_or_eol(p)
                        .then_some(NestedListMarker::Ordered);
                }

                if p.at(MD_SETEXT_UNDERLINE_LITERAL) && is_single_dash_setext_marker(p.cur_text()) {
                    p.bump(MD_SETEXT_UNDERLINE_LITERAL);
                    return marker_followed_by_whitespace_or_eol(p)
                        .then_some(NestedListMarker::Bullet);
                }

                if p.at(T![-]) || p.at(T![*]) || p.at(T![+]) {
                    p.bump(p.cur());
                    return marker_followed_by_whitespace_or_eol(p)
                        .then_some(NestedListMarker::Bullet);
                }

                if p.at(MD_TEXTUAL_LITERAL) && is_textual_bullet_marker(p.cur_text()) {
                    p.bump(MD_TEXTUAL_LITERAL);
                    return marker_followed_by_whitespace_or_eol(p)
                        .then_some(NestedListMarker::Bullet);
                }

                if p.at(MD_TEXTUAL_LITERAL) && textual_starts_with_ordered_marker(p.cur_text()) {
                    p.bump(MD_TEXTUAL_LITERAL);
                    return Some(NestedListMarker::Ordered);
                }

                None
            });

            if let Some(nested_marker) = nested_marker {
                while p.at(MD_TEXTUAL_LITERAL) && is_whitespace_only(p.cur_text()) {
                    p.parse_as_skipped_trivia_tokens(|p| p.bump(MD_TEXTUAL_LITERAL));
                }

                let prev_virtual = p.state().virtual_line_start;
                let prev_required = p.state().list_item_required_indent;
                p.state_mut().virtual_line_start = Some(p.cur_range().start());
                p.state_mut().list_item_required_indent = 0;

                let parsed = match nested_marker {
                    NestedListMarker::Bullet => parse_bullet_list_item(p),
                    NestedListMarker::Ordered => {
                        p.set_force_ordered_list_marker(true);
                        p.force_relex_regular();
                        let parsed = parse_order_list_item(p);
                        p.set_force_ordered_list_marker(false);
                        parsed
                    }
                };
                if parsed.is_absent() {
                    let parsed_kind = parse_any_block_with_indent_code_policy(p, true);
                    last_block_was_paragraph = parsed_kind == ParsedBlockKind::Paragraph;
                } else {
                    last_block_was_paragraph = false;
                }
                first_line = false;

                p.state_mut().virtual_line_start = prev_virtual;
                p.state_mut().list_item_required_indent = prev_required;
                continue;
            }
        }

        if first_line && spaces_after_marker > INDENT_CODE_BLOCK_SPACES {
            parse_indent_code_block_in_list_first_line(p);
            last_block_was_paragraph = false;
            last_was_blank = false;
            first_line = false;
            continue;
        }
        // Blank line handling happens above, before consuming quote prefixes.

        // After the first line, check indentation for continuation
        // Skip this check on the first line (content on same line as marker)
        let mut restore_virtual_line_start = None;
        if !first_line && (p.at_line_start() || line_started_with_quote_prefix) {
            // Get indentation of current line
            let indent = line_indent_from_current(p);

            if indent < marker_indent {
                break;
            }

            if indent >= required_indent {
                let allow_indent_code_block = !last_block_was_paragraph || prev_was_blank;
                let is_indent_code_block =
                    allow_indent_code_block && indent >= required_indent + INDENT_CODE_BLOCK_SPACES;
                if !is_indent_code_block {
                    // Sufficient indentation - skip it and continue
                    p.skip_line_indent(required_indent);
                    let prev_virtual = p.state().virtual_line_start;
                    p.state_mut().virtual_line_start = Some(p.cur_range().start());
                    restore_virtual_line_start = Some(prev_virtual);

                    if at_bullet_list_item(p) {
                        let _ = parse_bullet_list_item(p);
                        last_block_was_paragraph = false;
                        first_line = false;
                        p.state_mut().virtual_line_start = prev_virtual;
                        continue;
                    }
                    if at_order_list_item(p) {
                        let _ = parse_order_list_item(p);
                        last_block_was_paragraph = false;
                        first_line = false;
                        p.state_mut().virtual_line_start = prev_virtual;
                        continue;
                    }
                }
            } else {
                // Insufficient indentation - check for block interrupts

                // A new list marker at this indentation starts a sibling item
                if at_bullet_list_item_with_base_indent(p, marker_indent)
                    || at_order_list_item_with_base_indent(p, marker_indent)
                {
                    break;
                }

                // Check if this line starts a block-level construct that can
                // interrupt paragraphs (headers, quotes, thematic breaks, etc.)
                if at_block_interrupt(p) {
                    break;
                }

                // Otherwise, this is "lazy continuation" per CommonMark §5.2:
                // Content continues without meeting the indent requirement.
                // Don't skip indent, just continue parsing at actual position.
                if !last_block_was_paragraph {
                    break;
                }
            }
        }

        let is_blank_line = p.at_blank_line();
        if is_blank_line {
            has_blank_line = true;
            last_was_blank = true;
        } else {
            last_was_blank = false;
        }

        // After parsing any block, we'll be on a new line (or EOF)
        first_line = false;

        // Parse the next block
        // parse_any_block_with_indent_code_policy handles paragraphs, code blocks, etc.
        // It consumes newlines as MdNewline if they are blank lines.
        let allow_indent_code_block = !last_block_was_paragraph || prev_was_blank;
        let parsed_kind = parse_any_block_with_indent_code_policy(p, allow_indent_code_block);
        last_block_was_paragraph = parsed_kind == ParsedBlockKind::Paragraph;
        if let Some(prev_virtual) = restore_virtual_line_start {
            p.state_mut().virtual_line_start = prev_virtual;
        }
    }

    m.complete(p, MD_BLOCK_LIST);
    ListItemBlankInfo {
        has_blank_line,
        ends_with_blank_line: last_was_blank,
    }
}

fn parse_indent_code_block_in_list_first_line(p: &mut MarkdownParser) {
    let m = p.start();
    let content = p.start();

    loop {
        if p.at(T![EOF]) {
            break;
        }

        if p.at(NEWLINE) {
            if list_newline_is_blank_line(p) && !list_has_following_indented_code_line(p) {
                break;
            }
            let text_m = p.start();
            p.bump_remap(MD_TEXTUAL_LITERAL);
            text_m.complete(p, MD_TEXTUAL);
            continue;
        }

        if p.at_line_start() && !at_indent_code_block(p) {
            if at_blank_line_start(p) {
                if list_has_following_indented_code_line(p) {
                    consume_blank_line(p);
                    continue;
                }
                break;
            }
            break;
        }

        let text_m = p.start();
        p.bump_remap(MD_TEXTUAL_LITERAL);
        text_m.complete(p, MD_TEXTUAL);
    }

    content.complete(p, MD_INLINE_ITEM_LIST);
    m.complete(p, MD_INDENT_CODE_BLOCK);
}

fn list_has_following_indented_code_line(p: &mut MarkdownParser) -> bool {
    p.lookahead(|p| {
        while p.at_line_start() && at_blank_line_start(p) {
            while p.at(MD_TEXTUAL_LITERAL) {
                let text = p.cur_text();
                if text == " " || text == "\t" {
                    p.bump(MD_TEXTUAL_LITERAL);
                } else {
                    break;
                }
            }

            if p.at(NEWLINE) {
                p.bump(NEWLINE);
            } else {
                break;
            }
        }

        at_indent_code_block(p)
    })
}

fn list_newline_is_blank_line(p: &MarkdownParser) -> bool {
    let start: usize = p.cur_range().start().into();
    if start == 0 {
        return true;
    }

    let source = p.source().source_text();
    let prev = source.as_bytes()[start - 1];
    prev == b'\n' || prev == b'\r'
}

enum BlankLineAction {
    ContinueItem,
    EndItemAfterBlank,
    EndItemBeforeBlank,
}

fn classify_blank_line(
    p: &mut MarkdownParser,
    required_indent: usize,
    marker_indent: usize,
) -> BlankLineAction {
    p.lookahead(|p| {
        // Skip ALL consecutive blank lines (not just one).
        // Per CommonMark §5.3, multiple blank lines between items still
        // belong to the same list - they just make it "loose".
        loop {
            let line_is_blank = p.lookahead(|p| {
                while p.at(MD_TEXTUAL_LITERAL) {
                    let text = p.cur_text();
                    if text == " " || text == "\t" {
                        p.bump(MD_TEXTUAL_LITERAL);
                    } else {
                        break;
                    }
                }
                p.at(NEWLINE) || p.at(T![EOF])
            });

            if !line_is_blank {
                break;
            }

            while p.at(MD_TEXTUAL_LITERAL) {
                let text = p.cur_text();
                if text == " " || text == "\t" {
                    p.bump(MD_TEXTUAL_LITERAL);
                } else {
                    break;
                }
            }

            if p.at(NEWLINE) {
                p.bump(NEWLINE);
                continue;
            }

            break;
        }

        if p.at(T![EOF]) {
            return BlankLineAction::EndItemBeforeBlank;
        }

        // Otherwise, keep blank line as part of item only if indentation is sufficient.
        let indent = p.line_start_leading_indent();
        if indent >= required_indent {
            return BlankLineAction::ContinueItem;
        }

        // If next non-blank line starts a new list item, this is a blank line between items.
        if indent <= marker_indent + 3
            && (at_bullet_list_item_with_base_indent(p, marker_indent)
                || at_order_list_item_with_base_indent(p, marker_indent))
        {
            return BlankLineAction::EndItemAfterBlank;
        }

        BlankLineAction::EndItemBeforeBlank
    })
}

fn classify_blank_line_in_quote(
    p: &mut MarkdownParser,
    required_indent: usize,
    marker_indent: usize,
    quote_depth: usize,
) -> BlankLineAction {
    p.lookahead(|p| {
        loop {
            let blank_indent = p.lookahead(|p| {
                if !consume_quote_prefix_without_virtual(p, quote_depth) {
                    return None;
                }
                Some(line_indent_from_current(p))
            });

            if let Some(indent) = blank_indent
                && indent < required_indent
            {
                return BlankLineAction::EndItemBeforeBlank;
            }

            let line_is_blank = p.lookahead(|p| {
                if !consume_quote_prefix_without_virtual(p, quote_depth) {
                    return false;
                }
                while p.at(MD_TEXTUAL_LITERAL) && is_whitespace_only(p.cur_text()) {
                    p.bump(MD_TEXTUAL_LITERAL);
                }
                p.at(NEWLINE) || p.at(T![EOF])
            });

            if !line_is_blank {
                break;
            }

            if !consume_quote_prefix_without_virtual(p, quote_depth) {
                return BlankLineAction::EndItemBeforeBlank;
            }

            while p.at(MD_TEXTUAL_LITERAL) && is_whitespace_only(p.cur_text()) {
                p.bump(MD_TEXTUAL_LITERAL);
            }

            if p.at(NEWLINE) {
                p.bump(NEWLINE);
                continue;
            }

            return BlankLineAction::EndItemBeforeBlank;
        }

        if p.at(T![EOF]) {
            return BlankLineAction::EndItemBeforeBlank;
        }

        let prev_virtual = p.state().virtual_line_start;
        let has_prefix = consume_quote_prefix(p, quote_depth);
        if !has_prefix {
            p.state_mut().virtual_line_start = prev_virtual;
            return BlankLineAction::EndItemBeforeBlank;
        }
        let indent = line_indent_from_current(p);
        p.state_mut().virtual_line_start = prev_virtual;
        if indent >= required_indent {
            return BlankLineAction::ContinueItem;
        }

        if indent <= marker_indent + 3 {
            let is_list_marker = p.lookahead(|p| {
                skip_leading_whitespace_tokens(p);

                if p.at(MD_ORDERED_LIST_MARKER) {
                    p.bump(MD_ORDERED_LIST_MARKER);
                    return marker_followed_by_whitespace_or_eol(p);
                }

                if p.at(MD_SETEXT_UNDERLINE_LITERAL) {
                    if !is_single_dash_setext_marker(p.cur_text()) {
                        return false;
                    }
                    p.bump(MD_SETEXT_UNDERLINE_LITERAL);
                    return marker_followed_by_whitespace_or_eol(p);
                }

                if p.at(MD_TEXTUAL_LITERAL) && is_textual_bullet_marker(p.cur_text()) {
                    p.bump(MD_TEXTUAL_LITERAL);
                    return marker_followed_by_whitespace_or_eol(p);
                }

                if p.at(T![-]) || p.at(T![*]) || p.at(T![+]) {
                    p.bump(p.cur());
                    return marker_followed_by_whitespace_or_eol(p);
                }

                false
            });

            if is_list_marker {
                return BlankLineAction::EndItemAfterBlank;
            }
        }

        BlankLineAction::EndItemBeforeBlank
    })
}

fn at_blank_line_start(p: &mut MarkdownParser) -> bool {
    if !p.at_line_start() {
        return false;
    }

    at_blank_line_after_prefix(p)
}

fn at_blank_line_after_prefix(p: &mut MarkdownParser) -> bool {
    p.lookahead(|p| {
        if p.at(NEWLINE) {
            return p.at_blank_line();
        }
        if p.at(T![EOF]) {
            return true;
        }
        while p.at(MD_TEXTUAL_LITERAL) {
            let text = p.cur_text();
            if text == " " || text == "\t" {
                p.bump(MD_TEXTUAL_LITERAL);
            } else {
                break;
            }
        }

        if p.at(NEWLINE) {
            return p.source().at_line_start_with_whitespace();
        }

        p.at(T![EOF])
    })
}

fn consume_blank_line(p: &mut MarkdownParser) {
    while p.at(MD_TEXTUAL_LITERAL) {
        let text = p.cur_text();
        if text == " " || text == "\t" {
            p.parse_as_skipped_trivia_tokens(|p| p.bump(MD_TEXTUAL_LITERAL));
        } else {
            break;
        }
    }

    if p.at(NEWLINE) {
        let m = p.start();
        p.bump(NEWLINE);
        m.complete(p, MD_NEWLINE);
    }
}

/// Check if there's a bullet list item after skipping blank lines.
///
/// Per CommonMark §5.3, blank lines between list items don't end the list,
/// they just make it "loose". This function peeks ahead across blank lines
/// to see if another bullet item follows.
fn has_bullet_item_after_blank_lines(p: &mut MarkdownParser) -> bool {
    has_list_item_after_blank_lines(p, |p| {
        if p.at(T![-]) || p.at(T![*]) || p.at(T![+]) {
            p.bump(p.cur());
            marker_followed_by_whitespace_or_eol(p)
        } else {
            false
        }
    })
}

/// Check if there's an ordered list item after skipping blank lines.
///
/// Per CommonMark §5.3, blank lines between list items don't end the list,
/// they just make it "loose". This function peeks ahead across blank lines
/// to see if another ordered item follows.
fn has_ordered_item_after_blank_lines(p: &mut MarkdownParser) -> bool {
    has_list_item_after_blank_lines(p, |p| p.at(MD_ORDERED_LIST_MARKER))
}

fn has_list_item_after_blank_lines<F>(p: &mut MarkdownParser, has_marker: F) -> bool
where
    F: Fn(&mut MarkdownParser) -> bool,
{
    p.lookahead(|p| {
        // Skip all blank lines
        loop {
            // Skip whitespace on current line
            while p.at(MD_TEXTUAL_LITERAL) {
                let text = p.cur_text();
                if text == " " || text == "\t" {
                    p.bump(MD_TEXTUAL_LITERAL);
                } else {
                    break;
                }
            }

            // If at NEWLINE, consume it and continue checking
            if p.at(NEWLINE) {
                p.bump(NEWLINE);
                continue;
            }

            // Reached non-blank content or EOF
            break;
        }

        // Check for marker directly (avoid nested lookahead issues)
        // Skip leading indent (up to 3 spaces for list items)
        let mut indent = 0;
        while p.at(MD_TEXTUAL_LITERAL) {
            let text = p.cur_text();
            if text == " " {
                indent += 1;
                p.bump(MD_TEXTUAL_LITERAL);
            } else if text == "\t" {
                indent += 4;
                p.bump(MD_TEXTUAL_LITERAL);
            } else {
                break;
            }
        }

        // More than 3 spaces indent = indented code block, not a list item
        if indent > 3 {
            return false;
        }

        has_marker(p)
    })
}
