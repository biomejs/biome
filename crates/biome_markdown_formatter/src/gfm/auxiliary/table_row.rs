use crate::gfm::auxiliary::table::{CachedGfmTableCell, GfmTableAlignment, GfmTableLayout};
use crate::markdown::auxiliary::quote_prefix::FormatMdQuotePrefixOptions;
use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{GfmTableRow, GfmTableRowFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGfmTableRow;

/// Prepared column data and output policy for formatting one table row.
///
/// `cells` must correspond one-to-one with the row's source cells, and `widths`
/// must contain an entry for every source cell. Rows fall back to unaligned
/// structured formatting when either condition is not met.
pub(crate) struct FormatGfmTableRowOptions<'a> {
    /// Preformatted cells in source order.
    pub(crate) cells: &'a [CachedGfmTableCell],
    /// Cell width of each column, excluding padding and pipes.
    pub(crate) widths: &'a [usize],
    /// Alignment encoded by each delimiter cell.
    pub(crate) alignments: &'a [GfmTableAlignment],
    /// How columns are padded.
    pub(crate) layout: GfmTableLayout,
    /// Whether to emit quote prefixes stored on the row.
    pub(crate) preserve_quote_prefixes: bool,
}

impl FormatNodeRule<GfmTableRow> for FormatGfmTableRow {
    fn fmt_fields(&self, node: &GfmTableRow, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_gfm_table_row(node, None, f)
    }
}

pub(crate) fn format_gfm_table_row(
    node: &GfmTableRow,
    options: Option<FormatGfmTableRowOptions<'_>>,
    f: &mut MarkdownFormatter,
) -> FormatResult<()> {
        let GfmTableRowFields {
            quote_prefixes,
            l_pipe_token,
            cells,
            r_pipe_token,
            newline_token,
        } = node.as_fields();
        let cell_count = cells.elements().count();
        let options = options.filter(|options| {
            options.cells.len() == cell_count && options.widths.len() >= cell_count
        });

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
            for (index, ((element, cell), width)) in cells
                .elements()
                .zip(options.cells.iter())
                .zip(options.widths.iter().copied())
                .enumerate()
            {
                let spaces = width.saturating_sub(cell.width);
                let before = match options
                    .alignments
                    .get(index)
                    .copied()
                    .unwrap_or(GfmTableAlignment::Default)
                {
                    GfmTableAlignment::Right => spaces,
                    GfmTableAlignment::Center => spaces / 2,
                    GfmTableAlignment::Default | GfmTableAlignment::Left => 0,
                };

                match options.layout {
                    GfmTableLayout::Aligned => {
                        let padding = " ".repeat(before + 1);
                        write!(f, [text(&padding, None)])?;
                    }
                    GfmTableLayout::CompactWhenBroken(group_id) => {
                        write!(f, [text(" ", None)])?;
                        if before > 0 {
                            let padding = " ".repeat(before);
                            write!(
                                f,
                                [if_group_fits_on_line(&text(&padding, None))
                                    .with_group_id(Some(group_id))]
                            )?;
                        }
                    }
                }
                if let Some(content) = &cell.content {
                    f.write_element(content.clone())?;
                }
                match options.layout {
                    GfmTableLayout::Aligned => {
                        let padding = " ".repeat(spaces.saturating_sub(before) + 1);
                        write!(f, [text(&padding, None)])?;
                    }
                    GfmTableLayout::CompactWhenBroken(group_id) => {
                        write!(f, [text(" ", None)])?;
                        if spaces > before {
                            let padding = " ".repeat(spaces.saturating_sub(before));
                            write!(
                                f,
                                [if_group_fits_on_line(&text(&padding, None))
                                    .with_group_id(Some(group_id))]
                            )?;
                        }
                    }
                }
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
