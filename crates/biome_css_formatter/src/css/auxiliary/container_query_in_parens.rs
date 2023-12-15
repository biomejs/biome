use crate::prelude::*;
use biome_css_syntax::CssContainerQueryInParens;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerQueryInParens;
impl FormatNodeRule<CssContainerQueryInParens> for FormatCssContainerQueryInParens {
    fn fmt_fields(
        &self,
        node: &CssContainerQueryInParens,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
