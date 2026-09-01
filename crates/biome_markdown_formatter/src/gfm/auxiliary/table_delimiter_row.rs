use crate::gfm::auxiliary::{
    table::GfmTableLayout, table_delimiter_cell::FormatGfmTableDelimiterCellOptions,
};
use crate::markdown::auxiliary::quote_prefix::FormatMdQuotePrefixOptions;
use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{GfmTableDelimiterRow, GfmTableDelimiterRowFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGfmTableDelimiterRow;

/// Prepared column data and output policy for a table delimiter row.
///
/// `widths` must contain an entry for every delimiter cell. Delimiter rows fall
/// back to unaligned structured formatting when the widths do not cover all
/// source cells.
pub(crate) struct FormatGfmTableDelimiterRowOptions<'a> {
    /// Cell width of each column, excluding padding and pipes.
    pub(crate) widths: &'a [usize],
    /// How columns are padded.
    pub(crate) layout: GfmTableLayout,
    /// Whether to emit quote prefixes stored on the row.
    pub(crate) preserve_quote_prefixes: bool,
}

impl FormatNodeRule<GfmTableDelimiterRow> for FormatGfmTableDelimiterRow {
    fn fmt_fields(
        &self,
        node: &GfmTableDelimiterRow,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        format_gfm_table_delimiter_row(node, None, f)
    }
}

pub(crate) fn format_gfm_table_delimiter_row(
    node: &GfmTableDelimiterRow,
    options: Option<FormatGfmTableDelimiterRowOptions<'_>>,
    f: &mut MarkdownFormatter,
) -> FormatResult<()> {
        let GfmTableDelimiterRowFields {
            quote_prefixes,
            l_pipe_token,
            cells,
            r_pipe_token,
            newline_token,
        } = node.as_fields();
        let cell_count = cells.elements().count();
        let options = options.filter(|options| options.widths.len() >= cell_count);

        for prefix in quote_prefixes.iter() {
            write!(
                f,
                [prefix.format().with_options(FormatMdQuotePrefixOptions {
                    should_remove: options
                        .as_ref()
                        .is_some_and(|options| !options.preserve_quote_prefixes),
                })]
            )?;
        }
        if let Some(pipe) = l_pipe_token {
            write!(f, [format_replaced(&pipe, &token("|"))])?;
        } else {
            write!(f, [token("|")])?;
        }
        if let Some(options) = options {
            for (element, width) in cells.elements().zip(options.widths.iter().copied()) {
                write!(
                    f,
                    [element
                        .node()?
                        .format()
                        .with_options(FormatGfmTableDelimiterCellOptions {
                            width,
                            layout: options.layout,
                        })]
                )?;
                if let Some(pipe) = element.trailing_separator()? {
                    write!(f, [format_replaced(pipe, &token("|"))])?;
                }
            }
        } else {
            write!(f, [cells.format()])?;
        }
        if let Some(pipe) = r_pipe_token {
            write!(f, [format_replaced(&pipe, &token("|"))])?;
        } else {
            write!(f, [token("|")])?;
        }
        if let Some(newline) = newline_token {
            write!(f, [format_removed(&newline)])?;
        }
        Ok(())
    }
