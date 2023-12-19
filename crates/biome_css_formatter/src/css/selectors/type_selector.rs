use crate::prelude::*;
use biome_css_syntax::CssTypeSelector;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssTypeSelector;
impl FormatNodeRule<CssTypeSelector> for FormatCssTypeSelector {
    fn fmt_fields(&self, node: &CssTypeSelector, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
