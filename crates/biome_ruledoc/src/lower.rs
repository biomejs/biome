//! Lowers the lossless Markdown CST into rule-document HIR.
//!
//! Fenced code content needs semantic normalization before analyzers can consume
//! it. The CST preserves opening line breaks, container prefixes, indentation,
//! tabs, and line-ending spelling, while a documentation example needs the code
//! that CommonMark exposes after removing structural syntax. Typed Markdown
//! nodes normalize their local text; this module adds parser metadata for list
//! indentation and produces the rule-document representation.

use crate::{
    RuleDocCodeBlock, RuleDocHeading, RuleDocHtml, RuleDocHtmlKind, RuleDocument, RuleDocumentItem,
};
use biome_markdown_parser::MarkdownParse;
use biome_markdown_syntax::{
    MdBlockList, MdBullet, MdFencedCodeBlock, MdHeader, MdHtmlBlock, MdInlineHtml, MdSetextHeader,
};
use biome_rowan::{AstNode, WalkEvent};

const TAB_STOP_SPACES: usize = 4;
const INDENT_CODE_BLOCK_SPACES: usize = 4;
const MAX_BLOCK_PREFIX_INDENT: usize = 3;

/// Collects the document constructs that carry rule-document behavior.
///
/// A preorder traversal preserves source order, including inline HTML nested in
/// headings or paragraphs. Ordinary prose is intentionally omitted because its
/// rendering remains the responsibility of the documentation consumer. Block
/// HTML is split into physical-line fragments because HTML validation consumes
/// each source line as an independent fragment.
pub(crate) fn lower_document(parse: &MarkdownParse) -> RuleDocument {
    let mut items = Vec::new();

    for event in parse.syntax().preorder() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };

        if let Some(heading) = MdHeader::cast(node.clone()) {
            items.push(RuleDocumentItem::Heading(RuleDocHeading {
                range: node.text_range_with_trivia(),
                level: heading.level(),
            }));
        } else if let Some(heading) = MdSetextHeader::cast(node.clone()) {
            items.push(RuleDocumentItem::Heading(RuleDocHeading {
                range: node.text_range_with_trivia(),
                level: heading.level(),
            }));
        } else if let Some(code_block) = MdFencedCodeBlock::cast(node.clone()) {
            items.push(RuleDocumentItem::FencedCodeBlock(lower_fenced_code_block(
                parse, code_block,
            )));
        } else if MdHtmlBlock::can_cast(node.kind()) {
            items.push(RuleDocumentItem::RawHtml(RuleDocHtml {
                range: node.text_range_with_trivia(),
                kind: RuleDocHtmlKind::Block,
                source_fragments: split_physical_lines(&node.to_string()),
            }));
        } else if MdInlineHtml::can_cast(node.kind()) {
            items.push(RuleDocumentItem::RawHtml(RuleDocHtml {
                range: node.text_range_with_trivia(),
                kind: RuleDocHtmlKind::Inline,
                source_fragments: vec![node.to_string()],
            }));
        }
    }

    RuleDocument::new(items)
}

/// Converts a fenced block from lossless source text to executable code.
///
/// Document-level blocks store their body in one literal token, while blocks in
/// lists or quotes store per-line inline nodes with structural prefixes. Both
/// representations include the line break after the opening fence. The lowering
/// removes that line break, normalizes line endings, strips exactly the active
/// quote prefixes, and then removes list and fence indentation by visual column.
/// Literal indentation beyond those structural columns remains part of the body.
fn lower_fenced_code_block(
    parse: &MarkdownParse,
    code_block: MdFencedCodeBlock,
) -> RuleDocCodeBlock {
    let info = code_block.info_string();
    let mut body = code_block.content_text();

    let list_indent = direct_list_indent(parse, &code_block);
    let quote_depth = code_block.quote_depth();
    let fence_indent = code_block.opening_indent_width();
    let absolute_column = code_block.line_start_column();
    let content_indent = (list_indent + fence_indent).max(absolute_column);

    // Quote markers precede indentation in the raw line. Removing them first
    // gives indentation the same column origin as the fenced block parser.
    if quote_depth > 0 {
        body = strip_quote_prefixes(&body, quote_depth);
    }
    if content_indent > 0 {
        body = strip_indent_preserve_tabs(&body, content_indent);
    }

    RuleDocCodeBlock {
        range: code_block.syntax().text_range_with_trivia(),
        info,
        body,
    }
}

/// Returns the structural list indentation for a directly contained block.
///
/// Parser metadata records the effective content column for every list item.
/// Only a code block whose block list belongs directly to an `MdBullet` uses
/// that item's indentation; a block behind another container is normalized by
/// that container instead. More than four spaces after a marker count as one
/// content space, following CommonMark list-item indentation rules.
fn direct_list_indent(parse: &MarkdownParse, code_block: &MdFencedCodeBlock) -> usize {
    let Some(bullet) = code_block
        .syntax()
        .parent()
        .and_then(MdBlockList::cast)
        .and_then(|list| list.syntax().parent())
        .and_then(MdBullet::cast)
    else {
        return 0;
    };
    let range = bullet.syntax().text_trimmed_range();

    parse
        .list_item_indents()
        .iter()
        .find(|entry| entry.range == range)
        .map_or(0, |entry| {
            if entry.spaces_after_marker > INDENT_CODE_BLOCK_SPACES {
                entry.marker_indent + entry.marker_width + 1
            } else {
                entry.marker_indent + entry.marker_width + entry.spaces_after_marker.max(1)
            }
        })
}

/// Removes up to `strip_columns` leading visual columns from every line.
///
/// Tabs wholly beyond the structural prefix remain literal tabs. When a tab
/// crosses the removal boundary, the portion after the boundary becomes spaces
/// so the code keeps its original visual alignment.
fn strip_indent_preserve_tabs(text: &str, strip_columns: usize) -> String {
    map_lines(text, |line, output| {
        let mut column = 0usize;
        let mut content_start = 0usize;

        for (index, character) in line.char_indices() {
            if column >= strip_columns {
                content_start = index;
                break;
            }
            match character {
                '\t' => {
                    let next_column = column + (TAB_STOP_SPACES - (column % TAB_STOP_SPACES));
                    if next_column > strip_columns {
                        output.extend(std::iter::repeat_n(' ', next_column - strip_columns));
                        content_start = index + 1;
                        break;
                    }
                    column = next_column;
                }
                ' ' => column += 1,
                _ => {
                    content_start = index;
                    break;
                }
            }
            content_start = index + character.len_utf8();
        }

        if content_start < line.len() {
            output.push_str(&line[content_start..]);
        }
    })
}

/// Removes at most `quote_depth` CommonMark quote prefixes from every line.
///
/// A prefix may have up to three leading columns and one space or tab after
/// `>`. Stopping at the requested depth preserves `>` characters that belong to
/// the fenced code itself.
fn strip_quote_prefixes(text: &str, quote_depth: usize) -> String {
    map_lines(text, |line, output| {
        let mut remaining = line;
        for _ in 0..quote_depth {
            let mut index = 0usize;
            let mut column = 0usize;

            while index < remaining.len() {
                match remaining.as_bytes()[index] {
                    b' ' => {
                        column += 1;
                        index += 1;
                    }
                    b'\t' => {
                        column += TAB_STOP_SPACES - (column % TAB_STOP_SPACES);
                        index += 1;
                    }
                    _ => break,
                }
                if column > MAX_BLOCK_PREFIX_INDENT {
                    index = 0;
                    break;
                }
            }

            if index >= remaining.len() || remaining.as_bytes()[index] != b'>' {
                break;
            }
            index += 1;
            if index < remaining.len() && matches!(remaining.as_bytes()[index], b' ' | b'\t') {
                index += 1;
            }
            remaining = &remaining[index..];
        }

        output.push_str(remaining);
    })
}

/// Splits block HTML while retaining each physical line ending.
///
/// The HTML tag checker is intentionally fragment-sensitive: an unterminated
/// comment in one fragment does not hide tags in later fragments. Keeping one
/// fragment per source line preserves that validation contract. Inline HTML is
/// not passed through this helper and remains a single fragment.
fn split_physical_lines(source: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line_start = 0usize;
    let mut index = 0usize;
    let bytes = source.as_bytes();

    while index < bytes.len() {
        match bytes[index] {
            b'\n' => {
                index += 1;
                lines.push(source[line_start..index].to_string());
                line_start = index;
            }
            b'\r' => {
                index += 1;
                if bytes.get(index) == Some(&b'\n') {
                    index += 1;
                }
                lines.push(source[line_start..index].to_string());
                line_start = index;
            }
            _ => index += 1,
        }
    }

    if line_start < source.len() {
        lines.push(source[line_start..].to_string());
    }
    lines
}

/// Applies a normalization function to each line while preserving `\n`
/// separators in the output.
fn map_lines(text: &str, mut map: impl FnMut(&str, &mut String)) -> String {
    let mut output = String::new();
    for (index, line) in text.split('\n').enumerate() {
        if index > 0 {
            output.push('\n');
        }
        map(line.strip_suffix('\r').unwrap_or(line), &mut output);
    }
    output
}
