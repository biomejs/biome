//! Shared layout computation for GFM table rows.
//!
//! Cell contents are formatted and interned before any row is emitted. The
//! resulting display widths and delimiter alignments are then shared by the
//! header, delimiter, and body row formatters, so every row uses the same
//! column layout.

use crate::prelude::*;
use crate::{
    context::ProseWrap,
    gfm::auxiliary::{
        table_delimiter_row::{
            FormatGfmTableDelimiterRowOptions, FormatGfmTableDelimiterRowWithOptions,
        },
        table_row::{FormatGfmTableRowOptions, FormatGfmTableRowWithOptions},
    },
    quote::{quote_line_prefix, should_format_quote_structurally},
};
use biome_formatter::{FormatOptions, GroupId, format_args, printer::Printer, write};
use biome_markdown_syntax::{GfmTable, GfmTableDelimiterRow, GfmTableRow, MdBullet, MdQuote};
use unicode_width::UnicodeWidthStr;

/// Minimum cell width in display columns, excluding padding and pipes.
pub(crate) const MIN_GFM_TABLE_CELL_WIDTH: usize = 3;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGfmTable;

/// Formatted table-cell content cached for measurement and final emission.
#[derive(Debug)]
pub(crate) struct CachedGfmTableCell {
    /// The interned content, or `None` when the cell is empty.
    pub(crate) content: Option<FormatElement>,
    /// The number of Unicode display columns occupied by the printed content.
    pub(crate) width: usize,
}

/// The alignment encoded by a delimiter cell's optional colon markers.
#[derive(Clone, Copy, Debug)]
pub(crate) enum GfmTableAlignment {
    /// No colon markers and therefore no explicit alignment.
    Default,
    /// A leading colon aligns content to the left.
    Left,
    /// Leading and trailing colons center the content.
    Center,
    /// A trailing colon aligns content to the right.
    Right,
}

/// Controls whether computed column padding is retained when a table breaks.
#[derive(Clone, Copy, Debug)]
pub(crate) enum GfmTableLayout {
    /// Always emits enough padding to align every column.
    Aligned,
    /// Emits alignment padding only while the associated group fits on one line.
    CompactWhenBroken(GroupId),
}

/// Preformatted cells and computed column metadata for one table.
///
/// Every computed width is at least [`MIN_GFM_TABLE_CELL_WIDTH`], so an
/// unaligned delimiter cell contains at least three dashes.
struct PreparedGfmTable {
    /// Preformatted header cells in source order.
    header: Vec<CachedGfmTableCell>,
    /// Preformatted body cells grouped by source row.
    body: Vec<Vec<CachedGfmTableCell>>,
    /// Maximum cell width required by each column, excluding padding and pipes.
    widths: Vec<usize>,
    /// Alignment requested by each delimiter cell.
    alignments: Vec<GfmTableAlignment>,
}

impl PreparedGfmTable {
    fn build(
        header: &GfmTableRow,
        delimiter: &GfmTableDelimiterRow,
        body: impl Iterator<Item = GfmTableRow>,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<Self> {
        let header = Self::cache_row(header, f)?;
        let body = body
            .map(|row| Self::cache_row(&row, f))
            .collect::<FormatResult<Vec<_>>>()?;

        let mut widths = vec![
            MIN_GFM_TABLE_CELL_WIDTH;
            std::iter::once(&header)
                .chain(&body)
                .map(Vec::len)
                .max()
                .unwrap_or_default()
        ];
        for row in std::iter::once(&header).chain(&body) {
            for (width, cell) in widths.iter_mut().zip(row) {
                *width = (*width).max(cell.width);
            }
        }

        let alignments = delimiter
            .cells()
            .iter()
            .map(|cell| {
                let cell = cell?;
                Ok(
                    match (
                        cell.l_colon_token().is_some(),
                        cell.r_colon_token().is_some(),
                    ) {
                        (true, true) => GfmTableAlignment::Center,
                        (true, false) => GfmTableAlignment::Left,
                        (false, true) => GfmTableAlignment::Right,
                        (false, false) => GfmTableAlignment::Default,
                    },
                )
            })
            .collect::<FormatResult<Vec<_>>>()?;

        Ok(Self {
            header,
            body,
            widths,
            alignments,
        })
    }

    fn cache_row(
        row: &GfmTableRow,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<Vec<CachedGfmTableCell>> {
        row.cells()
            .iter()
            .map(|cell| {
                let cell = cell?;
                let content = f.intern(&cell.format())?;
                let document = Document::new(content.clone().into_iter().collect());
                let printed = Printer::new(f.options().as_print_options()).print(&document)?;
                let text = printed.as_code();

                Ok(CachedGfmTableCell {
                    content,
                    width: UnicodeWidthStr::width(text),
                })
            })
            .collect()
    }
}

impl FormatNodeRule<GfmTable> for FormatGfmTable {
    fn fmt_fields(&self, node: &GfmTable, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let fields = node.as_fields();
        let header = fields.header?;
        let delimiter = fields.delimiter?;
        let body = fields.body;

        if node.syntax().descendants().any(|descendant| {
            f.context().comments().is_suppressed(&descendant)
                || f.context().comments().is_global_suppressed(&descendant)
        }) {
            return write!(f, [format_suppressed_node(node.syntax())]);
        }

        let table = PreparedGfmTable::build(&header, &delimiter, body.iter(), f)?;

        let prose_wrap = f.options().prose_wrap();
        let preserve_quote_prefixes = prose_wrap == ProseWrap::Preserve;
        let layout = if prose_wrap == ProseWrap::Never {
            GfmTableLayout::CompactWhenBroken(f.group_id("gfmTable"))
        } else {
            GfmTableLayout::Aligned
        };

        let line_prefix = quote_line_prefix(node.syntax())?;
        // When set, every row after the header that has quote prefixes in the
        // source starts with the prefix of the quotes and list items that
        // enclose the table, printed from the start of the line, and the row's
        // own quote prefixes are removed.
        let quote_prefix = (preserve_quote_prefixes
            && !line_prefix.is_empty()
            && cannot_reuse_source_quote_prefixes(node)?)
        .then_some(&line_prefix);

        let content = format_with(|f| {
            let header = FormatGfmTableRowWithOptions::new(
                &header,
                Some(FormatGfmTableRowOptions {
                    cells: &table.header,
                    widths: &table.widths,
                    alignments: &table.alignments,
                    layout,
                    preserve_quote_prefixes,
                }),
            );
            match layout {
                GfmTableLayout::Aligned => write!(f, [header])?,
                GfmTableLayout::CompactWhenBroken(group_id) => {
                    write!(f, [group(&header).with_group_id(Some(group_id))])?
                }
            }
            let delimiter_quote_prefix =
                quote_prefix.filter(|_| !delimiter.quote_prefixes().is_empty());
            if let Some(prefix) = delimiter_quote_prefix {
                write!(
                    f,
                    [dedent_to_root(&format_args![
                        hard_line_break(),
                        prefix.format(true)
                    ])]
                )?;
            } else {
                write!(f, [hard_line_break()])?;
            }
            write!(
                f,
                [FormatGfmTableDelimiterRowWithOptions::new(
                    &delimiter,
                    Some(FormatGfmTableDelimiterRowOptions {
                        widths: &table.widths,
                        layout,
                        preserve_quote_prefixes: preserve_quote_prefixes
                            && delimiter_quote_prefix.is_none(),
                    }),
                )]
            )?;

            for (row, cells) in body.iter().zip(&table.body) {
                let row_quote_prefix = quote_prefix.filter(|_| !row.quote_prefixes().is_empty());
                if let Some(prefix) = row_quote_prefix {
                    write!(
                        f,
                        [dedent_to_root(&format_args![
                            hard_line_break(),
                            prefix.format(true)
                        ])]
                    )?;
                } else {
                    write!(f, [hard_line_break()])?;
                }
                write!(
                    f,
                    [FormatGfmTableRowWithOptions::new(
                        &row,
                        Some(FormatGfmTableRowOptions {
                            cells,
                            widths: &table.widths,
                            alignments: &table.alignments,
                            layout,
                            preserve_quote_prefixes: preserve_quote_prefixes
                                && row_quote_prefix.is_none(),
                        }),
                    )]
                )?;
            }

            Ok(())
        });

        write!(f, [expand_parent()])?;
        write!(f, [content])?;
        write!(f, [hard_line_break()])
    }
}

/// Returns whether printing the quote prefixes stored on the table's rows
/// would misplace them.
///
/// This is the case when a list item encloses the table, since the list item
/// indents its content, and when the list item is inside the quote, the
/// indentation that it requires is stored as trivia of the row's first pipe.
/// It is also the case when an enclosing quote is formatted structurally, since
/// it indents its content as well. Otherwise the stored prefixes are kept, so
/// that every row uses the same prefix as the header line, which the quote
/// prints from the source.
fn cannot_reuse_source_quote_prefixes(table: &GfmTable) -> FormatResult<bool> {
    for ancestor in table.syntax().ancestors() {
        if MdBullet::can_cast(ancestor.kind()) {
            return Ok(true);
        }
        if let Some(quote) = MdQuote::cast(ancestor)
            && should_format_quote_structurally(&quote)?
        {
            return Ok(true);
        }
    }
    Ok(false)
}
