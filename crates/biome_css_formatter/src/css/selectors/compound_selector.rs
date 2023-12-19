use crate::prelude::*;
use biome_css_syntax::CssCompoundSelector;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCompoundSelector;
impl FormatNodeRule<CssCompoundSelector> for FormatCssCompoundSelector {
    fn fmt_fields(&self, node: &CssCompoundSelector, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
