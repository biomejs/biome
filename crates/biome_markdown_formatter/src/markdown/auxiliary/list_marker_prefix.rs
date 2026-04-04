use crate::prelude::*;
use biome_markdown_syntax::MdListMarkerPrefix;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdListMarkerPrefix;
impl FormatNodeRule<MdListMarkerPrefix> for FormatMdListMarkerPrefix {
    fn fmt_fields(&self, node: &MdListMarkerPrefix, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
