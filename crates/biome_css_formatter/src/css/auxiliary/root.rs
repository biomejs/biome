use crate::prelude::*;
use biome_css_syntax::CssRoot;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRoot;
impl FormatNodeRule<CssRoot> for FormatCssRoot {
    fn fmt_fields(&self, node: &CssRoot, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
