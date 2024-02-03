use crate::prelude::*;
use biome_css_syntax::CssFontFeatureValuesItem;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFontFeatureValuesItem;
impl FormatNodeRule<CssFontFeatureValuesItem> for FormatCssFontFeatureValuesItem {
    fn fmt_fields(
        &self,
        node: &CssFontFeatureValuesItem,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
