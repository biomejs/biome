use crate::prelude::*;
use biome_markdown_syntax::MdLinkReferenceDefinition;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdLinkReferenceDefinition;
impl FormatNodeRule<MdLinkReferenceDefinition> for FormatMdLinkReferenceDefinition {
    fn fmt_fields(
        &self,
        node: &MdLinkReferenceDefinition,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
