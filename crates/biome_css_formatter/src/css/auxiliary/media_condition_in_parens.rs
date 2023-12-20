use crate::prelude::*;
use biome_css_syntax::CssMediaConditionInParens;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaConditionInParens;
impl FormatNodeRule<CssMediaConditionInParens> for FormatCssMediaConditionInParens {
    fn fmt_fields(
        &self,
        node: &CssMediaConditionInParens,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
