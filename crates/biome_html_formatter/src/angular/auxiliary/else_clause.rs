use crate::prelude::*;
use biome_html_syntax::AngularElseClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularElseClause;
impl FormatNodeRule<AngularElseClause> for FormatAngularElseClause {
    fn fmt_fields(&self, node: &AngularElseClause, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
