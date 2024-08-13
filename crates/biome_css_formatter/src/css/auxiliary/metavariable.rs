use crate::prelude::*;
use biome_css_syntax::CssMetavariable;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMetavariable;
impl FormatNodeRule<CssMetavariable> for FormatCssMetavariable {
    fn fmt_fields(&self, node: &CssMetavariable, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
