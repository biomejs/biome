use crate::prelude::*;
use biome_css_syntax::CssGritMetavariable;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssGritMetavariable;
impl FormatNodeRule<CssGritMetavariable> for FormatCssGritMetavariable {
    fn fmt_fields(&self, node: &CssGritMetavariable, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
