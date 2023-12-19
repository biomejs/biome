use crate::prelude::*;
use biome_css_syntax::CssMediaQueryFeatureBoolean;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaQueryFeatureBoolean;
impl FormatNodeRule<CssMediaQueryFeatureBoolean> for FormatCssMediaQueryFeatureBoolean {
    fn fmt_fields(
        &self,
        node: &CssMediaQueryFeatureBoolean,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
