use super::link_block::at_link_block;
use super::list::{at_bullet_list_item, at_order_list_item};
use super::quote::consume_quote_prefix;
use super::{
    INDENT_CODE_BLOCK_SPACES, MAX_BLOCK_PREFIX_INDENT, TAB_STOP_SPACES, at_block_interrupt,
    at_indent_code_block,
};
use crate::MarkdownParser;
use crate::parser::DeferredInlineFlavor;
use crate::syntax::html_block::at_html_block;
use biome_markdown_syntax::T;
use biome_markdown_syntax::kind::MarkdownSyntaxKind::*;
use biome_parser::{
    Parser,
    prelude::ParsedSyntax::{self, *},
};
use biome_rowan::TextSize;

#[derive(Debug)]
struct TableRowInfo {
    cells: Vec<TableCellRange>,
    leading_pipe: bool,
    trailing_pipe: bool,
}

#[derive(Debug)]
struct TableCellRange {
    content_start: TextSize,
    content_end: TextSize,
}

pub(crate) fn at_gfm_table(p: &MarkdownParser) -> bool {
    gfm_table_info(p, false).is_some()
}

pub(crate) fn at_gfm_table_with_container_prefix(p: &MarkdownParser) -> bool {
    gfm_table_info(p, true).is_some()
}

fn gfm_table_info(
    p: &MarkdownParser,
    header_has_container_prefix: bool,
) -> Option<(TableRowInfo, TableRowInfo)> {
    let (source, start) = table_source(p, header_has_container_prefix)?;
    let header_line = first_source_line(source);
    if header_line.ending_len == 0 {
        return None;
    }
    let rest = source.get(header_line.next_offset()..)?;
    let delimiter_line = first_source_line(rest);

    let (header_content, header_prefix_len) = if header_has_container_prefix {
        table_container_content(p, header_line.content)?
    } else {
        (header_line.content, 0)
    };
    let header_start = start.checked_add(TextSize::try_from(header_prefix_len).ok()?)?;
    let header = table_row_info(header_content, header_start)?;
    let delimiter_line_offset = TextSize::try_from(header_line.next_offset()).ok()?;
    let delimiter_line_start = start.checked_add(delimiter_line_offset)?;
    let (delimiter_content, delimiter_prefix_len) =
        table_container_content(p, delimiter_line.content)?;
    let delimiter_prefix_len = TextSize::try_from(delimiter_prefix_len).ok()?;
    let delimiter_start = delimiter_line_start.checked_add(delimiter_prefix_len)?;
    let delimiter = table_row_info(delimiter_content, delimiter_start)?;

    (has_unescaped_pipe(header_content)
        && header.cells.len() == delimiter.cells.len()
        && delimiter.cells.iter().all(|cell| {
            let Some(start) = cell.content_start.checked_sub(delimiter_start) else {
                return false;
            };
            let Some(end) = cell.content_end.checked_sub(delimiter_start) else {
                return false;
            };
            delimiter_content
                .get(usize::from(start)..usize::from(end))
                .is_some_and(is_table_delimiter_cell)
        }))
    .then_some((header, delimiter))
}

pub(crate) fn parse_gfm_table(p: &mut MarkdownParser) -> ParsedSyntax {
    let Some((header, delimiter)) = gfm_table_info(p, false) else {
        return Absent;
    };

    let m = p.start();
    parse_table_row(p, &header, false).ok();

    parse_table_delimiter_row(p, &delimiter).ok();

    let body = p.start();
    while at_table_body_row(p) {
        let Some(row) = table_row_info_from_parser(p, true) else {
            break;
        };
        parse_table_row(p, &row, true).ok();
    }
    body.complete(p, GFM_TABLE_ROW_LIST);

    Present(m.complete(p, GFM_TABLE))
}

fn table_row_info_from_parser(
    p: &MarkdownParser,
    has_container_prefix: bool,
) -> Option<TableRowInfo> {
    let (source, start) = table_source(p, has_container_prefix)?;
    let line = first_source_line(source);
    if !has_container_prefix {
        return table_row_info(line.content, start);
    }

    let (content, prefix_len) = table_container_content(p, line.content)?;
    table_row_info(
        content,
        start.checked_add(TextSize::try_from(prefix_len).ok()?)?,
    )
}

fn at_table_body_row(p: &mut MarkdownParser) -> bool {
    if p.at(T![EOF]) {
        return false;
    }

    let Some((source, _)) = table_source(p, true) else {
        return false;
    };
    let line = first_source_line(source).content;
    let Some((content, _)) = table_container_content(p, line) else {
        return false;
    };
    if content.trim_matches([' ', '\t']).is_empty() {
        return false;
    }

    !starts_table_terminating_block(p, content)
}

fn parse_table_row(
    p: &mut MarkdownParser,
    row: &TableRowInfo,
    has_container_prefix: bool,
) -> ParsedSyntax {
    let m = p.start();
    parse_table_quote_prefixes(p, has_container_prefix);
    p.enter_table_lex_mode();
    if row.leading_pipe {
        expect_table_pipe(p);
    }

    let cells = p.start();
    for (index, cell) in row.cells.iter().enumerate() {
        parse_table_cell(p, cell).ok();
        if index + 1 < row.cells.len() {
            expect_table_pipe(p);
        }
    }
    cells.complete(p, GFM_TABLE_CELL_LIST);

    if row.trailing_pipe && !p.at(NEWLINE) {
        expect_table_pipe(p);
    }
    p.leave_table_lex_mode();
    p.eat(NEWLINE);
    Present(m.complete(p, GFM_TABLE_ROW))
}

fn parse_table_delimiter_row(p: &mut MarkdownParser, row: &TableRowInfo) -> ParsedSyntax {
    let m = p.start();
    parse_table_quote_prefixes(p, true);
    p.enter_table_lex_mode();
    if row.leading_pipe {
        expect_table_pipe(p);
    }

    let cells = p.start();
    for (index, cell) in row.cells.iter().enumerate() {
        parse_table_delimiter_cell(p, cell).ok();
        if index + 1 < row.cells.len() {
            expect_table_pipe(p);
        }
    }
    cells.complete(p, GFM_TABLE_DELIMITER_CELL_LIST);

    if row.trailing_pipe && !p.at(NEWLINE) {
        expect_table_pipe(p);
    }
    p.leave_table_lex_mode();
    p.eat(NEWLINE);
    Present(m.complete(p, GFM_TABLE_DELIMITER_ROW))
}

fn parse_table_quote_prefixes(p: &mut MarkdownParser, has_container_prefix: bool) {
    let prefixes = p.start();
    if has_container_prefix && p.state().block_quote_depth > 0 {
        let depth = p.state().block_quote_depth;
        let consumed = consume_quote_prefix(p, depth);
        debug_assert!(consumed);
    }
    prefixes.complete(p, MD_QUOTE_PREFIX_LIST);
}

fn starts_table_terminating_block(p: &MarkdownParser, content: &str) -> bool {
    let mut probe = MarkdownParser::new(content, p.options().clone());
    at_indent_code_block(&mut probe)
        || at_block_interrupt(&mut probe)
        || at_bullet_list_item(&mut probe)
        || at_order_list_item(&mut probe)
        || at_html_block(&mut probe)
        || at_link_block(&mut probe)
}

fn table_container_content<'source>(
    p: &MarkdownParser,
    line: &'source str,
) -> Option<(&'source str, usize)> {
    let quote_depth = p.state().block_quote_depth;
    let required_indent = p.state().list_item_required_indent;

    if let Some(indent_len) = strip_required_indent(line, required_indent)
        && let Some(after_indent) = line.get(indent_len..)
        && let Some(quote_len) = strip_quote_prefixes(after_indent, quote_depth)
        && let Some(prefix_len) = indent_len.checked_add(quote_len)
        && let Some(content) = line.get(prefix_len..)
    {
        return Some((content, prefix_len));
    }

    let quote_len = strip_quote_prefixes(line, quote_depth)?;
    let indent_len = strip_required_indent(line.get(quote_len..)?, required_indent)?;
    let prefix_len = quote_len.checked_add(indent_len)?;
    Some((line.get(prefix_len..)?, prefix_len))
}

fn table_source<'parser>(
    p: &'parser MarkdownParser,
    include_container_prefix: bool,
) -> Option<(&'parser str, TextSize)> {
    if !include_container_prefix {
        return Some((p.source_after_current(), p.cur_range().start()));
    }

    let source = p.source().source_text();
    let current_start = usize::from(p.cur_range().start());
    let before_current = source.get(..current_start)?;
    let line_start = before_current
        .rfind(['\n', '\r'])
        .map_or(0, |index| index + 1);
    Some((
        source.get(line_start..)?,
        TextSize::try_from(line_start).ok()?,
    ))
}

fn strip_required_indent(line: &str, required_indent: usize) -> Option<usize> {
    let mut byte_offset = 0usize;
    let mut column = 0usize;
    while column < required_indent {
        match line.as_bytes().get(byte_offset)? {
            b' ' => column += 1,
            b'\t' => column += TAB_STOP_SPACES - (column % TAB_STOP_SPACES),
            _ => return None,
        }
        byte_offset += 1;
    }
    Some(byte_offset)
}

fn strip_quote_prefixes(line: &str, depth: usize) -> Option<usize> {
    let mut byte_offset = 0usize;
    for _ in 0..depth {
        let mut column = 0usize;
        while let Some(byte) = line.as_bytes().get(byte_offset) {
            let next_column = match byte {
                b' ' => column + 1,
                b'\t' => column + (TAB_STOP_SPACES - (column % TAB_STOP_SPACES)),
                _ => break,
            };
            if next_column > MAX_BLOCK_PREFIX_INDENT {
                break;
            }
            column = next_column;
            byte_offset += 1;
        }

        if line.as_bytes().get(byte_offset) != Some(&b'>') {
            return None;
        }
        byte_offset += 1;
        if matches!(line.as_bytes().get(byte_offset), Some(b' ' | b'\t')) {
            byte_offset += 1;
        }
    }
    Some(byte_offset)
}

fn expect_table_pipe(p: &mut MarkdownParser) {
    p.expect(PIPE);
}

fn parse_table_cell(p: &mut MarkdownParser, cell: &TableCellRange) -> ParsedSyntax {
    let m = p.start();

    if cell.content_start < cell.content_end {
        debug_assert_eq!(p.cur_range().start(), cell.content_start);
        let deferred = p.start_deferred_inline(DeferredInlineFlavor::TableCell);
        let content = p.start();
        p.re_lex_span(cell.content_end, MD_TEXTUAL_LITERAL);
        let text = p.start();
        p.bump_remap(MD_TEXTUAL_LITERAL);
        text.complete(p, MD_TEXTUAL);
        content.complete(p, MD_INLINE_ITEM_LIST);
        p.finish_deferred_inline_at(deferred, cell.content_end);
    } else {
        let content = p.start();
        content.complete(p, MD_INLINE_ITEM_LIST);
    }

    Present(m.complete(p, GFM_TABLE_CELL))
}

fn parse_table_delimiter_cell(p: &mut MarkdownParser, _: &TableCellRange) -> ParsedSyntax {
    let m = p.start();

    p.eat(COLON);
    let dashes = p.start();
    while p.at(MINUS) {
        let dash = p.start();
        p.bump(MINUS);
        dash.complete(p, GFM_TABLE_DELIMITER_DASH);
    }
    dashes.complete(p, GFM_TABLE_DELIMITER_DASH_LIST);
    p.eat(COLON);

    Present(m.complete(p, GFM_TABLE_DELIMITER_CELL))
}

fn table_row_info(line: &str, start: TextSize) -> Option<TableRowInfo> {
    if leading_indent_columns(line) >= INDENT_CODE_BLOCK_SPACES {
        return None;
    }

    let bytes = line.as_bytes();
    let mut content_start = 0usize;
    let mut content_end = bytes.len();
    while content_start < content_end && matches!(bytes.get(content_start), Some(b' ' | b'\t')) {
        content_start += 1;
    }
    while content_end > content_start && matches!(bytes.get(content_end - 1), Some(b' ' | b'\t')) {
        content_end -= 1;
    }

    let leading_pipe = bytes.get(content_start) == Some(&b'|');
    if leading_pipe {
        content_start += 1;
    }
    let trailing_pipe = content_end > content_start
        && bytes.get(content_end - 1) == Some(&b'|')
        && !is_escaped(bytes, content_end - 1);
    if trailing_pipe {
        content_end -= 1;
    }

    let mut boundaries = vec![content_start];
    let mut index = content_start;
    while index < content_end {
        if bytes.get(index) == Some(&b'|') && !is_escaped(bytes, index) {
            boundaries.push(index + 1);
        }
        index += 1;
    }
    boundaries.push(content_end.checked_add(1)?);

    let mut cells = Vec::with_capacity(boundaries.len().saturating_sub(1));
    for pair in boundaries.windows(2) {
        let [cell_start, next_start] = pair else {
            return None;
        };
        let cell_end = next_start.saturating_sub(1);
        let cell = line.get(*cell_start..cell_end)?;
        let content = cell.trim_matches([' ', '\t']);
        let leading_whitespace = cell
            .len()
            .checked_sub(cell.trim_start_matches([' ', '\t']).len())?;
        let content_start = cell_start.checked_add(leading_whitespace)?;
        let content_end = content_start.checked_add(content.len())?;
        cells.push(TableCellRange {
            content_start: start.checked_add(TextSize::try_from(content_start).ok()?)?,
            content_end: start.checked_add(TextSize::try_from(content_end).ok()?)?,
        });
    }

    Some(TableRowInfo {
        cells,
        leading_pipe,
        trailing_pipe,
    })
}

fn leading_indent_columns(line: &str) -> usize {
    let mut columns = 0usize;
    for byte in line.bytes() {
        match byte {
            b' ' => columns += 1,
            b'\t' => columns += TAB_STOP_SPACES - (columns % TAB_STOP_SPACES),
            _ => break,
        }
    }
    columns
}

fn is_escaped(bytes: &[u8], index: usize) -> bool {
    let Some(before) = bytes.get(..index) else {
        return false;
    };
    before
        .iter()
        .rev()
        .take_while(|byte| **byte == b'\\')
        .count()
        % 2
        == 1
}

fn has_unescaped_pipe(line: &str) -> bool {
    let bytes = line.as_bytes();
    bytes
        .iter()
        .enumerate()
        .any(|(index, byte)| *byte == b'|' && !is_escaped(bytes, index))
}

fn is_table_delimiter_cell(cell: &str) -> bool {
    let cell = cell.trim_matches([' ', '\t']);
    let cell = cell.strip_prefix(':').unwrap_or(cell);
    let cell = cell.strip_suffix(':').unwrap_or(cell);
    !cell.is_empty() && cell.bytes().all(|byte| byte == b'-')
}

struct SourceLine<'source> {
    content: &'source str,
    ending_len: usize,
}

impl SourceLine<'_> {
    fn next_offset(&self) -> usize {
        self.content.len() + self.ending_len
    }
}

fn first_source_line(source: &str) -> SourceLine<'_> {
    let Some(index) = source
        .bytes()
        .position(|byte| matches!(byte, b'\n' | b'\r'))
    else {
        return SourceLine {
            content: source,
            ending_len: 0,
        };
    };
    let ending_len = if source.as_bytes().get(index) == Some(&b'\r')
        && source.as_bytes().get(index + 1) == Some(&b'\n')
    {
        2
    } else {
        1
    };
    let Some(content) = source.get(..index) else {
        return SourceLine {
            content: source,
            ending_len: 0,
        };
    };
    SourceLine {
        content,
        ending_len,
    }
}
