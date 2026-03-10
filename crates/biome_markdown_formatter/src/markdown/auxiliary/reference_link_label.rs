use crate::prelude::*;
use biome_markdown_syntax::MdReferenceLinkLabel;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdReferenceLinkLabel;
impl FormatNodeRule<MdReferenceLinkLabel> for FormatMdReferenceLinkLabel {
    fn fmt_fields(
        &self,
        node: &MdReferenceLinkLabel,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
