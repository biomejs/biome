use crate::prelude::*;
use biome_css_syntax::CssMediaQueryFeature;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaQueryFeature;
impl FormatNodeRule<CssMediaQueryFeature> for FormatCssMediaQueryFeature {
    fn fmt_fields(&self, node: &CssMediaQueryFeature, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
