use crate::prelude::*;
use biome_html_syntax::AngularElseIfClause;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularElseIfClause;
impl FormatNodeRule<AngularElseIfClause> for FormatAngularElseIfClause {
    fn fmt_fields(&self, node: &AngularElseIfClause, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
