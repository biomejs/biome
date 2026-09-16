use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::GfmTableDelimiterCellList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGfmTableDelimiterCellList;
impl FormatRule<GfmTableDelimiterCellList> for FormatGfmTableDelimiterCellList {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &GfmTableDelimiterCellList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        for element in node.elements() {
            write!(f, [element.node()?.format()])?;
            if let Some(pipe) = element.trailing_separator()? {
                write!(f, [format_replaced(pipe, &token("|"))])?;
            }
        }
        Ok(())
    }
}
