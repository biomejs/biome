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
//!
//! ## Traversal
//!
//! The renderer uses `biome_rowan`'s `.preorder()` visitor to walk the CST and
//! emit HTML for enter/leave events instead of manual recursive descent. Nodes
//! that need post-processing (like paragraphs, headers, list items, and
//! reference links) are buffered until `Leave` so their final HTML wrapper can
//! be decided with full context.

use biome_markdown_syntax::{
    AnyMdBlock, AnyMdCodeBlock, AnyMdInline, AnyMdLeafBlock, MarkdownLanguage, MdAutolink,
    MdBlockList, MdBullet, MdBulletListItem, MdDocument, MdEntityReference, MdFencedCodeBlock,
    MdHardLine, MdHeader, MdHtmlBlock, MdIndentCodeBlock, MdInlineCode, MdInlineEmphasis,
    MdInlineHtml, MdInlineImage, MdInlineItalic, MdInlineItemList, MdInlineLink, MdLinkBlock,
    MdLinkDestination, MdLinkLabel, MdLinkReferenceDefinition, MdLinkTitle, MdOrderedListItem,
    MdParagraph, MdQuote, MdReferenceImage, MdReferenceLink, MdReferenceLinkLabel, MdSetextHeader,
    MdSoftBreak, MdTextual, MdThematicBreakBlock,
};
use biome_rowan::{AstNode, AstNodeList, Direction, SyntaxNode, TextRange, WalkEvent};
use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};
use std::collections::HashMap;

use crate::parser::{ListItemIndent, ListTightness, QuoteIndent};
use crate::syntax::reference::normalize_reference_label;
use crate::syntax::{INDENT_CODE_BLOCK_SPACES, MAX_BLOCK_PREFIX_INDENT, TAB_STOP_SPACES};

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
                // Expand to next tab stop
                let spaces = TAB_STOP_SPACES - (column % TAB_STOP_SPACES);
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
    strip_indent_preserve_tabs_with_offset(text, strip_cols, 0)
}

fn strip_indent_preserve_tabs_with_offset(
    text: &str,
    strip_cols: usize,
    first_line_column: usize,
) -> String {
    let mut first_line = true;
    map_lines(text, |line, result| {
        let mut col = if first_line { first_line_column } else { 0 };
        first_line = false;
        let mut char_idx = 0;

        // Find where to start copying (after stripping strip_cols columns)
        for (idx, c) in line.char_indices() {
            if col >= strip_cols {
                char_idx = idx;
                break;
            }
            match c {
                '\t' => {
                    let next_col = col + (TAB_STOP_SPACES - (col % TAB_STOP_SPACES));
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
                        col += TAB_STOP_SPACES - (col % TAB_STOP_SPACES);
                        idx += 1;
                    }
                    _ => break,
                }

                if col > MAX_BLOCK_PREFIX_INDENT {
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
    list_item_indents: HashMap<TextRange, ListItemIndent>,
    /// Quote marker indents by text range
    quote_indents: HashMap<TextRange, QuoteIndent>,
}

impl HtmlRenderContext {
    /// Create a new rendering context from parsed document data.
    pub fn new(
        document: &MdDocument,
        list_tightness: &[ListTightness],
        list_item_indents: &[ListItemIndent],
        quote_indents: &[QuoteIndent],
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
        self.link_definitions.get(normalized.as_ref())
    }

    /// Check if a list at the given range is tight.
    pub fn is_list_tight(&self, range: TextRange) -> bool {
        self.list_tightness.get(&range).copied().unwrap_or(false)
    }

    pub fn list_item_indent(&self, range: TextRange) -> Option<&ListItemIndent> {
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
    list_item_indents: &[ListItemIndent],
    quote_indents: &[QuoteIndent],
) -> String {
    let ctx = HtmlRenderContext::new(document, list_tightness, list_item_indents, quote_indents);
    HtmlRenderer::new(&ctx).render(document.syntax())
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
            if definitions.contains_key(normalized.as_ref()) {
                continue;
            }

            let url = collect_inline_text(&dest.content());
            let url = process_link_destination(&url);

            let title = def.title().map(|t| {
                let text = collect_inline_text(&t.content());
                process_link_title(&text)
            });

            definitions.insert(normalized.into_owned(), (url, title));
        }
    }

    definitions
}

// ============================================================================
// Visitor Rendering
// ============================================================================

struct HtmlRenderer<'a> {
    ctx: &'a HtmlRenderContext,
    buffers: Vec<Buffer>,
    list_stack: Vec<ListState>,
    list_item_stack: Vec<ListItemState>,
    quote_indent_stack: Vec<usize>,
    quote_indent: usize,
    depth: usize,
    opaque_depth: Option<usize>,
    skip_children_depth: Option<usize>,
    suppressed_inline_nodes: Vec<Vec<SyntaxNode<MarkdownLanguage>>>,
}

struct Buffer {
    kind: BufferKind,
    content: String,
}

enum BufferKind {
    Root,
    Paragraph(ParagraphState),
    Header(HeaderState),
    SetextHeader(HeaderState),
    ReferenceLink(ReferenceLinkState),
    ListItem,
}

struct ParagraphState {
    in_tight_list: bool,
    quote_indent: usize,
    suppress_wrapping: bool,
}

struct HeaderState {
    level: usize,
}

struct ReferenceLinkState {
    label_display: Option<String>,
    url: Option<String>,
    title: Option<String>,
}

struct ListState {
    is_tight: bool,
}

struct ListItemState {
    is_tight: bool,
    leading_newline: bool,
    trim_trailing_newline: bool,
    is_empty: bool,
    block_indents: HashMap<TextRange, BlockIndent>,
}

#[derive(Clone, Copy, Default)]
struct BlockIndent {
    indent: usize,
    first_line_column: usize,
}

impl<'a> HtmlRenderer<'a> {
    fn new(ctx: &'a HtmlRenderContext) -> Self {
        Self {
            ctx,
            buffers: vec![Buffer {
                kind: BufferKind::Root,
                content: String::new(),
            }],
            list_stack: Vec::new(),
            list_item_stack: Vec::new(),
            quote_indent_stack: Vec::new(),
            quote_indent: 0,
            depth: 0,
            opaque_depth: None,
            skip_children_depth: None,
            suppressed_inline_nodes: Vec::new(),
        }
    }

    fn render(mut self, root: &SyntaxNode<MarkdownLanguage>) -> String {
        for event in root.preorder() {
            match event {
                WalkEvent::Enter(node) => {
                    if self.opaque_depth.is_some() {
                        self.depth += 1;
                        continue;
                    }
                    if let Some(skip) = self.skip_children_depth
                        && self.depth > skip
                    {
                        self.depth += 1;
                        continue;
                    }

                    self.enter(node);
                    self.depth += 1;
                }
                WalkEvent::Leave(node) => {
                    self.depth = self.depth.saturating_sub(1);
                    if let Some(opaque_depth) = self.opaque_depth {
                        if self.depth == opaque_depth {
                            self.opaque_depth = None;
                        }
                        continue;
                    }

                    if let Some(skip) = self.skip_children_depth {
                        if self.depth > skip {
                            continue;
                        }
                        if self.depth == skip {
                            self.leave(node);
                            self.skip_children_depth = None;
                            continue;
                        }
                    }

                    self.leave(node);
                }
            }
        }

        self.buffers
            .pop()
            .map(|buffer| buffer.content)
            .unwrap_or_default()
    }

    fn enter(&mut self, node: SyntaxNode<MarkdownLanguage>) {
        if MdInlineItemList::cast(node.clone()).is_some()
            && self
                .suppressed_inline_nodes
                .iter()
                .flatten()
                .any(|suppressed| *suppressed == node)
        {
            self.opaque_depth = Some(self.depth);
            return;
        }

        if MdParagraph::cast(node.clone()).is_some() {
            let suppress_wrapping = node.parent().and_then(MdHeader::cast).is_some()
                || node.parent().and_then(MdSetextHeader::cast).is_some();
            let is_direct_list_item = node
                .parent()
                .and_then(MdBlockList::cast)
                .and_then(|list| list.syntax().parent())
                .and_then(MdBullet::cast)
                .is_some();
            let in_tight_list = is_direct_list_item
                && self
                    .list_item_stack
                    .last()
                    .is_some_and(|state| state.is_tight);
            let state = ParagraphState {
                in_tight_list,
                quote_indent: self.quote_indent,
                suppress_wrapping,
            };
            self.push_buffer(BufferKind::Paragraph(state));
            return;
        }

        if let Some(header) = MdHeader::cast(node.clone()) {
            let level = header_level(&header);
            self.push_buffer(BufferKind::Header(HeaderState { level }));
            return;
        }

        if let Some(header) = MdSetextHeader::cast(node.clone()) {
            let level = setext_header_level(&header);
            self.push_buffer(BufferKind::SetextHeader(HeaderState { level }));
            return;
        }

        if let Some(quote) = MdQuote::cast(node.clone()) {
            self.push_str("<blockquote>\n");
            let marker_indent = self.ctx.quote_indent(quote.syntax().text_trimmed_range());
            self.quote_indent += marker_indent;
            self.quote_indent_stack.push(marker_indent);
            return;
        }

        if let Some(list) = MdBulletListItem::cast(node.clone()) {
            let is_tight = self.ctx.is_list_tight(list.syntax().text_trimmed_range());
            self.list_stack.push(ListState { is_tight });
            self.push_str("<ul>\n");
            return;
        }

        if let Some(list) = MdOrderedListItem::cast(node.clone()) {
            let is_tight = self.ctx.is_list_tight(list.syntax().text_trimmed_range());
            self.list_stack.push(ListState { is_tight });

            let start = list
                .md_bullet_list()
                .first()
                .and_then(|bullet| bullet.bullet().ok())
                .map_or(1, |marker| {
                    let text = marker.text();
                    text.trim_start()
                        .chars()
                        .take_while(|c| c.is_ascii_digit())
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap_or(1)
                });

            if start == 1 {
                self.push_str("<ol>\n");
            } else {
                self.push_str("<ol start=\"");
                self.push_str(&start.to_string());
                self.push_str("\">\n");
            }
            return;
        }

        if let Some(bullet) = MdBullet::cast(node.clone()) {
            let list_is_tight = self.list_stack.last().is_some_and(|state| state.is_tight);
            let blocks: Vec<_> = bullet.content().iter().collect();
            let item_has_blank_line = blocks
                .windows(2)
                .any(|pair| is_newline_block(&pair[0]) && is_newline_block(&pair[1]));
            let is_tight = list_is_tight && !item_has_blank_line;
            let is_empty = is_empty_content(&blocks);

            let first_is_paragraph = blocks.first().is_some_and(is_paragraph_block);
            let last_is_paragraph = blocks
                .iter()
                .rev()
                .find(|b| !is_newline_block(b))
                .is_some_and(is_paragraph_block);

            let leading_newline = !is_tight || !first_is_paragraph;
            let trim_trailing_newline = is_tight && last_is_paragraph;

            let list_indent = self
                .ctx
                .list_item_indent(bullet.syntax().text_trimmed_range());
            let (indent, first_line_code_indent, first_line_column) = match list_indent {
                Some(entry) => {
                    let base = list_item_required_indent(entry);
                    let first_line_code =
                        (entry.spaces_after_marker > INDENT_CODE_BLOCK_SPACES).then_some(base);
                    let column = entry.marker_indent + entry.marker_width;
                    (base, first_line_code, column)
                }
                None => (0, None, 0),
            };

            let mut block_indents = HashMap::new();
            for (idx, block) in blocks.iter().enumerate() {
                let block_indent = if idx == 0 {
                    match (first_line_code_indent, block) {
                        (
                            Some(code_indent),
                            AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::AnyMdCodeBlock(
                                AnyMdCodeBlock::MdIndentCodeBlock(_),
                            )),
                        ) => code_indent,
                        _ => indent,
                    }
                } else {
                    indent
                };

                let column_for_block = if idx == 0
                    && first_line_code_indent.is_some()
                    && matches!(
                        block,
                        AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::AnyMdCodeBlock(
                            AnyMdCodeBlock::MdIndentCodeBlock(_)
                        ))
                    ) {
                    first_line_column
                } else {
                    0
                };

                block_indents.insert(
                    block.syntax().text_trimmed_range(),
                    BlockIndent {
                        indent: block_indent,
                        first_line_column: column_for_block,
                    },
                );
            }

            self.list_item_stack.push(ListItemState {
                is_tight,
                leading_newline,
                trim_trailing_newline,
                is_empty,
                block_indents,
            });
            self.push_buffer(BufferKind::ListItem);
            if is_empty {
                self.skip_children_depth = Some(self.depth);
            }
            return;
        }

        if let Some(code) = MdFencedCodeBlock::cast(node.clone()) {
            let block_indent = self.block_indent(code.syntax().text_trimmed_range());
            let quote_indent = self.quote_indent;
            render_fenced_code_block(&code, self.out_mut(), block_indent.indent, quote_indent);
            self.opaque_depth = Some(self.depth);
            return;
        }

        if let Some(code) = MdIndentCodeBlock::cast(node.clone()) {
            let block_indent = self.block_indent(code.syntax().text_trimmed_range());
            let quote_indent = self.quote_indent;
            if block_indent.first_line_column > 0 {
                render_indented_code_block_in_list(
                    &code,
                    self.out_mut(),
                    block_indent.indent,
                    quote_indent,
                    block_indent.first_line_column,
                );
            } else {
                render_indented_code_block(
                    &code,
                    self.out_mut(),
                    block_indent.indent,
                    quote_indent,
                );
            }
            self.opaque_depth = Some(self.depth);
            return;
        }

        if let Some(html) = MdHtmlBlock::cast(node.clone()) {
            let is_inline = node
                .parent()
                .and_then(biome_markdown_syntax::MdInlineItemList::cast)
                .is_some();
            if is_inline {
                let content = collect_raw_inline_text(&html.content());
                self.push_str(&content);
            } else {
                let block_indent = self.block_indent(html.syntax().text_trimmed_range());
                let quote_indent = self.quote_indent;
                render_html_block(&html, self.out_mut(), block_indent.indent, quote_indent);
            }
            self.opaque_depth = Some(self.depth);
            return;
        }

        if MdThematicBreakBlock::cast(node.clone()).is_some() {
            self.push_str("<hr />\n");
            return;
        }

        if MdLinkReferenceDefinition::cast(node.clone()).is_some()
            || MdLinkBlock::cast(node.clone()).is_some()
        {
            self.opaque_depth = Some(self.depth);
            return;
        }

        if MdLinkDestination::cast(node.clone()).is_some()
            || MdLinkLabel::cast(node.clone()).is_some()
            || MdLinkTitle::cast(node.clone()).is_some()
            || MdReferenceLinkLabel::cast(node.clone()).is_some()
        {
            self.opaque_depth = Some(self.depth);
            return;
        }

        if MdInlineEmphasis::cast(node.clone()).is_some() {
            self.push_str("<strong>");
            return;
        }

        if MdInlineItalic::cast(node.clone()).is_some() {
            self.push_str("<em>");
            return;
        }

        if let Some(code) = MdInlineCode::cast(node.clone()) {
            render_inline_code(&code, self.out_mut());
            self.opaque_depth = Some(self.depth);
            return;
        }

        if let Some(link) = MdInlineLink::cast(node.clone()) {
            let dest = collect_inline_text(&link.destination());
            let dest = process_link_destination(&dest);
            self.suppressed_inline_nodes
                .push(vec![link.destination().syntax().clone()]);

            self.push_str("<a href=\"");
            self.push_str(&escape_html_attribute(&dest));
            self.push_str("\"");

            if let Some(title) = link.title() {
                render_link_title(&title, self.out_mut());
            }

            self.push_str(">");
            return;
        }

        if let Some(img) = MdInlineImage::cast(node.clone()) {
            let alt = extract_alt_text(&img.alt(), self.ctx);
            let dest = collect_inline_text(&img.destination());
            let dest = process_link_destination(&dest);

            self.push_str("<img src=\"");
            self.push_str(&escape_html_attribute(&dest));
            self.push_str("\" alt=\"");
            self.push_str(&escape_html_attribute(&alt));
            self.push_str("\"");

            if let Some(title) = img.title() {
                render_link_title(&title, self.out_mut());
            }

            self.push_str(" />");
            self.opaque_depth = Some(self.depth);
            return;
        }

        if let Some(link) = MdReferenceLink::cast(node.clone()) {
            let text_raw = collect_inline_text(&link.text());
            let (label, label_display) =
                resolve_reference_label(link.label(), text_raw, |label_node| {
                    collect_inline_text(&label_node.label())
                });
            let (url, title) = self
                .ctx
                .get_link_definition(&label)
                .map_or((None, None), |(url, title)| {
                    (Some(url.clone()), title.clone())
                });
            self.push_buffer(BufferKind::ReferenceLink(ReferenceLinkState {
                label_display,
                url,
                title,
            }));
            return;
        }

        if let Some(img) = MdReferenceImage::cast(node.clone()) {
            let alt = extract_alt_text(&img.alt(), self.ctx);
            let alt_raw = collect_inline_text(&img.alt());
            let (label, label_display) =
                resolve_reference_label(img.label(), alt_raw, |label_node| {
                    collect_inline_text(&label_node.label())
                });

            if let Some((url, title)) = self.ctx.get_link_definition(&label) {
                self.push_str("<img src=\"");
                self.push_str(&escape_html_attribute(url));
                self.push_str("\" alt=\"");
                self.push_str(&escape_html_attribute(&alt));
                self.push_str("\"");

                if let Some(t) = title {
                    self.push_str(" title=\"");
                    self.push_str(&escape_html_attribute(t));
                    self.push_str("\"");
                }

                self.push_str(" />");
            } else {
                self.push_str("![");
                self.push_str(&escape_html(&alt));
                self.push_str("]");
                if let Some(label) = label_display {
                    self.push_str("[");
                    self.push_str(&escape_html(&label));
                    self.push_str("]");
                }
            }

            self.opaque_depth = Some(self.depth);
            return;
        }

        if let Some(autolink) = MdAutolink::cast(node.clone()) {
            render_autolink(&autolink, self.out_mut());
            self.opaque_depth = Some(self.depth);
            return;
        }

        if let Some(html) = MdInlineHtml::cast(node.clone()) {
            render_inline_html(&html, self.out_mut());
            self.opaque_depth = Some(self.depth);
            return;
        }

        if let Some(hard) = MdHardLine::cast(node.clone()) {
            if is_last_inline_item(&node) {
                if let Ok(token) = hard.value_token()
                    && token.text().starts_with('\\')
                {
                    self.push_str("\\");
                }
            } else {
                self.push_str("<br />\n");
            }
            return;
        }

        if MdSoftBreak::cast(node.clone()).is_some() {
            self.push_str("\n");
            return;
        }

        if let Some(text) = MdTextual::cast(node.clone()) {
            render_textual(&text, self.out_mut());
            return;
        }

        if let Some(entity) = MdEntityReference::cast(node) {
            render_entity_reference(&entity, self.out_mut());
        }
    }

    fn leave(&mut self, node: SyntaxNode<MarkdownLanguage>) {
        if MdParagraph::cast(node.clone()).is_some() {
            let buffer = self.pop_buffer();
            if let BufferKind::Paragraph(state) = buffer.kind {
                if state.suppress_wrapping {
                    self.push_str(&buffer.content);
                    return;
                }
                let mut content = buffer.content;
                if state.quote_indent > 0 {
                    content = strip_quote_prefixes(&content, state.quote_indent);
                }
                let content = strip_paragraph_indent(
                    content.trim_matches(|c| c == ' ' || c == '\n' || c == '\r'),
                );

                if state.in_tight_list {
                    self.push_str(&content);
                    self.push_str("\n");
                } else {
                    self.push_str("<p>");
                    self.push_str(&content);
                    self.push_str("</p>\n");
                }
            }
            return;
        }

        if MdHeader::cast(node.clone()).is_some() || MdSetextHeader::cast(node.clone()).is_some() {
            let buffer = self.pop_buffer();
            if let BufferKind::Header(state) | BufferKind::SetextHeader(state) = buffer.kind {
                self.push_str("<h");
                self.push_str(&state.level.to_string());
                self.push_str(">");
                self.push_str(buffer.content.trim());
                self.push_str("</h");
                self.push_str(&state.level.to_string());
                self.push_str(">\n");
            }
            return;
        }

        if MdQuote::cast(node.clone()).is_some() {
            self.push_str("</blockquote>\n");
            if let Some(indent) = self.quote_indent_stack.pop() {
                self.quote_indent = self.quote_indent.saturating_sub(indent);
            }
            return;
        }

        if MdBulletListItem::cast(node.clone()).is_some() {
            self.push_str("</ul>\n");
            self.list_stack.pop();
            return;
        }

        if MdOrderedListItem::cast(node.clone()).is_some() {
            self.push_str("</ol>\n");
            self.list_stack.pop();
            return;
        }

        if MdBullet::cast(node.clone()).is_some() {
            let buffer = self.pop_buffer();
            let state = self.list_item_stack.pop();
            if let (BufferKind::ListItem, Some(state)) = (buffer.kind, state) {
                if state.is_empty {
                    self.push_str("<li></li>\n");
                    return;
                }

                self.push_str("<li>");
                if state.leading_newline {
                    self.push_str("\n");
                }

                let mut content = buffer.content;
                if state.trim_trailing_newline && content.ends_with('\n') {
                    content.pop();
                }
                self.push_str(&content);
                self.push_str("</li>\n");
            }
            return;
        }

        if MdInlineEmphasis::cast(node.clone()).is_some() {
            self.push_str("</strong>");
            return;
        }

        if MdInlineItalic::cast(node.clone()).is_some() {
            self.push_str("</em>");
            return;
        }

        if MdInlineLink::cast(node.clone()).is_some() {
            self.suppressed_inline_nodes.pop();
            self.push_str("</a>");
            return;
        }

        if MdReferenceLink::cast(node).is_some() {
            let buffer = self.pop_buffer();
            if let BufferKind::ReferenceLink(state) = buffer.kind {
                if let Some(url) = state.url {
                    self.push_str("<a href=\"");
                    self.push_str(&escape_html_attribute(&url));
                    self.push_str("\"");
                    if let Some(title) = state.title {
                        self.push_str(" title=\"");
                        self.push_str(&escape_html_attribute(&title));
                        self.push_str("\"");
                    }
                    self.push_str(">");
                    self.push_str(&buffer.content);
                    self.push_str("</a>");
                } else {
                    self.push_str("[");
                    self.push_str(&buffer.content);
                    self.push_str("]");
                    if let Some(label) = state.label_display {
                        self.push_str("[");
                        self.push_str(&escape_html(&label));
                        self.push_str("]");
                    }
                }
            }
        }
    }

    fn push_buffer(&mut self, kind: BufferKind) {
        self.buffers.push(Buffer {
            kind,
            content: String::new(),
        });
    }

    fn pop_buffer(&mut self) -> Buffer {
        self.buffers.pop().unwrap_or(Buffer {
            kind: BufferKind::Root,
            content: String::new(),
        })
    }

    fn out_mut(&mut self) -> &mut String {
        &mut self.buffers.last_mut().expect("missing buffer").content
    }

    fn push_str(&mut self, value: &str) {
        self.out_mut().push_str(value);
    }

    fn block_indent(&self, range: TextRange) -> BlockIndent {
        self.list_item_stack
            .last()
            .and_then(|state| state.block_indents.get(&range).copied())
            .unwrap_or_default()
    }
}

fn is_last_inline_item(node: &SyntaxNode<MarkdownLanguage>) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };
    let Some(list) = biome_markdown_syntax::MdInlineItemList::cast(parent) else {
        return false;
    };
    list.iter().last().is_some_and(|item| item.syntax() == node)
}

/// Strip leading whitespace from paragraph continuation lines.
///
/// Per CommonMark §4.8, paragraph continuation lines can have any amount of
/// initial whitespace, and that whitespace is stripped in the output.
/// The first line keeps its content unchanged; subsequent lines have all
/// leading spaces and tabs stripped.
fn strip_paragraph_indent(content: &str) -> String {
    let mut first_line = true;
    map_lines(content, |line, out| {
        if first_line {
            // First line: keep as-is
            first_line = false;
            out.push_str(line);
        } else {
            // Continuation lines: strip ALL leading whitespace
            out.push_str(line.trim_start());
        }
    })
}

/// Render an ATX header (# style).
fn header_level(header: &MdHeader) -> usize {
    // Count total hash characters in the before list.
    // The lexer emits all consecutive `#` chars as a single HASH token,
    // so we sum the text lengths of all hash tokens.
    // Use text_trimmed() to exclude any leading trivia (skipped indentation spaces).
    header
        .before()
        .iter()
        .filter_map(|h| h.hash_token().ok())
        .map(|tok| tok.text_trimmed().len())
        .sum::<usize>()
        .clamp(1, 6)
}

/// Render a setext header (underline style).
fn setext_header_level(header: &MdSetextHeader) -> usize {
    if let Ok(underline) = header.underline_token() {
        let text = underline.text();
        if text.trim_start().starts_with('=') {
            1
        } else {
            2
        }
    } else {
        1
    }
}

// ============================================================================
// Code Block Rendering
// ============================================================================

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
                    '\t' => indent += TAB_STOP_SPACES - (indent % TAB_STOP_SPACES),
                    '\n' | '\r' => indent = 0, // Reset at newlines
                    _ => {}
                }
            }
        }
        indent
    });
    let container_indent = list_indent + quote_indent;
    let fence_indent = fence_leading_indent
        .saturating_sub(container_indent)
        .min(MAX_BLOCK_PREFIX_INDENT);
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
    if quote_indent > 0 {
        content = strip_quote_prefixes(&content, quote_indent);
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

fn render_indented_code_block_in_list(
    code: &MdIndentCodeBlock,
    out: &mut String,
    list_indent: usize,
    quote_indent: usize,
    first_line_column: usize,
) {
    out.push_str("<pre><code>");

    let mut content = collect_raw_inline_text(&code.content());
    if content.starts_with('\n') {
        content = content[1..].to_string();
    }

    let content = strip_indent_preserve_tabs_with_offset(
        &content,
        4 + list_indent + quote_indent,
        first_line_column,
    );
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

/// Render textual content.
fn render_textual(text: &MdTextual, out: &mut String) {
    if let Ok(token) = text.value_token() {
        // Use text_trimmed() to exclude skipped trivia (e.g., indentation stripped during parsing)
        let raw = token.text_trimmed();
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
        if text.trim().is_empty() {
            (fallback, None)
        } else {
            (text.clone(), Some(text))
        }
    } else {
        (fallback, None)
    }
}

/// Render an autolink.
fn render_autolink(autolink: &MdAutolink, out: &mut String) {
    let content = collect_raw_inline_text(&autolink.value());

    // Check if it's an email autolink
    let is_email = content.contains('@') && !content.contains(':');

    // Autolinks must NOT process backslash escapes or entity decoding.
    // Only percent-encode for URL safety.
    let href = if is_email {
        format!("mailto:{}", content)
    } else {
        percent_encode_uri(&content)
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
                && bytes[i + 1].is_ascii_hexdigit()
                && bytes[i + 2].is_ascii_hexdigit()
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

/// Extract plain text for image alt attribute.
/// Per CommonMark, the alt text is the content with inline formatting stripped
/// but text from nested links/images preserved (recursively extracting their text).
fn extract_alt_text(
    list: &biome_markdown_syntax::MdInlineItemList,
    ctx: &HtmlRenderContext,
) -> String {
    let mut result = String::new();
    for item in list.iter() {
        extract_alt_text_inline(&item, ctx, &mut result);
    }
    result
}

fn extract_alt_text_inline(inline: &AnyMdInline, ctx: &HtmlRenderContext, out: &mut String) {
    // NOTE: This function collects raw text WITHOUT HTML escaping.
    // The final escaping happens in escape_html_attribute when writing the alt attribute.
    match inline {
        AnyMdInline::MdTextual(text) => {
            // Process backslash escapes but don't escape HTML
            if let Ok(token) = text.value_token() {
                let raw = token.text_trimmed();
                let processed = process_escapes(raw);
                out.push_str(&processed);
            }
        }
        AnyMdInline::MdInlineEmphasis(em) => {
            out.push_str(&extract_alt_text(&em.content(), ctx));
        }
        AnyMdInline::MdInlineItalic(italic) => {
            out.push_str(&extract_alt_text(&italic.content(), ctx));
        }
        AnyMdInline::MdInlineCode(code) => {
            // Plain text only — no <code> tags for alt attribute
            let content = collect_raw_inline_text(&code.content());
            let content = content.replace('\n', " ");
            let content = if content.starts_with(' ')
                && content.ends_with(' ')
                && content.len() > 2
                && content.chars().any(|c| c != ' ')
            {
                content[1..content.len() - 1].to_string()
            } else {
                content
            };
            out.push_str(&content);
        }
        AnyMdInline::MdInlineLink(link) => {
            // Extract text content from link text
            out.push_str(&extract_alt_text(&link.text(), ctx));
        }
        AnyMdInline::MdInlineImage(img) => {
            // Recursively extract alt text from nested image
            out.push_str(&extract_alt_text(&img.alt(), ctx));
        }
        AnyMdInline::MdReferenceLink(link) => {
            out.push_str(&extract_alt_text(&link.text(), ctx));
        }
        AnyMdInline::MdReferenceImage(img) => {
            out.push_str(&extract_alt_text(&img.alt(), ctx));
        }
        AnyMdInline::MdAutolink(autolink) => {
            let content = collect_raw_inline_text(&autolink.value());
            out.push_str(&content);
        }
        AnyMdInline::MdHardLine(_) | AnyMdInline::MdSoftBreak(_) => {
            out.push(' ');
        }
        AnyMdInline::MdEntityReference(entity) => {
            // Decode entity but don't escape HTML
            if let Ok(token) = entity.value_token() {
                let text = token.text();
                if let Some(decoded) = decode_entity(text) {
                    out.push_str(&decoded);
                } else {
                    // Unknown entity - pass through as-is
                    out.push_str(text);
                }
            }
        }
        AnyMdInline::MdInlineHtml(_) | AnyMdInline::MdHtmlBlock(_) => {
            // HTML tags are stripped in alt text
        }
        AnyMdInline::MdQuotePrefix(_) => {
            // Quote prefixes don't contribute text to alt attributes
        }
    }
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
        AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdParagraph(_))
    )
}

/// Check if a block is a newline (produces no output).
fn is_newline_block(block: &AnyMdBlock) -> bool {
    matches!(
        block,
        AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdNewline(_))
    )
}

/// Check if blocks are effectively empty (empty or only newlines).
fn is_empty_content(blocks: &[AnyMdBlock]) -> bool {
    blocks.is_empty() || blocks.iter().all(is_newline_block)
}

fn list_item_required_indent(entry: &ListItemIndent) -> usize {
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
    fn test_emphasis_complex_cases() {
        // Test: Nested
        let parsed = parse_markdown("**bold *and italic* text**\n");
        assert_eq!(
            parsed.syntax().kind(),
            biome_markdown_syntax::MarkdownSyntaxKind::MD_DOCUMENT,
            "Nested failed: {}",
            parsed.syntax()
        );

        // Test: Rule of 3
        let parsed = parse_markdown("***bold italic***\n");
        assert_eq!(
            parsed.syntax().kind(),
            biome_markdown_syntax::MarkdownSyntaxKind::MD_DOCUMENT,
            "Rule of 3 failed: {}",
            parsed.syntax()
        );

        // Test: Multiple runs
        let parsed = parse_markdown("*a **b** c*\n");
        assert_eq!(
            parsed.syntax().kind(),
            biome_markdown_syntax::MarkdownSyntaxKind::MD_DOCUMENT,
            "Multiple runs failed: {}",
            parsed.syntax()
        );

        // Test: Overlapping
        let parsed = parse_markdown("*foo**bar**baz*\n");
        assert_eq!(
            parsed.syntax().kind(),
            biome_markdown_syntax::MarkdownSyntaxKind::MD_DOCUMENT,
            "Overlapping failed: {}",
            parsed.syntax()
        );

        // Test: Unbalanced emphasis (CommonMark example 442)
        // **foo* should produce *<em>foo</em>
        let parsed = parse_markdown("**foo*\n");
        let html = document_to_html(
            &parsed.tree(),
            parsed.list_tightness(),
            parsed.list_item_indents(),
            parsed.quote_indents(),
        );
        assert_eq!(
            html,
            "<p>*<em>foo</em></p>\n",
            "Unbalanced: {}",
            parsed.syntax()
        );
    }

    #[test]
    fn test_example_431() {
        // Test: Example 431 - nested emphasis with triple star closer
        // **foo *bar*** should produce <strong>foo <em>bar</em></strong>
        let parsed = parse_markdown("**foo *bar***\n");
        let html = document_to_html(
            &parsed.tree(),
            parsed.list_tightness(),
            parsed.list_item_indents(),
            parsed.quote_indents(),
        );
        assert_eq!(
            html,
            "<p><strong>foo <em>bar</em></strong></p>\n",
            "Example 431: {}",
            parsed.syntax()
        );
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

    #[test]
    fn test_percent_encode_uri() {
        let input = format!("https://a{}b.c/%20/%", '\u{1F44D}');
        let encoded = percent_encode_uri(&input);
        assert_eq!(encoded, "https://a%F0%9F%91%8Db.c/%20/%25");
    }

    #[test]
    fn test_process_link_destination_decodes_entities() {
        let encoded = process_link_destination("https://example.com/&lt;");
        assert_eq!(encoded, "https://example.com/%3C");
    }

    #[test]
    fn test_paren_depth_limit_in_destination() {
        let dest = format!("x{}y{}", "(".repeat(32), ")".repeat(32));
        let input = format!("[a]({dest})\n");
        let parsed = parse_markdown(&input);
        let html = document_to_html(
            &parsed.tree(),
            parsed.list_tightness(),
            parsed.list_item_indents(),
            parsed.quote_indents(),
        );
        let expected = format!("<p><a href=\"{dest}\">a</a></p>\n");
        assert_eq!(html, expected);
    }

    #[test]
    fn test_paren_depth_limit_exceeded_in_destination() {
        let dest = format!("x{}y{}", "(".repeat(33), ")".repeat(33));
        let input = format!("[a]({dest})\n");
        let parsed = parse_markdown(&input);
        let html = document_to_html(
            &parsed.tree(),
            parsed.list_tightness(),
            parsed.list_item_indents(),
            parsed.quote_indents(),
        );
        let expected_dest = format!("x{}", "(".repeat(32));
        let trailing = ")".repeat(34);
        let expected = format!("<p><a href=\"{expected_dest}\">a</a>(y{trailing}</p>\n");
        assert_eq!(html, expected);
    }

    #[test]
    fn test_title_with_escaped_closing_quote() {
        let parsed = parse_markdown("[a](/url \"title with \\\" quote\")\n");
        let html = document_to_html(
            &parsed.tree(),
            parsed.list_tightness(),
            parsed.list_item_indents(),
            parsed.quote_indents(),
        );
        assert_eq!(
            html,
            "<p><a href=\"/url\" title=\"title with &quot; quote\">a</a></p>\n"
        );
    }

    #[test]
    fn test_hard_line_break_at_end_of_block_is_literal() {
        let parsed = parse_markdown("foo\\\\\n");
        let html = document_to_html(
            &parsed.tree(),
            parsed.list_tightness(),
            parsed.list_item_indents(),
            parsed.quote_indents(),
        );
        assert_eq!(html, "<p>foo\\</p>\n");
    }
}
