use crate::prelude::*;
use biome_markdown_syntax::MdxBlockExpression;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdxBlockExpression;
impl FormatNodeRule<MdxBlockExpression> for FormatMdxBlockExpression {
    fn fmt_fields(&self, node: &MdxBlockExpression, f: &mut MarkdownFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
