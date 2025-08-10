use crate::prelude::*;
use biome_css_syntax::CssTwCustomVariantShorthand;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssTwCustomVariantShorthand;
impl FormatNodeRule<CssTwCustomVariantShorthand> for FormatCssTwCustomVariantShorthand {
    fn fmt_fields(
        &self,
        node: &CssTwCustomVariantShorthand,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
