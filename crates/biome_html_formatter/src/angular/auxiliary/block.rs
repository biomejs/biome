use crate::prelude::*;
use biome_html_syntax::AnyAngularBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyAngularBlock;
impl FormatNodeRule<AnyAngularBlock> for FormatAnyAngularBlock {
    fn fmt_fields(&self, node: &AnyAngularBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
