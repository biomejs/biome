use crate::prelude::*;
use biome_css_syntax::CssContainerOrQuery;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerOrQuery;
impl FormatNodeRule<CssContainerOrQuery> for FormatCssContainerOrQuery {
    fn fmt_fields(&self, node: &CssContainerOrQuery, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
