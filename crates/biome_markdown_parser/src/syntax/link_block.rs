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
        if p.line_start_leading_indent() > 3 {
            return false;
        }

        p.skip_line_indent(3);

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
    let mut label_len = 0;
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

    // Destination is required
    if p.at(EOF) || p.at(NEWLINE) {
        return false;
    }

    // Skip destination
    if !skip_destination_tokens(p) {
        return false;
    }

    // Skip optional whitespace after destination (lookahead only)
    skip_whitespace_tokens(p);

    // Check what follows destination
    if p.at(EOF) {
        return true; // Valid: destination only, EOF
    }

    if p.at(NEWLINE) {
        // Check for title on next line
        p.bump_link_definition();
        skip_whitespace_tokens(p);

        if at_title_start(p) {
            return skip_title_tokens(p);
        }
        // No title on next line - destination-only is valid
        return true;
    }

    // Check for optional title on same line
    if at_title_start(p) {
        return skip_title_tokens(p);
    }

    // Non-whitespace, non-title after destination = invalid trailing text
    false
}

/// Skip whitespace tokens (spaces/tabs) in lookahead.
fn skip_whitespace_tokens(p: &mut MarkdownParser) {
    while !p.at(EOF) && !p.at(NEWLINE) {
        let text = p.cur_text();
        if text.chars().all(|c| c == ' ' || c == '\t') && !text.is_empty() {
            p.bump_link_definition();
        } else {
            break;
        }
    }
}

/// Check if at a title start token.
fn at_title_start(p: &MarkdownParser) -> bool {
    let text = p.cur_text();
    text.starts_with('"') || text.starts_with('\'') || p.at(L_PAREN)
}

/// Skip destination tokens in lookahead. Returns false if destination is invalid.
fn skip_destination_tokens(p: &mut MarkdownParser) -> bool {
    if p.at(L_ANGLE) {
        // Angle-bracketed destination
        p.bump_link_definition();
        loop {
            if p.at(EOF) || p.at(NEWLINE) {
                return false; // Unterminated angle bracket
            }
            if p.at(R_ANGLE) {
                p.bump_link_definition();
                // Consume separator whitespace into destination
                skip_whitespace_tokens(p);
                return true;
            }
            p.bump_link_definition();
        }
    } else {
        // Bare destination with balanced parentheses
        let mut paren_depth = 0i32;
        let mut has_content = false;
        let mut saw_separator = false;

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
                break;
            }

            if p.at(L_PAREN) {
                paren_depth += 1;
            } else if p.at(R_PAREN) {
                if paren_depth > 0 {
                    paren_depth -= 1;
                } else {
                    break; // Unbalanced ) ends destination
                }
            }

            has_content = true;
            saw_separator = false;
            p.bump_link_definition();
        }
        has_content
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
    if first_text.len() >= 2 {
        let is_complete = if close_char == ')' {
            first_text.ends_with(')')
        } else {
            first_text.ends_with(close_char)
        };
        if is_complete {
            p.bump_link_definition();
            skip_whitespace_tokens(p);
            return p.at(EOF) || p.at(NEWLINE);
        }
    }

    p.bump_link_definition();

    // Multi-token title: find closing delimiter
    loop {
        if p.at(EOF) {
            return false; // Unterminated title
        }

        // Check for closing delimiter
        let is_close = if close_char == ')' {
            p.at(R_PAREN)
        } else {
            p.cur_text().ends_with(close_char)
        };

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

    p.skip_line_indent(3);

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
    if at_link_title(p) {
        parse_link_title(p);
    } else if p.at(NEWLINE) && title_on_next_line(p) {
        // Title is on the next line per CommonMark §4.7
        // We parse the newline and whitespace as part of the title
        parse_link_title_after_newline(p);
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

            if p.at(L_PAREN) {
                paren_depth += 1;
            } else if p.at(R_PAREN) {
                if paren_depth > 0 {
                    paren_depth -= 1;
                } else {
                    break; // Unbalanced ) ends bare destination
                }
            }

            bump_textual_link_def(p);
        }
    }

    list.complete(p, MD_INLINE_ITEM_LIST);
    m.complete(p, MD_LINK_DESTINATION);
}

/// Consume the current token as MdTextual using LinkDefinition context.
/// This ensures whitespace produces separate tokens for destination/title parsing.
fn bump_textual_link_def(p: &mut MarkdownParser) {
    use crate::lexer::MarkdownLexContext;

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

/// Parse a link title that appears on the next line after a newline.
///
/// Per CommonMark §4.7, titles can appear on the line following the destination.
fn parse_link_title_after_newline(p: &mut MarkdownParser) {
    let m = p.start();
    let list = p.start();

    // Include the newline as textual content
    bump_textual_link_def(p);

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

/// Parse title content until closing delimiter.
///
/// Inside title quotes, we use Regular context so whitespace doesn't split tokens.
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
        return;
    }

    // Multi-token title: consume until closing delimiter
    // Use Regular context inside title so whitespace doesn't split tokens
    loop {
        if p.at(EOF) {
            break;
        }

        // Check for closing delimiter
        let is_close = if close_char == ')' {
            p.at(R_PAREN)
        } else {
            p.cur_text().ends_with(close_char)
        };
        if is_close {
            // Use Regular context for title content
            bump_textual(p);
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
