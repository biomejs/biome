use crate::prelude::*;
use biome_html_syntax::AngularDeferPrefetchOnClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularDeferPrefetchOnClause;
impl FormatNodeRule<AngularDeferPrefetchOnClause> for FormatAngularDeferPrefetchOnClause {
    fn fmt_fields(
        &self,
        node: &AngularDeferPrefetchOnClause,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
