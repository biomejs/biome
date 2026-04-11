use crate::prelude::*;
use biome_markdown_syntax::MdContinuationIndent;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdContinuationIndent;
impl FormatNodeRule<MdContinuationIndent> for FormatMdContinuationIndent {
    fn fmt_fields(
        &self,
        node: &MdContinuationIndent,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
