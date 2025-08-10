use crate::prelude::*;
use biome_css_syntax::TwCustomVariantShorthand;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwCustomVariantShorthand;
impl FormatNodeRule<TwCustomVariantShorthand> for FormatTwCustomVariantShorthand {
    fn fmt_fields(
        &self,
        node: &TwCustomVariantShorthand,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
