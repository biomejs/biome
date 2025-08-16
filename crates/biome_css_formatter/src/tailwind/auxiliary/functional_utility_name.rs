use crate::prelude::*;
use biome_css_syntax::TwFunctionalUtilityName;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwFunctionalUtilityName;
impl FormatNodeRule<TwFunctionalUtilityName> for FormatTwFunctionalUtilityName {
    fn fmt_fields(&self, node: &TwFunctionalUtilityName, f: &mut CssFormatter) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
