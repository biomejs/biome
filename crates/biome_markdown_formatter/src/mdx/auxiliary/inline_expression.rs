use crate::prelude::*;
use biome_markdown_syntax::MdxInlineExpression;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdxInlineExpression;
impl FormatNodeRule<MdxInlineExpression> for FormatMdxInlineExpression {
    fn fmt_fields(
        &self,
        node: &MdxInlineExpression,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
