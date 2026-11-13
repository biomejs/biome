use crate::prelude::*;
use biome_html_syntax::AngularDeferHydrateOnClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularDeferHydrateOnClause;
impl FormatNodeRule<AngularDeferHydrateOnClause> for FormatAngularDeferHydrateOnClause {
    fn fmt_fields(
        &self,
        node: &AngularDeferHydrateOnClause,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
