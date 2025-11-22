use crate::prelude::*;
use biome_css_syntax::CssAttrFallbackValue;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAttrFallbackValue;
impl FormatNodeRule<CssAttrFallbackValue> for FormatCssAttrFallbackValue {
    fn fmt_fields(&self, node: &CssAttrFallbackValue, f: &mut CssFormatter) -> FormatResult<()> {
        todo!()
        // format_verbatim_node(node.syntax()).fmt(f)
    }
}
