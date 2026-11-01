//! MDX constructs that hold JavaScript: ESM blocks and expressions.
//!
//! The JavaScript stays opaque here. Each construct stores its code in a
//! single literal token, and the workspace parses that token as an embedded
//! JavaScript snippet. This module only decides where each construct ends,
//! using [`JsExtent`] so that a `}` or a blank line inside a string, a
//! comment, a template literal, or JSX doesn't end it early.
//!
//! The rules follow [`micromark-extension-mdxjs`](https://github.com/micromark/micromark-extension-mdxjs),
//! the reference implementation of MDX 3.

use crate::MarkdownParser;
use crate::syntax::parse_error::unclosed_mdx_expression;
use biome_js_extent::{JsExtent, Jsx};
use biome_markdown_syntax::MarkdownSyntaxKind::*;
use biome_markdown_syntax::T;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax::{self, *};
use biome_rowan::{TextRange, TextSize};

/// JSX in MDX follows React rather than HTML: an element without `/>` keeps
/// its children open until its closing tag, even when it is named `br`.
const JSX: Jsx = Jsx::Enabled {
    is_void_element: no_void_elements,
};

fn no_void_elements(_name: &[u8]) -> bool {
    false
}

/// Returns whether the current line starts an ESM block or a block
/// expression.
pub(crate) fn at_mdx_block(p: &mut MarkdownParser) -> bool {
    at_esm_block(p) || block_expression_close(p).is_some()
}

/// Parses an ESM block or a block expression at the current line.
pub(crate) fn parse_mdx_block(p: &mut MarkdownParser) -> ParsedSyntax {
    if at_esm_block(p) {
        return parse_esm_block(p);
    }
    match block_expression_close(p) {
        Some(close) => parse_block_expression(p, close),
        None => Absent,
    }
}

/// Returns whether the current line starts with `import ` or `export ` at
/// column 1, outside any block quote or list.
///
/// MDX recognizes ESM only at the start of a line: indented lines, lines in
/// containers, and keywords without a following space (such as
/// `import("a")` or `exporting`) are paragraph text.
fn at_esm_block(p: &MarkdownParser) -> bool {
    let state = p.state();
    if state.block_quote_depth > 0 || state.list_nesting_depth > 0 {
        return false;
    }
    if !p.is_at_line_start() || p.line_start_leading_indent() > 0 {
        return false;
    }
    let rest = p.source_after_current();
    rest.starts_with("import ") || rest.starts_with("export ")
}

/// Parses an ESM block, from its first line up to the next blank line.
///
/// Grammar: MdxEsmBlock = content: 'mdx_esm_literal'
fn parse_esm_block(p: &mut MarkdownParser) -> ParsedSyntax {
    let m = p.start();
    let source = p.source_after_current();
    let length = JsExtent::esm_length(source.as_bytes());
    let length = content_end(&source[..length]);
    let end = p.cur_range().start() + TextSize::from(length as u32);
    p.re_lex_span(end, MDX_ESM_LITERAL);
    p.bump(MDX_ESM_LITERAL);
    let block = m.complete(p, MDX_ESM_BLOCK);
    parse_line_ending(p).ok();
    Present(block)
}

/// Where a block expression ends.
#[derive(Debug, Clone, Copy)]
enum ExpressionClose {
    /// The offset of the closing `}`.
    At(TextSize),
    /// No `}` closes the expression before the end of the source.
    Missing,
}

/// Returns where the block expression starting on the current line ends, or
/// `None` when the line doesn't hold a block expression.
///
/// A block expression is a `{` at the start of the line, after any amount of
/// indentation, whose `}` is followed only by spaces and tabs up to the end
/// of the line. Any other text after the `}` makes the line a paragraph that
/// contains inline expressions.
fn block_expression_close(p: &MarkdownParser) -> Option<ExpressionClose> {
    if !p.is_at_line_start() {
        return None;
    }
    let rest = p.source_after_current();
    let indent = rest.len() - rest.trim_start_matches([' ', '\t']).len();
    let body = rest[indent..].strip_prefix('{')?;
    let length = JsExtent::expression_length(body.as_bytes(), JSX);
    if length == body.len() {
        return Some(ExpressionClose::Missing);
    }
    let after = body[length + 1..].trim_start_matches([' ', '\t']);
    if !(after.is_empty() || after.starts_with(['\n', '\r'])) {
        return None;
    }
    let close = indent + 1 + length;
    Some(ExpressionClose::At(
        p.cur_range().start() + TextSize::from(close as u32),
    ))
}

/// Parses an expression that is alone on its line.
///
/// Grammar: MdxBlockExpression = indent: MdIndentTokenList '{' content: MdxExpressionContent? '}'
fn parse_block_expression(p: &mut MarkdownParser, close: ExpressionClose) -> ParsedSyntax {
    let m = p.start();
    p.emit_line_indent(usize::MAX);
    if !p.at(T!['{']) {
        m.abandon(p);
        return Absent;
    }
    let close = match close {
        ExpressionClose::At(offset) => Some(offset),
        ExpressionClose::Missing => None,
    };
    bump_expression_body(p, close, p.source_after_current().len());
    // Spaces and tabs after the `}` don't belong to the expression.
    loop {
        if p.at(MD_HARD_LINE_LITERAL) {
            p.force_relex_heading_content();
        }
        let is_space = p.at(MD_TEXTUAL_LITERAL)
            && !p.cur_text().is_empty()
            && p.cur_text()
                .bytes()
                .all(|byte| matches!(byte, b' ' | b'\t'));
        if !is_space {
            break;
        }
        p.consume_as_whitespace_trivia();
    }
    let block = m.complete(p, MDX_BLOCK_EXPRESSION);
    parse_line_ending(p).ok();
    Present(block)
}

/// Parses an expression inside a paragraph or heading.
///
/// An inline expression can span lines, but not a blank line: a `{` that is
/// still open at the end of its paragraph is an error.
///
/// Grammar: MdxInlineExpression = '{' content: MdxExpressionContent? '}'
pub(crate) fn parse_mdx_inline_expression(p: &mut MarkdownParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }
    let m = p.start();
    let rest = p.source_after_current();
    let limit = paragraph_end(rest);
    let body = &rest[1..limit];
    let length = JsExtent::expression_length(body.as_bytes(), JSX);
    let close =
        (length < body.len()).then(|| p.cur_range().start() + TextSize::from((1 + length) as u32));
    bump_expression_body(p, close, limit);
    Present(m.complete(p, MDX_INLINE_EXPRESSION))
}

/// Consumes the `{`, the expression content, and the `}` of an expression
/// into the node that the caller started.
///
/// Without a `close`, the content runs to `limit`, measured in bytes from
/// the `{`, and the missing `}` is reported.
fn bump_expression_body(p: &mut MarkdownParser, close: Option<TextSize>, limit: usize) {
    let open = p.cur_range();
    let content_end = match close {
        Some(close) => close,
        None => {
            let body = &p.source_after_current()[1..limit];
            open.start() + TextSize::from((1 + content_end(body)) as u32)
        }
    };
    p.bump(T!['{']);
    parse_expression_content(p, content_end).ok();
    match close {
        Some(close) => {
            p.re_lex_span(close + TextSize::from(1), T!['}']);
            p.bump(T!['}']);
        }
        None => {
            let opening = TextRange::at(open.start(), TextSize::from(1));
            p.error(unclosed_mdx_expression(p, opening));
        }
    }
}

/// Parses the JavaScript of an expression, from the current position up to
/// `end`, as one literal. An empty expression `{}` has no content.
///
/// Grammar: MdxExpressionContent = value: 'mdx_expression_literal'
fn parse_expression_content(p: &mut MarkdownParser, end: TextSize) -> ParsedSyntax {
    if end <= p.cur_range().start() {
        return Absent;
    }
    let m = p.start();
    p.re_lex_span(end, MDX_EXPRESSION_LITERAL);
    p.bump(MDX_EXPRESSION_LITERAL);
    Present(m.complete(p, MDX_EXPRESSION_CONTENT))
}

/// Returns the length of the expression at the start of `bytes`, which
/// starts with `{`, including both braces, or `bytes.len()` when no `}`
/// closes it.
pub(crate) fn mdx_expression_len(bytes: &[u8]) -> usize {
    let body = &bytes[1..];
    let length = JsExtent::expression_length(body, JSX);
    if length < body.len() {
        length + 2
    } else {
        bytes.len()
    }
}

/// Parses the line ending after a block as its own newline node.
///
/// Grammar: MdNewline = value: 'NEWLINE'
fn parse_line_ending(p: &mut MarkdownParser) -> ParsedSyntax {
    if !p.at(NEWLINE) {
        return Absent;
    }
    let m = p.start();
    p.bump(NEWLINE);
    Present(m.complete(p, MD_NEWLINE))
}

/// Returns the length of `text` without its trailing blank lines. The last
/// line with content keeps its trailing spaces but not its line ending.
fn content_end(text: &str) -> usize {
    let last = text.trim_end_matches([' ', '\t', '\n', '\r']).len();
    text[last..]
        .find(['\n', '\r'])
        .map_or(text.len(), |offset| last + offset)
}

/// Returns the offset in `text` of the line ending that precedes the first
/// blank line, or `text.len()` when no blank line follows. The first line is
/// never blank, because it holds the construct that starts the scan.
fn paragraph_end(text: &str) -> usize {
    let mut offset = 0;
    while let Some(line_end) = text[offset..].find(['\n', '\r']) {
        let line_end = offset + line_end;
        let next_line = if text[line_end..].starts_with("\r\n") {
            line_end + 2
        } else {
            line_end + 1
        };
        let after = &text[next_line..];
        let line = after.split(['\n', '\r']).next().unwrap_or_default();
        if line.bytes().all(|byte| matches!(byte, b' ' | b'\t')) {
            return line_end;
        }
        offset = next_line;
    }
    text.len()
}
