use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::GfmTableRowList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGfmTableRowList;
impl FormatRule<GfmTableRowList> for FormatGfmTableRowList {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &GfmTableRowList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        for (index, row) in node.iter().enumerate() {
            if index > 0 {
                write!(f, [hard_line_break()])?;
            }
            write!(f, [row.format()])?;
        }
        Ok(())
    }
}
