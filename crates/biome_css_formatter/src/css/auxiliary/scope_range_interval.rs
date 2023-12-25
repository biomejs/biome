use crate::prelude::*;
use biome_css_syntax::CssScopeRangeInterval;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssScopeRangeInterval;
impl FormatNodeRule<CssScopeRangeInterval> for FormatCssScopeRangeInterval {
    fn fmt_fields(&self, node: &CssScopeRangeInterval, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
