use crate::prelude::*;
use biome_css_syntax::CssPseudoClassSelector;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassSelector;
impl FormatNodeRule<CssPseudoClassSelector> for FormatCssPseudoClassSelector {
    fn fmt_fields(&self, node: &CssPseudoClassSelector, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
