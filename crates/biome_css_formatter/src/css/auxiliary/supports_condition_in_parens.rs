use crate::prelude::*;
use biome_css_syntax::CssSupportsConditionInParens;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSupportsConditionInParens;
impl FormatNodeRule<CssSupportsConditionInParens> for FormatCssSupportsConditionInParens {
    fn fmt_fields(
        &self,
        node: &CssSupportsConditionInParens,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
