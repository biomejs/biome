use crate::prelude::*;
use biome_css_syntax::CssContainerSizeFeatureInParens;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerSizeFeatureInParens;
impl FormatNodeRule<CssContainerSizeFeatureInParens> for FormatCssContainerSizeFeatureInParens {
    fn fmt_fields(
        &self,
        node: &CssContainerSizeFeatureInParens,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
