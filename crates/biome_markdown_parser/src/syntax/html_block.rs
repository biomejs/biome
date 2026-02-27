//! HTML block parsing for Markdown.
//!
//! HTML content is captured as raw text rather than fully parsed. This keeps
//! the markdown parser simple; full HTML parsing can be added via workspace
//! snippets integration in the future (similar to how `<script>`/`<style>` are
//! handled in the HTML parser).
//!
//! **Note:** Not 100% CommonMark §4.6 compliant. All HTML blocks terminate at
//! blank lines, whereas CommonMark types 1-5 have specific terminators (`-->`, `?>`, etc.).

use biome_markdown_syntax::MarkdownSyntaxKind::*;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax::{self, *};

use crate::MarkdownParser;
use crate::syntax::MAX_BLOCK_PREFIX_INDENT;
use crate::syntax::quote::{consume_quote_prefix_without_virtual, has_quote_prefix};

/// Check if we're at an HTML block (line start, up to 3 spaces indent, then `<`).
pub(crate) fn at_html_block(p: &mut MarkdownParser) -> bool {
    p.lookahead(|p| {
        if !p.at_line_start() && !p.at_start_of_input() {
            return false;
        }

        if p.line_start_leading_indent() > MAX_BLOCK_PREFIX_INDENT {
            return false;
        }

        p.skip_line_indent(MAX_BLOCK_PREFIX_INDENT);

        p.at(L_ANGLE) && is_html_like_content(p)
    })
}

/// Check if content after `<` looks like HTML (tag, comment, declaration, etc.).
fn is_html_like_content(p: &MarkdownParser) -> bool {
    html_block_kind(p).is_some()
}

#[derive(Clone, Copy)]
enum HtmlBlockKind {
    /// CommonMark HTML block type 1: raw `<script>`, `<pre>`, `<style>`, `<textarea>`.
    Type1(Type1Tag),
    /// CommonMark HTML block type 2: HTML comment (`<!-- ... -->`).
    Type2,
    /// CommonMark HTML block type 3: processing instruction (`<? ... ?>`).
    Type3,
    /// CommonMark HTML block type 4: declaration (`<![A-Z] ... >`).
    Type4,
    /// CommonMark HTML block type 5: CDATA section (`<![CDATA[ ... ]]>`).
    Type5,
    /// CommonMark HTML block type 6: block-level tag (e.g. `<div>`).
    Type6,
    /// CommonMark HTML block type 7: any other tag on its own line.
    Type7,
}

#[derive(Clone, Copy)]
enum Type1Tag {
    /// `<script>` raw HTML block.
    Script,
    /// `<pre>` raw HTML block.
    Pre,
    /// `<style>` raw HTML block.
    Style,
    /// `<textarea>` raw HTML block.
    Textarea,
}

/// Determine HTML block type using raw source to match line-based CommonMark rules.
/// Token lookahead is insufficient here because lexer contexts can split or merge
/// tokens across `<...>` boundaries, and we need the exact line text.
fn html_block_kind(p: &MarkdownParser) -> Option<HtmlBlockKind> {
    let remaining = p.source_after_current();
    // Skip whitespace trivia that may precede the '<' token
    let remaining = remaining.trim_start_matches([' ', '\t']);
    if !remaining.starts_with('<') {
        return None;
    }

    let after_angle = &remaining[1..];

    // Comment
    if after_angle.starts_with("!--") {
        return Some(HtmlBlockKind::Type2);
    }

    // Processing instruction
    if after_angle.starts_with('?') {
        return Some(HtmlBlockKind::Type3);
    }

    // CDATA
    if after_angle.starts_with("![CDATA[") {
        return Some(HtmlBlockKind::Type5);
    }

    // Declaration: <!LETTER
    if let Some(rest) = after_angle.strip_prefix('!')
        && rest.chars().next().is_some_and(|c| c.is_ascii_uppercase())
    {
        return Some(HtmlBlockKind::Type4);
    }

    let is_closing = after_angle.starts_with('/');
    let tag_name = html_tag_name(after_angle)?;
    if !is_closing && let Some(tag) = type1_tag(tag_name) {
        return Some(HtmlBlockKind::Type1(tag));
    }

    if BLOCK_TAGS.iter().any(|t| t.eq_ignore_ascii_case(tag_name)) {
        return Some(HtmlBlockKind::Type6);
    }

    let line = first_line(remaining);
    if line_has_only_tag(line) {
        Some(HtmlBlockKind::Type7)
    } else {
        None
    }
}

fn html_tag_name(after_angle: &str) -> Option<&str> {
    let tag_start = after_angle.strip_prefix('/').unwrap_or(after_angle);
    let bytes = tag_start.as_bytes();
    let first = *bytes.first()?;
    if !first.is_ascii_alphabetic() {
        return None;
    }

    let tag_end = bytes
        .iter()
        .position(|b| !b.is_ascii_alphanumeric() && *b != b'-')
        .unwrap_or(tag_start.len());
    let tag_name = &tag_start[..tag_end];

    let boundary = tag_start.as_bytes().get(tag_end).copied();
    if matches!(
        boundary,
        None | Some(b' ' | b'\t' | b'\n' | b'\r' | b'\x0c' | b'>' | b'/')
    ) {
        Some(tag_name)
    } else {
        None
    }
}

fn type1_tag(tag_name: &str) -> Option<Type1Tag> {
    if tag_name.eq_ignore_ascii_case("script") {
        Some(Type1Tag::Script)
    } else if tag_name.eq_ignore_ascii_case("pre") {
        Some(Type1Tag::Pre)
    } else if tag_name.eq_ignore_ascii_case("style") {
        Some(Type1Tag::Style)
    } else if tag_name.eq_ignore_ascii_case("textarea") {
        Some(Type1Tag::Textarea)
    } else {
        None
    }
}

fn first_line(text: &str) -> &str {
    text.split_once(['\n', '\r']).map_or(text, |(line, _)| line)
}

/// Check if a line contains only a valid HTML open or close tag (for type 7 HTML blocks).
/// Uses the same validation as inline HTML (CommonMark §6.8) to ensure proper tag structure.
fn line_has_only_tag(line: &str) -> bool {
    let bytes = line.as_bytes();
    if !bytes.starts_with(b"<") {
        return false;
    }

    // Use inline HTML validator which properly checks tag name, attributes, etc.
    let Some(html_len) = super::inline::is_inline_html(line) else {
        return false;
    };

    // After the tag, only whitespace is allowed
    line[html_len..].chars().all(|c| c == ' ' || c == '\t')
}

/// Block-level tags that can interrupt paragraphs.
const BLOCK_TAGS: &[&str] = &[
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
    "search",
    "section",
    "summary",
    "table",
    "tbody",
    "td",
    "tfoot",
    "th",
    "thead",
    "title",
    "tr",
    "track",
    "ul",
];

/// Only block-level HTML and special constructs interrupt paragraphs.
pub(crate) fn at_html_block_interrupt(p: &mut MarkdownParser) -> bool {
    p.lookahead(|p| {
        if p.line_start_leading_indent() > MAX_BLOCK_PREFIX_INDENT {
            return false;
        }

        let Some(kind) = html_block_kind(p) else {
            return false;
        };

        matches!(
            kind,
            HtmlBlockKind::Type1 { .. }
                | HtmlBlockKind::Type2
                | HtmlBlockKind::Type3
                | HtmlBlockKind::Type4
                | HtmlBlockKind::Type5
                | HtmlBlockKind::Type6
        )
    })
}

/// Parse HTML block as raw text until blank line.
pub(crate) fn parse_html_block(p: &mut MarkdownParser) -> ParsedSyntax {
    p.skip_line_indent(MAX_BLOCK_PREFIX_INDENT);

    if !p.at(L_ANGLE) {
        return Absent;
    }

    let Some(kind) = html_block_kind(p) else {
        return Absent;
    };

    let m = p.start();
    let content_m = p.start();

    match kind {
        HtmlBlockKind::Type1(tag) => {
            let terminator = match tag {
                Type1Tag::Script => "</script>",
                Type1Tag::Pre => "</pre>",
                Type1Tag::Style => "</style>",
                Type1Tag::Textarea => "</textarea>",
            };
            parse_until_terminator(p, terminator, true);
        }
        HtmlBlockKind::Type2 => parse_until_terminator(p, "-->", false),
        HtmlBlockKind::Type3 => parse_until_terminator(p, "?>", false),
        HtmlBlockKind::Type4 => parse_until_terminator(p, ">", false),
        HtmlBlockKind::Type5 => parse_until_terminator(p, "]]>", false),
        HtmlBlockKind::Type6 | HtmlBlockKind::Type7 => parse_until_blank_line(p),
    }

    content_m.complete(p, MD_INLINE_ITEM_LIST);
    Present(m.complete(p, MD_HTML_BLOCK))
}

fn parse_until_blank_line(p: &mut MarkdownParser) {
    while !p.at(EOF) {
        if p.at(NEWLINE) {
            if p.at_blank_line() {
                let text_m = p.start();
                p.bump_remap(MD_TEXTUAL_LITERAL);
                text_m.complete(p, MD_TEXTUAL);
                break;
            }
            // Consume the newline first, then check if the next line exits the container
            let text_m = p.start();
            p.bump_remap(MD_TEXTUAL_LITERAL);
            text_m.complete(p, MD_TEXTUAL);

            if at_container_boundary(p) {
                break;
            }
            skip_container_prefixes(p);
            continue;
        }

        // For non-newline tokens, check container boundary (handles virtual line start)
        if at_container_boundary(p) {
            break;
        }

        let text_m = p.start();
        p.bump_remap(MD_TEXTUAL_LITERAL);
        text_m.complete(p, MD_TEXTUAL);
    }
}

fn parse_until_terminator(p: &mut MarkdownParser, terminator: &str, case_insensitive: bool) {
    let mut line = String::new();

    while !p.at(EOF) {
        let text = p.cur_text();
        let is_newline = p.at(NEWLINE);
        line.push_str(text);

        let text_m = p.start();
        p.bump_remap(MD_TEXTUAL_LITERAL);
        text_m.complete(p, MD_TEXTUAL);

        if is_newline {
            if line_contains(&line, terminator, case_insensitive) {
                break;
            }
            line.clear();
            // Check container boundary after consuming newline
            if at_container_boundary(p) {
                break;
            }
            skip_container_prefixes(p);
        }
    }
}

fn line_contains(line: &str, needle: &str, case_insensitive: bool) -> bool {
    if !case_insensitive {
        return line.contains(needle);
    }

    let hay = line.as_bytes();
    let needle = needle.as_bytes();
    if needle.is_empty() || hay.len() < needle.len() {
        return false;
    }

    for i in 0..=hay.len() - needle.len() {
        if hay[i..i + needle.len()]
            .iter()
            .zip(needle.iter())
            .all(|(a, b)| a.eq_ignore_ascii_case(b))
        {
            return true;
        }
    }

    false
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
        // Skip if at virtual line start — the quote prefix was already consumed
        // by the container parser that set this virtual start position.
        if p.state()
            .virtual_line_start
            .is_none_or(|vls| vls != p.cur_range().start())
        {
            return true;
        }
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
