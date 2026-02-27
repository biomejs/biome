use crate::prelude::*;
use biome_markdown_syntax::MdCodeNameList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdCodeNameList;
impl FormatRule<MdCodeNameList> for FormatMdCodeNameList {
    type Context = MarkdownFormatContext;
    fn fmt(&self, node: &MdCodeNameList, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
