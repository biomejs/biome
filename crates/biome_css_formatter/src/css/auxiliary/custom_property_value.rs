use crate::prelude::*;
use crate::verbatim::format_css_verbatim_node;
use biome_css_syntax::CssCustomPropertyValue;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomPropertyValue;
impl FormatNodeRule<CssCustomPropertyValue> for FormatCssCustomPropertyValue {
    fn fmt_fields(&self, node: &CssCustomPropertyValue, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
