use crate::prelude::*;
use biome_html_syntax::AngularLoadingClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularLoadingClause;
impl FormatNodeRule<AngularLoadingClause> for FormatAngularLoadingClause {
    fn fmt_fields(&self, node: &AngularLoadingClause, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
