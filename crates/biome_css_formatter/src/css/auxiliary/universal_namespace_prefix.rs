use crate::prelude::*;
use biome_css_syntax::CssUniversalNamespacePrefix;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUniversalNamespacePrefix;
impl FormatNodeRule<CssUniversalNamespacePrefix> for FormatCssUniversalNamespacePrefix {
    fn fmt_fields(
        &self,
        node: &CssUniversalNamespacePrefix,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
