use crate::prelude::*;
use biome_css_syntax::CssParameter;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssParameter;
impl FormatNodeRule<CssParameter> for FormatCssParameter {
    fn fmt_fields(&self, node: &CssParameter, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
