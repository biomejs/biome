use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::GfmTableCellList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGfmTableCellList;
impl FormatRule<GfmTableCellList> for FormatGfmTableCellList {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &GfmTableCellList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        for element in node.elements() {
            write!(f, [space(), element.node()?.format(), space()])?;
            if let Some(pipe) = element.trailing_separator()? {
                write!(f, [format_replaced(pipe, &token("|"))])?;
            }
        }
        Ok(())
    }
}
