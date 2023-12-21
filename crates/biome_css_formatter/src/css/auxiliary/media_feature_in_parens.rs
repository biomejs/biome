use crate::prelude::*;
use biome_css_syntax::CssMediaFeatureInParens;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaFeatureInParens;
impl FormatNodeRule<CssMediaFeatureInParens> for FormatCssMediaFeatureInParens {
    fn fmt_fields(&self, node: &CssMediaFeatureInParens, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
