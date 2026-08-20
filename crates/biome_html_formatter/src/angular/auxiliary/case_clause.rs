use crate::prelude::*;
use biome_html_syntax::AngularCaseClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularCaseClause;
impl FormatNodeRule<AngularCaseClause> for FormatAngularCaseClause {
    fn fmt_fields(&self, node: &AngularCaseClause, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
