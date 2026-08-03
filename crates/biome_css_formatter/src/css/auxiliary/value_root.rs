use crate::prelude::*;
use biome_css_syntax::CssValueRoot;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssValueRoot;
impl FormatNodeRule<CssValueRoot> for FormatCssValueRoot {
    fn fmt_fields(&self, node: &CssValueRoot, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
