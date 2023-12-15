use crate::prelude::*;
use biome_css_syntax::CssAttributeSelector;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAttributeSelector;
impl FormatNodeRule<CssAttributeSelector> for FormatCssAttributeSelector {
    fn fmt_fields(&self, node: &CssAttributeSelector, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
