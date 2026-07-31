use crate::prelude::*;
use biome_html_syntax::AngularMinimumTimeClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularMinimumTimeClause;
impl FormatNodeRule<AngularMinimumTimeClause> for FormatAngularMinimumTimeClause {
    fn fmt_fields(
        &self,
        node: &AngularMinimumTimeClause,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
