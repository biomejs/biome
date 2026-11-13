use crate::prelude::*;
use biome_html_syntax::AngularDefaultClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularDefaultClause;
impl FormatNodeRule<AngularDefaultClause> for FormatAngularDefaultClause {
    fn fmt_fields(&self, node: &AngularDefaultClause, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
