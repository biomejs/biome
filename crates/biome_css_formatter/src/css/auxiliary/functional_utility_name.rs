use crate::prelude::*;
use biome_css_syntax::CssFunctionalUtilityName;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFunctionalUtilityName;
impl FormatNodeRule<CssFunctionalUtilityName> for FormatCssFunctionalUtilityName {
    fn fmt_fields(
        &self,
        node: &CssFunctionalUtilityName,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
