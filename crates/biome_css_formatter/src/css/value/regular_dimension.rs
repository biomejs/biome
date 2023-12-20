use crate::prelude::*;
use biome_css_syntax::CssRegularDimension;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRegularDimension;
impl FormatNodeRule<CssRegularDimension> for FormatCssRegularDimension {
    fn fmt_fields(&self, node: &CssRegularDimension, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
