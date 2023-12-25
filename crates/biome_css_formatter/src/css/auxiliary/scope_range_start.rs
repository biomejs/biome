use crate::prelude::*;
use biome_css_syntax::CssScopeRangeStart;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssScopeRangeStart;
impl FormatNodeRule<CssScopeRangeStart> for FormatCssScopeRangeStart {
    fn fmt_fields(&self, node: &CssScopeRangeStart, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
