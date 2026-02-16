use crate::prelude::*;
use biome_markdown_syntax::MdHash;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdHash;
impl FormatNodeRule<MdHash> for FormatMdHash {
    fn fmt_fields(&self, node: &MdHash, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
