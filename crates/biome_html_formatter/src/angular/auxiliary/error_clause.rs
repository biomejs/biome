use crate::prelude::*;
use biome_html_syntax::AngularErrorClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularErrorClause;
impl FormatNodeRule<AngularErrorClause> for FormatAngularErrorClause {
    fn fmt_fields(&self, node: &AngularErrorClause, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
