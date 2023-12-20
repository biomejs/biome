use crate::prelude::*;
use biome_css_syntax::CssAttributeName;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAttributeName;
impl FormatNodeRule<CssAttributeName> for FormatCssAttributeName {
    fn fmt_fields(&self, node: &CssAttributeName, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
