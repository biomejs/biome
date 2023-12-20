use crate::prelude::*;
use biome_css_syntax::CssNamedNamespacePrefix;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssNamedNamespacePrefix;
impl FormatNodeRule<CssNamedNamespacePrefix> for FormatCssNamedNamespacePrefix {
    fn fmt_fields(&self, node: &CssNamedNamespacePrefix, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
