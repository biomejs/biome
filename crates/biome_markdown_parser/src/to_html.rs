//! HTML renderer for the Biome Markdown CST.
//!
//! This module provides a CST-to-HTML renderer for validating CommonMark spec
//! compliance. It handles the architectural mismatch between Biome's lossless
//! CST (which preserves tabs) and CommonMark's HTML output requirements.
//!
//! ## Purpose
//!
//! This is a **test harness module** designed for validating the markdown parser
//! against the CommonMark specification. It is not intended for production HTML
//! rendering. For production use cases, consider:
//!
//! - Using a dedicated markdown-to-HTML library
//! - Implementing a streaming/zero-copy renderer
//!
//! ## Key Design Decisions
//!
//! 1. **Tab Expansion**: The CST preserves raw `\t` characters for losslessness.
//!    This renderer expands tabs only for structural indentation purposes (per
//!    CommonMark §2.2), preserving literal tabs in code block content.
//!
//! 2. **Entity Decoding**: Uses the [`htmlize`] crate for WHATWG-compliant
//!    HTML5 entity decoding, supporting all 2000+ named entities.
//!
//! 3. **Code Block Content**: The CST may include structural newlines; we skip
//!    the leading newline after a fence marker.
//!
//! 4. **Line Endings**: Handles both LF (`\n`) and CRLF (`\r\n`) line endings
//!    via the [`split_lines`] helper.
//!
//! ## Performance Notes
//!
//! This implementation prioritizes correctness over performance. Each rendering
//! pass may allocate multiple intermediate strings. For production rendering,
//! consider a single-buffer approach using `fmt::Write` or direct string building.

use biome_markdown_syntax::{
    AnyCodeBlock, AnyContainerBlock, AnyLeafBlock, AnyMdBlock, AnyMdInline, MdAutolink, MdBullet,
    MdBulletListItem, MdDocument, MdEntityReference, MdFencedCodeBlock, MdHeader, MdHtmlBlock,
    MdIndentCodeBlock, MdInlineCode, MdInlineHtml, MdInlineImage, MdInlineLink,
    MdLinkReferenceDefinition, MdLinkTitle, MdOrderedListItem, MdParagraph, MdQuote,
    MdReferenceImage, MdReferenceLink, MdSetextHeader, MdTextual,
};
use biome_rowan::{AstNode, AstNodeList, Direction, TextRange};
use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};
use std::collections::HashMap;

use crate::link_reference::normalize_reference_label;
use crate::parser::ListTightness;

// ============================================================================
// Line Handling Utilities
// ============================================================================

/// Split text into lines, handling both LF (\n) and CRLF (\r\n) line endings.
///
/// Unlike `str::lines()`, this preserves the information needed for proper
/// line-by-line processing in code blocks. Returns an iterator over line content
/// (without the line ending).
fn split_lines(text: &str) -> impl Iterator<Item = &str> {
    text.split('\n')
        .map(|line| line.strip_suffix('\r').unwrap_or(line))
}

fn map_lines<F>(text: &str, mut f: F) -> String
where
    F: FnMut(&str, &mut String),
{
    let mut result = String::new();
    for (i, line) in split_lines(text).enumerate() {
        if i > 0 {
            result.push('\n');
        }
        f(line, &mut result);
    }
    result
}

// ============================================================================
// Tab Expansion
// ============================================================================

/// Expand tabs to spaces based on 4-space tab stops.
///
/// CommonMark spec §2.2: "Tabs are expanded to spaces with a tab stop of 4 characters."
/// The column position determines how many spaces a tab expands to.
#[cfg(test)]
fn expand_tabs(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut column = 0;

    for c in text.chars() {
        match c {
            '\t' => {
                // Expand to next 4-space tab stop
                let spaces = 4 - (column % 4);
                for _ in 0..spaces {
                    result.push(' ');
                }
                column += spaces;
            }
            '\n' => {
                result.push('\n');
                column = 0; // Reset column at newline
            }
            _ => {
                result.push(c);
                column += 1;
            }
        }
    }

    result
}

/// Strip structural indentation from code block content while preserving literal tabs.
///
/// CommonMark §2.2: Tabs are expanded for structural purposes (determining indentation)
/// but preserved literally in content. This function:
/// 1. Calculates column position treating tabs as 4-space stops
/// 2. Strips the first `strip_cols` columns of indentation
/// 3. Preserves literal tabs in the remaining content
fn strip_indent_preserve_tabs(text: &str, strip_cols: usize) -> String {
    map_lines(text, |line, result| {
        let mut col = 0;
        let mut char_idx = 0;

        // Find where to start copying (after stripping strip_cols columns)
        for (idx, c) in line.char_indices() {
            if col >= strip_cols {
                char_idx = idx;
                break;
            }
            match c {
                '\t' => {
                    let next_col = col + (4 - (col % 4));
                    if next_col > strip_cols {
                        // Tab crosses the strip boundary - add remaining spaces
                        let spaces = next_col - strip_cols;
                        for _ in 0..spaces {
                            result.push(' ');
                        }
                        char_idx = idx + 1;
                        break;
                    }
                    col = next_col;
                }
                ' ' => col += 1,
                _ => {
                    // Non-whitespace before strip_cols - keep from here
                    char_idx = idx;
                    break;
                }
            }
            char_idx = idx + c.len_utf8();
        }

        // Append the rest of the line (preserving literal tabs)
        if char_idx < line.len() {
            result.push_str(&line[char_idx..]);
        }
    })
}

fn strip_quote_prefixes(text: &str, quote_indent: usize) -> String {
    map_lines(text, |line, result| {
        let mut remaining = line;
        loop {
            let mut idx = 0usize;
            let mut col = 0usize;

            // Scan up to 3 spaces/tabs for the optional quote indent.
            while idx < remaining.len() {
                let c = remaining.as_bytes()[idx];
                match c {
                    b' ' => {
                        col += 1;
                        idx += 1;
                    }
                    b'\t' => {
                        col += 4 - (col % 4);
                        idx += 1;
                    }
                    _ => break,
                }

                if col > 3 {
                    idx = 0;
                    break;
                }
            }

            if idx > 0 || col == 0 {
                if idx < remaining.len() {
                    if remaining[idx..].starts_with("&gt;") {
                        idx += 4;
                        if idx < remaining.len()
                            && matches!(remaining.as_bytes()[idx], b' ' | b'\t')
                        {
                            idx += 1;
                        }
                        remaining = &remaining[idx..];
                        continue;
                    }
                    if remaining.as_bytes()[idx] == b'>' {
                        idx += 1;
                        if idx < remaining.len()
                            && matches!(remaining.as_bytes()[idx], b' ' | b'\t')
                        {
                            idx += 1;
                        }
                        remaining = &remaining[idx..];
                        continue;
                    }
                }

                if quote_indent > 1 && idx > 0 && remaining.len() > 1 {
                    remaining = &remaining[1..];
                    continue;
                }
            }

            break;
        }

        result.push_str(remaining);
    })
}

// ============================================================================
// Context and Main Entry Point
// ============================================================================

/// Context for HTML rendering, containing link reference definitions
/// and list tightness information.
pub struct HtmlRenderContext {
    /// Link reference definitions: label -> (url, title)
    link_definitions: HashMap<String, (String, Option<String>)>,
    /// List tightness by text range
    list_tightness: HashMap<TextRange, bool>,
    /// List item indentation details by text range
    list_item_indents: HashMap<TextRange, crate::parser::ListItemIndent>,
    /// Quote marker indents by text range
    quote_indents: HashMap<TextRange, crate::parser::QuoteIndent>,
}

impl HtmlRenderContext {
    /// Create a new rendering context from parsed document data.
    pub fn new(
        document: &MdDocument,
        list_tightness: &[ListTightness],
        list_item_indents: &[crate::parser::ListItemIndent],
        quote_indents: &[crate::parser::QuoteIndent],
    ) -> Self {
        let link_definitions = collect_link_definitions(document);
        let list_tightness_map = list_tightness
            .iter()
            .map(|lt| (lt.range, lt.is_tight))
            .collect();
        let list_item_indent_map = list_item_indents
            .iter()
            .map(|item| (item.range, item.clone()))
            .collect();
        let quote_indent_map = quote_indents
            .iter()
            .map(|item| (item.range, item.clone()))
            .collect();

        Self {
            link_definitions,
            list_tightness: list_tightness_map,
            list_item_indents: list_item_indent_map,
            quote_indents: quote_indent_map,
        }
    }

    /// Look up a link reference definition by normalized label.
    pub fn get_link_definition(&self, label: &str) -> Option<&(String, Option<String>)> {
        let normalized = normalize_reference_label(label);
        self.link_definitions.get(&normalized)
    }

    /// Check if a list at the given range is tight.
    pub fn is_list_tight(&self, range: TextRange) -> bool {
        self.list_tightness.get(&range).copied().unwrap_or(false)
    }

    pub fn list_item_indent(&self, range: TextRange) -> Option<&crate::parser::ListItemIndent> {
        self.list_item_indents.get(&range)
    }

    pub fn quote_indent(&self, range: TextRange) -> usize {
        self.quote_indents.get(&range).map_or(0, |item| item.indent)
    }
}

/// Render a markdown document to HTML.
pub fn document_to_html(
    document: &MdDocument,
    list_tightness: &[ListTightness],
    list_item_indents: &[crate::parser::ListItemIndent],
    quote_indents: &[crate::parser::QuoteIndent],
) -> String {
    let ctx = HtmlRenderContext::new(document, list_tightness, list_item_indents, quote_indents);
    let mut html = String::new();

    for block in document.value() {
        render_block(&block, &ctx, &mut html, false, 0, 0);
    }

    html
}

// ============================================================================
// Link Reference Collection
// ============================================================================

/// Collect link reference definitions from the document.
fn collect_link_definitions(document: &MdDocument) -> HashMap<String, (String, Option<String>)> {
    let mut definitions = HashMap::new();

    for node in document.syntax().descendants() {
        if let Some(def) = MdLinkReferenceDefinition::cast(node)
            && let (Ok(label), Ok(dest)) = (def.label(), def.destination())
        {
            let label_text = collect_inline_text(&label.content());
            let normalized = normalize_reference_label(&label_text);
            if normalized.is_empty() {
                continue;
            }

            // Only keep first definition (per CommonMark spec)
            if definitions.contains_key(&normalized) {
                continue;
            }

            let url = collect_inline_text(&dest.content());
            let url = process_link_destination(&url);

            let title = def.title().map(|t| {
                let text = collect_inline_text(&t.content());
                process_link_title(&text)
            });

            definitions.insert(normalized, (url, title));
        }
    }

    definitions
}

// ============================================================================
// Block Rendering
// ============================================================================

/// Render a block element to HTML.
fn render_block(
    block: &AnyMdBlock,
    ctx: &HtmlRenderContext,
    out: &mut String,
    in_tight_list: bool,
    list_indent: usize,
    quote_indent: usize,
) {
    match block {
        AnyMdBlock::AnyLeafBlock(leaf) => {
            render_leaf_block(leaf, ctx, out, in_tight_list, list_indent, quote_indent);
        }
        AnyMdBlock::AnyContainerBlock(container) => {
            render_container_block(container, ctx, out, list_indent, quote_indent);
        }
    }
}

/// Render a leaf block to HTML.
fn render_leaf_block(
    block: &AnyLeafBlock,
    ctx: &HtmlRenderContext,
    out: &mut String,
    in_tight_list: bool,
    list_indent: usize,
    quote_indent: usize,
) {
    match block {
        AnyLeafBlock::MdParagraph(para) => {
            render_paragraph(para, ctx, out, in_tight_list, quote_indent);
        }
        AnyLeafBlock::MdHeader(header) => {
            render_atx_header(header, ctx, out);
        }
        AnyLeafBlock::MdSetextHeader(header) => {
            render_setext_header(header, ctx, out);
        }
        AnyLeafBlock::AnyCodeBlock(code) => {
            render_code_block(code, out, list_indent, quote_indent);
        }
        AnyLeafBlock::MdThematicBreakBlock(_) => {
            out.push_str("<hr />\n");
        }
        AnyLeafBlock::MdHtmlBlock(html) => {
            render_html_block(html, out, list_indent, quote_indent);
        }
        AnyLeafBlock::MdLinkReferenceDefinition(_) => {
            // Link reference definitions don't produce output
        }
        AnyLeafBlock::MdLinkBlock(_) => {
            // MdLinkBlock is an internal structure, skip it
        }
        AnyLeafBlock::MdNewline(_) => {
            // Blank lines don't produce output
        }
    }
}

/// Render a container block to HTML.
fn render_container_block(
    block: &AnyContainerBlock,
    ctx: &HtmlRenderContext,
    out: &mut String,
    list_indent: usize,
    quote_indent: usize,
) {
    match block {
        AnyContainerBlock::MdQuote(quote) => {
            render_blockquote(quote, ctx, out, list_indent, quote_indent);
        }
        AnyContainerBlock::MdBulletListItem(list) => {
            render_bullet_list(list, ctx, out, quote_indent);
        }
        AnyContainerBlock::MdOrderedListItem(list) => {
            render_ordered_list(list, ctx, out, quote_indent);
        }
    }
}

/// Render a paragraph.
fn render_paragraph(
    para: &MdParagraph,
    ctx: &HtmlRenderContext,
    out: &mut String,
    in_tight_list: bool,
    quote_indent: usize,
) {
    let mut content = render_inline_list(&para.list(), ctx);
    if quote_indent > 0 {
        content = strip_quote_prefixes(&content, quote_indent);
    }
    // Trim both ends - leading whitespace can appear from parser including
    // the space after list markers in the paragraph content
    let content = strip_paragraph_indent(
        content
            .trim_matches(|c| c == ' ' || c == '\n' || c == '\r')
    );

    if in_tight_list {
        // In tight lists, paragraphs are rendered without <p> tags
        out.push_str(&content);
        out.push('\n');
    } else {
        out.push_str("<p>");
        out.push_str(&content);
        out.push_str("</p>\n");
    }
}

fn strip_paragraph_indent(content: &str) -> String {
    map_lines(content, |line, out| {
        let mut stripped = 0usize;
        let mut at_line_start = true;
        for ch in line.chars() {
            if at_line_start {
                if ch == ' ' && stripped < 4 {
                    stripped += 1;
                    continue;
                }
                at_line_start = false;
            }
            out.push(ch);
        }
    })
}

/// Render an ATX header (# style).
fn render_atx_header(header: &MdHeader, ctx: &HtmlRenderContext, out: &mut String) {
    // Count total hash characters in the before list.
    // The lexer emits all consecutive `#` chars as a single HASH token,
    // so we sum the text lengths of all hash tokens.
    // Use text_trimmed() to exclude any leading trivia (skipped indentation spaces).
    let level = header
        .before()
        .iter()
        .filter_map(|h| h.hash_token().ok())
        .map(|tok| tok.text_trimmed().len())
        .sum::<usize>()
        .clamp(1, 6);

    out.push_str("<h");
    out.push_str(&level.to_string());
    out.push('>');

    if let Some(content) = header.content() {
        let text = render_inline_list(&content.list(), ctx);
        out.push_str(text.trim());
    }

    out.push_str("</h");
    out.push_str(&level.to_string());
    out.push_str(">\n");
}

/// Render a setext header (underline style).
fn render_setext_header(header: &MdSetextHeader, ctx: &HtmlRenderContext, out: &mut String) {
    let level = if let Ok(underline) = header.underline_token() {
        let text = underline.text();
        if text.trim_start().starts_with('=') {
            1
        } else {
            2
        }
    } else {
        1
    };

    out.push_str("<h");
    out.push_str(&level.to_string());
    out.push('>');

    let text = render_inline_list(&header.content(), ctx);
    out.push_str(text.trim());

    out.push_str("</h");
    out.push_str(&level.to_string());
    out.push_str(">\n");
}

// ============================================================================
// Code Block Rendering
// ============================================================================

/// Render a code block (fenced or indented).
fn render_code_block(
    code: &AnyCodeBlock,
    out: &mut String,
    list_indent: usize,
    quote_indent: usize,
) {
    match code {
        AnyCodeBlock::MdFencedCodeBlock(fenced) => {
            render_fenced_code_block(fenced, out, list_indent, quote_indent);
        }
        AnyCodeBlock::MdIndentCodeBlock(indented) => {
            render_indented_code_block(indented, out, list_indent, quote_indent);
        }
    }
}

/// Render a fenced code block.
///
/// Handles the architectural issue where the CST content may include the
/// newline immediately after the opening fence. We skip this leading newline.
/// Also strips the fence's indentation from each content line per CommonMark.
fn render_fenced_code_block(
    code: &MdFencedCodeBlock,
    out: &mut String,
    list_indent: usize,
    quote_indent: usize,
) {
    out.push_str("<pre><code");

    // Determine the fence indentation by looking at leading trivia
    let fence_leading_indent = code.l_fence().ok().map_or(0, |fence| {
        // Count leading spaces/tabs in the trivia before the fence
        let trivia = fence.leading_trivia();
        let mut indent = 0usize;
        for piece in trivia.pieces() {
            let text = piece.text();
            for c in text.chars() {
                match c {
                    ' ' => indent += 1,
                    '\t' => indent += 4 - (indent % 4),
                    '\n' | '\r' => indent = 0, // Reset at newlines
                    _ => {}
                }
            }
        }
        indent
    });
    let container_indent = list_indent + quote_indent;
    let fence_indent = fence_leading_indent.saturating_sub(container_indent).min(3);
    let content_indent = container_indent + fence_indent;

    // Get info string (language) - process escapes
    let info_string: String = code
        .code_list()
        .iter()
        .filter_map(|item| {
            item.syntax()
                .descendants_with_tokens(Direction::Next)
                .filter_map(|el| el.into_token())
                .map(|tok| tok.text().to_string())
                .next()
        })
        .collect::<String>();
    let info_string = info_string.trim();

    // Extract just the language part (before first space) and process escapes
    let language = info_string.split_whitespace().next().unwrap_or("");
    let language = process_escapes(language);
    let language = htmlize::unescape(&language);

    if !language.is_empty() {
        out.push_str(" class=\"language-");
        out.push_str(&escape_html_attribute(language.as_ref()));
        out.push('"');
    }

    out.push('>');

    // Get raw content and handle leading newline
    let mut content = collect_raw_inline_text(&code.content());

    // Skip leading newline if present (it's part of the fence structure, not content)
    if content.starts_with('\n') {
        content = content[1..].to_string();
    }

    // Strip container + fence indentation from content lines
    if content_indent > 0 {
        content = strip_indent_preserve_tabs(&content, content_indent);
    }

    // Escape HTML but preserve the content structure
    out.push_str(&escape_html(&content));

    out.push_str("</code></pre>\n");
}

/// Render an indented code block.
///
/// Indented code blocks require stripping 4 spaces of indentation after
/// tab expansion.
fn render_indented_code_block(
    code: &MdIndentCodeBlock,
    out: &mut String,
    list_indent: usize,
    quote_indent: usize,
) {
    out.push_str("<pre><code>");

    let mut content = collect_raw_inline_text(&code.content());
    // Drop a leading newline from list marker-only lines.
    if content.starts_with('\n') {
        content = content[1..].to_string();
    }
    // Strip 4 columns of structural indent but preserve literal tabs in content
    let content = strip_indent_preserve_tabs(&content, 4 + list_indent + quote_indent);
    out.push_str(&escape_html(&content));

    out.push_str("</code></pre>\n");
}

/// Render an HTML block.
fn render_html_block(
    html: &MdHtmlBlock,
    out: &mut String,
    list_indent: usize,
    quote_indent: usize,
) {
    let mut content = collect_raw_inline_text(&html.content());
    if list_indent > 0 {
        content = strip_indent_preserve_tabs(&content, list_indent);
    }
    if quote_indent > 0 {
        content = strip_quote_prefixes(&content, quote_indent);
    }
    out.push_str(&content);
    if !content.ends_with('\n') {
        out.push('\n');
    }
}

// ============================================================================
// Container Block Rendering
// ============================================================================

/// Render a blockquote.
fn render_blockquote(
    quote: &MdQuote,
    ctx: &HtmlRenderContext,
    out: &mut String,
    list_indent: usize,
    quote_indent: usize,
) {
    out.push_str("<blockquote>\n");

    let content = quote.content();
    let marker_indent = ctx.quote_indent(quote.syntax().text_trimmed_range());
    for block in content.iter() {
        render_block(
            &block,
            ctx,
            out,
            false,
            list_indent,
            quote_indent + marker_indent,
        );
    }

    out.push_str("</blockquote>\n");
}

/// Render a bullet (unordered) list.
fn render_bullet_list(
    list: &MdBulletListItem,
    ctx: &HtmlRenderContext,
    out: &mut String,
    quote_indent: usize,
) {
    let range = list.syntax().text_trimmed_range();
    let mut is_tight = ctx.is_list_tight(range);
    let has_blank_lines = list.md_bullet_list().iter().any(|bullet| {
        bullet
            .content()
            .iter()
            .any(|block| matches!(block, AnyMdBlock::AnyLeafBlock(AnyLeafBlock::MdNewline(_))))
    });
    if has_blank_lines {
        is_tight = false;
    }

    out.push_str("<ul>\n");

    for bullet in list.md_bullet_list() {
        render_list_item(&bullet, ctx, out, is_tight, quote_indent);
    }

    out.push_str("</ul>\n");
}

/// Render an ordered list.
fn render_ordered_list(
    list: &MdOrderedListItem,
    ctx: &HtmlRenderContext,
    out: &mut String,
    quote_indent: usize,
) {
    let range = list.syntax().text_trimmed_range();
    let mut is_tight = ctx.is_list_tight(range);
    let has_blank_lines = list.md_bullet_list().iter().any(|bullet| {
        bullet
            .content()
            .iter()
            .any(|block| matches!(block, AnyMdBlock::AnyLeafBlock(AnyLeafBlock::MdNewline(_))))
    });
    if has_blank_lines {
        is_tight = false;
    }

    // Get starting number from first item
    let start = list
        .md_bullet_list()
        .first()
        .and_then(|bullet| bullet.bullet().ok())
        .map_or(1, |marker| {
            let text = marker.text();
            // Extract number from "1." or "1)" format
            text.trim_start()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u32>()
                .unwrap_or(1)
        });

    if start == 1 {
        out.push_str("<ol>\n");
    } else {
        out.push_str("<ol start=\"");
        out.push_str(&start.to_string());
        out.push_str("\">\n");
    }

    for bullet in list.md_bullet_list() {
        render_list_item(&bullet, ctx, out, is_tight, quote_indent);
    }

    out.push_str("</ol>\n");
}

/// Render a list item.
fn render_list_item(
    bullet: &MdBullet,
    ctx: &HtmlRenderContext,
    out: &mut String,
    is_tight: bool,
    quote_indent: usize,
) {
    out.push_str("<li>");

    let list_indent = ctx.list_item_indent(bullet.syntax().text_trimmed_range());
    let blocks: Vec<_> = bullet.content().iter().collect();
    let item_has_blank_line = blocks.iter().enumerate().any(|(index, block)| {
        if !is_newline_block(block) {
            return false;
        }

        // Ignore the marker-line newline when content follows.
        if index == 0 && blocks.iter().skip(1).any(|block| !is_newline_block(block)) {
            return false;
        }

        true
    });
    let is_tight = is_tight && !item_has_blank_line;

    let (indent, first_line_code_indent) = match list_indent {
        Some(entry) => {
            let base = list_item_required_indent(entry);
            let first_line_code =
                (entry.spaces_after_marker > INDENT_CODE_BLOCK_SPACES).then_some(1);
            (base, first_line_code)
        }
        None => (0, None),
    };

    if is_empty_content(&blocks) {
        out.push_str("</li>\n");
        return;
    }

    if is_tight {
        if blocks.len() == 1 && is_paragraph_block(&blocks[0]) {
            // Tight list with single paragraph: no newline after <li>
            if let Some(block) = blocks.first() {
                let block_indent = match (first_line_code_indent, block) {
                    (
                        Some(code_indent),
                        AnyMdBlock::AnyLeafBlock(AnyLeafBlock::AnyCodeBlock(
                            AnyCodeBlock::MdIndentCodeBlock(_),
                        )),
                    ) => code_indent,
                    _ => indent,
                };
                render_block(block, ctx, out, true, block_indent, quote_indent);
            }
            // Remove trailing newline for tight lists
            if out.ends_with('\n') {
                out.pop();
            }
        } else if blocks.first().is_some_and(is_paragraph_block) {
            // Tight list with multiple blocks: render paragraph inline with <li>
            if let Some(first) = blocks.first() {
                let block_indent = match (first_line_code_indent, first) {
                    (
                        Some(code_indent),
                        AnyMdBlock::AnyLeafBlock(AnyLeafBlock::AnyCodeBlock(
                            AnyCodeBlock::MdIndentCodeBlock(_),
                        )),
                    ) => code_indent,
                    _ => indent,
                };
                render_block(first, ctx, out, true, block_indent, quote_indent);
            }
            for block in blocks.iter().skip(1) {
                render_block(block, ctx, out, true, indent, quote_indent);
            }
        } else {
            out.push('\n');
            for (idx, block) in blocks.iter().enumerate() {
                let block_indent = if idx == 0 {
                    match (first_line_code_indent, block) {
                        (
                            Some(code_indent),
                            AnyMdBlock::AnyLeafBlock(AnyLeafBlock::AnyCodeBlock(
                                AnyCodeBlock::MdIndentCodeBlock(_),
                            )),
                        ) => code_indent,
                        _ => indent,
                    }
                } else {
                    indent
                };
                render_block(block, ctx, out, true, block_indent, quote_indent);
            }
        }
    } else {
        // Loose list or multiple blocks
        out.push('\n');
        for (idx, block) in blocks.iter().enumerate() {
            let block_indent = if idx == 0 {
                match (first_line_code_indent, block) {
                    (
                        Some(code_indent),
                        AnyMdBlock::AnyLeafBlock(AnyLeafBlock::AnyCodeBlock(
                            AnyCodeBlock::MdIndentCodeBlock(_),
                        )),
                    ) => code_indent,
                    _ => indent,
                }
            } else {
                indent
            };
            render_block(block, ctx, out, false, block_indent, quote_indent);
        }
    }

    out.push_str("</li>\n");
}

// ============================================================================
// Inline Rendering
// ============================================================================

/// Render an inline item list to HTML string.
fn render_inline_list(
    list: &biome_markdown_syntax::MdInlineItemList,
    ctx: &HtmlRenderContext,
) -> String {
    let mut result = String::new();
    let items: Vec<_> = list.iter().collect();
    let len = items.len();

    for (i, item) in items.iter().enumerate() {
        let is_last = i == len - 1;

        // Special handling for hard line breaks at end of block
        if is_last && let AnyMdInline::MdHardLine(hard) = item {
            // Per CommonMark: hard line break at end of block is ignored
            // But if it was a backslash, output the backslash
            if let Ok(token) = hard.value_token()
                && token.text().starts_with('\\')
            {
                result.push('\\');
            }
            // Otherwise (trailing spaces), output nothing
            continue;
        }

        render_inline(item, ctx, &mut result);
    }
    result
}

/// Render an inline element.
fn render_inline(inline: &AnyMdInline, ctx: &HtmlRenderContext, out: &mut String) {
    match inline {
        AnyMdInline::MdTextual(text) => {
            render_textual(text, out);
        }
        AnyMdInline::MdInlineEmphasis(em) => {
            out.push_str("<strong>");
            out.push_str(&render_inline_list(&em.content(), ctx));
            out.push_str("</strong>");
        }
        AnyMdInline::MdInlineItalic(italic) => {
            out.push_str("<em>");
            out.push_str(&render_inline_list(&italic.content(), ctx));
            out.push_str("</em>");
        }
        AnyMdInline::MdInlineCode(code) => {
            render_inline_code(code, out);
        }
        AnyMdInline::MdInlineLink(link) => {
            render_inline_link(link, ctx, out);
        }
        AnyMdInline::MdInlineImage(img) => {
            render_inline_image(img, ctx, out);
        }
        AnyMdInline::MdReferenceLink(link) => {
            render_reference_link(link, ctx, out);
        }
        AnyMdInline::MdReferenceImage(img) => {
            render_reference_image(img, ctx, out);
        }
        AnyMdInline::MdAutolink(autolink) => {
            render_autolink(autolink, out);
        }
        AnyMdInline::MdInlineHtml(html) => {
            render_inline_html(html, out);
        }
        AnyMdInline::MdHtmlBlock(html) => {
            // Inline HTML block (rare case)
            let content = collect_raw_inline_text(&html.content());
            out.push_str(&content);
        }
        AnyMdInline::MdHardLine(_) => {
            out.push_str("<br />\n");
        }
        AnyMdInline::MdSoftBreak(_) => {
            out.push('\n');
        }
        AnyMdInline::MdEntityReference(entity) => {
            render_entity_reference(entity, out);
        }
    }
}

/// Render textual content.
fn render_textual(text: &MdTextual, out: &mut String) {
    if let Ok(token) = text.value_token() {
        let raw = token.text();
        // Process backslash escapes and escape HTML
        let processed = process_escapes(raw);
        out.push_str(&escape_html(&processed));
    }
}

/// Render inline code.
fn render_inline_code(code: &MdInlineCode, out: &mut String) {
    out.push_str("<code>");

    let content = collect_raw_inline_text(&code.content());
    // Code spans: normalize line endings to spaces
    let content = content.replace('\n', " ");
    // Code spans: strip one leading/trailing space if content has both
    // and the content isn't all spaces
    let content = if content.starts_with(' ')
        && content.ends_with(' ')
        && content.len() > 2
        && content.chars().any(|c| c != ' ')
    {
        content[1..content.len() - 1].to_string()
    } else {
        content
    };

    out.push_str(&escape_html(&content));
    out.push_str("</code>");
}

/// Render an inline link.
fn render_inline_link(link: &MdInlineLink, ctx: &HtmlRenderContext, out: &mut String) {
    let text = render_inline_list(&link.text(), ctx);
    let dest = collect_inline_text(&link.destination());
    let dest = process_link_destination(&dest);

    out.push_str("<a href=\"");
    out.push_str(&escape_html_attribute(&dest));
    out.push('"');

    if let Some(title) = link.title() {
        render_link_title(&title, out);
    }

    out.push('>');
    out.push_str(&text);
    out.push_str("</a>");
}

/// Render an inline image.
fn render_inline_image(img: &MdInlineImage, ctx: &HtmlRenderContext, out: &mut String) {
    let alt = render_inline_list(&img.alt(), ctx);
    // Strip HTML tags from alt text
    let alt = strip_html_tags(&alt);

    let dest = collect_inline_text(&img.destination());
    let dest = process_link_destination(&dest);

    out.push_str("<img src=\"");
    out.push_str(&escape_html_attribute(&dest));
    out.push_str("\" alt=\"");
    out.push_str(&escape_html_attribute(&alt));
    out.push('"');

    if let Some(title) = img.title() {
        render_link_title(&title, out);
    }

    out.push_str(" />");
}

/// Render a reference link.
fn render_reference_link(link: &MdReferenceLink, ctx: &HtmlRenderContext, out: &mut String) {
    let text = render_inline_list(&link.text(), ctx);
    let text_raw = collect_inline_text(&link.text());

    render_reference_common(
        link.label(),
        text_raw.clone(),
        |label_node| collect_inline_text(&label_node.label()),
        ctx,
        out,
        |url, title, out| {
            out.push_str("<a href=\"");
            out.push_str(&escape_html_attribute(url));
            out.push('"');

            if let Some(t) = title {
                out.push_str(" title=\"");
                out.push_str(&escape_html_attribute(t));
                out.push('"');
            }

            out.push('>');
            out.push_str(&text);
            out.push_str("</a>");
        },
        |label_display, out| {
            // No definition found - output as literal text
            out.push('[');
            out.push_str(&text);
            out.push(']');
            if let Some(label) = label_display {
                out.push('[');
                out.push_str(&escape_html(&label));
                out.push(']');
            }
        },
    );
}

/// Render a reference image.
fn render_reference_image(img: &MdReferenceImage, ctx: &HtmlRenderContext, out: &mut String) {
    let alt = render_inline_list(&img.alt(), ctx);
    let alt = strip_html_tags(&alt);
    let alt_raw = collect_inline_text(&img.alt());

    render_reference_common(
        img.label(),
        alt_raw.clone(),
        |label_node| collect_inline_text(&label_node.label()),
        ctx,
        out,
        |url, title, out| {
            out.push_str("<img src=\"");
            out.push_str(&escape_html_attribute(url));
            out.push_str("\" alt=\"");
            out.push_str(&escape_html_attribute(&alt));
            out.push('"');

            if let Some(t) = title {
                out.push_str(" title=\"");
                out.push_str(&escape_html_attribute(t));
                out.push('"');
            }

            out.push_str(" />");
        },
        |label_display, out| {
            // No definition found - output as literal text
            out.push_str("![");
            out.push_str(&alt);
            out.push(']');
            if let Some(label) = label_display {
                out.push('[');
                out.push_str(&escape_html(&label));
                out.push(']');
            }
        },
    );
}

fn resolve_reference_label<L, F>(
    label_node: Option<L>,
    fallback: String,
    label_text: F,
) -> (String, Option<String>)
where
    F: FnOnce(&L) -> String,
{
    if let Some(node) = label_node {
        let text = label_text(&node);
        (text.clone(), Some(text))
    } else {
        (fallback, None)
    }
}

fn render_reference_common<L, FLabel, FFound, FMissing>(
    label_node: Option<L>,
    fallback: String,
    label_text: FLabel,
    ctx: &HtmlRenderContext,
    out: &mut String,
    on_found: FFound,
    on_missing: FMissing,
) where
    FLabel: FnOnce(&L) -> String,
    FFound: FnOnce(&str, Option<&str>, &mut String),
    FMissing: FnOnce(Option<String>, &mut String),
{
    // Get label (if explicit) or use the fallback
    let (label, label_display) = resolve_reference_label(label_node, fallback, label_text);

    if let Some((url, title)) = ctx.get_link_definition(&label) {
        on_found(url, title.as_deref(), out);
    } else {
        on_missing(label_display, out);
    }
}

/// Render an autolink.
fn render_autolink(autolink: &MdAutolink, out: &mut String) {
    let content = collect_raw_inline_text(&autolink.value());

    // Check if it's an email autolink
    let is_email = content.contains('@') && !content.contains(':');

    let href = if is_email {
        format!("mailto:{}", content)
    } else {
        process_link_destination(&content)
    };

    out.push_str("<a href=\"");
    out.push_str(&escape_html_attribute(&href));
    out.push_str("\">");
    out.push_str(&escape_html(&content));
    out.push_str("</a>");
}

/// Render inline HTML.
fn render_inline_html(html: &MdInlineHtml, out: &mut String) {
    let content = collect_raw_inline_text(&html.value());
    out.push_str(&content);
}

/// Render an entity reference.
fn render_entity_reference(entity: &MdEntityReference, out: &mut String) {
    if let Ok(token) = entity.value_token() {
        let text = token.text();
        // Decode known entities or pass through
        if let Some(decoded) = decode_entity(text) {
            out.push_str(&escape_html(&decoded));
        } else {
            // Unknown entity - pass through as-is (escaped)
            out.push_str(&escape_html(text));
        }
    }
}

/// Render a link title attribute.
fn render_link_title(title: &MdLinkTitle, out: &mut String) {
    let text = collect_inline_text(&title.content());
    let text = process_link_title(&text);

    out.push_str(" title=\"");
    out.push_str(&escape_html_attribute(&text));
    out.push('"');
}

// ============================================================================
// Text Collection Helpers
// ============================================================================

/// Collect all text from an inline list (for processing).
fn collect_inline_text(list: &biome_markdown_syntax::MdInlineItemList) -> String {
    let mut text = String::new();
    for token in list
        .syntax()
        .descendants_with_tokens(Direction::Next)
        .filter_map(|element| element.into_token())
    {
        text.push_str(token.text());
    }
    text
}

/// Collect raw inline text without processing escapes.
fn collect_raw_inline_text(list: &biome_markdown_syntax::MdInlineItemList) -> String {
    let mut text = String::new();
    for item in list.iter() {
        collect_raw_inline_item(&item, &mut text);
    }
    text
}

/// Collect raw text from a single inline item.
fn collect_raw_inline_item(item: &AnyMdInline, out: &mut String) {
    match item {
        AnyMdInline::MdTextual(text) => {
            if let Ok(token) = text.value_token() {
                out.push_str(token.text());
            }
        }
        AnyMdInline::MdSoftBreak(_) => {
            out.push('\n');
        }
        AnyMdInline::MdHardLine(_) => {
            out.push('\n');
        }
        _ => {
            // For other inline elements, collect their tokens
            for token in item
                .syntax()
                .descendants_with_tokens(Direction::Next)
                .filter_map(|element| element.into_token())
            {
                out.push_str(token.text());
            }
        }
    }
}

// ============================================================================
// Text Processing
// ============================================================================

/// Process backslash escapes in text.
fn process_escapes(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\'
            && let Some(&next) = chars.peek()
            && is_escapable(next)
        {
            result.push(next);
            chars.next();
            continue;
        }
        result.push(c);
    }

    result
}

/// Check if a character can be escaped in markdown.
fn is_escapable(c: char) -> bool {
    matches!(
        c,
        '!' | '"'
            | '#'
            | '$'
            | '%'
            | '&'
            | '\''
            | '('
            | ')'
            | '*'
            | '+'
            | ','
            | '-'
            | '.'
            | '/'
            | ':'
            | ';'
            | '<'
            | '='
            | '>'
            | '?'
            | '@'
            | '['
            | '\\'
            | ']'
            | '^'
            | '_'
            | '`'
            | '{'
            | '|'
            | '}'
            | '~'
    )
}

/// Process a link destination (remove angle brackets, decode escapes).
fn process_link_destination(dest: &str) -> String {
    let dest = dest.trim();

    // Remove angle brackets if present
    let dest = if dest.starts_with('<') && dest.ends_with('>') {
        &dest[1..dest.len() - 1]
    } else {
        dest
    };

    // Process escapes
    let dest = process_escapes(dest);
    let decoded = htmlize::unescape(&dest).into_owned();
    percent_encode_uri(&decoded)
}

/// Process a link title (remove quotes, decode escapes).
fn process_link_title(title: &str) -> String {
    let title = title.trim();

    // Remove surrounding quotes
    let title = if (title.starts_with('"') && title.ends_with('"'))
        || (title.starts_with('\'') && title.ends_with('\''))
        || (title.starts_with('(') && title.ends_with(')'))
    {
        &title[1..title.len() - 1]
    } else {
        title
    };

    // Process escapes
    let title = process_escapes(title);
    htmlize::unescape(&title).into_owned()
}

fn percent_encode_uri(value: &str) -> String {
    let mut result = String::new();
    let mut last = 0;

    for (i, c) in value.char_indices() {
        if c == '%' {
            let bytes = value.as_bytes();
            if i + 2 < bytes.len()
                && bytes[i + 1].is_ascii_alphanumeric()
                && bytes[i + 2].is_ascii_alphanumeric()
            {
                if last < i {
                    result.push_str(
                        &utf8_percent_encode(&value[last..i], URI_ENCODE_SET).to_string(),
                    );
                }
                result.push_str(&value[i..i + 3]);
                last = i + 3;
            }
        }
    }

    if last < value.len() {
        result.push_str(&utf8_percent_encode(&value[last..], URI_ENCODE_SET).to_string());
    }

    result
}

const URI_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'%')
    .add(b'<')
    .add(b'>')
    .add(b'\\')
    .add(b'[')
    .add(b']')
    .add(b'^')
    .add(b'`')
    .add(b'{')
    .add(b'|')
    .add(b'}');

// ============================================================================
// HTML Escaping
// ============================================================================

/// Escape HTML special characters.
fn escape_html(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '"' => result.push_str("&quot;"),
            _ => result.push(c),
        }
    }
    result
}

/// Escape HTML attribute values.
fn escape_html_attribute(text: &str) -> String {
    escape_html(text)
}

/// Strip HTML tags from text (for image alt text).
fn strip_html_tags(text: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;

    for c in text.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            result.push(c);
        }
    }

    result
}

// ============================================================================
// Entity Decoding
// ============================================================================

/// Decode HTML entities using the WHATWG standard.
///
/// Handles numeric character references (&#123; and &#x7B;) and all standard
/// HTML5 named entities via the `htmlize` crate.
fn decode_entity(entity: &str) -> Option<String> {
    use std::borrow::Cow;

    // Use htmlize for WHATWG-compliant entity decoding
    let decoded = htmlize::unescape(entity);

    match decoded {
        // If the entity was decoded (string changed), return the decoded value
        Cow::Owned(s) if s != entity => Some(s),
        // If unchanged and it looks like an entity, it's invalid
        Cow::Borrowed(_) if entity.starts_with('&') && entity.ends_with(';') => None,
        Cow::Owned(s) if entity.starts_with('&') && entity.ends_with(';') && s == entity => None,
        // Otherwise return as-is
        Cow::Owned(s) => Some(s),
        Cow::Borrowed(s) => Some(s.to_string()),
    }
}
fn is_paragraph_block(block: &AnyMdBlock) -> bool {
    matches!(
        block,
        AnyMdBlock::AnyLeafBlock(AnyLeafBlock::MdParagraph(_))
    )
}

/// Check if a block is a newline (produces no output).
fn is_newline_block(block: &AnyMdBlock) -> bool {
    matches!(block, AnyMdBlock::AnyLeafBlock(AnyLeafBlock::MdNewline(_)))
}

/// Check if blocks are effectively empty (empty or only newlines).
fn is_empty_content(blocks: &[AnyMdBlock]) -> bool {
    blocks.is_empty() || blocks.iter().all(is_newline_block)
}

const INDENT_CODE_BLOCK_SPACES: usize = 4;

fn list_item_required_indent(entry: &crate::parser::ListItemIndent) -> usize {
    if entry.spaces_after_marker > INDENT_CODE_BLOCK_SPACES {
        entry.marker_indent + entry.marker_width + 1
    } else {
        entry.marker_indent + entry.marker_width + entry.spaces_after_marker.max(1)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_markdown;

    #[test]
    fn test_tab_expansion() {
        assert_eq!(expand_tabs("\tfoo"), "    foo");
        assert_eq!(expand_tabs("a\tb"), "a   b");
        assert_eq!(expand_tabs("ab\tc"), "ab  c");
        assert_eq!(expand_tabs("abc\td"), "abc d");
        assert_eq!(expand_tabs("abcd\te"), "abcd    e");
        assert_eq!(expand_tabs("\t\tfoo"), "        foo");
    }

    #[test]
    fn test_tab_expansion_multiline() {
        assert_eq!(expand_tabs("a\tb\nc\td"), "a   b\nc   d");
    }

    #[test]
    fn test_simple_paragraph() {
        let parsed = parse_markdown("Hello, world!\n");
        let html = document_to_html(
            &parsed.tree(),
            parsed.list_tightness(),
            parsed.list_item_indents(),
            parsed.quote_indents(),
        );
        assert_eq!(html, "<p>Hello, world!</p>\n");
    }

    #[test]
    fn test_atx_header() {
        let parsed = parse_markdown("# Hello\n");
        let html = document_to_html(
            &parsed.tree(),
            parsed.list_tightness(),
            parsed.list_item_indents(),
            parsed.quote_indents(),
        );
        assert_eq!(html, "<h1>Hello</h1>\n");
    }

    #[test]
    fn test_emphasis() {
        let parsed = parse_markdown("*italic* and **bold**\n");
        let html = document_to_html(
            &parsed.tree(),
            parsed.list_tightness(),
            parsed.list_item_indents(),
            parsed.quote_indents(),
        );
        assert_eq!(html, "<p><em>italic</em> and <strong>bold</strong></p>\n");
    }

    #[test]
    fn test_escape_html() {
        assert_eq!(escape_html("a & b < c > d"), "a &amp; b &lt; c &gt; d");
    }

    #[test]
    fn test_decode_entity() {
        assert_eq!(decode_entity("&amp;"), Some("&".to_string()));
        assert_eq!(decode_entity("&#65;"), Some("A".to_string()));
        assert_eq!(decode_entity("&#x41;"), Some("A".to_string()));
        assert_eq!(decode_entity("&nbsp;"), Some("\u{00A0}".to_string()));
        // U+0000 should become replacement character
        assert_eq!(decode_entity("&#0;"), Some("\u{FFFD}".to_string()));
    }
}
