use crate::prelude::*;
use biome_css_syntax::CssNamespace;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssNamespace;
impl FormatNodeRule<CssNamespace> for FormatCssNamespace {
    fn fmt_fields(&self, node: &CssNamespace, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
