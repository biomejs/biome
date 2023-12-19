use crate::prelude::*;
use biome_css_syntax::CssContainerStyleQueryInParens;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerStyleQueryInParens;
impl FormatNodeRule<CssContainerStyleQueryInParens> for FormatCssContainerStyleQueryInParens {
    fn fmt_fields(
        &self,
        node: &CssContainerStyleQueryInParens,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
