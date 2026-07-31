use crate::prelude::*;
use biome_html_syntax::AngularLetBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularLetBlock;
impl FormatNodeRule<AngularLetBlock> for FormatAngularLetBlock {
    fn fmt_fields(&self, node: &AngularLetBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
