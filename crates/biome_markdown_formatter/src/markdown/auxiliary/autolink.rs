use crate::prelude::*;
use biome_markdown_syntax::MdAutolink;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdAutolink;
impl FormatNodeRule<MdAutolink> for FormatMdAutolink {
    fn fmt_fields(&self, node: &MdAutolink, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
