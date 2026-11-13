use crate::prelude::*;
use biome_html_syntax::AngularIfAsClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularIfAsClause;
impl FormatNodeRule<AngularIfAsClause> for FormatAngularIfAsClause {
    fn fmt_fields(&self, node: &AngularIfAsClause, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
