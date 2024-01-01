use crate::prelude::*;
use biome_css_syntax::CssPageSelectorPseudo;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPageSelectorPseudo;
impl FormatNodeRule<CssPageSelectorPseudo> for FormatCssPageSelectorPseudo {
    fn fmt_fields(&self, node: &CssPageSelectorPseudo, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
