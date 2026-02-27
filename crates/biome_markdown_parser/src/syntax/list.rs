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

use biome_rowan::{TextRange, TextSize};

use crate::MarkdownParser;
use crate::syntax::fenced_code_block::parse_fenced_code_block;
use crate::syntax::header::{parse_header_content, parse_trailing_hashes};
use crate::syntax::html_block::{at_html_block, parse_html_block};
use crate::syntax::parse_any_block_with_indent_code_policy;
use crate::syntax::parse_error::list_nesting_too_deep;
use crate::syntax::quote::{
    at_quote_indented_code_start, consume_quote_prefix, consume_quote_prefix_without_virtual,
    has_quote_prefix, parse_quote, parse_quote_block_list, skip_optional_marker_space,
};
use crate::syntax::thematic_break_block::parse_thematic_break_block;
use crate::syntax::with_virtual_line_start;
use crate::syntax::{
    INDENT_CODE_BLOCK_SPACES, MAX_BLOCK_PREFIX_INDENT, TAB_STOP_SPACES, at_block_interrupt,
    at_indent_code_block, is_paragraph_like,
};

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
/// Compute the marker indent for list parsing.
///
/// For normal cases, this returns the leading whitespace count from
/// `line_start_leading_indent()`. For virtual line start cases (nested list
/// detection), we compute the actual column position from the source text
/// to ensure correct indented code block detection in nested lists.
///
/// Raw source scan is required because leading whitespace may be consumed
/// as trivia during list parsing, so token-based lookahead loses the true
/// column needed for CommonMark's indent rules.
fn compute_marker_indent(p: &MarkdownParser) -> usize {
    if p.state().virtual_line_start == Some(p.cur_range().start()) {
        // Inside block quotes, treat the virtual line start as column 0.
        if p.state().block_quote_depth > 0 {
            return p.line_start_leading_indent();
        }

        // Virtual line start: compute actual column from source text.
        // The leading whitespace was skipped as trivia, but we need the
        // real column for indented code block detection in nested lists.
        let source = p.source().source_text();
        let pos: usize = p.cur_range().start().into();

        // Find the start of the current line
        let line_start = source[..pos].rfind('\n').map_or(0, |i| i + 1);

        // Count columns from line start to current position
        let mut column = 0;
        for c in source[line_start..pos].chars() {
            match c {
                '\t' => column += TAB_STOP_SPACES - (column % TAB_STOP_SPACES),
                _ => column += 1,
            }
        }
        column
    } else {
        // Normal case: use the standard leading indent count
        p.source().line_start_leading_indent()
    }
}

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
        indent <= MAX_BLOCK_PREFIX_INDENT
    } else {
        indent >= base_indent && indent <= base_indent + MAX_BLOCK_PREFIX_INDENT
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

/// Check if the remaining content forms a thematic break pattern.
///
/// Per CommonMark §4.1, a thematic break is 3 or more matching characters
/// (`*`, `-`, or `_`) on a line by itself, optionally with spaces between them.
///
/// This function checks the source text directly since the lexer may not
/// produce MD_THEMATIC_BREAK_LITERAL in all contexts (e.g., after list markers).
/// Token lookahead is insufficient here because the marker may be lexed as
/// textual content within list item contexts.
fn is_thematic_break_pattern(p: &mut MarkdownParser) -> bool {
    // Get the remaining text on the current line
    let source = p.source_after_current();

    // Find the end of the line
    let line_end = source.find('\n').unwrap_or(source.len());
    let line = &source[..line_end];

    // Determine which character to check for
    let first_char = line.trim_start().chars().next();
    let break_char = match first_char {
        Some('*' | '-' | '_') => first_char.unwrap(),
        _ => return false,
    };

    // Count the break characters (must be at least 3)
    let mut count = 0usize;
    for c in line.chars() {
        if c == break_char {
            count += 1;
        } else if c != ' ' && c != '\t' {
            // Non-whitespace, non-break character - not a thematic break
            return false;
        }
    }

    count >= 3
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
    //
    // Any NEWLINE we see at this position (after the item-terminating newline)
    // represents a blank line between items. We don't use at_blank_line() here
    // because it checks if what comes AFTER the newline is blank, but we're
    // already past one newline - any additional newlines ARE blank lines.
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
    let prev_is_tight = *is_tight;
    let prev_last_item_ends_with_blank = *last_item_ends_with_blank;

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

    if parsed.is_absent() {
        // The blank lines we skipped didn't lead to a valid item in this list.
        // Restore tightness — the blank lines belong to a parent context.
        *is_tight = prev_is_tight;
        *last_item_ends_with_blank = prev_last_item_ends_with_blank;
    } else {
        update_list_tightness(blank_info, is_tight, last_item_ends_with_blank);
    }

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
    /// The indentation level of the list marker (0 for top-level).
    marker_indent: usize,
}

impl BulletList {
    fn new(marker_indent: usize) -> Self {
        Self {
            is_tight: true,
            last_item_ends_with_blank: false,
            marker_kind: None,
            marker_indent,
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
        let marker_indent = self.marker_indent;

        // Check blank line at line start with indent awareness BEFORE
        // delegating to is_at_list_end_common (which uses non-indent-aware check).
        if p.at_line_start() && at_blank_line_start(p) {
            let result = !has_bullet_item_after_blank_lines_at_indent(p, marker_indent);

            return result;
        }

        is_at_list_end_common(
            p,
            self.marker_kind,
            at_bullet_list_item,
            current_bullet_marker,
            has_bullet_item_after_blank_lines,
            |p, _marker_kind| {
                let next_is_bullet_at_indent = p.lookahead(|p| {
                    p.bump(NEWLINE);
                    // Count indent before marker (tabs expand to next tab stop)
                    let mut indent = 0usize;
                    while p.at(MD_TEXTUAL_LITERAL) {
                        let text = p.cur_text();
                        if text == " " {
                            indent += 1;
                            p.bump(MD_TEXTUAL_LITERAL);
                        } else if text == "\t" {
                            indent += TAB_STOP_SPACES - (indent % TAB_STOP_SPACES);
                            p.bump(MD_TEXTUAL_LITERAL);
                        } else {
                            break;
                        }
                    }
                    // Check indent matches this list's marker indent
                    let indent_ok = if marker_indent == 0 {
                        indent <= MAX_BLOCK_PREFIX_INDENT
                    } else {
                        indent >= marker_indent && indent <= marker_indent + MAX_BLOCK_PREFIX_INDENT
                    };
                    if !indent_ok {
                        return false;
                    }
                    if p.at(T![-]) || p.at(T![*]) || p.at(T![+]) {
                        p.bump(p.cur());
                        return marker_followed_by_whitespace_or_eol(p);
                    }
                    false
                });
                if next_is_bullet_at_indent {
                    Some(false)
                } else {
                    // Check if bullet after blank lines is at correct indent
                    let has_item = p.lookahead(|p| {
                        has_bullet_item_after_blank_lines_at_indent(p, marker_indent)
                    });
                    Some(!has_item)
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

    // Compute the marker indent (leading whitespace before the first marker)
    let marker_indent = compute_marker_indent(p);

    // Use ParseNodeList to parse the list with proper recovery
    let mut list_helper = BulletList::new(marker_indent);
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

    // Compute the marker indent, handling both normal and virtual line start cases.
    // For virtual line start (nested list detection), we compute the actual column
    // to ensure correct indented code block detection.
    let marker_indent = compute_marker_indent(p);
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
    //
    // For the setext-remapped case (marker_token_text is Some), the token includes
    // trailing spaces before the newline. This means the first line is empty
    // (marker + whitespace + newline), and the trailing spaces shouldn't count
    // for indentation purposes. Per CommonMark, the required indent is marker_width + 1.
    let (spaces_after_marker, first_line_empty) = if let Some(text) = marker_token_text.as_deref() {
        // Setext token case: token is "- " or "-  " etc. followed by newline
        // The first line is empty, so use minimum indent (marker_width + 1)
        let spaces = count_spaces_after_dash_in_token(text, marker_indent + marker_width);
        (spaces, true)
    } else {
        let spaces =
            count_spaces_after_marker(p.source_after_current(), marker_indent + marker_width);
        // Check if first line is empty by looking at what follows
        let first_empty = p.lookahead(|p| {
            // Skip any whitespace
            while p.at(MD_TEXTUAL_LITERAL) && is_whitespace_only(p.cur_text()) {
                p.bump(MD_TEXTUAL_LITERAL);
            }
            // If we hit newline or EOF, first line is empty
            p.at(NEWLINE) || p.at(T![EOF])
        });
        (spaces, first_empty)
    };

    // Set required indent for continuation lines
    // Required indent = marker width + spaces after marker (minimum 1)
    // BUT: if first line is empty (marker + whitespace + newline), use minimum indent
    let prev_required_indent = p.state().list_item_required_indent;
    let prev_marker_indent = p.state().list_item_marker_indent;
    p.state_mut().list_item_required_indent = if spaces_after_marker > INDENT_CODE_BLOCK_SPACES {
        marker_indent + marker_width + 1
    } else if first_line_empty {
        // Empty first line: use minimum indent (marker + 1 space)
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

    // Compute the marker indent, handling both normal and virtual line start cases.
    // For virtual line start (nested list detection), we compute the actual column
    // to ensure correct indented code block detection.
    let marker_indent = compute_marker_indent(p);
    skip_list_marker_indent(p);

    // Get marker width from actual token text (e.g., "1." = 2, "10." = 3)
    let marker_width = p.cur_text().len();

    // Bump the ordered list marker
    p.bump(MD_ORDERED_LIST_MARKER);

    // Count spaces after marker to determine required indentation.
    // Per CommonMark §5.2, content aligns to first non-space after marker.
    let spaces_after_marker =
        count_spaces_after_marker(p.source_after_current(), marker_indent + marker_width);

    // Check if first line is empty (marker followed by only whitespace + newline)
    let first_line_empty = p.lookahead(|p| {
        while p.at(MD_TEXTUAL_LITERAL) && is_whitespace_only(p.cur_text()) {
            p.bump(MD_TEXTUAL_LITERAL);
        }
        p.at(NEWLINE) || p.at(T![EOF])
    });

    // Set required indent for continuation lines
    // Required indent = marker width + spaces after marker (minimum 1)
    // BUT: if first line is empty (marker + whitespace + newline), use minimum indent
    let prev_required_indent = p.state().list_item_required_indent;
    let prev_marker_indent = p.state().list_item_marker_indent;
    p.state_mut().list_item_required_indent = if spaces_after_marker > INDENT_CODE_BLOCK_SPACES {
        marker_indent + marker_width + 1
    } else if first_line_empty {
        // Empty first line: use minimum indent (marker + 1 space)
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
            '\t' => column += TAB_STOP_SPACES - (column % TAB_STOP_SPACES),
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
            '\t' => column += TAB_STOP_SPACES - (column % TAB_STOP_SPACES),
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
            '\t' => column += TAB_STOP_SPACES - (column % TAB_STOP_SPACES),
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
                    let advance = TAB_STOP_SPACES - (column % TAB_STOP_SPACES);
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
                indent += TAB_STOP_SPACES - (indent % TAB_STOP_SPACES);
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
                        let advance = TAB_STOP_SPACES - (column % TAB_STOP_SPACES);
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
                    indent += TAB_STOP_SPACES - (indent % TAB_STOP_SPACES);
                    i += 1;
                }
                _ => return Some(indent),
            }
        }

        let newline_index = newline_index?;
        line_start = newline_index + 1;
    }
}

// #region Supporting types for `parse_list_item_block_content`

/// Outcome of a loop-body phase in the list-item parser.
enum LoopAction {
    Break,
    Continue,
    FallThrough,
}

/// Whether `virtual_line_start` needs to be restored after parsing a block.
enum VirtualLineRestore {
    /// No restore needed.
    None,
    /// Restore to the given value after parsing the continuation block.
    Restore(Option<TextSize>),
}

/// Result of the continuation-indent check.
struct ContinuationResult {
    action: LoopAction,
    /// Whether virtual_line_start must be restored after parsing the block.
    restore: VirtualLineRestore,
}

/// Mutable loop state for `parse_list_item_block_content`.
struct ListItemLoopState {
    has_blank_line: bool,
    last_was_blank: bool,
    last_block_was_paragraph: bool,
    first_line: bool,
    required_indent: usize,
    marker_indent: usize,
}

impl ListItemLoopState {
    fn new(p: &MarkdownParser) -> Self {
        Self {
            has_blank_line: false,
            last_was_blank: false,
            last_block_was_paragraph: false,
            first_line: true,
            required_indent: p.state().list_item_required_indent,
            marker_indent: p.state().list_item_marker_indent,
        }
    }

    /// Record that a block-level construct was parsed on the first line.
    fn record_first_line_block(&mut self) {
        self.last_block_was_paragraph = false;
        self.last_was_blank = false;
        self.first_line = false;
    }

    /// Record that a blank line was encountered.
    fn record_blank(&mut self) {
        self.has_blank_line = true;
        self.last_was_blank = true;
    }
}

/// Nested list marker kind detected on the first line.
enum NestedListMarker {
    Bullet,
    Ordered,
}

// #endregion

// #region Helper functions for `parse_list_item_block_content`

/// Handle all blank-line detection and classification (phases 1-5).
///
/// Returns `(action, line_has_quote_prefix)`. The boolean is needed by the
/// orchestrator for quote prefix consumption afterward.
///
/// May return any `LoopAction` variant. `FallThrough` means no blank-line
/// handling applied and the caller should proceed to subsequent phases.
fn handle_blank_lines(p: &mut MarkdownParser, state: &mut ListItemLoopState) -> (LoopAction, bool) {
    let quote_depth = p.state().block_quote_depth;

    // Phase 1: Quote depth early exit — if we're past the first line and the
    // next quoted content has insufficient indent, break.
    if !state.first_line
        && quote_depth > 0
        && quote_only_line_indent_at_current(p, quote_depth).is_some()
        && let Some(next_indent) = next_quote_content_indent(p, quote_depth)
        && next_indent < state.required_indent
    {
        return (LoopAction::Break, false);
    }

    let newline_has_quote_prefix = quote_depth > 0
        && p.at(NEWLINE)
        && (p.at_line_start() || p.has_preceding_line_break())
        && has_quote_prefix(p, quote_depth);

    // Phase 2: Quote-only blank line detection (e.g., ">>").
    if !state.first_line && quote_depth > 0 && p.at(NEWLINE) {
        let is_quote_blank_line = p.lookahead(|p| {
            p.bump(NEWLINE);
            if !has_quote_prefix(p, quote_depth) {
                return false;
            }
            consume_quote_prefix_without_virtual(p, quote_depth);
            while p.at(MD_TEXTUAL_LITERAL) && is_whitespace_only(p.cur_text()) {
                p.bump(MD_TEXTUAL_LITERAL);
            }
            p.at(NEWLINE) || p.at(T![EOF])
        });

        if is_quote_blank_line {
            let m = p.start();
            p.bump(NEWLINE);
            m.complete(p, MD_NEWLINE);
            consume_quote_prefix(p, quote_depth);
            consume_blank_line(p);
            state.record_blank();
            state.first_line = false;
            return (LoopAction::Continue, false);
        }
    }

    // Phase 3: Non-quote blank line classification.
    if !state.first_line && p.at(NEWLINE) && !p.at_blank_line() && !newline_has_quote_prefix {
        let action = classify_blank_line(p, state.required_indent, state.marker_indent);
        let is_blank = list_newline_is_blank_line(p);
        let result = apply_blank_line_action(p, state, action, is_blank);
        return (result, false);
    }

    // Phase 4: Quote-only line after prefix.
    let line_has_quote_prefix = quote_depth > 0
        && (p.at_line_start() || p.has_preceding_line_break())
        && (has_quote_prefix(p, quote_depth)
            || quote_only_line_indent_at_current(p, quote_depth).is_some());

    if line_has_quote_prefix {
        let is_quote_only_line = p.lookahead(|p| {
            consume_quote_prefix_without_virtual(p, quote_depth);
            while p.at(MD_TEXTUAL_LITERAL) && is_whitespace_only(p.cur_text()) {
                p.bump(MD_TEXTUAL_LITERAL);
            }
            p.at(NEWLINE) || p.at(T![EOF])
        });

        if is_quote_only_line {
            consume_quote_prefix(p, quote_depth);
            consume_blank_line(p);
            if !state.first_line {
                state.has_blank_line = true;
            }
            state.last_was_blank = true;
            state.first_line = false;
            return (LoopAction::Continue, line_has_quote_prefix);
        }
    }

    // Phase 5: Blank line after prefix with indent checks.
    let blank_line_after_prefix = if line_has_quote_prefix {
        p.lookahead(|p| {
            consume_quote_prefix_without_virtual(p, quote_depth);
            at_blank_line_after_prefix(p)
        })
    } else {
        at_blank_line_after_prefix(p)
    };

    // On the first line, if at a blank line at NEWLINE, fall through to
    // handle_first_line_marker_only below.
    if state.first_line && blank_line_after_prefix && p.at(NEWLINE) {
        return (LoopAction::FallThrough, line_has_quote_prefix);
    }

    if (p.at_line_start() || line_has_quote_prefix) && blank_line_after_prefix {
        if line_has_quote_prefix
            && quote_only_line_indent_at_current(p, quote_depth).is_some()
            && let Some(next_indent) = next_quote_content_indent(p, quote_depth)
        {
            if next_indent >= state.required_indent {
                consume_quote_prefix(p, quote_depth);
                consume_blank_line(p);
                if !state.first_line {
                    state.has_blank_line = true;
                }
                state.last_was_blank = true;
                state.first_line = false;
                return (LoopAction::Continue, line_has_quote_prefix);
            } else {
                return (LoopAction::Break, line_has_quote_prefix);
            }
        }
        let marker_line_break = state.first_line;
        let action = if quote_depth > 0 {
            classify_blank_line_in_quote(p, state.required_indent, state.marker_indent, quote_depth)
        } else {
            classify_blank_line(p, state.required_indent, state.marker_indent)
        };
        let result = apply_blank_line_action_with_prefix(
            p,
            state,
            action,
            line_has_quote_prefix,
            quote_depth,
            marker_line_break,
        );
        return (result, line_has_quote_prefix);
    }

    (LoopAction::FallThrough, line_has_quote_prefix)
}

/// Apply a `BlankLineAction` from phase 3 (non-quote blank line classification).
fn apply_blank_line_action(
    p: &mut MarkdownParser,
    state: &mut ListItemLoopState,
    action: BlankLineAction,
    is_blank: bool,
) -> LoopAction {
    match action {
        BlankLineAction::ContinueItem => {
            consume_blank_line(p);
            if is_blank {
                state.has_blank_line = true;
            }
            state.last_was_blank = is_blank;
            LoopAction::Continue
        }
        BlankLineAction::EndItemAfterBlank => {
            consume_blank_line(p);
            state.record_blank();
            LoopAction::Break
        }
        BlankLineAction::EndItemAtBoundary => {
            consume_blank_line(p);
            if is_blank {
                state.has_blank_line = true;
                state.last_was_blank = true;
            }
            LoopAction::Break
        }
        BlankLineAction::EndItemBeforeBlank => LoopAction::Break,
    }
}

/// Apply a `BlankLineAction` from phase 5 (blank line after prefix).
fn apply_blank_line_action_with_prefix(
    p: &mut MarkdownParser,
    state: &mut ListItemLoopState,
    action: BlankLineAction,
    line_has_quote_prefix: bool,
    quote_depth: usize,
    marker_line_break: bool,
) -> LoopAction {
    match action {
        BlankLineAction::ContinueItem => {
            if line_has_quote_prefix {
                consume_quote_prefix(p, quote_depth);
            }
            consume_blank_line(p);
            if !marker_line_break {
                state.has_blank_line = true;
            }
            state.last_was_blank = true;
            state.first_line = false;
            LoopAction::Continue
        }
        BlankLineAction::EndItemAfterBlank | BlankLineAction::EndItemAtBoundary => {
            // In the blank_line_after_prefix path, we know there's an
            // actual blank line, so EndItemAtBoundary behaves like EndItemAfterBlank.
            if line_has_quote_prefix {
                consume_quote_prefix(p, quote_depth);
            }
            consume_blank_line(p);
            if !marker_line_break {
                state.has_blank_line = true;
            }
            state.last_was_blank = true;
            LoopAction::Break
        }
        BlankLineAction::EndItemBeforeBlank => LoopAction::Break,
    }
}

/// Handle the first line when at NEWLINE with no inline content (marker-only).
///
/// Returns `Break` if the item is empty, `Continue` if the next line should be
/// processed, or `FallThrough` if this phase doesn't apply.
fn handle_first_line_marker_only(
    p: &mut MarkdownParser,
    state: &mut ListItemLoopState,
) -> LoopAction {
    if !state.first_line || !p.at(NEWLINE) {
        return LoopAction::FallThrough;
    }

    let next_is_sibling = p.lookahead(|p| {
        p.bump(NEWLINE);
        if p.at_line_start() {
            at_bullet_list_item_with_base_indent(p, state.marker_indent)
                || at_order_list_item_with_base_indent(p, state.marker_indent)
        } else {
            false
        }
    });

    // Marker-only line: consume the newline as trivia and continue.
    p.parse_as_skipped_trivia_tokens(|p| p.bump(NEWLINE));
    state.first_line = false;
    state.last_was_blank = false;

    if next_is_sibling {
        return LoopAction::Continue;
    }

    // Now check if we're at a blank line (the line immediately after marker is empty).
    // Per CommonMark: if marker-only line is followed by a blank line,
    // the item is truly empty and subsequent content is outside the list.
    let now_at_blank_line = p.lookahead(|p| {
        while p.at(MD_TEXTUAL_LITERAL) && is_whitespace_only(p.cur_text()) {
            p.bump(MD_TEXTUAL_LITERAL);
        }
        p.at(NEWLINE) || p.at(T![EOF])
    });

    if now_at_blank_line {
        return LoopAction::Break;
    }

    LoopAction::Continue
}

/// Parse block-level constructs on the first line of a list item.
///
/// Handles: fenced code, HTML block, ATX heading, blockquote, thematic break,
/// nested list, and indent code block. Returns `Continue` if a block was parsed,
/// `FallThrough` if none matched.
fn parse_first_line_blocks(
    p: &mut MarkdownParser,
    state: &mut ListItemLoopState,
    spaces_after_marker: usize,
) -> LoopAction {
    if !state.first_line {
        return LoopAction::FallThrough;
    }

    // Fenced code block
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

        let parsed = with_virtual_line_start(p, p.cur_range().start(), parse_fenced_code_block);
        if parsed.is_present() {
            state.record_first_line_block();
            return LoopAction::Continue;
        }
    }

    // HTML block
    let html_block_start =
        p.lookahead(|p| with_virtual_line_start(p, p.cur_range().start(), at_html_block));

    if html_block_start {
        let parsed = with_virtual_line_start(p, p.cur_range().start(), parse_html_block);
        if parsed.is_present() {
            state.record_first_line_block();
            return LoopAction::Continue;
        }
    }

    // ATX heading
    if parse_first_line_atx_heading(p, state) {
        return LoopAction::Continue;
    }

    // Blockquote
    if parse_first_line_blockquote(p, state) {
        return LoopAction::Continue;
    }

    // Thematic break (check BEFORE nested list markers per CommonMark §4.1)
    let is_thematic_break = p.lookahead(|p| {
        while p.at(MD_TEXTUAL_LITERAL) && is_whitespace_only(p.cur_text()) {
            p.bump(MD_TEXTUAL_LITERAL);
        }
        if p.at(MD_THEMATIC_BREAK_LITERAL) {
            return true;
        }
        is_thematic_break_pattern(p)
    });

    if is_thematic_break {
        if parse_thematic_break_block(p).is_present() {
            state.record_first_line_block();
        }
        return LoopAction::Continue;
    }

    // Nested list
    let nested_marker = p.lookahead(|p| {
        while p.at(MD_TEXTUAL_LITERAL) && is_whitespace_only(p.cur_text()) {
            p.bump(MD_TEXTUAL_LITERAL);
        }

        if p.at(MD_ORDERED_LIST_MARKER) {
            p.bump(MD_ORDERED_LIST_MARKER);
            return marker_followed_by_whitespace_or_eol(p).then_some(NestedListMarker::Ordered);
        }

        if p.at(MD_SETEXT_UNDERLINE_LITERAL) && is_single_dash_setext_marker(p.cur_text()) {
            p.bump(MD_SETEXT_UNDERLINE_LITERAL);
            return marker_followed_by_whitespace_or_eol(p).then_some(NestedListMarker::Bullet);
        }

        if p.at(T![-]) || p.at(T![*]) || p.at(T![+]) {
            p.bump(p.cur());
            return marker_followed_by_whitespace_or_eol(p).then_some(NestedListMarker::Bullet);
        }

        if p.at(MD_TEXTUAL_LITERAL) && is_textual_bullet_marker(p.cur_text()) {
            p.bump(MD_TEXTUAL_LITERAL);
            return marker_followed_by_whitespace_or_eol(p).then_some(NestedListMarker::Bullet);
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
            let parsed = parse_any_block_with_indent_code_policy(p, true);
            state.last_block_was_paragraph = if let Present(ref marker) = parsed {
                is_paragraph_like(marker.kind(p))
            } else {
                false
            };
        } else {
            state.last_block_was_paragraph = false;
        }
        state.first_line = false;

        p.state_mut().virtual_line_start = prev_virtual;
        p.state_mut().list_item_required_indent = prev_required;
        return LoopAction::Continue;
    }

    // Indent code block (spaces_after_marker > INDENT_CODE_BLOCK_SPACES)
    if spaces_after_marker > INDENT_CODE_BLOCK_SPACES {
        parse_indent_code_block_in_list_first_line(p);
        state.record_first_line_block();
        return LoopAction::Continue;
    }

    LoopAction::FallThrough
}

/// Parse an ATX heading on the first line of list item content.
/// Returns `true` if a heading was parsed.
fn parse_first_line_atx_heading(p: &mut MarkdownParser, state: &mut ListItemLoopState) -> bool {
    let atx_heading_info = p.lookahead(|p| {
        while p.at(MD_TEXTUAL_LITERAL) && is_whitespace_only(p.cur_text()) {
            p.bump(MD_TEXTUAL_LITERAL);
        }
        let is_hash =
            p.at(T![#]) || (p.at(MD_TEXTUAL_LITERAL) && p.cur_text().chars().all(|c| c == '#'));
        if !is_hash {
            return None;
        }
        let text = p.cur_text();
        let hash_count = text.len();
        if !(1..=6).contains(&hash_count) {
            return None;
        }
        p.bump(p.cur());
        if p.at(NEWLINE) || p.at(T![EOF]) {
            return Some(hash_count);
        }
        if p.at(MD_TEXTUAL_LITERAL) {
            let t = p.cur_text();
            if t.starts_with(' ') || t.starts_with('\t') {
                return Some(hash_count);
            }
        }
        None
    });

    if atx_heading_info.is_none() {
        return false;
    }

    // Skip leading whitespace as trivia
    while p.at(MD_TEXTUAL_LITERAL) && is_whitespace_only(p.cur_text()) {
        p.parse_as_skipped_trivia_tokens(|p| p.bump(MD_TEXTUAL_LITERAL));
    }

    let header_m = p.start();

    // Can't reuse header::parse_hash_list(): in list context `#` may be lexed as
    // MD_TEXTUAL_LITERAL and requires bump_remap. Keep in sync with parse_hash_list().
    let hash_list_m = p.start();
    let hash_m = p.start();
    if p.at(T![#]) {
        p.bump(T![#]);
    } else {
        p.bump_remap(T![#]);
    }
    hash_m.complete(p, MD_HASH);
    hash_list_m.complete(p, MD_HASH_LIST);

    parse_header_content(p);
    parse_trailing_hashes(p);

    header_m.complete(p, MD_HEADER);

    state.record_first_line_block();
    true
}

/// Parse a blockquote on the first line of list item content.
/// Returns `true` if a blockquote was parsed.
fn parse_first_line_blockquote(p: &mut MarkdownParser, state: &mut ListItemLoopState) -> bool {
    let blockquote_start = p.lookahead(|p| {
        while p.at(MD_TEXTUAL_LITERAL) && is_whitespace_only(p.cur_text()) {
            p.bump(MD_TEXTUAL_LITERAL);
        }
        p.at(T![>]) || (p.at(MD_TEXTUAL_LITERAL) && p.cur_text() == ">")
    });

    if !blockquote_start {
        return false;
    }

    // Skip leading whitespace as trivia
    while p.at(MD_TEXTUAL_LITERAL) && is_whitespace_only(p.cur_text()) {
        p.parse_as_skipped_trivia_tokens(|p| p.bump(MD_TEXTUAL_LITERAL));
    }

    let prev_virtual = p.state().virtual_line_start;
    let prev_required = p.state().list_item_required_indent;
    p.state_mut().virtual_line_start = Some(p.cur_range().start());
    p.state_mut().list_item_required_indent = 0;

    let parsed = if p.at(MD_TEXTUAL_LITERAL) && p.cur_text() == ">" {
        let quote_m = p.start();
        p.bump_remap(T![>]);
        p.state_mut().block_quote_depth += 1;

        let has_indented_code = at_quote_indented_code_start(p);
        let marker_space = skip_optional_marker_space(p, has_indented_code);
        p.set_virtual_line_start();

        parse_quote_block_list(p);

        p.state_mut().block_quote_depth -= 1;
        let completed = quote_m.complete(p, MD_QUOTE);
        let range = completed.range(p);
        let indent = 1 + if marker_space { 1 } else { 0 };
        p.record_quote_indent(range, indent);

        state.record_first_line_block();
        true
    } else {
        // T![>] case: parse_quote can handle it directly
        let parsed = parse_quote(p);
        if parsed.is_present() {
            state.record_first_line_block();
            true
        } else {
            false
        }
    };

    p.state_mut().virtual_line_start = prev_virtual;
    p.state_mut().list_item_required_indent = prev_required;
    parsed
}

/// After the first line, verify indent level for continuation.
///
/// Handles nested list detection at sufficient indent, sibling/block interrupt
/// at insufficient indent, and lazy continuation.
///
/// Returns `ContinuationResult`. `FallThrough` means indent checks passed and
/// the caller should proceed to parse the block.
fn check_continuation_indent(
    p: &mut MarkdownParser,
    state: &mut ListItemLoopState,
    line_started_with_quote_prefix: bool,
    prev_was_blank: bool,
) -> ContinuationResult {
    if state.first_line || (!p.at_line_start() && !line_started_with_quote_prefix) {
        return ContinuationResult {
            action: LoopAction::FallThrough,
            restore: VirtualLineRestore::None,
        };
    }

    let indent = line_indent_from_current(p);

    if indent < state.marker_indent {
        return ContinuationResult {
            action: LoopAction::Break,
            restore: VirtualLineRestore::None,
        };
    }

    if indent >= state.required_indent {
        let allow_indent_code_block = !state.last_block_was_paragraph || prev_was_blank;
        let is_indent_code_block =
            allow_indent_code_block && indent >= state.required_indent + INDENT_CODE_BLOCK_SPACES;
        if !is_indent_code_block {
            // Sufficient indentation - skip it and continue
            p.skip_line_indent(state.required_indent);
            let prev_virtual = p.state().virtual_line_start;
            p.state_mut().virtual_line_start = Some(p.cur_range().start());

            if at_bullet_list_item(p) {
                let _ = parse_bullet_list_item(p);
                state.last_block_was_paragraph = false;
                state.first_line = false;
                p.state_mut().virtual_line_start = prev_virtual;
                return ContinuationResult {
                    action: LoopAction::Continue,
                    restore: VirtualLineRestore::None,
                };
            }
            if at_order_list_item(p) {
                let _ = parse_order_list_item(p);
                state.last_block_was_paragraph = false;
                state.first_line = false;
                p.state_mut().virtual_line_start = prev_virtual;
                return ContinuationResult {
                    action: LoopAction::Continue,
                    restore: VirtualLineRestore::None,
                };
            }

            return ContinuationResult {
                action: LoopAction::FallThrough,
                restore: VirtualLineRestore::Restore(prev_virtual),
            };
        }
    } else {
        // Insufficient indentation - check for block interrupts
        if at_bullet_list_item_with_base_indent(p, state.marker_indent)
            || at_order_list_item_with_base_indent(p, state.marker_indent)
        {
            return ContinuationResult {
                action: LoopAction::Break,
                restore: VirtualLineRestore::None,
            };
        }

        if at_block_interrupt(p) {
            return ContinuationResult {
                action: LoopAction::Break,
                restore: VirtualLineRestore::None,
            };
        }

        // Lazy continuation per CommonMark §5.2
        if !state.last_block_was_paragraph {
            return ContinuationResult {
                action: LoopAction::Break,
                restore: VirtualLineRestore::None,
            };
        }
    }

    ContinuationResult {
        action: LoopAction::FallThrough,
        restore: VirtualLineRestore::None,
    }
}

/// Parse the continuation block and update state. Restores `virtual_line_start`
/// if required by `restore`.
fn parse_continuation_block(
    p: &mut MarkdownParser,
    state: &mut ListItemLoopState,
    prev_was_blank: bool,
    restore: VirtualLineRestore,
) {
    let is_blank_line = p.at_blank_line();
    if is_blank_line {
        state.record_blank();
    } else {
        state.last_was_blank = false;
    }

    state.first_line = false;

    let allow_indent_code_block = !state.last_block_was_paragraph || prev_was_blank;
    let parsed = parse_any_block_with_indent_code_policy(p, allow_indent_code_block);
    state.last_block_was_paragraph = if let Present(ref marker) = parsed {
        is_paragraph_like(marker.kind(p))
    } else {
        false
    };

    if let VirtualLineRestore::Restore(prev_virtual) = restore {
        p.state_mut().virtual_line_start = prev_virtual;
    }
}

// #endregion

// #region Orchestrator

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
    let mut state = ListItemLoopState::new(p);

    loop {
        if p.at(T![EOF]) {
            break;
        }

        // Blank line handling (phases 1-5)
        let (action, line_has_quote_prefix) = handle_blank_lines(p, &mut state);
        match action {
            LoopAction::Break => break,
            LoopAction::Continue => continue,
            LoopAction::FallThrough => {}
        }

        // Consume quote prefix if present
        let quote_depth = p.state().block_quote_depth;
        if line_has_quote_prefix {
            consume_quote_prefix(p, quote_depth);
        }
        let line_started_with_quote_prefix = line_has_quote_prefix;
        let prev_was_blank = state.last_was_blank;

        // First-line: marker-only (no inline content)
        match handle_first_line_marker_only(p, &mut state) {
            LoopAction::Break => break,
            LoopAction::Continue => continue,
            LoopAction::FallThrough => {}
        }

        // First-line: block-level constructs
        match parse_first_line_blocks(p, &mut state, spaces_after_marker) {
            LoopAction::Continue => continue,
            LoopAction::FallThrough => {}
            LoopAction::Break => break,
        }

        // Continuation indent check
        let cont = check_continuation_indent(
            p,
            &mut state,
            line_started_with_quote_prefix,
            prev_was_blank,
        );
        match cont.action {
            LoopAction::Break => break,
            LoopAction::Continue => continue,
            LoopAction::FallThrough => {}
        }

        // Parse block content
        parse_continuation_block(p, &mut state, prev_was_blank, cont.restore);
    }

    m.complete(p, MD_BLOCK_LIST);
    ListItemBlankInfo {
        has_blank_line: state.has_blank_line,
        ends_with_blank_line: state.last_was_blank,
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
    /// End item; actual blank lines were found before the next item.
    EndItemAfterBlank,
    /// End item; no actual blank lines, just a normal item boundary.
    EndItemAtBoundary,
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
        let mut blank_lines_found = 0usize;
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

            blank_lines_found += 1;

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
        if indent <= marker_indent + MAX_BLOCK_PREFIX_INDENT
            && (at_bullet_list_item_with_base_indent(p, marker_indent)
                || at_order_list_item_with_base_indent(p, marker_indent))
        {
            // The first "blank line" is just the item-ending newline.
            // Only report actual blank lines if more than 1 was found.
            if blank_lines_found > 1 {
                return BlankLineAction::EndItemAfterBlank;
            }
            return BlankLineAction::EndItemAtBoundary;
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

        if indent <= marker_indent + MAX_BLOCK_PREFIX_INDENT {
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

/// Like `has_bullet_item_after_blank_lines` but also checks that the
/// bullet marker is at the expected indent level for this list.
fn has_bullet_item_after_blank_lines_at_indent(
    p: &mut MarkdownParser,
    expected_indent: usize,
) -> bool {
    has_list_item_after_blank_lines_at_indent(p, expected_indent, |p| {
        if p.at(T![-]) || p.at(T![*]) || p.at(T![+]) {
            p.bump(p.cur());
            marker_followed_by_whitespace_or_eol(p)
        } else {
            false
        }
    })
}

fn has_list_item_after_blank_lines_at_indent<F>(
    p: &mut MarkdownParser,
    expected_indent: usize,
    has_marker: F,
) -> bool
where
    F: Fn(&mut MarkdownParser) -> bool,
{
    p.lookahead(|p| {
        // Skip all blank lines
        loop {
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

        let mut indent = 0;
        while p.at(MD_TEXTUAL_LITERAL) {
            let text = p.cur_text();
            if text == " " {
                indent += 1;
                p.bump(MD_TEXTUAL_LITERAL);
            } else if text == "\t" {
                indent += TAB_STOP_SPACES - (indent % TAB_STOP_SPACES);
                p.bump(MD_TEXTUAL_LITERAL);
            } else {
                break;
            }
        }

        // Check indent matches the list's marker indent range
        if expected_indent == 0 {
            if indent > MAX_BLOCK_PREFIX_INDENT {
                return false;
            }
        } else if indent < expected_indent || indent > expected_indent + MAX_BLOCK_PREFIX_INDENT {
            return false;
        }

        has_marker(p)
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
                indent += TAB_STOP_SPACES - (indent % TAB_STOP_SPACES);
                p.bump(MD_TEXTUAL_LITERAL);
            } else {
                break;
            }
        }

        // More than 3 spaces indent = indented code block, not a list item
        if indent > MAX_BLOCK_PREFIX_INDENT {
            return false;
        }

        has_marker(p)
    })
}

// #endregion
