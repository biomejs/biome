use crate::prelude::*;
use biome_markdown_syntax::GfmTableDelimiterDashList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGfmTableDelimiterDashList;
impl FormatRule<GfmTableDelimiterDashList> for FormatGfmTableDelimiterDashList {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &GfmTableDelimiterDashList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
