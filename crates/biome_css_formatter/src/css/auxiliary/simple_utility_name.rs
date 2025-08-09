use crate::prelude::*;
use biome_css_syntax::CssSimpleUtilityName;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSimpleUtilityName;
impl FormatNodeRule<CssSimpleUtilityName> for FormatCssSimpleUtilityName {
    fn fmt_fields(&self, node: &CssSimpleUtilityName, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
