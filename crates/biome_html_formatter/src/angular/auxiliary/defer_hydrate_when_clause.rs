use crate::prelude::*;
use biome_html_syntax::AngularDeferHydrateWhenClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularDeferHydrateWhenClause;
impl FormatNodeRule<AngularDeferHydrateWhenClause> for FormatAngularDeferHydrateWhenClause {
    fn fmt_fields(
        &self,
        node: &AngularDeferHydrateWhenClause,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
