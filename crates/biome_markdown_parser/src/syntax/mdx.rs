//! MDX constructs: ESM blocks, expressions, and JSX tags.
//!
//! The JavaScript in these constructs stays opaque here. Each construct
//! stores its code in a single literal token, and the workspace parses that
//! token as an embedded JavaScript snippet. This module only decides where
//! each construct ends, using [`JsExtent`] so that a `}`, a `>`, or a blank
//! line inside a string, a comment, a template literal, or JSX doesn't end
//! it early.
//!
//! JSX tags are flat: an opening tag and its closing tag are sibling nodes
//! around the Markdown between them, as the reference tokenizer emits them.
//! [`jsx_tag_pairing_diagnostics`] checks afterwards that the tags pair up.
//!
//! A line that holds only tags and expressions is a flow line: its
//! constructs are blocks, and the line can interrupt a paragraph. Tags and
//! expressions anywhere else are inline. The rules follow
//! [`micromark-extension-mdxjs`](https://github.com/micromark/micromark-extension-mdxjs),
//! the reference implementation of MDX 3.

use crate::MarkdownParser;
use crate::syntax::parse_error::{
    invalid_mdx_tag_name, invalid_mdx_tag_start, mdx_closing_tag_with_attributes,
    mdx_element_attribute_value, mdx_self_closing_closing_tag, unclosed_mdx_expression,
    unclosed_mdx_tag,
};
use biome_js_extent::{JsExtent, Jsx};
use biome_markdown_syntax::MarkdownSyntaxKind::*;
use biome_markdown_syntax::text_ext::blank_quote_markers;
use biome_markdown_syntax::{MarkdownSyntaxNode, T};
use biome_parser::Parser;
use biome_parser::diagnostic::ParseDiagnostic;
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

// #region Blocks

/// Returns whether the current line starts an ESM block or is a flow line.
pub(crate) fn at_mdx_block(p: &mut MarkdownParser) -> bool {
    at_esm_block(p) || at_mdx_flow_line(p)
}

/// Returns whether the current line holds only JSX tags and expressions,
/// separated by spaces and tabs. Such a line can interrupt a paragraph.
pub(crate) fn at_mdx_flow_line(p: &MarkdownParser) -> bool {
    p.is_at_line_start()
        && is_flow_line(
            p.source_after_current().trim_start_matches([' ', '\t']),
            p.state().block_quote_depth,
        )
}

/// Parses an ESM block, or every construct of a flow line.
///
/// A flow line produces one block per construct. The returned marker is the
/// last of them.
pub(crate) fn parse_mdx_block(p: &mut MarkdownParser) -> ParsedSyntax {
    if at_esm_block(p) {
        return parse_esm_block(p);
    }
    if !at_mdx_flow_line(p) {
        return Absent;
    }

    let mut last = None;
    let mut first = true;
    loop {
        let item = flow_item(
            p.source_after_current().trim_start_matches([' ', '\t']),
            p.state().block_quote_depth,
        );
        let block = match item {
            Some(FlowItem::Tag { length }) => parse_block_tag(p, length, first),
            Some(FlowItem::Expression { close }) => parse_block_expression(p, close, first),
            None => break,
        };
        first = false;
        match block {
            Present(marker) => last = Some(marker),
            Absent => break,
        }
        skip_spaces(p);
        if p.at(NEWLINE) || p.at(T![EOF]) {
            break;
        }
    }
    parse_line_ending(p).ok();
    last.map_or(Absent, Present)
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

/// A construct on a flow line.
#[derive(Debug, Clone, Copy)]
enum FlowItem {
    /// A JSX tag of `length` bytes.
    Tag { length: usize },
    /// An expression whose `}` is at `close` bytes from its `{`, or `None`
    /// when no `}` closes it before the end of the source.
    Expression { close: Option<usize> },
}

/// Returns the construct at the start of `text`, when it is a complete JSX
/// tag or an expression.
///
/// An expression on a flow line may span blank lines, so its `}` is searched
/// up to the end of the source. A tag must end before the next blank line.
fn flow_item(text: &str, quote_depth: usize) -> Option<FlowItem> {
    if let Some(body) = text.strip_prefix('{') {
        let length = JsExtent::expression_length(body.as_bytes(), JSX);
        let close = (length < body.len()).then_some(1 + length);
        return Some(FlowItem::Expression { close });
    }
    if !text.starts_with('<') || tag_start(text) != TagStart::Tag {
        return None;
    }
    let length = tag_length(text, quote_depth)?;
    Some(FlowItem::Tag { length })
}

/// Returns whether `text`, which starts at the first construct of a line,
/// holds only JSX tags and expressions separated by spaces and tabs, up to
/// the end of the line. An unclosed expression takes the rest of the source,
/// so it ends the line too.
///
/// An expression can only be followed by a tag on the same line: `{a} {b}`
/// is a paragraph with two inline expressions, while `<A>{b}</A>` is a flow
/// line.
fn is_flow_line(text: &str, quote_depth: usize) -> bool {
    let mut rest = text;
    let mut after_expression = false;
    loop {
        let item = flow_item(rest, quote_depth);
        if after_expression && !matches!(item, Some(FlowItem::Tag { .. })) {
            return false;
        }
        let length = match item {
            Some(FlowItem::Tag { length }) => {
                after_expression = false;
                length
            }
            Some(FlowItem::Expression { close: Some(close) }) => {
                after_expression = true;
                close + 1
            }
            Some(FlowItem::Expression { close: None }) => return true,
            None => return false,
        };
        rest = rest[length..].trim_start_matches([' ', '\t']);
        if rest.is_empty() || rest.starts_with(['\n', '\r']) {
            return true;
        }
    }
}

/// Parses a JSX tag on a flow line. Only the first construct of the line
/// owns the line's indentation.
///
/// Grammar: MdxJsxBlockTag = indent: MdIndentTokenList value: 'mdx_jsx_tag_literal'
fn parse_block_tag(p: &mut MarkdownParser, length: usize, first: bool) -> ParsedSyntax {
    let m = p.start();
    p.emit_line_indent(if first { usize::MAX } else { 0 });
    if !p.at(L_ANGLE) {
        m.abandon(p);
        return Absent;
    }
    bump_tag(p, length);
    Present(m.complete(p, MDX_JSX_BLOCK_TAG))
}

/// Parses an expression on a flow line. Only the first construct of the
/// line owns the line's indentation.
///
/// Grammar: MdxBlockExpression = indent: MdIndentTokenList '{' content: MdxExpressionContent? '}'
fn parse_block_expression(
    p: &mut MarkdownParser,
    close: Option<usize>,
    first: bool,
) -> ParsedSyntax {
    let m = p.start();
    p.emit_line_indent(if first { usize::MAX } else { 0 });
    if !p.at(T!['{']) {
        m.abandon(p);
        return Absent;
    }
    let open = p.cur_range().start();
    let close = close.map(|close| open + TextSize::from(close as u32));
    bump_expression_body(p, close, p.source_after_current().len());
    Present(m.complete(p, MDX_BLOCK_EXPRESSION))
}

// #endregion

// #region Inline constructs

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
    let limit = paragraph_end(rest, p.state().block_quote_depth > 0);
    let body = &rest[1..limit];
    let length = JsExtent::expression_length(body.as_bytes(), JSX);
    let close =
        (length < body.len()).then(|| p.cur_range().start() + TextSize::from((1 + length) as u32));
    bump_expression_body(p, close, limit);
    Present(m.complete(p, MDX_INLINE_EXPRESSION))
}

/// Parses a JSX tag inside a paragraph or heading.
///
/// Returns `Absent`, leaving the `<` to be parsed as text, when the `<` is
/// followed by a space or a line ending, as in `1 < 2`. A `<` followed by
/// any other character that can't start a tag, or a tag without its `>`,
/// is an error.
///
/// Grammar: MdxJsxInlineTag = value: 'mdx_jsx_tag_literal'
pub(crate) fn parse_mdx_inline_tag(p: &mut MarkdownParser) -> ParsedSyntax {
    if !p.at(L_ANGLE) {
        return Absent;
    }
    let rest = p.source_after_current();
    let opening = TextRange::at(p.cur_range().start(), TextSize::from(1));
    match tag_start(rest) {
        TagStart::Text => Absent,
        TagStart::Invalid { character } => {
            let comment = rest.starts_with("<!--");
            p.error(invalid_mdx_tag_start(p, opening, character, comment));
            Absent
        }
        TagStart::Tag => {
            let Some(length) = tag_length(rest, p.state().block_quote_depth) else {
                p.error(unclosed_mdx_tag(p, opening));
                return Absent;
            };
            let m = p.start();
            bump_tag(p, length);
            Present(m.complete(p, MDX_JSX_INLINE_TAG))
        }
    }
}

// #endregion

// #region Shared parts

/// Consumes the tag of `length` bytes at the current `<` as one literal,
/// reporting tag shapes that MDX rejects.
fn bump_tag(p: &mut MarkdownParser, length: usize) {
    let start = p.cur_range().start();
    let range = TextRange::at(start, TextSize::from(length as u32));
    let (tag_errors, element_value) = {
        let text = &p.source_after_current()[..length];
        let tag = JsxTag::from_text(text);
        (
            (
                tag.closing && tag.has_attributes,
                tag.closing && tag.self_closing,
                invalid_name_character(tag.name).map(|character| (character, tag.name.to_string())),
            ),
            element_attribute_value(text),
        )
    };
    let (attributes_on_closing, self_closing_closing, invalid_name) = tag_errors;
    if let Some((character, name)) = invalid_name {
        p.error(invalid_mdx_tag_name(p, range, character, &name));
    }
    if attributes_on_closing {
        p.error(mdx_closing_tag_with_attributes(p, range));
    }
    if self_closing_closing {
        p.error(mdx_self_closing_closing_tag(p, range));
    }
    if let Some(offset) = element_value {
        let value = TextRange::at(start + TextSize::from(offset as u32), TextSize::from(1));
        p.error(mdx_element_attribute_value(p, value));
    }
    p.re_lex_span(range.end(), MDX_JSX_TAG_LITERAL);
    p.bump(MDX_JSX_TAG_LITERAL);
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

/// Consumes spaces and tabs after a flow construct as whitespace trivia.
fn skip_spaces(p: &mut MarkdownParser) {
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
///
/// Inside a block quote, a line that holds only `>` markers is blank too.
fn paragraph_end(text: &str, in_quote: bool) -> usize {
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
        let is_blank = line
            .bytes()
            .all(|byte| matches!(byte, b' ' | b'\t') || (in_quote && byte == b'>'));
        if is_blank {
            return line_end;
        }
        offset = next_line;
    }
    text.len()
}

/// Returns the length of the JSX tag at the start of `text`, or `None` when
/// no `>` closes it before the end of its paragraph.
///
/// Inside block quotes, the `>` markers that start the tag's other lines
/// belong to the quote, so they're blanked before the tag is measured.
fn tag_length(text: &str, quote_depth: usize) -> Option<usize> {
    let limit = paragraph_end(text, quote_depth > 0);
    if quote_depth == 0 {
        return JsExtent::tag_length(&text.as_bytes()[..limit]);
    }
    let blanked = blank_quote_markers(&text[..limit], quote_depth);
    JsExtent::tag_length(blanked.as_bytes())
}

// #endregion

// #region JSX tags

/// How a `<` relates to JSX.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TagStart {
    /// The `<` starts a tag: it is followed by a name, `/`, or `>`.
    Tag,
    /// The `<` is text, because a space or a line ending follows it.
    Text,
    /// The `<` is followed by `character`, which can't start a tag.
    Invalid { character: char },
}

/// Classifies the `<` at the start of `text`, which starts with `<`.
fn tag_start(text: &str) -> TagStart {
    let Some(character) = text[1..].chars().next() else {
        return TagStart::Text;
    };
    match character {
        ' ' | '\t' | '\n' | '\r' => TagStart::Text,
        '/' | '>' | '$' | '_' => TagStart::Tag,
        _ if character.is_alphabetic() => TagStart::Tag,
        _ => TagStart::Invalid { character },
    }
}

/// The shape of a JSX tag, read from its source text.
#[derive(Debug)]
struct JsxTag<'a> {
    /// The tag name without whitespace, empty for a fragment.
    name: &'a str,
    /// Whether the tag starts with `</`.
    closing: bool,
    /// Whether the tag ends with `/>`.
    self_closing: bool,
    /// Whether anything other than the name sits between `<` and `>`.
    has_attributes: bool,
}

impl<'a> JsxTag<'a> {
    fn from_text(text: &'a str) -> Self {
        let inner = text.strip_prefix('<').unwrap_or(text);
        let inner = inner.strip_suffix('>').unwrap_or(inner);
        let (closing, inner) = match inner.strip_prefix('/') {
            Some(rest) => (true, rest),
            None => (false, inner),
        };
        let inner = inner.trim_start();
        let (self_closing, inner) = match inner.trim_end().strip_suffix('/') {
            Some(rest) => (true, rest),
            None => (false, inner),
        };
        let name_end = inner
            .find(|character: char| character.is_whitespace() || character == '{')
            .unwrap_or(inner.len());
        let name = &inner[..name_end];
        let has_attributes = !inner[name_end..].trim().is_empty();
        Self {
            name,
            closing,
            self_closing,
            has_attributes,
        }
    }

    /// Returns how the tag reads in a diagnostic, such as `<Note>`.
    fn display(&self) -> String {
        if self.closing {
            format!("</{}>", self.name)
        } else {
            format!("<{}>", self.name)
        }
    }
}

/// Returns the first character of `name` that can't appear in a JSX tag
/// name. A name is a JavaScript identifier that may contain `-`, followed by
/// either any number of `.member` parts or a single `:local` part. A
/// fragment has an empty name.
fn invalid_name_character(name: &str) -> Option<char> {
    if name.is_empty() {
        return None;
    }
    for part in name.split(['.', ':']) {
        let mut characters = part.chars();
        match characters.next() {
            // An empty part means a separator at the start, at the end, or
            // next to another separator.
            None => {
                return name
                    .chars()
                    .find(|character| matches!(character, '.' | ':'));
            }
            Some(first) if !is_name_start(first) => return Some(first),
            Some(_) => {}
        }
        if let Some(character) = characters.find(|character| !is_name_continue(*character)) {
            return Some(character);
        }
    }
    let mut separators = name
        .chars()
        .filter(|character| matches!(character, '.' | ':'));
    match separators.next() {
        Some(':') => separators.next(),
        Some(_) => separators.find(|character| *character == ':'),
        None => None,
    }
}

fn is_name_start(character: char) -> bool {
    character.is_alphabetic() || matches!(character, '$' | '_')
}

fn is_name_continue(character: char) -> bool {
    character.is_alphanumeric() || matches!(character, '$' | '_' | '-' | '\u{200c}' | '\u{200d}')
}

/// Returns the offset of the `<` in an attribute value written as an
/// element, such as `a=<b />`, which MDX doesn't support.
fn element_attribute_value(text: &str) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut index = 1;
    while let Some(&byte) = bytes.get(index) {
        match byte {
            b'"' | b'\'' => {
                let close = text[index + 1..].find(byte as char)?;
                index += close + 2;
            }
            b'{' => {
                let body = &bytes[index + 1..];
                index += JsExtent::expression_length(body, JSX) + 2;
            }
            b'=' => {
                let value = text[index + 1..].trim_start();
                if value.starts_with('<') {
                    return Some(text.len() - value.len());
                }
                index += 1;
            }
            _ => index += 1,
        }
    }
    None
}

/// Reports JSX tags that don't pair up in the tree rooted at `root`.
///
/// Tags pair up within one list of blocks or one paragraph, because an
/// element can't start in one block and end in another. A closing tag
/// without a matching open tag, and an open tag still open at the end of
/// its list, are errors.
pub(crate) fn jsx_tag_pairing_diagnostics(root: &MarkdownSyntaxNode) -> Vec<ParseDiagnostic> {
    let mut diagnostics = Vec::new();
    for list in root.descendants() {
        let tag_kind = match list.kind() {
            MD_BLOCK_LIST => MDX_JSX_BLOCK_TAG,
            MD_INLINE_ITEM_LIST => MDX_JSX_INLINE_TAG,
            _ => continue,
        };
        let mut open = Vec::new();
        for tag in list.children().filter(|child| child.kind() == tag_kind) {
            let text = tag.text_trimmed().to_string();
            let text = text.trim_start();
            let range = tag.text_trimmed_range();
            let tag = JsxTag::from_text(text);
            if tag.self_closing || invalid_name_character(tag.name).is_some() {
                continue;
            }
            if !tag.closing {
                open.push((tag.display(), range));
                continue;
            }
            let expected = format!("<{}>", tag.name);
            match open.last() {
                Some((opening, _)) if *opening == expected => {
                    open.pop();
                }
                Some((opening, opening_range)) => {
                    diagnostics.push(
                        ParseDiagnostic::new(
                            format!(
                                "Unexpected closing tag `{}`, expected the closing tag of `{opening}`.",
                                tag.display()
                            ),
                            range,
                        )
                        .with_detail(*opening_range, "the open tag is here"),
                    );
                    // A closing tag that matches an outer open tag closes the
                    // tags opened since then; any other one is ignored.
                    if let Some(index) = open.iter().rposition(|(opening, _)| *opening == expected)
                    {
                        open.truncate(index);
                    }
                }
                None => diagnostics.push(
                    ParseDiagnostic::new(
                        format!(
                            "Unexpected closing tag `{}`, there is no open tag to close.",
                            tag.display()
                        ),
                        range,
                    )
                    .with_hint("Remove the closing tag, or add its opening tag before it."),
                ),
            }
        }
        for (opening, range) in open {
            let place = if tag_kind == MDX_JSX_BLOCK_TAG {
                "its block"
            } else {
                "its paragraph"
            };
            diagnostics.push(
                ParseDiagnostic::new(
                    format!("Expected a closing tag for `{opening}` before the end of {place}."),
                    range,
                )
                .with_hint("Add the closing tag, or make the tag self-closing with `/>`."),
            );
        }
    }
    diagnostics.sort_by_key(|diagnostic| diagnostic.span().map(|span| span.start()));
    diagnostics
}

// #endregion
