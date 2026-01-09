//! HTML block parsing for Markdown (CommonMark §4.6).
//!
//! Per CommonMark §4.6, there are 7 types of HTML blocks:
//!
//! 1. `<script`, `<pre`, `<style`, `<textarea` - ends at closing tag
//! 2. `<!--` - ends at `-->`
//! 3. `<?` - ends at `?>`
//! 4. `<!` + uppercase letter - ends at `>`
//! 5. `<![CDATA[` - ends at `]]>`
//! 6. Block-level HTML tags (div, p, table, etc.) - ends at blank line
//! 7. Other tags - ends at blank line
//!
//! HTML blocks start at the beginning of a line (possibly indented by up to 3 spaces).
//!
//! ## Recovery Limits
//!
//! To prevent unbounded consumption for malformed documents, we limit HTML block
//! parsing to a maximum number of tokens. This provides better error recovery
//! when termination markers are missing.

use biome_markdown_syntax::MarkdownSyntaxKind::*;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax::{self, *};

use crate::MarkdownParser;
use crate::syntax::quote::{consume_quote_prefix_without_virtual, has_quote_prefix};

/// Maximum number of tokens to consume before giving up on finding an HTML block terminator.
/// This provides reasonable error recovery for unclosed HTML blocks while still
/// supporting large valid HTML blocks (e.g., embedded SVGs or complex tables).
const MAX_HTML_BLOCK_TOKENS: usize = 10_000;

/// Type 1 tags that end with a specific closing tag (case-insensitive).
const TYPE1_TAGS: &[&str] = &["script", "pre", "style", "textarea"];

/// Type 6 tags that end at a blank line (case-insensitive).
/// These are block-level HTML elements.
const TYPE6_TAGS: &[&str] = &[
    "address",
    "article",
    "aside",
    "base",
    "basefont",
    "blockquote",
    "body",
    "caption",
    "center",
    "col",
    "colgroup",
    "dd",
    "details",
    "dialog",
    "dir",
    "div",
    "dl",
    "dt",
    "fieldset",
    "figcaption",
    "figure",
    "footer",
    "form",
    "frame",
    "frameset",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "head",
    "header",
    "hr",
    "html",
    "iframe",
    "legend",
    "li",
    "link",
    "main",
    "menu",
    "menuitem",
    "nav",
    "noframes",
    "ol",
    "optgroup",
    "option",
    "p",
    "param",
    "section",
    "source",
    "summary",
    "table",
    "tbody",
    "td",
    "template",
    "tfoot",
    "th",
    "thead",
    "title",
    "tr",
    "track",
    "ul",
];

/// The type of HTML block, determining how it ends.
#[derive(Debug, Clone, Copy, PartialEq)]
enum HtmlBlockType {
    /// Type 1: script, pre, style, textarea - ends at closing tag
    Type1(&'static str),
    /// Type 2: HTML comment - ends at `-->`
    Type2,
    /// Type 3: Processing instruction - ends at `?>`
    Type3,
    /// Type 4: Declaration - ends at `>`
    Type4,
    /// Type 5: CDATA - ends at `]]>`
    Type5,
    /// Type 6: Block-level tags - ends at blank line
    Type6,
    /// Type 7: Other tags - ends at blank line
    Type7,
}

/// Check if we're at the start of an HTML block.
///
/// HTML blocks can start at the beginning of a line (up to 3 spaces indentation).
/// We detect by looking for `<` followed by appropriate patterns.
pub(crate) fn at_html_block(p: &mut MarkdownParser) -> bool {
    p.lookahead(|p| {
        if !p.at_line_start() && !p.at_start_of_input() {
            return false;
        }
        // Must be at line start (with at most 3 spaces indentation)
        if p.line_start_leading_indent() > 3 {
            return false;
        }

        p.skip_line_indent(3);

        // Must be at `<`
        if !p.at(L_ANGLE) {
            return false;
        }

        // Look ahead to determine the type
        detect_html_block_type(p).is_some()
    })
}

/// HTML blocks of type 7 do not interrupt paragraphs per CommonMark.
pub(crate) fn at_html_block_interrupt(p: &mut MarkdownParser) -> bool {
    p.lookahead(|p| {
        if !at_html_block(p) {
            return false;
        }
        !matches!(detect_html_block_type(p), Some(HtmlBlockType::Type7))
    })
}

/// Detect what type of HTML block this is.
fn detect_html_block_type(p: &MarkdownParser) -> Option<HtmlBlockType> {
    let remaining = p.source_after_current();
    if !remaining.starts_with('<') {
        return None;
    }

    let line_end = remaining.find(['\n', '\r']).unwrap_or(remaining.len());
    let line = &remaining[..line_end];
    let after_angle = &line[1..];

    // Type 2: HTML comment
    if after_angle.starts_with("!--") {
        return Some(HtmlBlockType::Type2);
    }

    // Type 5: CDATA
    if after_angle.starts_with("![CDATA[") {
        return Some(HtmlBlockType::Type5);
    }

    // Type 4: Declaration (<!LETTER)
    if let Some(c) = after_angle.strip_prefix('!')
        && c.chars().next().is_some_and(|ch| ch.is_ascii_uppercase())
    {
        return Some(HtmlBlockType::Type4);
    }

    // Type 3: Processing instruction
    if after_angle.starts_with('?') {
        return Some(HtmlBlockType::Type3);
    }

    // Check for closing tag prefix
    let is_closing = after_angle.starts_with('/');
    let tag_start = if is_closing {
        &after_angle[1..]
    } else {
        after_angle
    };

    // Extract tag name (letters only, case-insensitive)
    let tag_name: String = tag_start
        .chars()
        .take_while(|c| c.is_ascii_alphabetic() || c.is_ascii_digit())
        .collect();

    if tag_name.is_empty() {
        return None;
    }

    // Check what follows the tag name
    let after_tag = &tag_start[tag_name.len()..];
    let valid_suffix = after_tag.is_empty()
        || after_tag.starts_with(|c: char| c.is_ascii_whitespace())
        || after_tag.starts_with('>')
        || after_tag.starts_with("/>");

    if !valid_suffix {
        return None;
    }

    // Type 1: script, pre, style, textarea (case-insensitive, opening tag only)
    for &type1_tag in TYPE1_TAGS {
        if !is_closing && tag_name.eq_ignore_ascii_case(type1_tag) {
            return Some(HtmlBlockType::Type1(type1_tag));
        }
    }

    // Type 6: Block-level tags (case-insensitive)
    for &type6_tag in TYPE6_TAGS {
        if tag_name.eq_ignore_ascii_case(type6_tag) {
            return Some(HtmlBlockType::Type6);
        }
    }

    // Type 7: Other valid open/close tag
    // Only treat as HTML block if the line contains only the tag.
    if line_has_only_tag(line) {
        return Some(HtmlBlockType::Type7);
    }

    None
}

/// Parse an HTML block.
///
/// Grammar: MdHtmlBlock = content: MdInlineItemList
///
/// The entire HTML block content is stored as raw text within an inline item list.
pub(crate) fn parse_html_block(p: &mut MarkdownParser) -> ParsedSyntax {
    p.skip_line_indent(3);

    let html_block_type = match detect_html_block_type(p) {
        Some(t) => t,
        None => return Absent,
    };

    let m = p.start();
    let content_m = p.start();

    // Parse content based on block type
    match html_block_type {
        HtmlBlockType::Type1(tag) => parse_type1_block(p, tag),
        HtmlBlockType::Type2 => parse_until_sequence(p, "-->"),
        HtmlBlockType::Type3 => parse_until_sequence(p, "?>"),
        HtmlBlockType::Type4 => parse_until_char(p, '>'),
        HtmlBlockType::Type5 => parse_until_sequence(p, "]]>"),
        HtmlBlockType::Type6 | HtmlBlockType::Type7 => parse_until_blank_line(p),
    }

    content_m.complete(p, MD_INLINE_ITEM_LIST);
    Present(m.complete(p, MD_HTML_BLOCK))
}

/// Parse Type 1 HTML block (script, pre, style, textarea) until closing tag.
fn parse_type1_block(p: &mut MarkdownParser, tag: &str) {
    let closing = format!("</{tag}");
    let mut token_count = 0;

    // Consume tokens until we find the closing tag (case-insensitive)
    while !p.at(EOF) && token_count < MAX_HTML_BLOCK_TOKENS {
        if current_line_contains_sequence_case_insensitive(p, &closing) {
            // Consume the line containing the closing tag
            consume_line_with_prefixes(p);
            break;
        }

        token_count += consume_line_with_prefixes(p);
        if at_container_boundary(p) {
            break;
        }
        skip_container_prefixes(p);
    }
}

/// Parse until we find a specific character.
fn parse_until_char(p: &mut MarkdownParser, target: char) {
    let mut token_count = 0;

    while !p.at(EOF) && token_count < MAX_HTML_BLOCK_TOKENS {
        if current_line_contains_char(p, target) {
            // Consume the line containing the target
            consume_line_with_prefixes(p);
            break;
        }

        token_count += consume_line_with_prefixes(p);
        if at_container_boundary(p) {
            break;
        }
        skip_container_prefixes(p);
    }
}

/// Parse until we find a specific sequence.
fn parse_until_sequence(p: &mut MarkdownParser, sequence: &str) {
    let mut token_count = 0;

    while !p.at(EOF) && token_count < MAX_HTML_BLOCK_TOKENS {
        if current_line_contains_sequence(p, sequence) {
            consume_line_with_prefixes(p);
            break;
        }

        token_count += consume_line_with_prefixes(p);
        if at_container_boundary(p) {
            break;
        }
        skip_container_prefixes(p);
    }
}

/// Parse until we hit a blank line.
///
/// # NEWLINE Handling
///
/// NEWLINE is an explicit token. We consume until we find a blank line
/// (NEWLINE followed by optional whitespace then another NEWLINE/EOF).
fn parse_until_blank_line(p: &mut MarkdownParser) {
    let mut token_count = 0;

    while !p.at(EOF) && token_count < MAX_HTML_BLOCK_TOKENS {
        // At NEWLINE, check if it's a blank line
        if p.at(NEWLINE) {
            if p.at_blank_line() {
                // Consume this NEWLINE and stop
                let text_m = p.start();
                p.bump_remap(MD_TEXTUAL_LITERAL);
                text_m.complete(p, MD_TEXTUAL);
                break;
            }

            // Not a blank line - consume NEWLINE and continue
            let text_m = p.start();
            p.bump_remap(MD_TEXTUAL_LITERAL);
            text_m.complete(p, MD_TEXTUAL);
            token_count += 1;
            if at_container_boundary(p) {
                break;
            }
            skip_container_prefixes(p);
            continue;
        }

        // Consume current token
        let text_m = p.start();
        p.bump_remap(MD_TEXTUAL_LITERAL);
        text_m.complete(p, MD_TEXTUAL);
        token_count += 1;
    }
}

/// Consume all tokens on the current line.
/// Limited to prevent runaway consumption on pathological input.
fn consume_line_with_prefixes(p: &mut MarkdownParser) -> usize {
    // Single line shouldn't have many tokens, but add limit for safety
    const MAX_LINE_TOKENS: usize = 1000;
    let mut token_count = 0;

    while !p.at(EOF) && token_count < MAX_LINE_TOKENS {
        let text_m = p.start();
        let current = p.cur();

        // Bump the current token
        p.bump_remap(MD_TEXTUAL_LITERAL);
        text_m.complete(p, MD_TEXTUAL);
        token_count += 1;

        // Stop after consuming a newline or if this was end of file
        if current == NEWLINE {
            break;
        }
    }

    token_count
}

fn current_line_contains_sequence(p: &MarkdownParser, sequence: &str) -> bool {
    let remaining = p.source_after_current();
    let line_end = remaining.find(['\n', '\r']).unwrap_or(remaining.len());
    remaining[..line_end].contains(sequence)
}

fn current_line_contains_sequence_case_insensitive(p: &MarkdownParser, sequence: &str) -> bool {
    let remaining = p.source_after_current();
    let line_end = remaining.find(['\n', '\r']).unwrap_or(remaining.len());
    let line = &remaining[..line_end];
    if sequence.len() > line.len() {
        return false;
    }
    line.as_bytes()
        .windows(sequence.len())
        .any(|w| w.eq_ignore_ascii_case(sequence.as_bytes()))
}

fn current_line_contains_char(p: &MarkdownParser, target: char) -> bool {
    let remaining = p.source_after_current();
    let line_end = remaining.find(['\n', '\r']).unwrap_or(remaining.len());
    remaining[..line_end].contains(target)
}

fn line_has_only_tag(line: &str) -> bool {
    if let Some(pos) = line.find('>') {
        line[pos + 1..].trim().is_empty()
    } else {
        false
    }
}

fn skip_container_prefixes(p: &mut MarkdownParser) {
    let quote_depth = p.state().block_quote_depth;
    if quote_depth > 0 && has_quote_prefix(p, quote_depth) {
        consume_quote_prefix_without_virtual(p, quote_depth);
        p.state_mut().virtual_line_start = Some(p.cur_range().start());
    }

    let required_indent = p.state().list_item_required_indent;
    if required_indent > 0 {
        p.skip_line_indent(required_indent);
        p.state_mut().virtual_line_start = Some(p.cur_range().start());
    }
}

fn at_container_boundary(p: &mut MarkdownParser) -> bool {
    let quote_depth = p.state().block_quote_depth;
    if quote_depth > 0 && p.at_line_start() && !has_quote_prefix(p, quote_depth) {
        return true;
    }

    let required_indent = p.state().list_item_required_indent;
    if required_indent > 0 && p.at_line_start() {
        let indent = p.line_start_leading_indent();
        if indent < required_indent {
            return true;
        }
    }

    false
}
