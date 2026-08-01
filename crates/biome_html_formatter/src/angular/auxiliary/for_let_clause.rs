use crate::prelude::*;
use biome_html_syntax::AngularForLetClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularForLetClause;
impl FormatNodeRule<AngularForLetClause> for FormatAngularForLetClause {
    fn fmt_fields(&self, node: &AngularForLetClause, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
