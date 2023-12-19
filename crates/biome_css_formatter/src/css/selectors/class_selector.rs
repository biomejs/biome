use crate::prelude::*;
use biome_css_syntax::CssClassSelector;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssClassSelector;
impl FormatNodeRule<CssClassSelector> for FormatCssClassSelector {
    fn fmt_fields(&self, node: &CssClassSelector, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
