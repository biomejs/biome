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
use crate::syntax::quote::{consume_quote_prefix_without_virtual, has_quote_prefix};

/// Check if we're at an HTML block (line start, up to 3 spaces indent, then `<`).
pub(crate) fn at_html_block(p: &mut MarkdownParser) -> bool {
    p.lookahead(|p| {
        if !p.at_line_start() && !p.at_start_of_input() {
            return false;
        }

        if p.line_start_leading_indent() > 3 {
            return false;
        }

        p.skip_line_indent(3);

        p.at(L_ANGLE) && is_html_like_content(p)
    })
}

/// Check if content after `<` looks like HTML (tag, comment, declaration, etc.).
fn is_html_like_content(p: &MarkdownParser) -> bool {
    let remaining = p.source_after_current();
    if !remaining.starts_with('<') {
        return false;
    }

    let after_angle = &remaining[1..];

    // Comment, CDATA, declaration, or processing instruction
    if after_angle.starts_with("!--")
        || after_angle.starts_with("![CDATA[")
        || after_angle.starts_with('?')
    {
        return true;
    }

    // Declaration: <!LETTER
    if let Some(rest) = after_angle.strip_prefix('!')
        && rest.chars().next().is_some_and(|c| c.is_ascii_uppercase())
    {
        return true;
    }

    // Tag: <tagname or </tagname
    let tag_start = after_angle.strip_prefix('/').unwrap_or(after_angle);
    tag_start
        .chars()
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic())
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
    "pre",
    "script",
    "section",
    "source",
    "style",
    "summary",
    "table",
    "tbody",
    "td",
    "template",
    "textarea",
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
        if !at_html_block(p) {
            return false;
        }

        let remaining = p.source_after_current();
        if remaining.len() < 2 {
            return false;
        }

        let after_angle = &remaining[1..];

        // Special constructs always interrupt
        if after_angle.starts_with("!--")
            || after_angle.starts_with("![CDATA[")
            || after_angle.starts_with('!')
            || after_angle.starts_with('?')
        {
            return true;
        }

        // Check for block-level tag
        let tag_start = after_angle.strip_prefix('/').unwrap_or(after_angle);
        let tag_name: String = tag_start
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric())
            .collect();

        BLOCK_TAGS.iter().any(|t| t.eq_ignore_ascii_case(&tag_name))
    })
}

/// Parse HTML block as raw text until blank line.
pub(crate) fn parse_html_block(p: &mut MarkdownParser) -> ParsedSyntax {
    p.skip_line_indent(3);

    if !p.at(L_ANGLE) {
        return Absent;
    }

    let m = p.start();
    let content_m = p.start();

    parse_until_blank_line(p);

    content_m.complete(p, MD_INLINE_ITEM_LIST);
    Present(m.complete(p, MD_HTML_BLOCK))
}

fn parse_until_blank_line(p: &mut MarkdownParser) {
    while !p.at(EOF) {
        if p.at(NEWLINE) && p.at_blank_line() {
            let text_m = p.start();
            p.bump_remap(MD_TEXTUAL_LITERAL);
            text_m.complete(p, MD_TEXTUAL);
            break;
        }

        if at_container_boundary(p) {
            break;
        }

        let text_m = p.start();
        let is_newline = p.at(NEWLINE);
        p.bump_remap(MD_TEXTUAL_LITERAL);
        text_m.complete(p, MD_TEXTUAL);

        if is_newline {
            skip_container_prefixes(p);
        }
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
