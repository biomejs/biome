use crate::prelude::*;
use biome_css_syntax::CssFontFeatureValuesBlock;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFontFeatureValuesBlock;
impl FormatNodeRule<CssFontFeatureValuesBlock> for FormatCssFontFeatureValuesBlock {
    fn fmt_fields(
        &self,
        node: &CssFontFeatureValuesBlock,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
