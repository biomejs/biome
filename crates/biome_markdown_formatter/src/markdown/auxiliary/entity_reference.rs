use crate::prelude::*;
use biome_markdown_syntax::MdEntityReference;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdEntityReference;
impl FormatNodeRule<MdEntityReference> for FormatMdEntityReference {
    fn fmt_fields(&self, node: &MdEntityReference, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
