use crate::prelude::*;
use biome_css_syntax::CssMediaType;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaType;
impl FormatNodeRule<CssMediaType> for FormatCssMediaType {
    fn fmt_fields(&self, node: &CssMediaType, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
