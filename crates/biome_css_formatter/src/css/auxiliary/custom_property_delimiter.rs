use crate::prelude::*;
use crate::verbatim::format_css_verbatim_node;
use biome_css_syntax::CssCustomPropertyDelimiter;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomPropertyDelimiter;
impl FormatNodeRule<CssCustomPropertyDelimiter> for FormatCssCustomPropertyDelimiter {
    fn fmt_fields(
        &self,
        node: &CssCustomPropertyDelimiter,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
