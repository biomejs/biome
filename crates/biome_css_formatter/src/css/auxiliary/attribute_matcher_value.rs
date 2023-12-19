use crate::prelude::*;
use biome_css_syntax::CssAttributeMatcherValue;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAttributeMatcherValue;
impl FormatNodeRule<CssAttributeMatcherValue> for FormatCssAttributeMatcherValue {
    fn fmt_fields(
        &self,
        node: &CssAttributeMatcherValue,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
