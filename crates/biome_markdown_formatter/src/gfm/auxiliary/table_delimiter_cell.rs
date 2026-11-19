use crate::gfm::auxiliary::table::{GfmTableLayout, MIN_GFM_TABLE_CELL_WIDTH};
use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{GfmTableDelimiterCell, GfmTableDelimiterCellFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGfmTableDelimiterCell {
    options: Option<FormatGfmTableDelimiterCellOptions>,
}

/// Column width and output policy for one delimiter cell.
#[derive(Clone, Copy, Debug)]
pub(crate) struct FormatGfmTableDelimiterCellOptions {
    /// Cell width of the column, excluding padding and pipes.
    pub(crate) width: usize,
    /// Policy controlling whether dashes beyond the minimum are conditional.
    pub(crate) layout: GfmTableLayout,
}

impl FormatNodeRule<GfmTableDelimiterCell> for FormatGfmTableDelimiterCell {
    fn fmt_fields(
        &self,
        node: &GfmTableDelimiterCell,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        let options = self.options.unwrap_or(FormatGfmTableDelimiterCellOptions {
            width: MIN_GFM_TABLE_CELL_WIDTH,
            layout: GfmTableLayout::Aligned,
        });
        let GfmTableDelimiterCellFields {
            l_colon_token,
            dashes,
            r_colon_token,
        } = node.as_fields();
        let colon_count =
            usize::from(l_colon_token.is_some()) + usize::from(r_colon_token.is_some());

        write!(f, [text(" ", None)])?;
        if let Some(colon) = l_colon_token {
            write!(f, [format_replaced(&colon, &token(":"))])?;
        }
        if f.context().comments().is_suppressed(dashes.syntax()) {
            write!(f, [format_suppressed_node(dashes.syntax())])?;
        } else if dashes
            .iter()
            .any(|dash| f.context().comments().is_suppressed(dash.syntax()))
        {
            write!(f, [dashes.format()])?;
        } else {
            let mut dashes = dashes.iter();
            if let Some(first) = dashes.next() {
                let dash_token = first.minus_token()?;
                let aligned_dashes = options.width.saturating_sub(colon_count);
                let compact_dashes = MIN_GFM_TABLE_CELL_WIDTH.saturating_sub(colon_count);
                let replacement =
                    format_with(|f| match options.layout {
                        GfmTableLayout::Aligned => {
                            for _ in 0..aligned_dashes {
                                write!(f, [token("-")])?;
                            }
                            Ok(())
                        }
                        GfmTableLayout::CompactWhenBroken(group_id) => {
                            for _ in 0..compact_dashes {
                                write!(f, [token("-")])?;
                            }
                            for _ in compact_dashes..aligned_dashes {
                                write!(
                                    f,
                                    [if_group_fits_on_line(&token("-"))
                                        .with_group_id(Some(group_id))]
                                )?;
                            }
                            Ok(())
                        }
                    });
                write!(f, [format_replaced(&dash_token, &replacement)])?;
                for dash in dashes {
                    write!(f, [format_removed(&dash.minus_token()?)])?;
                }
            } else {
                write!(f, [token("---")])?;
            }
        }
        if let Some(colon) = r_colon_token {
            write!(f, [format_replaced(&colon, &token(":"))])?;
        }
        write!(f, [text(" ", None)])
    }
}

impl FormatRuleWithOptions<GfmTableDelimiterCell> for FormatGfmTableDelimiterCell {
    type Options = FormatGfmTableDelimiterCellOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.options = Some(options);
        self
    }
}
