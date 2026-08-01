use crate::prelude::*;
use biome_html_syntax::AngularDeferHydrateNeverClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularDeferHydrateNeverClause;
impl FormatNodeRule<AngularDeferHydrateNeverClause> for FormatAngularDeferHydrateNeverClause {
    fn fmt_fields(
        &self,
        node: &AngularDeferHydrateNeverClause,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
