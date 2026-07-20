use crate::prelude::*;
use crate::verbatim::format_css_verbatim_node;
use biome_css_syntax::CssCustomPropertyBracketedBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomPropertyBracketedBlock;
impl FormatNodeRule<CssCustomPropertyBracketedBlock> for FormatCssCustomPropertyBracketedBlock {
    fn fmt_fields(
        &self,
        node: &CssCustomPropertyBracketedBlock,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
