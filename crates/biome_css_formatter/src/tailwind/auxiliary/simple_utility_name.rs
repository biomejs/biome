use crate::prelude::*;
use biome_css_syntax::TwSimpleUtilityName;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwSimpleUtilityName;
impl FormatNodeRule<TwSimpleUtilityName> for FormatTwSimpleUtilityName {
    fn fmt_fields(&self, node: &TwSimpleUtilityName, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
