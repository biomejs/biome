//! Link reference definition parsing for Markdown (CommonMark §4.7).
//!
//! A link reference definition is a block-level construct that defines a label
//! for later reference. The syntax is:
//!
//! ```markdown
//! [label]: url "optional title"
//! [label]: url 'optional title'
//! [label]: url (optional title)
//! [label]: <url>
//! ```
//!
//! These definitions are not rendered but provide targets for reference links.
//!
//! # CommonMark Spec §4.7 Requirements
//!
//! 1. Label is enclosed in `[` and `]`, followed by `:`
//! 2. Label may contain up to 999 characters (no unescaped `]`)
//! 3. Destination can be angle-bracketed `<url>` or bare URL (no whitespace)
//! 4. Optional title can be quoted with `"`, `'`, or `()`
//! 5. Labels are case-insensitive and whitespace-normalized for matching

use biome_markdown_syntax::MarkdownSyntaxKind::*;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax::{self, *};

use crate::MarkdownParser;
use crate::lexer::MarkdownLexContext;
use crate::syntax::reference::normalize_reference_label;
use crate::syntax::{
    LinkDestinationKind, MAX_BLOCK_PREFIX_INDENT, MAX_LINK_DESTINATION_PAREN_DEPTH,
    ParenDepthResult, ends_with_unescaped_close, try_update_paren_depth,
    validate_link_destination_text,
};

/// Maximum label length per CommonMark spec (999 characters).
const MAX_LABEL_LENGTH: usize = 999;

/// Check if we're at the start of a link reference definition.
///
/// A link reference definition starts with `[` at the beginning of a line
/// (with up to 3 spaces of indentation allowed).
///
/// We use token-based lookahead to verify the pattern: `[label]: destination`
/// where label doesn't contain unescaped `]` or `[`, and is followed by `:`.
pub(crate) fn at_link_block(p: &mut MarkdownParser) -> bool {
    p.lookahead(|p| {
        // Must be at line start (or start of input)
        if !p.at_line_start() && !p.at_start_of_input() {
            return false;
        }

        // Check for up to 3 spaces of indentation (more means indented code block)
        if p.line_start_leading_indent() > MAX_BLOCK_PREFIX_INDENT {
            return false;
        }

        p.skip_line_indent(MAX_BLOCK_PREFIX_INDENT);

        // Must start with `[`
        if !p.at(L_BRACK) {
            return false;
        }

        // Use token-based lookahead to verify this is a valid link reference definition
        is_valid_link_definition_lookahead(p)
    })
}

/// Token-based lookahead to verify a link reference definition.
///
/// This advances tokens to check: `[label]: destination [title]?`
/// Returns true if the pattern is valid, false otherwise.
/// Does NOT build nodes - just validates the structure.
fn is_valid_link_definition_lookahead(p: &mut MarkdownParser) -> bool {
    // Expect [
    if !p.at(L_BRACK) {
        return false;
    }
    p.bump_any();

    // Parse label: consume tokens until ] or invalid state
    // Also collect the label text for normalization check.
    let mut label_len = 0;
    let mut label_text = String::new();
    loop {
        if p.at(EOF) {
            return false;
        }
        if p.at(NEWLINE) && p.at_blank_line() {
            return false; // Blank line ends link definition
        }
        if p.at(R_BRACK) {
            break;
        }
        if p.at(L_BRACK) {
            return false; // Unescaped [ inside label not allowed
        }

        let text = p.cur_text();
        label_text.push_str(text);

        // Check for escape sequences
        if text.starts_with('\\') && text.len() > 1 {
            label_len += 1; // Count escaped char
        } else {
            label_len += text.chars().count();
        }

        if label_len > MAX_LABEL_LENGTH {
            return false;
        }
        p.bump_any();
    }

    // Label must be non-empty
    if label_len == 0 {
        return false;
    }

    // Label must also be non-empty after normalization (e.g., `[\n ]` normalizes to empty)
    let normalized = normalize_reference_label(&label_text);
    if normalized.is_empty() {
        return false;
    }

    // Expect ]
    if !p.at(R_BRACK) {
        return false;
    }
    p.bump_any();

    // Expect : immediately (no whitespace allowed per CommonMark)
    if !p.at(COLON) {
        return false;
    }
    p.bump_any();

    // Re-lex the current token in LinkDefinition context so whitespace is tokenized.
    p.re_lex_link_definition();

    // Skip optional whitespace after colon (before destination or newline)
    skip_whitespace_tokens(p);

    // Per CommonMark §4.7, destination can be on the next line if there's a
    // single non-blank newline after the colon.
    if p.at(NEWLINE) {
        if p.at_blank_line() {
            return false; // Blank line = no destination
        }
        // Single newline - allow destination on next line
        p.bump_link_definition();
        skip_whitespace_tokens(p);
    }

    // Destination is required (can be on same line or next line now)
    if p.at(EOF) || p.at_blank_line() {
        return false;
    }

    // Skip destination and track whether there was whitespace after it
    let dest_result = skip_destination_tokens(p);
    if dest_result == DestinationResult::Invalid {
        return false;
    }
    let had_separator = dest_result == DestinationResult::ValidWithSeparator;

    // Check what follows destination
    if p.at(EOF) {
        return true; // Valid: destination only, EOF
    }

    if p.at(NEWLINE) {
        // Check for title on next line (newline counts as separator)
        p.bump_link_definition();
        skip_whitespace_tokens(p);

        if at_title_start(p) {
            // If title looks valid, it's included in the definition.
            // If title has trailing content, it's invalid - but the definition
            // is still valid (destination-only). The invalid title line will
            // be parsed as a paragraph. Per CommonMark §4.7.
            let _ = skip_title_tokens(p); // Ignore result - definition is valid either way
        }
        // Destination-only is valid, or destination+valid_title is valid
        return true;
    }

    // Check for optional title on same line - MUST be preceded by whitespace
    if at_title_start(p) {
        if !had_separator {
            // Title without preceding whitespace is invalid (e.g., `<bar>(baz)`)
            return false;
        }
        return skip_title_tokens(p);
    }

    // Non-whitespace, non-title after destination = invalid trailing text
    false
}

/// Skip whitespace tokens (spaces/tabs) in lookahead.
fn skip_whitespace_tokens(p: &mut MarkdownParser) {
    skip_whitespace_tokens_tracked(p);
}

/// Skip whitespace tokens (spaces/tabs) in lookahead and return whether any were skipped.
fn skip_whitespace_tokens_tracked(p: &mut MarkdownParser) -> bool {
    let mut skipped = false;
    while !p.at(EOF) && !p.at(NEWLINE) {
        let text = p.cur_text();
        if text.chars().all(|c| c == ' ' || c == '\t') && !text.is_empty() {
            p.bump_link_definition();
            skipped = true;
        } else {
            break;
        }
    }
    skipped
}

/// Check if at a title start token.
fn at_title_start(p: &MarkdownParser) -> bool {
    let text = p.cur_text();
    text.starts_with('"') || text.starts_with('\'') || p.at(L_PAREN)
}

/// Result of skipping destination tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DestinationResult {
    /// Invalid destination
    Invalid,
    /// Valid destination, no trailing whitespace found before title
    ValidNoSeparator,
    /// Valid destination with trailing whitespace (separator before potential title)
    ValidWithSeparator,
}

/// Skip destination tokens in lookahead. Returns the destination result.
fn skip_destination_tokens(p: &mut MarkdownParser) -> DestinationResult {
    // Skip optional leading whitespace before destination
    while !p.at(EOF) && !p.at(NEWLINE) {
        let text = p.cur_text();
        if text.chars().all(|c| c == ' ' || c == '\t') && !text.is_empty() {
            p.bump_link_definition();
        } else {
            break;
        }
    }

    if p.at(L_ANGLE) {
        // Angle-bracketed destination
        p.bump_link_definition();
        let mut pending_escape = false;
        loop {
            if p.at(EOF) || p.at(NEWLINE) {
                return DestinationResult::Invalid; // Unterminated angle bracket
            }
            if p.at(R_ANGLE) {
                if pending_escape {
                    if !validate_link_destination_text(
                        p.cur_text(),
                        LinkDestinationKind::Enclosed,
                        &mut pending_escape,
                    ) {
                        return DestinationResult::Invalid;
                    }
                    p.bump_link_definition();
                    continue;
                } else {
                    p.bump_link_definition();
                    // Check for trailing whitespace (separator)
                    let had_sep = skip_whitespace_tokens_tracked(p);
                    return if had_sep {
                        DestinationResult::ValidWithSeparator
                    } else {
                        DestinationResult::ValidNoSeparator
                    };
                }
            }
            if !validate_link_destination_text(
                p.cur_text(),
                LinkDestinationKind::Enclosed,
                &mut pending_escape,
            ) {
                return DestinationResult::Invalid;
            }
            p.bump_link_definition();
        }
    } else {
        // Bare destination with balanced parentheses
        let mut paren_depth = 0i32;
        let mut has_content = false;
        let mut saw_separator = false;
        let mut pending_escape = false;

        while !p.at(EOF) && !p.at(NEWLINE) {
            let text = p.cur_text();
            // Stop at whitespace
            if text.chars().all(|c| c == ' ' || c == '\t') && !text.is_empty() {
                if has_content {
                    saw_separator = true;
                }
                p.bump_link_definition();
                continue;
            }

            if at_title_start(p) && has_content && saw_separator {
                // Break here - we've found separator before title
                break;
            }

            if !validate_link_destination_text(text, LinkDestinationKind::Raw, &mut pending_escape)
            {
                return DestinationResult::Invalid;
            }

            match try_update_paren_depth(text, paren_depth, MAX_LINK_DESTINATION_PAREN_DEPTH) {
                ParenDepthResult::Ok(next_depth) => {
                    has_content = true;
                    saw_separator = false;
                    paren_depth = next_depth;
                    p.bump_link_definition();
                }
                ParenDepthResult::DepthExceeded | ParenDepthResult::UnmatchedClose => {
                    // For link reference definitions, both cases end the destination
                    break;
                }
            }
        }
        if !has_content {
            DestinationResult::Invalid
        } else if saw_separator {
            DestinationResult::ValidWithSeparator
        } else {
            DestinationResult::ValidNoSeparator
        }
    }
}

/// Skip title tokens in lookahead. Returns true if valid (ends at EOL/EOF).
fn skip_title_tokens(p: &mut MarkdownParser) -> bool {
    let close_char = if p.cur_text().starts_with('"') {
        '"'
    } else if p.cur_text().starts_with('\'') {
        '\''
    } else if p.at(L_PAREN) {
        ')'
    } else {
        return false;
    };

    // Check if first token is complete (e.g., `"title"`)
    let first_text = p.cur_text();
    if first_text.len() >= 2 && ends_with_unescaped_close(first_text, close_char) {
        p.bump_link_definition();
        skip_whitespace_tokens(p);
        return p.at(EOF) || p.at(NEWLINE);
    }

    p.bump_link_definition();

    // Multi-token title: find closing delimiter
    loop {
        if p.at(EOF) {
            return false; // Unterminated title
        }

        // Check for closing delimiter
        let is_close = ends_with_unescaped_close(p.cur_text(), close_char);

        if is_close {
            p.bump_link_definition();
            skip_whitespace_tokens(p);
            return p.at(EOF) || p.at(NEWLINE);
        }

        // Titles can span lines, but blank line ends them
        if p.at(NEWLINE) && p.at_blank_line() {
            return false;
        }

        p.bump_link_definition();
    }
}

/// Parse a link reference definition.
///
/// Grammar: `MdLinkReferenceDefinition = '[' label: MdLinkLabel ']' ':' destination: MdLinkDestination title: MdLinkTitle?`
///
/// Returns `Absent` if the current position is not a valid link reference definition.
pub(crate) fn parse_link_block(p: &mut MarkdownParser) -> ParsedSyntax {
    if !at_link_block(p) {
        return Absent;
    }

    let m = p.start();

    p.skip_line_indent(MAX_BLOCK_PREFIX_INDENT);

    // [ - opening bracket
    p.expect(L_BRACK);

    // Label - parse until ]
    parse_link_label(p);

    // ] - closing bracket
    p.expect(R_BRACK);

    // : - separator
    p.expect(COLON);

    // Re-lex the current token in LinkDefinition context so whitespace produces
    // separate tokens, allowing proper destination/title parsing.
    p.re_lex_link_definition();

    // Destination (required) - in LinkDefinition context, whitespace is separate
    parse_link_destination(p);

    // Optional title - can be on same line or next line per CommonMark §4.7
    // First, check for title on same line (at_link_title skips whitespace in lookahead)
    if at_link_title(p) {
        parse_link_title(p);
    } else {
        // Check for title on next line - need to skip trailing whitespace first
        // Also validate that the title is complete and has no trailing content
        let has_valid_title_after_newline = p.lookahead(|p| {
            while is_whitespace_token(p) {
                p.bump_link_definition();
            }
            if p.at(NEWLINE) && !p.at_blank_line() {
                // Check if there's a title starter on next line
                if !title_on_next_line(p) {
                    return false;
                }
                // Also validate that the title is complete (no trailing content)
                p.bump_link_definition(); // consume newline
                skip_whitespace_tokens(p); // skip leading whitespace on title line
                skip_title_tokens(p) // returns true only if title ends at EOL/EOF
            } else {
                false
            }
        });

        if has_valid_title_after_newline {
            // Title is on the next line per CommonMark §4.7
            // Include trailing whitespace + newline + leading whitespace as part of title
            parse_link_title_with_trailing_ws(p);
        }
    }

    Present(m.complete(p, MD_LINK_REFERENCE_DEFINITION))
}

/// Parse the label part of a link reference definition.
///
/// Grammar: MdLinkLabel = content: MdInlineItemList
///
/// The label is everything between `[` and `]`, excluding the brackets themselves.
fn parse_link_label(p: &mut MarkdownParser) {
    let m = p.start();
    let list = p.start();

    while !p.at(R_BRACK) && !p.at(EOF) {
        if p.at(NEWLINE) && p.at_blank_line() {
            break;
        }
        bump_textual(p);
    }

    list.complete(p, MD_INLINE_ITEM_LIST);
    m.complete(p, MD_LINK_LABEL);
}

/// Parse the destination part of a link reference definition.
///
/// Grammar: MdLinkDestination = content: MdInlineItemList
///
/// Destination can be:
/// - Angle-bracketed: `<url with spaces>`
/// - Bare URL: `url-without-spaces` (balanced parentheses allowed per CommonMark)
///
/// Uses LinkDefinition lex context so whitespace produces separate tokens.
fn parse_link_destination(p: &mut MarkdownParser) {
    let m = p.start();
    let list = p.start();

    // Include optional whitespace before destination in the destination node.
    while is_whitespace_token(p) {
        bump_textual_link_def(p);
    }

    // Per CommonMark §4.7, destination can be on the next line
    if p.at(NEWLINE) && !p.at_blank_line() {
        bump_textual_link_def(p);
        while is_whitespace_token(p) {
            bump_textual_link_def(p);
        }
    }

    if p.at(L_ANGLE) {
        // Angle-bracketed: consume < ... >
        bump_textual_link_def(p);
        while !p.at(R_ANGLE) && !p.at(EOF) && !p.at(NEWLINE) {
            bump_textual_link_def(p);
        }
        if p.at(R_ANGLE) {
            bump_textual_link_def(p);
        }
    } else {
        // Bare URL with balanced parentheses
        let mut paren_depth: i32 = 0;

        while !p.at(EOF) && !p.at(NEWLINE) {
            if is_whitespace_token(p) {
                break; // Bare destination stops at first whitespace
            }

            let text = p.cur_text();
            match try_update_paren_depth(text, paren_depth, MAX_LINK_DESTINATION_PAREN_DEPTH) {
                ParenDepthResult::Ok(next_depth) => {
                    paren_depth = next_depth;
                    bump_textual_link_def(p);
                }
                ParenDepthResult::DepthExceeded | ParenDepthResult::UnmatchedClose => {
                    break;
                }
            }
        }
    }

    list.complete(p, MD_INLINE_ITEM_LIST);
    m.complete(p, MD_LINK_DESTINATION);
}

/// Consume the current token as MdTextual using LinkDefinition context.
/// This ensures whitespace produces separate tokens for destination/title parsing.
fn bump_textual_link_def(p: &mut MarkdownParser) {
    let item = p.start();
    p.bump_remap_with_context(MD_TEXTUAL_LITERAL, MarkdownLexContext::LinkDefinition);
    item.complete(p, MD_TEXTUAL);
}

/// Check if we're at the start of a link title.
///
/// Title starts with `"`, `'`, or `(` but may be preceded by whitespace.
/// Uses lookahead to skip whitespace and check for title delimiter.
fn at_link_title(p: &mut MarkdownParser) -> bool {
    p.lookahead(|p| {
        // Skip whitespace before title
        while is_whitespace_token(p) {
            p.bump_link_definition();
        }
        let text = p.cur_text();
        text.starts_with('"') || text.starts_with('\'') || p.at(L_PAREN)
    })
}

/// Check if there's a title on the next line (when at NEWLINE).
///
/// Per CommonMark §4.7, title can appear on the line following destination.
/// This looks ahead past the newline and whitespace to check for title starter.
fn title_on_next_line(p: &MarkdownParser) -> bool {
    if !p.at(NEWLINE) {
        return false;
    }

    let source = p.source_after_current();
    let newline_len = p.cur_text().len();
    if source.len() <= newline_len {
        return false;
    }

    let after_newline = &source[newline_len..];
    let trimmed = after_newline.trim_start_matches([' ', '\t']);

    // Check for title starter
    trimmed.starts_with('"') || trimmed.starts_with('\'') || trimmed.starts_with('(')
}
/// Parse a link title that appears on next line, including trailing whitespace before newline.
///
/// This is used when there's trailing whitespace after the destination but before
/// the newline that precedes the title. The trailing whitespace is included in the
/// title node to maintain the grammar structure.
fn parse_link_title_with_trailing_ws(p: &mut MarkdownParser) {
    let m = p.start();
    let list = p.start();

    // Include trailing whitespace after destination
    while is_whitespace_token(p) {
        bump_textual_link_def(p);
    }

    // Include the newline
    if p.at(NEWLINE) {
        bump_textual_link_def(p);
    }

    // Include leading whitespace on title line
    while is_whitespace_token(p) {
        bump_textual_link_def(p);
    }

    // Force re-lex in Regular context so title content doesn't split at whitespace
    p.force_relex_regular();
    // Parse the actual title content
    parse_title_content(p, get_title_close_char(p));

    list.complete(p, MD_INLINE_ITEM_LIST);
    m.complete(p, MD_LINK_TITLE);
}

/// Parse the optional title part of a link reference definition.
///
/// Grammar: MdLinkTitle = content: MdInlineItemList
fn parse_link_title(p: &mut MarkdownParser) {
    let m = p.start();
    let list = p.start();

    // Include optional filler whitespace before title
    while is_whitespace_token(p) {
        bump_textual_link_def(p);
    }

    // Force re-lex in Regular context so title content doesn't split at whitespace
    p.force_relex_regular();
    parse_title_content(p, get_title_close_char(p));

    list.complete(p, MD_INLINE_ITEM_LIST);
    m.complete(p, MD_LINK_TITLE);
}

/// Get the closing character for a title based on current token.
/// Returns None if not at a title start.
fn get_title_close_char(p: &MarkdownParser) -> Option<char> {
    let text = p.cur_text();
    if text.starts_with('"') {
        Some('"')
    } else if text.starts_with('\'') {
        Some('\'')
    } else if p.at(L_PAREN) {
        Some(')')
    } else {
        None
    }
}

/// Parse title content until closing delimiter, including trailing whitespace.
///
/// Inside title quotes, we use Regular context so whitespace doesn't split tokens.
/// Trailing whitespace after the title is also consumed to prevent spurious paragraphs.
fn parse_title_content(p: &mut MarkdownParser, close_char: Option<char>) {
    let Some(close_char) = close_char else {
        return;
    };

    // Check if first token is complete title (e.g., `"title"`)
    let text = p.cur_text();
    let is_complete = text.len() >= 2
        && ((close_char == ')' && text.ends_with(')'))
            || (close_char != ')' && text.ends_with(close_char)));

    // Bump the opening quote/first token using LinkDefinition context (for whitespace handling before)
    bump_textual_link_def(p);

    if is_complete {
        // Consume trailing whitespace after title (before newline)
        p.re_lex_link_definition();
        while is_whitespace_token(p) {
            bump_textual_link_def(p);
        }
        return;
    }

    // Multi-token title: consume until closing delimiter
    // Use Regular context inside title so whitespace doesn't split tokens
    loop {
        if p.at(EOF) {
            break;
        }

        // Check for closing delimiter (must be unescaped)
        let is_close = if close_char == ')' {
            p.at(R_PAREN)
        } else {
            ends_with_unescaped_close(p.cur_text(), close_char)
        };
        if is_close {
            // Use Regular context for title content
            bump_textual(p);
            // Consume trailing whitespace after title (before newline)
            p.re_lex_link_definition();
            while is_whitespace_token(p) {
                bump_textual_link_def(p);
            }
            break;
        }

        // Stop at blank line
        if p.at_blank_line() {
            break;
        }

        // Use Regular context for title content so whitespace doesn't split
        bump_textual(p);
    }
}

/// Check if current token is whitespace (space or tab).
fn is_whitespace_token(p: &MarkdownParser) -> bool {
    let text = p.cur_text();
    !text.is_empty() && text.chars().all(|c| c == ' ' || c == '\t')
}

/// Consume the current token as an MdTextual node.
///
/// This is a helper to reduce boilerplate for the common pattern:
/// `let item = p.start(); p.bump_remap(MD_TEXTUAL_LITERAL); item.complete(p, MD_TEXTUAL);`
fn bump_textual(p: &mut MarkdownParser) {
    let item = p.start();
    p.bump_remap(MD_TEXTUAL_LITERAL);
    item.complete(p, MD_TEXTUAL);
}
