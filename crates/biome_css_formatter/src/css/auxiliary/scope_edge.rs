use crate::prelude::*;
use biome_css_syntax::CssScopeEdge;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssScopeEdge;
impl FormatNodeRule<CssScopeEdge> for FormatCssScopeEdge {
    fn fmt_fields(&self, node: &CssScopeEdge, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
