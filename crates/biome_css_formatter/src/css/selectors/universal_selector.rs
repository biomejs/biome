use crate::prelude::*;
use biome_css_syntax::CssUniversalSelector;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUniversalSelector;
impl FormatNodeRule<CssUniversalSelector> for FormatCssUniversalSelector {
    fn fmt_fields(&self, node: &CssUniversalSelector, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
