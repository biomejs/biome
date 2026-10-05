use crate::markdown::lists::inline_item_list::FormatMdFormatInlineItemListOptions;
use crate::prelude::*;
use crate::shared::{TextPrintMode, TrimMode};
use biome_formatter::write;
use biome_markdown_syntax::{GfmTableCell, GfmTableCellFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGfmTableCell;
impl FormatNodeRule<GfmTableCell> for FormatGfmTableCell {
    fn fmt_fields(&self, node: &GfmTableCell, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let GfmTableCellFields { content } = node.as_fields();

        if content.iter().next().is_some() {
            write!(
                f,
                [content
                    .format()
                    .with_options(FormatMdFormatInlineItemListOptions {
                        print_mode: TextPrintMode::Trim(TrimMode::All),
                        keep_fences_in_italics: false,
                        text_context: Default::default(),
                    })]
            )?;
        }
        Ok(())
    }
}
