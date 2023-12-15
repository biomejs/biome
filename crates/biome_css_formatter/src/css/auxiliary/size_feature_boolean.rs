use crate::prelude::*;
use biome_css_syntax::CssSizeFeatureBoolean;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSizeFeatureBoolean;
impl FormatNodeRule<CssSizeFeatureBoolean> for FormatCssSizeFeatureBoolean {
    fn fmt_fields(&self, node: &CssSizeFeatureBoolean, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
