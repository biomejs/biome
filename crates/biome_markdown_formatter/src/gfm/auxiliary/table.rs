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
        table_delimiter_row::FormatGfmTableDelimiterRowOptions, table_row::FormatGfmTableRowOptions,
    },
};
use biome_formatter::{FormatOptions, GroupId, printer::Printer, write};
use biome_markdown_syntax::{GfmTable, GfmTableDelimiterRow, GfmTableRow};
use std::rc::Rc;
use unicode_width::UnicodeWidthStr;

/// Minimum cell width in display columns, excluding padding and pipes.
pub(crate) const MIN_GFM_TABLE_CELL_WIDTH: usize = 3;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGfmTable;

/// Formatted table-cell content cached for measurement and final emission.
#[derive(Clone, Debug)]
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
#[derive(Clone)]
struct PreparedGfmTable {
    /// Preformatted header cells in source order.
    header: Rc<[CachedGfmTableCell]>,
    /// Preformatted body cells grouped by source row.
    body: Vec<Rc<[CachedGfmTableCell]>>,
    /// Maximum cell width required by each column, excluding padding and pipes.
    widths: Rc<[usize]>,
    /// Alignment requested by each delimiter cell.
    alignments: Rc<[GfmTableAlignment]>,
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
            header: header.into(),
            body: body.into_iter().map(Rc::from).collect(),
            widths: widths.into(),
            alignments: alignments.into(),
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

    fn header_row_options(
        &self,
        layout: GfmTableLayout,
        preserve_quote_prefixes: bool,
    ) -> FormatGfmTableRowOptions {
        FormatGfmTableRowOptions {
            cells: Rc::clone(&self.header),
            widths: Rc::clone(&self.widths),
            alignments: Rc::clone(&self.alignments),
            layout,
            preserve_quote_prefixes,
        }
    }

    fn body_row_options(
        &self,
        cells: &Rc<[CachedGfmTableCell]>,
        layout: GfmTableLayout,
        preserve_quote_prefixes: bool,
    ) -> FormatGfmTableRowOptions {
        FormatGfmTableRowOptions {
            cells: Rc::clone(cells),
            widths: Rc::clone(&self.widths),
            alignments: Rc::clone(&self.alignments),
            layout,
            preserve_quote_prefixes,
        }
    }

    fn delimiter_options(
        &self,
        layout: GfmTableLayout,
        preserve_quote_prefixes: bool,
    ) -> FormatGfmTableDelimiterRowOptions {
        FormatGfmTableDelimiterRowOptions {
            widths: Rc::clone(&self.widths),
            layout,
            preserve_quote_prefixes,
        }
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

        let body_rows = body.iter().collect::<Vec<_>>();
        let table = PreparedGfmTable::build(&header, &delimiter, body_rows.clone().into_iter(), f)?;

        let prose_wrap = f.options().prose_wrap();
        let preserve_quote_prefixes = prose_wrap == ProseWrap::Preserve;
        let layout = if prose_wrap == ProseWrap::Never {
            GfmTableLayout::CompactWhenBroken(f.group_id("gfmTable"))
        } else {
            GfmTableLayout::Aligned
        };

        let content = format_with(|f| {
            let header = header
                .format()
                .with_options(table.header_row_options(layout, preserve_quote_prefixes));
            match layout {
                GfmTableLayout::Aligned => write!(f, [header])?,
                GfmTableLayout::CompactWhenBroken(group_id) => {
                    write!(f, [group(&header).with_group_id(Some(group_id))])?
                }
            }
            write!(f, [hard_line_break()])?;
            write!(
                f,
                [delimiter
                    .format()
                    .with_options(table.delimiter_options(layout, preserve_quote_prefixes))]
            )?;

            for (row, cells) in body_rows.iter().zip(&table.body) {
                write!(f, [hard_line_break()])?;
                write!(
                    f,
                    [row.format().with_options(table.body_row_options(
                        cells,
                        layout,
                        preserve_quote_prefixes,
                    ))]
                )?;
            }

            Ok(())
        });

        write!(f, [expand_parent()])?;
        write!(f, [content])?;
        write!(f, [hard_line_break()])
    }
}
