use crate::gfm::auxiliary::{
    table::GfmTableLayout, table_delimiter_cell::FormatGfmTableDelimiterCellOptions,
};
use crate::markdown::auxiliary::quote_prefix::FormatMdQuotePrefixOptions;
use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{GfmTableDelimiterRow, GfmTableDelimiterRowFields};
use std::rc::Rc;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGfmTableDelimiterRow {
    options: Option<FormatGfmTableDelimiterRowOptions>,
}

/// Prepared column data and output policy for a table delimiter row.
///
/// `widths` must contain an entry for every delimiter cell. Delimiter rows fall
/// back to unaligned structured formatting when the widths do not cover all
/// source cells.
#[derive(Clone, Debug)]
pub(crate) struct FormatGfmTableDelimiterRowOptions {
    /// Cell width of each column, excluding padding and pipes.
    pub(crate) widths: Rc<[usize]>,
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
        let GfmTableDelimiterRowFields {
            quote_prefixes,
            l_pipe_token,
            cells,
            r_pipe_token,
            newline_token,
        } = node.as_fields();
        let cell_count = cells.elements().count();
        let options = self
            .options
            .as_ref()
            .filter(|options| options.widths.len() >= cell_count);

        for prefix in quote_prefixes.iter() {
            write!(
                f,
                [prefix.format().with_options(FormatMdQuotePrefixOptions {
                    should_remove: options.is_some_and(|options| !options.preserve_quote_prefixes),
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
}

impl FormatRuleWithOptions<GfmTableDelimiterRow> for FormatGfmTableDelimiterRow {
    type Options = FormatGfmTableDelimiterRowOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.options = Some(options);
        self
    }
}
