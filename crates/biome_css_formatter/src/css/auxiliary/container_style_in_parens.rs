use crate::prelude::*;
use biome_css_syntax::CssContainerStyleInParens;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerStyleInParens;
impl FormatNodeRule<CssContainerStyleInParens> for FormatCssContainerStyleInParens {
    fn fmt_fields(
        &self,
        node: &CssContainerStyleInParens,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
