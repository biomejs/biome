use crate::prelude::*;
use biome_html_syntax::AngularDeferPrefetchWhenClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularDeferPrefetchWhenClause;
impl FormatNodeRule<AngularDeferPrefetchWhenClause> for FormatAngularDeferPrefetchWhenClause {
    fn fmt_fields(
        &self,
        node: &AngularDeferPrefetchWhenClause,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
