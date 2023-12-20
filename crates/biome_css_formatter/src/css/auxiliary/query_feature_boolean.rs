use crate::prelude::*;
use biome_css_syntax::CssQueryFeatureBoolean;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssQueryFeatureBoolean;
impl FormatNodeRule<CssQueryFeatureBoolean> for FormatCssQueryFeatureBoolean {
    fn fmt_fields(&self, node: &CssQueryFeatureBoolean, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
