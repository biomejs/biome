use crate::prelude::*;
use biome_css_syntax::CssGenericProperty;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssGenericProperty;
impl FormatNodeRule<CssGenericProperty> for FormatCssGenericProperty {
    fn fmt_fields(&self, node: &CssGenericProperty, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
