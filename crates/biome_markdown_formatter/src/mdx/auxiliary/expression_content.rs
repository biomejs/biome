use crate::prelude::*;
use biome_markdown_syntax::MdxExpressionContent;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdxExpressionContent;
impl FormatNodeRule<MdxExpressionContent> for FormatMdxExpressionContent {
    fn fmt_fields(
        &self,
        node: &MdxExpressionContent,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
