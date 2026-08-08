use crate::prelude::*;
use biome_html_syntax::AngularDeferWhenClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularDeferWhenClause;
impl FormatNodeRule<AngularDeferWhenClause> for FormatAngularDeferWhenClause {
    fn fmt_fields(&self, node: &AngularDeferWhenClause, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
