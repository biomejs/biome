use crate::prelude::*;
use biome_css_syntax::CssScopeRangeEnd;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssScopeRangeEnd;
impl FormatNodeRule<CssScopeRangeEnd> for FormatCssScopeRangeEnd {
    fn fmt_fields(&self, node: &CssScopeRangeEnd, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
