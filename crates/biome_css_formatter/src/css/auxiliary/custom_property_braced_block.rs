use crate::prelude::*;
use crate::verbatim::format_css_verbatim_node;
use biome_css_syntax::CssCustomPropertyBracedBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomPropertyBracedBlock;
impl FormatNodeRule<CssCustomPropertyBracedBlock> for FormatCssCustomPropertyBracedBlock {
    fn fmt_fields(
        &self,
        node: &CssCustomPropertyBracedBlock,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
